use speedway_api::prelude::*;
use solana_program::log::sol_log;
use spl_token::amount_to_ui_amount;
use steel::*;

/// Stash: Send Sprint FUEL rewards directly to Garage.
///
/// This is the frictionless path from Sprint rewards to Garage:
/// - 0% fee (no tax on stashing)
/// - Takes unclaimed FUEL rewards from Miner account (rewards_ore + refined_ore)
/// - Deposits directly into user's Garage (increases total_deposited)
/// - Updates max_payout = total_deposited * 365 / 100
/// - User must have an existing Garage account (use FuelUp first)
///
/// Users are incentivized to Stash rather than ClaimORE + FuelUp because:
/// - ClaimORE has 10% fee to other miners
/// - FuelUp has 10% tax (2% team + 5% ref + 3% pool)
/// - Stash has 0% fee - full amount goes to Garage
pub fn process_stash(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let clock = Clock::get()?;
    let [signer_info, miner_info, garage_info, treasury_info, board_info, ore_program] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate signer.
    signer_info.is_signer()?;

    // Validate miner PDA.
    let miner = miner_info
        .is_writable()?
        .has_seeds(&[MINER, &signer_info.key.to_bytes()], &speedway_api::ID)?
        .as_account_mut::<Miner>(&speedway_api::ID)?
        .assert_mut(|m| m.authority == *signer_info.key)?;

    // Validate garage PDA exists (user must have existing Garage account).
    if garage_info.data_is_empty() {
        return Err(OreError::GarageRequired.into());
    }
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

    // Update miner rewards before calculating available amount.
    miner.update_rewards(treasury);

    // Calculate total available FUEL rewards.
    let rewards_ore = miner.rewards_ore;
    let refined_ore = miner.refined_ore;
    let amount = rewards_ore
        .checked_add(refined_ore)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    if amount == 0 {
        return Err(OreError::NoRewardsAvailable.into());
    }

    // Clear miner rewards (they're being stashed).
    miner.rewards_ore = 0;
    miner.refined_ore = 0;
    miner.last_claim_ore_at = clock.unix_timestamp;

    // Update treasury totals.
    treasury.total_unclaimed = treasury.total_unclaimed.saturating_sub(rewards_ore);
    treasury.total_refined = treasury.total_refined.saturating_sub(refined_ore);

    // Update garage deposits (0% fee - full amount goes to deposits).
    garage.total_deposited = garage
        .total_deposited
        .checked_add(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;
    garage.last_action_at = clock.unix_timestamp;
    garage.update_max_payout();

    // Update treasury total garage TVL.
    treasury.total_garage_tvl = treasury
        .total_garage_tvl
        .checked_add(amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Emit event.
    program_log(
        &[board_info.clone(), ore_program.clone()],
        StashEvent {
            disc: OreEvent::Stash as u64,
            authority: *signer_info.key,
            amount,
            new_total_deposited: garage.total_deposited,
            new_max_payout: garage.max_payout,
            ts: clock.unix_timestamp,
        }
        .to_bytes(),
    )?;

    sol_log(&format!(
        "🏦 Stash: {} FUEL from Sprint → Garage (0% fee)",
        amount_to_ui_amount(amount, TOKEN_DECIMALS),
    ));

    Ok(())
}
