use speedway_api::prelude::*;
use solana_program::log::sol_log;
use spl_token::amount_to_ui_amount;
use steel::*;

/// Collect: Withdraw accrued Garage rewards to user's wallet.
///
/// Tax structure:
/// 1. Base collect tax: 10% of gross amount → goes to garage_pool
/// 2. Whale tax (applied AFTER base tax): 0-50% based on user's TVL %
///    - <1% of TVL: 0% whale tax
///    - >=1% of TVL: 5% whale tax
///    - >=2% of TVL: 10% whale tax
///    - ... (5% increment per 1% TVL tier)
///    - >=10% of TVL: 50% whale tax (max)
///    - Whale tax distribution: 10% to team, 90% to pool
///
/// Rewards are paid from the treasury garage_pool.
/// If the pool is insufficient, FUEL is minted as a backstop.
///
/// Rewards are calculated based on:
/// - 1% daily yield on total_deposited
/// - Capped at remaining payout (max_payout - total_claimed)
pub fn process_collect(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let clock = Clock::get()?;
    let [signer_info, garage_info, treasury_info, treasury_tokens_info, recipient_info, mint_info, team_tokens_info, board_info, system_program, token_program, associated_token_program, ore_program] =
        accounts
    else {
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
        .has_seeds(&[TREASURY], &speedway_api::ID)?
        .as_account_mut::<Treasury>(&speedway_api::ID)?;

    // Validate treasury token account.
    treasury_tokens_info
        .is_writable()?
        .as_associated_token_account(treasury_info.key, &MINT_ADDRESS)?;

    // Validate mint.
    mint_info.has_address(&MINT_ADDRESS)?.is_writable()?;

    // Validate team token account (for whale tax team portion).
    team_tokens_info
        .is_writable()?
        .as_associated_token_account(&ADMIN_FEE_COLLECTOR, &MINT_ADDRESS)?;

    // Validate board (for logging).
    board_info.as_account::<Board>(&speedway_api::ID)?;

    // Validate programs.
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;
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

    // Calculate base collect tax (10% of available rewards).
    let base_tax = available
        .checked_mul(COLLECT_TAX_BPS)
        .and_then(|v| v.checked_div(DENOMINATOR_BPS))
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Amount after base tax (used as base for whale tax calculation).
    let after_base_tax = available
        .checked_sub(base_tax)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Calculate whale tax based on user's TVL percentage.
    // Whale tax is applied AFTER the base 10% collect tax.
    let whale_tax_bps = calculate_whale_tax_bps(garage.total_deposited, treasury.total_garage_tvl);
    let whale_tax = after_base_tax
        .checked_mul(whale_tax_bps)
        .and_then(|v| v.checked_div(DENOMINATOR_BPS))
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Split whale tax: 10% to team, 90% to pool.
    let whale_tax_team = whale_tax
        .checked_mul(WHALE_TAX_TEAM_BPS)
        .and_then(|v| v.checked_div(DENOMINATOR_BPS))
        .ok_or(ProgramError::ArithmeticOverflow)?;

    let whale_tax_pool = whale_tax
        .checked_sub(whale_tax_team)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Net amount to transfer to user (after both taxes).
    let net_amount = after_base_tax
        .checked_sub(whale_tax)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Update garage.
    garage.total_claimed = garage
        .total_claimed
        .checked_add(available)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    garage.last_action_at = clock.unix_timestamp;

    // Check if now exhausted.
    let is_exhausted = garage.is_exhausted();

    // Add base tax + whale tax pool portion to treasury garage pool.
    let total_pool_fee = base_tax
        .checked_add(whale_tax_pool)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    treasury.garage_pool = treasury
        .garage_pool
        .checked_add(total_pool_fee)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Create recipient token account if needed.
    if recipient_info.data_is_empty() {
        create_associated_token_account(
            signer_info,
            signer_info,
            recipient_info,
            mint_info,
            system_program,
            token_program,
            associated_token_program,
        )?;
    } else {
        recipient_info
            .is_writable()?
            .as_associated_token_account(signer_info.key, &MINT_ADDRESS)?;
    }

    // Transfer FUEL from treasury to user.
    // First try from treasury token account, then mint if needed.
    let treasury_tokens = treasury_tokens_info.as_token_account()?;
    if treasury_tokens.amount() >= net_amount {
        // Transfer from treasury token account.
        transfer_signed(
            treasury_info,
            treasury_tokens_info,
            recipient_info,
            token_program,
            net_amount,
            &[TREASURY],
        )?;
        sol_log(&format!(
            "💸 Transferred {} FUEL from treasury pool",
            amount_to_ui_amount(net_amount, TOKEN_DECIMALS)
        ));
    } else {
        // Insufficient pool - mint new FUEL as backstop.
        // This should be rare and indicates high demand.
        let from_pool = treasury_tokens.amount();
        let to_mint = net_amount
            .checked_sub(from_pool)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        // Transfer available from pool.
        if from_pool > 0 {
            transfer_signed(
                treasury_info,
                treasury_tokens_info,
                recipient_info,
                token_program,
                from_pool,
                &[TREASURY],
            )?;
        }

        // Mint the remainder.
        mint_to_signed(
            mint_info,
            recipient_info,
            treasury_info,
            token_program,
            to_mint,
            &[TREASURY],
        )?;

        sol_log(&format!(
            "⚠️ Minted {} FUEL as backstop (pool had {})",
            amount_to_ui_amount(to_mint, TOKEN_DECIMALS),
            amount_to_ui_amount(from_pool, TOKEN_DECIMALS),
        ));
    }

    // Mint whale tax team portion to team token account (if any whale tax).
    if whale_tax_team > 0 {
        mint_to_signed(
            mint_info,
            team_tokens_info,
            treasury_info,
            token_program,
            whale_tax_team,
            &[TREASURY],
        )?;
    }

    // Emit event.
    program_log(
        &[board_info.clone(), ore_program.clone()],
        CollectEvent {
            disc: OreEvent::Collect as u64,
            authority: *signer_info.key,
            gross_amount: available,
            net_amount,
            base_tax,
            whale_tax,
            whale_tax_team,
            whale_tax_pool,
            new_total_claimed: garage.total_claimed,
            is_exhausted: if is_exhausted { 1 } else { 0 },
            _padding: [0; 7],
            ts: clock.unix_timestamp,
        }
        .to_bytes(),
    )?;

    sol_log(&format!(
        "💰 Collect: {} FUEL withdrawn (net: {}, base_tax: {}, whale_tax: {})",
        amount_to_ui_amount(available, TOKEN_DECIMALS),
        amount_to_ui_amount(net_amount, TOKEN_DECIMALS),
        amount_to_ui_amount(base_tax, TOKEN_DECIMALS),
        amount_to_ui_amount(whale_tax, TOKEN_DECIMALS),
    ));

    if is_exhausted {
        sol_log("🏁 Garage exhausted - max payout reached");
    }

    Ok(())
}
