use speedway_api::prelude::*;
use solana_program::log::sol_log;
use spl_token::amount_to_ui_amount;
use steel::*;

/// ClaimWallet: Claim Sprint FUEL rewards directly to wallet with 20% haircut.
///
/// Fee breakdown:
/// - 20% total haircut (WALLET_HAIRCUT_BPS = 2000)
/// - 75% of haircut burned (15% of total)
/// - 25% of haircut to team (5% of total)
/// - User receives 80% of rewards
///
/// This is the "impatient" path for users who want immediate liquidity
/// rather than compounding through Garage (Stash = 0% fee).
pub fn process_claim_wallet(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let clock = Clock::get()?;
    let [signer_info, miner_info, treasury_info, mint_info, recipient_info, team_tokens_info, board_info, system_program, token_program, associated_token_program, ore_program] =
        accounts
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

    // Validate treasury.
    let treasury = treasury_info
        .is_writable()?
        .has_seeds(&[TREASURY], &speedway_api::ID)?
        .as_account_mut::<Treasury>(&speedway_api::ID)?;

    // Validate mint.
    mint_info.has_address(&MINT_ADDRESS)?.is_writable()?;

    // Validate team token account.
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

    // Update miner rewards before calculating available amount.
    miner.update_rewards(treasury);

    // Calculate total available FUEL rewards.
    let rewards_ore = miner.rewards_ore;
    let refined_ore = miner.refined_ore;
    let gross_amount = rewards_ore
        .checked_add(refined_ore)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    if gross_amount == 0 {
        return Err(OreError::NoRewardsAvailable.into());
    }

    // Calculate 20% haircut.
    let haircut = gross_amount
        .checked_mul(WALLET_HAIRCUT_BPS)
        .and_then(|v| v.checked_div(DENOMINATOR_BPS))
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Split haircut: 75% burn, 25% team.
    let burn_amount = haircut
        .checked_mul(HAIRCUT_BURN_BPS)
        .and_then(|v| v.checked_div(DENOMINATOR_BPS))
        .ok_or(ProgramError::ArithmeticOverflow)?;

    let team_fee = haircut
        .checked_sub(burn_amount)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // User receives 80%.
    let net_amount = gross_amount
        .checked_sub(haircut)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Clear miner rewards.
    miner.rewards_ore = 0;
    miner.refined_ore = 0;
    miner.last_claim_ore_at = clock.unix_timestamp;

    // Update treasury totals.
    treasury.total_unclaimed = treasury.total_unclaimed.saturating_sub(rewards_ore);
    treasury.total_refined = treasury.total_refined.saturating_sub(refined_ore);

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

    // Mint net amount to user.
    mint_to_signed(
        mint_info,
        recipient_info,
        treasury_info,
        token_program,
        net_amount,
        &[TREASURY],
    )?;

    // Mint team fee to team token account.
    if team_fee > 0 {
        mint_to_signed(
            mint_info,
            team_tokens_info,
            treasury_info,
            token_program,
            team_fee,
            &[TREASURY],
        )?;
    }

    // Note: burn_amount is not minted at all (effectively burned by not minting).
    // This reduces circulating supply compared to minting and burning.

    // Emit event.
    program_log(
        &[board_info.clone(), ore_program.clone()],
        ClaimWalletEvent {
            disc: OreEvent::ClaimWallet as u64,
            authority: *signer_info.key,
            gross_amount,
            net_amount,
            burn_amount,
            team_fee,
            ts: clock.unix_timestamp,
        }
        .to_bytes(),
    )?;

    sol_log(&format!(
        "💸 ClaimWallet: {} FUEL (net: {}, burned: {}, team: {})",
        amount_to_ui_amount(gross_amount, TOKEN_DECIMALS),
        amount_to_ui_amount(net_amount, TOKEN_DECIMALS),
        amount_to_ui_amount(burn_amount, TOKEN_DECIMALS),
        amount_to_ui_amount(team_fee, TOKEN_DECIMALS),
    ));

    Ok(())
}
