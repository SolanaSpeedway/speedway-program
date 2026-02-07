use speedway_api::prelude::*;
use solana_program::clock::Clock;
use steel::*;

/// Initializes all required program PDAs. Can only be called once by the admin.
///
/// Creates:
/// - Treasury: Program's global token authority and yield pool
/// - Config: Stores admin address and program settings
/// - Board: Game state (current round, slots, epoch)
/// - Round 0: First game round
///
/// Accounts:
/// 0. `[signer]` Admin (must match ADMIN_ADDRESS)
/// 1. `[writable]` Treasury PDA
/// 2. `[writable]` Config PDA
/// 3. `[writable]` Board PDA
/// 4. `[writable]` Round 0 PDA
/// 5. `[]` System program
/// 6. `[]` Speedway program (self)
pub fn process_initialize(accounts: &[AccountInfo<'_>], _data: &[u8]) -> ProgramResult {
    // Parse accounts
    let [signer_info, treasury_info, config_info, board_info, round_info, system_program, ore_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate signer is the admin
    signer_info.is_signer()?;
    if *signer_info.key != ADMIN_ADDRESS {
        return Err(OreError::NotAuthorized.into());
    }

    // Validate programs
    system_program.is_program(&system_program::ID)?;
    ore_program.is_program(&speedway_api::ID)?;

    // Get clock for current slot
    let clock = Clock::get()?;
    let current_slot = clock.slot;

    // Create Treasury PDA
    treasury_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&[TREASURY], &speedway_api::ID)?;

    create_program_account::<Treasury>(
        treasury_info,
        ore_program,
        signer_info,
        &speedway_api::ID,
        &[TREASURY],
    )?;

    let treasury = treasury_info.as_account_mut::<Treasury>(&speedway_api::ID)?;
    treasury.balance = 0;
    treasury.buffer_a = 0;
    treasury.motherlode = 0;
    treasury.miner_rewards_factor = Numeric::ZERO;
    treasury.stake_rewards_factor = Numeric::ZERO;
    treasury.buffer_b = 0;
    treasury.total_refined = 0;
    treasury.total_staked = 0;
    treasury.total_unclaimed = 0;
    treasury.garage_pool = 0;
    treasury.total_garage_tvl = 0;

    // Create Config PDA
    config_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&[CONFIG], &speedway_api::ID)?;

    create_program_account::<Config>(
        config_info,
        ore_program,
        signer_info,
        &speedway_api::ID,
        &[CONFIG],
    )?;

    let config = config_info.as_account_mut::<Config>(&speedway_api::ID)?;
    config.admin = ADMIN_ADDRESS;
    config.buffer_a = [0u8; 32];
    config.buffer_b = [0u8; 32];
    config.buffer_c = [0u8; 32];
    config.buffer_d = [0u8; 32];
    config.buffer_e = [0u8; 8];

    // Create Board PDA
    board_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&[BOARD], &speedway_api::ID)?;

    create_program_account::<Board>(
        board_info,
        ore_program,
        signer_info,
        &speedway_api::ID,
        &[BOARD],
    )?;

    let board = board_info.as_account_mut::<Board>(&speedway_api::ID)?;
    board.round_id = 0;
    board.start_slot = current_slot;
    board.end_slot = current_slot.saturating_add(ONE_MINUTE_SLOTS);
    board.epoch_id = 0;

    // Create Round 0 PDA
    let round_id: u64 = 0;
    round_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&[ROUND, &round_id.to_le_bytes()], &speedway_api::ID)?;

    create_program_account::<Round>(
        round_info,
        ore_program,
        signer_info,
        &speedway_api::ID,
        &[ROUND, &round_id.to_le_bytes()],
    )?;

    let round = round_info.as_account_mut::<Round>(&speedway_api::ID)?;
    round.id = 0;
    round.deployed = [0u64; 25];
    round.slot_hash = [0u8; 32];
    round.count = [0u64; 25];
    round.expires_at = u64::MAX;
    round.motherlode = 0;
    round.rent_payer = *signer_info.key;
    round.top_miner = Pubkey::default();
    round.top_miner_reward = 0;
    round.total_deployed = 0;
    round.total_miners = 0;
    round.total_vaulted = 0;
    round.total_winnings = 0;

    solana_program::msg!("Speedway program initialized successfully!");
    solana_program::msg!("Treasury: {}", treasury_info.key);
    solana_program::msg!("Config: {}", config_info.key);
    solana_program::msg!("Board: {}", board_info.key);
    solana_program::msg!("Round 0: {}", round_info.key);

    Ok(())
}
