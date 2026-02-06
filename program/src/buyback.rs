use speedway_api::prelude::*;
use solana_program::log::sol_log;
use solana_program::native_token::lamports_to_sol;
use spl_token::amount_to_ui_amount;
use steel::*;

/// Swap vaulted SOL to FUEL, and burn 100% of the FUEL.
///
/// This is the buyback-and-burn mechanism for Solana Speedway:
/// - 9% of Sprint wagered SOL is collected for buyback
/// - SOL is swapped to FUEL via Jupiter
/// - 100% of acquired FUEL is burned (reducing circulating supply)
///
/// Note: Unlike the old ORE staking system, there is no staker share.
/// All acquired FUEL goes directly to burn to maximize deflation.
pub fn process_buyback(accounts: &[AccountInfo<'_>], data: &[u8]) -> ProgramResult {
    // Load accounts.
    let (ore_accounts, swap_accounts) = accounts.split_at(9);
    let [signer_info, board_info, _config_info, mint_info, treasury_info, treasury_fuel_info, treasury_sol_info, token_program, ore_program] =
        ore_accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?.has_address(&BURY_AUTHORITY)?;
    board_info.as_account_mut::<Board>(&speedway_api::ID)?;
    let fuel_mint = mint_info.has_address(&MINT_ADDRESS)?.as_mint()?;
    let _treasury = treasury_info.as_account_mut::<Treasury>(&speedway_api::ID)?;
    let treasury_fuel =
        treasury_fuel_info.as_associated_token_account(treasury_info.key, &MINT_ADDRESS)?;
    treasury_sol_info.as_associated_token_account(treasury_info.key, &SOL_MINT)?;
    token_program.is_program(&spl_token::ID)?;
    ore_program.is_program(&speedway_api::ID)?;

    // Sync native token balance.
    sync_native(treasury_sol_info)?;

    // Record pre-swap balances.
    let treasury_sol =
        treasury_sol_info.as_associated_token_account(treasury_info.key, &SOL_MINT)?;
    let pre_swap_fuel_balance = treasury_fuel.amount();
    let pre_swap_sol_balance = treasury_sol.amount();
    assert!(pre_swap_sol_balance > 0);

    // Record pre-swap mint supply.
    let pre_swap_mint_supply = fuel_mint.supply();

    // Record pre-swap treasury lamports.
    let pre_swap_treasury_lamports = treasury_info.lamports();

    // Build swap accounts.
    let accounts: Vec<AccountMeta> = swap_accounts
        .iter()
        .map(|acc| {
            let is_signer = acc.key == treasury_info.key;
            AccountMeta {
                pubkey: *acc.key,
                is_signer,
                is_writable: acc.is_writable,
            }
        })
        .collect();

    // Build swap accounts infos.
    let accounts_infos: Vec<AccountInfo> = swap_accounts
        .iter()
        .map(|acc| AccountInfo { ..acc.clone() })
        .collect();

    // Invoke swap program.
    invoke_signed(
        &Instruction {
            program_id: SWAP_PROGRAM,
            accounts,
            data: data.to_vec(),
        },
        &accounts_infos,
        &speedway_api::ID,
        &[TREASURY],
    )?;

    // Record post-swap treasury lamports.
    let post_swap_treasury_lamports = treasury_info.lamports();
    assert_eq!(
        post_swap_treasury_lamports, pre_swap_treasury_lamports,
        "Treasury lamports changed during swap: {} -> {}",
        pre_swap_treasury_lamports, post_swap_treasury_lamports
    );

    // Record post-swap mint supply.
    let post_swap_mint_supply = mint_info.as_mint()?.supply();
    assert_eq!(
        post_swap_mint_supply, pre_swap_mint_supply,
        "Mint supply changed during swap: {} -> {}",
        pre_swap_mint_supply, post_swap_mint_supply
    );

    // Record post-swap balances.
    let treasury_fuel =
        treasury_fuel_info.as_associated_token_account(treasury_info.key, &MINT_ADDRESS)?;
    let treasury_sol =
        treasury_sol_info.as_associated_token_account(treasury_info.key, &SOL_MINT)?;
    let post_swap_fuel_balance = treasury_fuel.amount();
    let post_swap_sol_balance = treasury_sol.amount();
    let total_fuel = post_swap_fuel_balance - pre_swap_fuel_balance;
    assert_eq!(post_swap_sol_balance, 0);
    assert!(post_swap_fuel_balance >= pre_swap_fuel_balance);
    sol_log(
        &format!(
            "📈 Swapped {} SOL into {} FUEL",
            lamports_to_sol(pre_swap_sol_balance),
            amount_to_ui_amount(total_fuel, TOKEN_DECIMALS),
        )
        .as_str(),
    );

    // Burn 100% of acquired FUEL (no staker share - staking was removed).
    let burn_amount = total_fuel;
    burn_signed(
        treasury_fuel_info,
        mint_info,
        treasury_info,
        token_program,
        burn_amount,
        &[TREASURY],
    )?;

    sol_log(
        &format!(
            "🔥 Burned {} FUEL (100% of buyback)",
            amount_to_ui_amount(burn_amount, TOKEN_DECIMALS)
        )
        .as_str(),
    );

    // Emit event.
    let mint = mint_info.as_mint()?;
    program_log(
        &[board_info.clone(), ore_program.clone()],
        BuryEvent {
            disc: OreEvent::Bury as u64,
            ore_buried: burn_amount,
            ore_shared: 0, // No staker share - 100% burned
            sol_amount: pre_swap_sol_balance,
            new_circulating_supply: mint.supply(),
            ts: Clock::get()?.unix_timestamp,
        }
        .to_bytes(),
    )?;

    Ok(())
}
