use entropy_api::state::Var;
use speedway_api::prelude::*;
use solana_program::{keccak, log::sol_log};
use steel::*;

// TODO Integrate admin fee

/// Pays out the winners and block reward.
pub fn process_reset(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Load accounts.
    let clock = Clock::get()?;
    let (ore_accounts, other_accounts) = accounts.split_at(14);
    sol_log(&format!("Ore accounts: {:?}", ore_accounts.len()).to_string());
    sol_log(&format!("Other accounts: {:?}", other_accounts.len()).to_string());
    let [signer_info, board_info, _config_info, fee_collector_info, mint_info, round_info, round_next_info, top_miner_info, treasury_info, treasury_tokens_info, system_program, token_program, ore_program, slot_hashes_sysvar] =
        ore_accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    signer_info.is_signer()?;
    let board = board_info
        .as_account_mut::<Board>(&speedway_api::ID)?
        .assert_mut(|b| clock.slot >= b.end_slot + INTERMISSION_SLOTS)?;
    fee_collector_info
        .is_writable()?
        .has_address(&ADMIN_FEE_COLLECTOR)?;
    let round = round_info
        .as_account_mut::<Round>(&speedway_api::ID)?
        .assert_mut(|r| r.id == board.round_id)?;
    round_next_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&[ROUND, &(board.round_id + 1).to_le_bytes()], &speedway_api::ID)?;
    let mint = mint_info.has_address(&MINT_ADDRESS)?.as_mint()?;
    let treasury = treasury_info.as_account_mut::<Treasury>(&speedway_api::ID)?;
    treasury_tokens_info.as_associated_token_account(&treasury_info.key, &mint_info.key)?;
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;
    ore_program.is_program(&speedway_api::ID)?;
    slot_hashes_sysvar.is_sysvar(&sysvar::slot_hashes::ID)?;

    // Open next round account.
    create_program_account::<Round>(
        round_next_info,
        ore_program,
        signer_info,
        &speedway_api::ID,
        &[ROUND, &(board.round_id + 1).to_le_bytes()],
    )?;
    let round_next = round_next_info.as_account_mut::<Round>(&speedway_api::ID)?;
    round_next.id = board.round_id + 1;
    round_next.deployed = [0; 25];
    round_next.slot_hash = [0; 32];
    round_next.count = [0; 25];
    round_next.expires_at = u64::MAX; // Set to max, to indicate round is waiting for first deploy to begin.
    round_next.rent_payer = *signer_info.key;
    round_next.motherlode = 0;
    round_next.top_miner = Pubkey::default();
    round_next.top_miner_reward = 0;
    round_next.total_deployed = 0;
    round_next.total_vaulted = 0;
    round_next.total_winnings = 0;

    // Sample random variable
    let (entropy_accounts, mint_accounts) = other_accounts.split_at(2);
    sol_log(&format!("Entropy accounts: {:?}", entropy_accounts.len()).to_string());
    sol_log(&format!("Mint accounts: {:?}", mint_accounts.len()).to_string());
    let [var_info, entropy_program] = entropy_accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    let var = var_info
        .has_address(&VAR_ADDRESS)?
        .as_account::<Var>(&entropy_api::ID)?
        .assert(|v| v.authority == *board_info.key)?
        .assert(|v| v.slot_hash != [0; 32])?
        .assert(|v| v.seed != [0; 32])?
        .assert(|v| v.value != [0; 32])?;
    entropy_program.is_program(&entropy_api::ID)?;

    // Print the seed and slot hash.
    let seed = keccak::Hash::new_from_array(var.seed);
    let slot_hash = keccak::Hash::new_from_array(var.slot_hash);
    sol_log(&format!("var slothash: {:?}", slot_hash).to_string());
    sol_log(&format!("var seed: {:?}", seed).to_string());

    // Read the finalized value from the var.
    let value = keccak::Hash::new_from_array(var.value);
    sol_log(&format!("var value: {:?}", value).to_string());
    round.slot_hash = var.value;

    // Exit early if no slot hash was found.
    let Some(r) = round.rng() else {
        // Slot hash could not be found, refund all SOL.
        round.total_vaulted = 0;
        round.total_winnings = 0;
        round.total_deployed = 0;

        // Emit event.
        program_log(
            &[board_info.clone(), ore_program.clone()],
            ResetEvent {
                disc: 0,
                round_id: round.id,
                start_slot: board.start_slot,
                end_slot: board.end_slot,
                winning_square: u64::MAX,
                top_miner: Pubkey::default(),
                num_winners: 0,
                motherlode: 0,
                total_deployed: round.total_deployed,
                total_vaulted: round.total_vaulted,
                total_winnings: round.total_winnings,
                total_minted: 0,
                ts: clock.unix_timestamp,
                rng: 0,
                deployed_winning_square: 0,
            }
            .to_bytes(),
        )?;

        // Update board for next round.
        board.round_id += 1;
        board.start_slot = clock.slot + 1;
        board.end_slot = u64::MAX;
        return Ok(());
    };

    // Calculate Sprint protocol fees (10% total = 1% team + 9% buyback).
    let team_fee = round
        .total_deployed
        .checked_mul(SPRINT_TEAM_FEE_BPS)
        .and_then(|v| v.checked_div(DENOMINATOR_BPS))
        .ok_or(ProgramError::ArithmeticOverflow)?;
    let buyback_fee = round
        .total_deployed
        .checked_mul(SPRINT_BUYBACK_FEE_BPS)
        .and_then(|v| v.checked_div(DENOMINATOR_BPS))
        .ok_or(ProgramError::ArithmeticOverflow)?;
    let total_protocol_fee = team_fee
        .checked_add(buyback_fee)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Get the winning square.
    let winning_square = round.winning_square(r);

    // If no one deployed on the winning square, vault all deployed.
    if round.deployed[winning_square] == 0 {
        // Vault all deployed (minus protocol fees).
        let vault_amount_no_winner = round
            .total_deployed
            .checked_sub(total_protocol_fee)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        round.total_vaulted = vault_amount_no_winner;
        treasury.balance = treasury
            .balance
            .checked_add(vault_amount_no_winner)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        // Emit event.
        program_log(
            &[board_info.clone(), ore_program.clone()],
            ResetEvent {
                disc: 0,
                round_id: round.id,
                start_slot: board.start_slot,
                end_slot: board.end_slot,
                winning_square: winning_square as u64,
                top_miner: Pubkey::default(),
                num_winners: 0,
                motherlode: 0,
                total_deployed: round.total_deployed,
                total_vaulted: round.total_vaulted,
                total_winnings: round.total_winnings,
                total_minted: 0,
                ts: clock.unix_timestamp,
                rng: r,
                deployed_winning_square: round.deployed[winning_square],
            }
            .to_bytes(),
        )?;

        // Update board for next round.
        board.round_id += 1;
        board.start_slot = clock.slot + 1;
        board.end_slot = u64::MAX;

        // Do SOL transfers: 1% team fee, 9% buyback to treasury, rest vaulted to treasury.
        round_info.send(team_fee, &fee_collector_info);
        round_info.send(
            buyback_fee
                .checked_add(vault_amount_no_winner)
                .ok_or(ProgramError::ArithmeticOverflow)?,
            &treasury_info,
        );
        return Ok(());
    }

    // Get winnings amount (total deployed on all non-winning squares).
    let raw_winnings = round.calculate_total_winnings(winning_square);

    // Calculate protocol fees on winnings: 1% team + 9% buyback = 10% total.
    let winnings_team_fee = raw_winnings
        .checked_mul(SPRINT_TEAM_FEE_BPS)
        .and_then(|v| v.checked_div(DENOMINATOR_BPS))
        .ok_or(ProgramError::ArithmeticOverflow)?;
    let winnings_buyback_fee = raw_winnings
        .checked_mul(SPRINT_BUYBACK_FEE_BPS)
        .and_then(|v| v.checked_div(DENOMINATOR_BPS))
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Winnings after protocol fee.
    let winnings = raw_winnings
        .checked_sub(winnings_team_fee)
        .and_then(|v| v.checked_sub(winnings_buyback_fee))
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Buyback fee goes to treasury for later buyback execution.
    round.total_winnings = winnings;
    round.total_vaulted = winnings_buyback_fee;
    treasury.balance = treasury
        .balance
        .checked_add(winnings_buyback_fee)
        .ok_or(ProgramError::ArithmeticOverflow)?;

    // Sanity check: total deployed >= vaulted + winnings + winners' stake + team fee.
    assert!(
        round.total_deployed
            >= round.total_vaulted
                .checked_add(round.total_winnings)
                .and_then(|v| v.checked_add(round.deployed[winning_square]))
                .and_then(|v| v.checked_add(winnings_team_fee))
                .unwrap_or(u64::MAX)
    );

    // Calculate mint amounts.
    let mut mint_supply = mint.supply();
    let mint_amount = MAX_SUPPLY.saturating_sub(mint_supply).min(ONE_FUEL);
    mint_supply += mint_amount;
    let motherlode_mint_amount = MAX_SUPPLY.saturating_sub(mint_supply).min(ONE_FUEL / 5);
    let total_mint_amount = mint_amount + motherlode_mint_amount;

    // Reward +1 ORE for the winning miner(s).
    round.top_miner_reward = mint_amount;

    // With 1 in 2 odds, split the +1 ORE reward.
    if round.is_split_reward(r) {
        round.top_miner = SPLIT_ADDRESS;
    }

    // Payout the motherlode if it was activated.
    if round.did_hit_motherlode(r) {
        round.motherlode = treasury.motherlode;
        treasury.motherlode = 0;
    }

    // Mint +0.2 ORE to the motherlode rewards pool.
    treasury.motherlode += motherlode_mint_amount;

    // Mint ORE to the treasury.
    let [mint_authority_info, mint_program] = mint_accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };
    mint_authority_info.as_account::<ore_mint_api::state::Authority>(&ore_mint_api::ID)?;
    mint_program.is_program(&ore_mint_api::ID)?;
    invoke_signed(
        &ore_mint_api::sdk::mint_ore(total_mint_amount),
        &[
            treasury_info.clone(),
            mint_authority_info.clone(),
            mint_info.clone(),
            treasury_tokens_info.clone(),
            token_program.clone(),
        ],
        &speedway_api::ID,
        &[TREASURY],
    )?;

    // Validate top miner (dry-run - no errors on failure).
    if round.top_miner != SPLIT_ADDRESS {
        if let Ok(miner) = top_miner_info.as_account::<Miner>(&speedway_api::ID) {
            if miner.round_id == round.id {
                let top_miner_sample = round.top_miner_sample(r, winning_square);
                if top_miner_sample >= miner.cumulative[winning_square]
                    && top_miner_sample
                        < miner.cumulative[winning_square] + miner.deployed[winning_square]
                {
                    sol_log("Top miner verified");
                } else {
                    sol_log("Top miner verification failed");
                }
            } else {
                sol_log("Top miner round id mismatch");
            }
        } else {
            sol_log("Top miner account cannot be parsed");
        }
    } else {
        sol_log("Split round");
    }

    // Emit event.
    program_log(
        &[board_info.clone(), ore_program.clone()],
        ResetEvent {
            disc: 0,
            round_id: round.id,
            start_slot: board.start_slot,
            end_slot: board.end_slot,
            winning_square: winning_square as u64,
            top_miner: round.top_miner,
            motherlode: round.motherlode,
            num_winners: round.count[winning_square],
            total_deployed: round.total_deployed,
            total_vaulted: round.total_vaulted,
            total_winnings: round.total_winnings,
            total_minted: total_mint_amount,
            ts: clock.unix_timestamp,
            rng: r,
            deployed_winning_square: round.deployed[winning_square],
        }
        .to_bytes(),
    )?;

    // Reset board.
    board.round_id += 1;
    board.start_slot = clock.slot + 1;
    board.end_slot = u64::MAX; // board.start_slot + 150;

    // Do SOL transfers: 1% team fee, 9% buyback to treasury.
    round_info.send(winnings_team_fee, &fee_collector_info);
    round_info.send(winnings_buyback_fee, &treasury_info);

    Ok(())
}
