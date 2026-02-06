use speedway_api::prelude::*;
use solana_program::log::sol_log;
use spl_token::amount_to_ui_amount;
use steel::*;

/// FuelUp: Deposit FUEL tokens into the Garage faucet system.
///
/// Fee breakdown (10% total):
/// - 2% → Team fee collector
/// - 5% → Referrer (credited to referrer's Garage)
/// - 3% → Pool (goes to Treasury garage_pool)
///
/// The remaining 90% is credited to the user's Garage as total_deposited.
/// Tokens are burned (not transferred) - principal is never withdrawable.
///
/// First deposit requires a valid referrer who has an existing Garage account.
pub fn process_fuel_up(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Parse data.
    let args = FuelUp::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    // Load accounts.
    let clock = Clock::get()?;
    let [signer_info, payer_info, mint_info, sender_info, garage_info, referrer_info, referrer_garage_info, treasury_info, team_fee_collector_info, board_info, system_program, token_program, ore_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate signers.
    signer_info.is_signer()?;
    payer_info.is_signer()?;

    // Validate mint is FUEL.
    mint_info.has_address(&MINT_ADDRESS)?.is_writable()?;

    // Validate sender token account.
    let sender = sender_info
        .is_writable()?
        .as_associated_token_account(&signer_info.key, &MINT_ADDRESS)?;

    // Validate garage PDA.
    garage_info
        .is_writable()?
        .has_seeds(&[GARAGE, &signer_info.key.to_bytes()], &speedway_api::ID)?;

    // Validate referrer is not self.
    if referrer_info.key == signer_info.key {
        return Err(OreError::InvalidReferrer.into());
    }

    // Validate treasury.
    let treasury = treasury_info
        .is_writable()?
        .as_account_mut::<Treasury>(&speedway_api::ID)?;

    // Validate team fee collector.
    team_fee_collector_info
        .is_writable()?
        .has_address(&ADMIN_FEE_COLLECTOR)?;

    // Validate board (for logging).
    board_info.as_account::<Board>(&speedway_api::ID)?;

    // Validate programs.
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;
    ore_program.is_program(&speedway_api::ID)?;

    // Validate minimum deposit.
    if amount < MIN_DEPOSIT {
        return Err(OreError::DepositBelowMinimum.into());
    }

    // Validate sender has enough tokens.
    if sender.amount() < amount {
        return Err(ProgramError::InsufficientFunds);
    }

    // Calculate fees using checked arithmetic.
    let team_fee = amount
        .checked_mul(FUEL_UP_TEAM_BPS)
        .and_then(|v| v.checked_div(DENOMINATOR_BPS))
        .ok_or(ProgramError::ArithmeticOverflow)?;

    let ref_fee = amount
        .checked_mul(FUEL_UP_REF_BPS)
        .and_then(|v| v.checked_div(DENOMINATOR_BPS))
        .ok_or(ProgramError::ArithmeticOverflow)?;

    let pool_fee = amount
        .checked_mul(FUEL_UP_POOL_BPS)
        .and_then(|v| v.checked_div(DENOMINATOR_BPS))
        .ok_or(ProgramError::ArithmeticOverflow)?;

    let total_tax = team_fee
        .checked_add(ref_fee)
        .and_then(|v| v.checked_add(pool_fee))
        .ok_or(ProgramError::ArithmeticOverflow)?;

    let net_amount = amount
        .checked_sub(total_tax)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Open or update garage account.
    let garage = if garage_info.data_is_empty() {
        // First deposit - referrer is mandatory.
        // Validate referrer has an existing Garage account.
        if referrer_garage_info.data_is_empty() {
            return Err(OreError::ReferrerNoGarage.into());
        }
        referrer_garage_info
            .is_writable()?
            .has_seeds(&[GARAGE, &referrer_info.key.to_bytes()], &speedway_api::ID)?
            .as_account_mut::<Garage>(&speedway_api::ID)?
            .assert_mut(|g| g.authority == *referrer_info.key)?;

        // Create new Garage account.
        create_program_account::<Garage>(
            garage_info,
            system_program,
            payer_info,
            &speedway_api::ID,
            &[GARAGE, &signer_info.key.to_bytes()],
        )?;

        let garage = garage_info.as_account_mut::<Garage>(&speedway_api::ID)?;
        garage.authority = *signer_info.key;
        garage.referrer = *referrer_info.key;
        garage.total_deposited = 0;
        garage.total_claimed = 0;
        garage.max_payout = 0;
        garage.last_action_at = clock.unix_timestamp;
        garage.created_at = clock.unix_timestamp;
        garage.direct_referrals = 0;
        garage._padding = 0;
        garage.lifetime_ref_earnings = 0;

        // Increment referrer's direct_referrals count.
        let referrer_garage = referrer_garage_info.as_account_mut::<Garage>(&speedway_api::ID)?;
        referrer_garage.direct_referrals = referrer_garage
            .direct_referrals
            .checked_add(1)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        garage
    } else {
        // Existing garage - validate and update.
        garage_info
            .as_account_mut::<Garage>(&speedway_api::ID)?
            .assert_mut(|g| g.authority == *signer_info.key)?
    };

    // Check if garage is exhausted.
    if garage.is_exhausted() {
        return Err(OreError::GarageExhausted.into());
    }

    // Update garage deposits.
    garage.total_deposited = garage
        .total_deposited
        .checked_add(net_amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    garage.last_action_at = clock.unix_timestamp;
    garage.update_max_payout();

    // Credit referral fee to referrer's Garage.
    // Only if referrer_garage_info is not empty (subsequent deposits may not have it).
    if !referrer_garage_info.data_is_empty() {
        let referrer_garage = referrer_garage_info.as_account_mut::<Garage>(&speedway_api::ID)?;
        referrer_garage.total_deposited = referrer_garage
            .total_deposited
            .checked_add(ref_fee)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        referrer_garage.lifetime_ref_earnings = referrer_garage
            .lifetime_ref_earnings
            .checked_add(ref_fee)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        referrer_garage.update_max_payout();
    }

    // Update treasury garage pool.
    treasury.garage_pool = treasury
        .garage_pool
        .checked_add(pool_fee)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Update treasury total garage TVL.
    treasury.total_garage_tvl = treasury
        .total_garage_tvl
        .checked_add(net_amount)
        .and_then(|v| v.checked_add(ref_fee))
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Burn ALL deposited tokens (principal is never withdrawable).
    burn(
        signer_info,
        sender_info,
        mint_info,
        token_program,
        amount,
    )?;

    sol_log(&format!(
        "🔥 Burned {} FUEL (deposit)",
        amount_to_ui_amount(amount, TOKEN_DECIMALS)
    ));

    // Transfer team fee in SOL to fee collector.
    // Note: Team fee is taken from the burned tokens conceptually.
    // The actual SOL transfer for team fee would need to come from elsewhere,
    // or we mint FUEL for the team. For now, we just track it.
    // TODO: Implement team fee distribution mechanism.

    // Emit event.
    program_log(
        &[board_info.clone(), ore_program.clone()],
        FuelUpEvent {
            disc: OreEvent::FuelUp as u64,
            authority: *signer_info.key,
            gross_amount: amount,
            net_amount,
            team_fee,
            ref_fee,
            pool_fee,
            referrer: garage.referrer,
            new_total_deposited: garage.total_deposited,
            new_max_payout: garage.max_payout,
            ts: clock.unix_timestamp,
        }
        .to_bytes(),
    )?;

    sol_log(&format!(
        "⛽ Fuel Up: {} FUEL deposited (net: {}, team: {}, ref: {}, pool: {})",
        amount_to_ui_amount(amount, TOKEN_DECIMALS),
        amount_to_ui_amount(net_amount, TOKEN_DECIMALS),
        amount_to_ui_amount(team_fee, TOKEN_DECIMALS),
        amount_to_ui_amount(ref_fee, TOKEN_DECIMALS),
        amount_to_ui_amount(pool_fee, TOKEN_DECIMALS),
    ));

    Ok(())
}
