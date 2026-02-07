use solana_program::{pubkey, pubkey::Pubkey};

/// The authority allowed to initialize the program.
/// TODO: Set this to the Speedway admin address before mainnet deployment
pub const ADMIN_ADDRESS: Pubkey = pubkey!("4mDJyuFSzhKrnhNAUtrsGhgekuLubvPXBLm5AziGnwUs");

/// The decimal precision of the FUEL token.
/// There are 100 billion indivisible units per FUEL (called "drops").
pub const TOKEN_DECIMALS: u8 = 11;

/// One FUEL token, denominated in indivisible units.
pub const ONE_FUEL: u64 = 10u64.pow(TOKEN_DECIMALS as u32);

/// The duration of one minute, in seconds.
pub const ONE_MINUTE: i64 = 60;

/// The duration of one hour, in seconds.
pub const ONE_HOUR: i64 = 60 * ONE_MINUTE;

/// The duration of one day, in seconds.
pub const ONE_DAY: i64 = 24 * ONE_HOUR;

/// The number of seconds for when the winning square expires.
pub const ONE_WEEK: i64 = 7 * ONE_DAY;

/// The number of slots in one week.
pub const ONE_MINUTE_SLOTS: u64 = 150;

/// The number of slots in one hour.
pub const ONE_HOUR_SLOTS: u64 = 60 * ONE_MINUTE_SLOTS;

/// The number of slots in 12 hours.
pub const TWELVE_HOURS_SLOTS: u64 = 12 * ONE_HOUR_SLOTS;

/// The number of slots in one day.
pub const ONE_DAY_SLOTS: u64 = 24 * ONE_HOUR_SLOTS;

/// The number of slots in one week.
pub const ONE_WEEK_SLOTS: u64 = 7 * ONE_DAY_SLOTS;

/// The number of slots for breather between rounds.
pub const INTERMISSION_SLOTS: u64 = 35;

/// The maximum token supply (5 million FUEL).
pub const MAX_SUPPLY: u64 = ONE_FUEL * 5_000_000;

/// The seed of the automation account PDA.
pub const AUTOMATION: &[u8] = b"automation";

/// The seed of the board account PDA.
pub const BOARD: &[u8] = b"board";

/// The seed of the config account PDA.
pub const CONFIG: &[u8] = b"config";

/// The seed of the miner account PDA.
pub const MINER: &[u8] = b"miner";

/// The seed of the seeker account PDA.
pub const SEEKER: &[u8] = b"seeker";

/// The seed of the square account PDA.
pub const SQUARE: &[u8] = b"square";

/// The seed of the stake account PDA.
pub const STAKE: &[u8] = b"stake";

/// The seed of the garage account PDA.
pub const GARAGE: &[u8] = b"garage";

/// The seed of the round account PDA.
pub const ROUND: &[u8] = b"round";

/// The seed of the treasury account PDA.
pub const TREASURY: &[u8] = b"treasury";

/// The address of the FUEL mint account.
/// TODO: Set this to the actual FUEL mint address before mainnet deployment
pub const MINT_ADDRESS: Pubkey = pubkey!("oreoU2P8bN6jkk3jbaiVxYnG1dCXcYxwhwyK9jSybcp");

/// The address of the SOL mint account.
pub const SOL_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

/// The address to indicate FUEL rewards are split between all miners.
/// TODO: Set this to the Speedway split address before mainnet deployment
pub const SPLIT_ADDRESS: Pubkey = pubkey!("SpLiT11111111111111111111111111111111111112");

/// The address to indicate automation is permissionless.
pub const EXECUTOR_ADDRESS: Pubkey = pubkey!("executor11111111111111111111111111111111112");

/// Denominator for fee calculations (basis points).
pub const DENOMINATOR_BPS: u64 = 10_000;

// ============================================================================
// Sprint Protocol Fees
// ============================================================================

/// Total Sprint protocol fee (10% of wagered SOL).
pub const SPRINT_PROTOCOL_FEE_BPS: u64 = 1000;

