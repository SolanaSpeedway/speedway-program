use speedway_api::prelude::*;
use solana_program::log::sol_log;
use spl_token::amount_to_ui_amount;
use steel::*;

/// Boost: Compound accrued Garage rewards back into total_deposited.
///
/// This increases the user's max_payout potential (365% of total_deposited).
/// Subject to a 5% boost tax which goes to the treasury garage_pool.
///
/// Rewards are calculated based on:
/// - 1% daily yield on total_deposited
/// - Capped at remaining payout (max_payout - total_claimed)
pub fn process_boost(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let clock = Clock::get()?;
    let [signer_info, garage_info, treasury_info, board_info, ore_program] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate signer.
    signer_info.is_signer()?;

    // Validate garage PDA.
    let garage = garage_info
        .is_writable()?
        .has_seeds(&[GARAGE, &signer_info.key.to_bytes()], &speedway_api::ID)?
        .as_account_mut::<Garage>(&speedway_api::ID)?
        .assert_mut(|g| g.authority == *signer_info.key)?;

    // Validate treasury.
    let treasury = treasury_info
        .is_writable()?
        .as_account_mut::<Treasury>(&speedway_api::ID)?;

    // Validate board (for logging).
    board_info.as_account::<Board>(&speedway_api::ID)?;

    // Validate program.
    ore_program.is_program(&speedway_api::ID)?;

    // Check if garage is exhausted.
    if garage.is_exhausted() {
        return Err(OreError::GarageExhausted.into());
    }

    // Calculate available rewards.
    let available = garage.calculate_available(clock.unix_timestamp);
    if available == 0 {
        return Err(OreError::NoRewardsAvailable.into());
    }

    // Calculate boost tax (5% of available rewards).
    let tax = available
        .checked_mul(BOOST_TAX_BPS)
        .and_then(|v| v.checked_div(DENOMINATOR_BPS))
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Net amount to add to deposits.
    let net_amount = available
        .checked_sub(tax)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Update garage.
    // Add net amount to total_deposited (this increases max_payout).
    garage.total_deposited = garage
        .total_deposited
        .checked_add(net_amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Mark rewards as claimed (they were converted to deposits).
    garage.total_claimed = garage
        .total_claimed
        .checked_add(available)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Update last action timestamp.
    garage.last_action_at = clock.unix_timestamp;

    // Recalculate max_payout with new total_deposited.
    garage.update_max_payout();

    // Add tax to treasury garage pool.
    treasury.garage_pool = treasury
        .garage_pool
        .checked_add(tax)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Update treasury total garage TVL (net amount was added to deposits).
    treasury.total_garage_tvl = treasury
        .total_garage_tvl
        .checked_add(net_amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Emit event.
    program_log(
        &[board_info.clone(), ore_program.clone()],
        BoostEvent {
            disc: OreEvent::Boost as u64,
            authority: *signer_info.key,
            gross_amount: available,
            net_amount,
            tax,
            new_total_deposited: garage.total_deposited,
            new_max_payout: garage.max_payout,
            ts: clock.unix_timestamp,
        }
        .to_bytes(),
    )?;

    sol_log(&format!(
        "🚀 Boost: {} FUEL compounded (net: {}, tax: {})",
        amount_to_ui_amount(available, TOKEN_DECIMALS),
        amount_to_ui_amount(net_amount, TOKEN_DECIMALS),
        amount_to_ui_amount(tax, TOKEN_DECIMALS),
    ));

    Ok(())
}