/// Team fee from Sprint (3% of wagered SOL).
pub const SPRINT_TEAM_FEE_BPS: u64 = 300;

/// Treasury fee from Sprint (7% of wagered SOL) - supports deflationary mechanics.
pub const SPRINT_TREASURY_FEE_BPS: u64 = 700;

/// Alias for backwards compatibility (treasury = buyback pool)
pub const SPRINT_BUYBACK_FEE_BPS: u64 = SPRINT_TREASURY_FEE_BPS;

// ============================================================================
// Garage Protocol Fees
// ============================================================================

/// Fuel Up burn percentage (55% of deposit burned permanently).
pub const FUEL_UP_BURN_BPS: u64 = 5500;

/// Pool fee from Fuel Up (28% of deposit).
pub const FUEL_UP_POOL_BPS: u64 = 2800;

/// Referral fee from Fuel Up (10% of deposit).
pub const FUEL_UP_REF_BPS: u64 = 1000;

/// Team fee from Fuel Up (7% of deposit).
pub const FUEL_UP_TEAM_BPS: u64 = 700;

/// Boost tax (5% of compounded amount).
pub const BOOST_TAX_BPS: u64 = 500;

/// Boost team share (2% of compounded amount).
pub const BOOST_TEAM_BPS: u64 = 200;

/// Boost pool share (3% of compounded amount).
pub const BOOST_POOL_BPS: u64 = 300;

/// Collect tax (10% of withdrawn amount).
pub const COLLECT_TAX_BPS: u64 = 1000;

/// Collect team share (2% of withdrawn amount).
pub const COLLECT_TEAM_BPS: u64 = 200;

/// Collect pool share (8% of withdrawn amount).
pub const COLLECT_POOL_BPS: u64 = 800;

// ============================================================================
// Whale Tax Schedule (applied AFTER base collect tax)
// ============================================================================

/// Whale tax thresholds in BPS (% of total Garage TVL).
/// Schedule: <1%=0%, >=1%=5%, >=2%=10%, >=3%=15%, >=4%=20%, >=5%=25%,
///           >=6%=30%, >=7%=35%, >=8%=40%, >=9%=45%, >=10%=50%
pub const WHALE_TAX_THRESHOLD_BPS: [u64; 10] = [
    100,  // 1% of TVL
    200,  // 2% of TVL
    300,  // 3% of TVL
    400,  // 4% of TVL
    500,  // 5% of TVL
    600,  // 6% of TVL
    700,  // 7% of TVL
    800,  // 8% of TVL
    900,  // 9% of TVL
    1000, // 10% of TVL
];

/// Whale tax rates corresponding to each threshold (5% increments).
pub const WHALE_TAX_RATE_BPS: [u64; 10] = [
    500,  // 5% whale tax at >=1% TVL
    1000, // 10% whale tax at >=2% TVL
    1500, // 15% whale tax at >=3% TVL
    2000, // 20% whale tax at >=4% TVL
    2500, // 25% whale tax at >=5% TVL
    3000, // 30% whale tax at >=6% TVL
    3500, // 35% whale tax at >=7% TVL
    4000, // 40% whale tax at >=8% TVL
    4500, // 45% whale tax at >=9% TVL
    5000, // 50% whale tax at >=10% TVL
];

/// Whale tax team share (30% of whale tax goes to team).
pub const WHALE_TAX_TEAM_BPS: u64 = 3000;

/// Whale tax pool share (70% of whale tax goes to pool).
pub const WHALE_TAX_POOL_BPS: u64 = 7000;

/// Wallet haircut for claiming FUEL to wallet instead of Garage (20%).
pub const WALLET_HAIRCUT_BPS: u64 = 2000;

/// Portion of wallet haircut that gets burned (50% of haircut = 10% of total).
pub const HAIRCUT_BURN_BPS: u64 = 5000;

/// Portion of wallet haircut that goes to team (50% of haircut = 10% of total).
pub const HAIRCUT_TEAM_BPS: u64 = 5000;

/// Daily yield rate (1.5% per day).
pub const DAILY_RATE_BPS: u64 = 150;

/// Maximum payout multiplier (365% of total deposited).
pub const MAX_PAYOUT_MULT: u64 = 365;

/// Minimum deposit amount (10 FUEL).
pub const MIN_DEPOSIT: u64 = ONE_FUEL * 10;

/// The address of the boost reserve token account.
/// TODO: Set this to the Speedway boost reserve address before mainnet deployment
pub const BOOST_RESERVE_TOKEN: Pubkey = pubkey!("Gce36ZUsBDJsoLrfCBxUB5Sfq2DsGunofStvxFx6rBiD");

/// The fee paid to bots if they checkpoint a user.
pub const CHECKPOINT_FEE: u64 = 10_000; // 0.00001 SOL

/// Amount paid to bots per transaction for auto-compounding staking yield, in lamports.
pub const COMPOUND_FEE_PER_TRANSACTION: u64 = 7_000;

/// The fee paid to the admin for each transaction.
pub const ADMIN_FEE: u64 = 100; // 1%

/// The address to receive the admin fee.
/// TODO: Set this to the Speedway team fee collector address before mainnet deployment
pub const ADMIN_FEE_COLLECTOR: Pubkey = pubkey!("DyB4Kv6V613gp2LWQTq1dwDYHGKuUEoDHnCouGUtxFiX");

/// The swap program used for buybacks.
pub const SWAP_PROGRAM: Pubkey = pubkey!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");

/// The address of the var account.
/// TODO: Set this to the Speedway var address before mainnet deployment
pub const VAR_ADDRESS: Pubkey = pubkey!("BWCaDY96Xe4WkFq1M7UiCCRcChsJ3p51L5KrGzhxgm2E");

/// The address which can call the bury and wrap instructions.
/// TODO: Set this to the Speedway bury authority before mainnet deployment
pub const BURY_AUTHORITY: Pubkey = pubkey!("HNWhK5f8RMWBqcA7mXJPaxdTPGrha3rrqUrri7HSKb3T");

// ============================================================================
// Whale Tax Calculation Helper
// ============================================================================

/// Calculate whale tax in BPS based on user's Garage balance as % of total TVL.
///
/// Schedule:
/// - <1% of TVL: 0% whale tax
/// - >=1% of TVL: 5% whale tax
/// - >=2% of TVL: 10% whale tax
/// - ... (5% increment per 1% TVL tier)
/// - >=10% of TVL: 50% whale tax (max)
///
/// Returns the whale tax rate in basis points.
pub fn calculate_whale_tax_bps(user_balance: u64, total_tvl: u64) -> u64 {
    // Prevent division by zero - no whale tax if TVL is 0
    if total_tvl == 0 {
        return 0;
    }

    // Calculate user's percentage of TVL in BPS
    // user_percentage = (user_balance * 10000) / total_tvl
    let user_percentage_bps = user_balance
        .saturating_mul(DENOMINATOR_BPS)
        .checked_div(total_tvl)
        .unwrap_or(0);

    // Find the appropriate tax rate based on thresholds
    // Iterate backwards through thresholds to find highest matching tier
    for i in (0..WHALE_TAX_THRESHOLD_BPS.len()).rev() {
        if user_percentage_bps >= WHALE_TAX_THRESHOLD_BPS[i] {
            return WHALE_TAX_RATE_BPS[i];
        }
    }

    // Below 1% - no whale tax
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that Sprint protocol fee constants are correctly defined.
    /// Protocol fee split is 3% team + 7% treasury = 10% total.
    #[test]
    fn test_sprint_fee_constants_defined() {
        // Verify team fee is 3% (300 BPS)
        assert_eq!(SPRINT_TEAM_FEE_BPS, 300, "Team fee should be 300 BPS (3%)");

        // Verify treasury fee is 7% (700 BPS)
        assert_eq!(SPRINT_TREASURY_FEE_BPS, 700, "Treasury fee should be 700 BPS (7%)");

        // Verify total protocol fee is 10% (1000 BPS)
        assert_eq!(SPRINT_PROTOCOL_FEE_BPS, 1000, "Protocol fee should be 1000 BPS (10%)");

        // Verify team + treasury = total protocol fee
        assert_eq!(
            SPRINT_TEAM_FEE_BPS + SPRINT_TREASURY_FEE_BPS,
            SPRINT_PROTOCOL_FEE_BPS,
            "Team fee + Treasury fee should equal total protocol fee"
        );
    }

    /// Test exact fee calculation: team_fee = total * 300 / 10000 (3%).
    /// Verifies the formula used in reset.rs for team fee.
    #[test]
    fn test_team_fee_calculation() {
        // Test with various total amounts (3% = 300 BPS)
        let test_cases: Vec<(u64, u64)> = vec![
            (100_000_000, 3_000_000),      // 100 SOL → 3 SOL team fee
            (1_000_000_000, 30_000_000),   // 1000 SOL → 30 SOL team fee
            (50_000_000, 1_500_000),       // 50 SOL → 1.5 SOL team fee
            (1_000_000, 30_000),           // 1 SOL → 0.03 SOL team fee
            (10_000, 300),                 // 0.00001 SOL → 0.0000003 SOL team fee
        ];

        for (total_deployed, expected_team_fee) in test_cases {
            let team_fee = total_deployed
                .checked_mul(SPRINT_TEAM_FEE_BPS)
                .and_then(|v| v.checked_div(DENOMINATOR_BPS))
                .expect("Team fee calculation should not overflow");

            assert_eq!(
                team_fee, expected_team_fee,
                "Team fee for {} should be {} (3%), got {}",
                total_deployed, expected_team_fee, team_fee
            );
        }
    }

    /// Test exact fee calculation: treasury_fee = total * 700 / 10000 (7%).
    /// Verifies the formula used in reset.rs for treasury fee.
    #[test]
    fn test_treasury_fee_calculation() {
        // Test with various total amounts (7% = 700 BPS)
        let test_cases: Vec<(u64, u64)> = vec![
            (100_000_000, 7_000_000),      // 100 SOL → 7 SOL treasury fee
            (1_000_000_000, 70_000_000),   // 1000 SOL → 70 SOL treasury fee
            (50_000_000, 3_500_000),       // 50 SOL → 3.5 SOL treasury fee
            (1_000_000, 70_000),           // 1 SOL → 0.07 SOL treasury fee
            (10_000, 700),                 // 0.00001 SOL → 0.0000007 SOL treasury fee
        ];

        for (total_deployed, expected_treasury_fee) in test_cases {
            let treasury_fee = total_deployed
                .checked_mul(SPRINT_TREASURY_FEE_BPS)
                .and_then(|v| v.checked_div(DENOMINATOR_BPS))
                .expect("Treasury fee calculation should not overflow");

            assert_eq!(
                treasury_fee, expected_treasury_fee,
                "Treasury fee for {} should be {} (7%), got {}",
                total_deployed, expected_treasury_fee, treasury_fee
            );
        }
    }

    /// Test that team fee + treasury fee = total protocol fee for any amount.
    /// This verifies the fee split invariant: 3% + 7% = 10%.
    ///
    /// Note: Due to integer division rounding, for very large amounts the sum of
    /// individually calculated fees may differ by 1 from the total protocol fee.
    /// This is acceptable as long as the difference is at most 1 lamport.
    #[test]
    fn test_fee_split_invariant() {
        let test_amounts: Vec<u64> = vec![
            1_000_000,         // 1 SOL (in lamports)
            100_000_000,       // 100 SOL
            1_000_000_000,     // 1000 SOL
            10_000_000_000,    // 10000 SOL
            100_000_000_000,   // 100000 SOL (realistic max round)
            1_000_000_000_000, // 1M SOL (extreme but valid)
        ];

        for total_deployed in test_amounts {
            let team_fee = total_deployed
                .checked_mul(SPRINT_TEAM_FEE_BPS)
                .and_then(|v| v.checked_div(DENOMINATOR_BPS))
                .expect("Team fee calculation overflow");

            let treasury_fee = total_deployed
                .checked_mul(SPRINT_TREASURY_FEE_BPS)
                .and_then(|v| v.checked_div(DENOMINATOR_BPS))
                .expect("Treasury fee calculation overflow");

            let total_protocol_fee = total_deployed
                .checked_mul(SPRINT_PROTOCOL_FEE_BPS)
                .and_then(|v| v.checked_div(DENOMINATOR_BPS))
                .expect("Protocol fee calculation overflow");

            // Allow for rounding difference of at most 1 lamport due to integer division
            let combined = team_fee + treasury_fee;
            let diff = if combined > total_protocol_fee {
                combined - total_protocol_fee
            } else {
                total_protocol_fee - combined
            };

            assert!(
                diff <= 1,
                "Team fee ({}) + Treasury fee ({}) should be within 1 of total protocol fee ({}) for amount {}, diff was {}",
                team_fee, treasury_fee, total_protocol_fee, total_deployed, diff
            );
        }
    }

    /// Test fee calculation with edge cases.
    #[test]
    fn test_fee_edge_cases() {
        // Zero amount
        let zero_team = 0u64
            .checked_mul(SPRINT_TEAM_FEE_BPS)
            .and_then(|v| v.checked_div(DENOMINATOR_BPS))
            .unwrap();
        let zero_treasury = 0u64
            .checked_mul(SPRINT_TREASURY_FEE_BPS)
            .and_then(|v| v.checked_div(DENOMINATOR_BPS))
            .unwrap();
        assert_eq!(zero_team, 0, "Team fee of 0 should be 0");
        assert_eq!(zero_treasury, 0, "Treasury fee of 0 should be 0");

        // Very small amount (less than DENOMINATOR_BPS)
        // 9999 lamports with 3% team fee: 9999 * 300 / 10000 = 299.97 → 299
        let small_team = 9999u64
            .checked_mul(SPRINT_TEAM_FEE_BPS)
            .and_then(|v| v.checked_div(DENOMINATOR_BPS))
            .unwrap();
        assert_eq!(small_team, 299, "Team fee of 9999 lamports should be 299 (3%)");

        // Minimum amount to get 1 lamport team fee at 3%: 34 lamports (34 * 300 / 10000 = 1.02 → 1)
        let min_team = 34u64
            .checked_mul(SPRINT_TEAM_FEE_BPS)
            .and_then(|v| v.checked_div(DENOMINATOR_BPS))
            .unwrap();
        assert_eq!(min_team, 1, "Team fee of 34 lamports should be 1 (3%)");
    }

    // ============================================================================
    // Whale Tax Tests
    // ============================================================================

    /// Test whale tax calculation at various TVL percentages.
    #[test]
    fn test_whale_tax_schedule() {
        let total_tvl = 1_000_000_000_000u64; // 1000 FUEL total TVL (in drops)

        // Test cases: (user_balance, expected_whale_tax_bps)
        let test_cases: Vec<(u64, u64)> = vec![
            // Below 1% - no whale tax
            (5_000_000_000, 0),         // 0.5% of TVL → 0%
            (9_999_999_999, 0),         // 0.9999...% of TVL → 0%
            // At or above 1% - 5% whale tax
            (10_000_000_000, 500),      // 1% of TVL → 5%
            (15_000_000_000, 500),      // 1.5% of TVL → 5%
            // At or above 2% - 10% whale tax
            (20_000_000_000, 1000),     // 2% of TVL → 10%
            (25_000_000_000, 1000),     // 2.5% of TVL → 10%
            // At or above 3% - 15% whale tax
            (30_000_000_000, 1500),     // 3% of TVL → 15%
            // At or above 4% - 20% whale tax
            (40_000_000_000, 2000),     // 4% of TVL → 20%
            // At or above 5% - 25% whale tax
            (50_000_000_000, 2500),     // 5% of TVL → 25%
            // At or above 6% - 30% whale tax
            (60_000_000_000, 3000),     // 6% of TVL → 30%
            // At or above 7% - 35% whale tax
            (70_000_000_000, 3500),     // 7% of TVL → 35%
            // At or above 8% - 40% whale tax
            (80_000_000_000, 4000),     // 8% of TVL → 40%
            // At or above 9% - 45% whale tax
            (90_000_000_000, 4500),     // 9% of TVL → 45%
            // At or above 10% - 50% whale tax (max)
            (100_000_000_000, 5000),    // 10% of TVL → 50%
            (150_000_000_000, 5000),    // 15% of TVL → 50% (max)
            (500_000_000_000, 5000),    // 50% of TVL → 50% (max)
        ];

        for (user_balance, expected_bps) in test_cases {
            let whale_tax_bps = calculate_whale_tax_bps(user_balance, total_tvl);
            let user_pct = (user_balance as f64 / total_tvl as f64) * 100.0;
            assert_eq!(
                whale_tax_bps, expected_bps,
                "Whale tax for {:.2}% of TVL should be {} BPS, got {}",
                user_pct, expected_bps, whale_tax_bps
            );
        }
    }

    /// Test whale tax edge cases.
    #[test]
    fn test_whale_tax_edge_cases() {
        // Zero TVL - should return 0 (no division by zero)
        assert_eq!(calculate_whale_tax_bps(100, 0), 0, "Zero TVL should return 0");

        // Zero user balance - should return 0
        assert_eq!(calculate_whale_tax_bps(0, 1000), 0, "Zero balance should return 0");

        // Both zero - should return 0
        assert_eq!(calculate_whale_tax_bps(0, 0), 0, "Both zero should return 0");

        // User owns 100% of TVL - should return max (50%)
        let tvl = 1_000_000_000_000u64;
        assert_eq!(calculate_whale_tax_bps(tvl, tvl), 5000, "100% ownership should return 50% whale tax");
    }

    /// Test whale tax distribution constants.
    #[test]
    fn test_whale_tax_distribution_constants() {
        // Whale tax team + pool should equal 100%
        assert_eq!(
            WHALE_TAX_TEAM_BPS + WHALE_TAX_POOL_BPS,
            DENOMINATOR_BPS,
            "Whale tax team + pool should equal 100%"
        );

        // Team is 30% of whale tax
        assert_eq!(WHALE_TAX_TEAM_BPS, 3000, "Whale tax team share should be 30%");

        // Pool is 70% of whale tax
        assert_eq!(WHALE_TAX_POOL_BPS, 7000, "Whale tax pool share should be 70%");
    }

    /// Test Garage fee constants.
    #[test]
    fn test_garage_fee_constants() {
        // Fuel Up: 55% burn + 28% pool + 10% ref + 7% team = 100%
        assert_eq!(
            FUEL_UP_BURN_BPS + FUEL_UP_POOL_BPS + FUEL_UP_REF_BPS + FUEL_UP_TEAM_BPS,
            DENOMINATOR_BPS,
            "Fuel Up fee split should equal 100%"
        );

        // Boost: 2% team + 3% pool = 5% total
        assert_eq!(
            BOOST_TEAM_BPS + BOOST_POOL_BPS,
            BOOST_TAX_BPS,
            "Boost fee split should equal total boost tax"
        );

        // Collect: 2% team + 8% pool = 10% total
        assert_eq!(
            COLLECT_TEAM_BPS + COLLECT_POOL_BPS,
            COLLECT_TAX_BPS,
            "Collect fee split should equal total collect tax"
        );

        // Wallet haircut: 50% burn + 50% team = 100% (of 20% haircut)
        assert_eq!(
            HAIRCUT_BURN_BPS + HAIRCUT_TEAM_BPS,
            DENOMINATOR_BPS,
            "Wallet haircut split should equal 100%"
        );

        // Daily rate is 1.5%
        assert_eq!(DAILY_RATE_BPS, 150, "Daily rate should be 150 BPS (1.5%)");
    }
}
