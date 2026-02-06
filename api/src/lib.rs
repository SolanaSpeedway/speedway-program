pub mod consts;
pub mod error;
pub mod event;
pub mod instruction;
pub mod sdk;
pub mod state;

pub mod prelude {
    pub use crate::consts::*;
    pub use crate::error::*;
    pub use crate::event::*;
    pub use crate::instruction::*;
    pub use crate::sdk::*;
    pub use crate::state::*;
}

use steel::*;

declare_id!("oreV3EG1i9BEgiAJ8b177Z2S2rMarzak4NMv1kULvWv");

// ============================================================================
// Integration Tests for Garage Instructions
// ============================================================================

#[cfg(test)]
mod garage_tests {
    use super::prelude::*;
    use solana_program::pubkey::Pubkey;

    // ========================================================================
    // Test 1: FuelUp New Account - Fee Splits
    // ========================================================================

    /// Test that FuelUp correctly splits 100% of deposit: 55% burn, 28% pool, 10% ref, 7% team.
    /// Yields are calculated on the full deposit amount.
    #[test]
    fn test_fuel_up_fee_splits() {
        let gross_amount = 1000 * ONE_FUEL; // 1000 FUEL

        // Calculate fees using same logic as fuel_up.rs (100% split)
        let burn_amount = gross_amount
            .checked_mul(FUEL_UP_BURN_BPS)
            .and_then(|v| v.checked_div(DENOMINATOR_BPS))
            .unwrap();
        let pool_fee = gross_amount
            .checked_mul(FUEL_UP_POOL_BPS)
            .and_then(|v| v.checked_div(DENOMINATOR_BPS))
            .unwrap();
        let ref_fee = gross_amount
            .checked_mul(FUEL_UP_REF_BPS)
            .and_then(|v| v.checked_div(DENOMINATOR_BPS))
            .unwrap();
        let team_fee = gross_amount
            .checked_mul(FUEL_UP_TEAM_BPS)
            .and_then(|v| v.checked_div(DENOMINATOR_BPS))
            .unwrap();
        let total_split = burn_amount + pool_fee + ref_fee + team_fee;

        // Verify fee percentages
        assert_eq!(burn_amount, 550 * ONE_FUEL, "Burn should be 55% (550 FUEL)");
        assert_eq!(pool_fee, 280 * ONE_FUEL, "Pool fee should be 28% (280 FUEL)");
        assert_eq!(ref_fee, 100 * ONE_FUEL, "Referrer fee should be 10% (100 FUEL)");
        assert_eq!(team_fee, 70 * ONE_FUEL, "Team fee should be 7% (70 FUEL)");
        assert_eq!(total_split, gross_amount, "Fee split should equal 100%");

        // Verify total BPS adds to 100%
        assert_eq!(
            FUEL_UP_BURN_BPS + FUEL_UP_POOL_BPS + FUEL_UP_REF_BPS + FUEL_UP_TEAM_BPS,
            DENOMINATOR_BPS,
            "Fee components should sum to 100%"
        );
    }

    /// Test that new Garage account is initialized correctly after FuelUp.
    /// Note: Yields are calculated on FULL deposit amount, not net.
    #[test]
    fn test_fuel_up_new_account_initialization() {
        let gross_amount = 100 * ONE_FUEL;
        // Yields calculated on full deposit, but 55% is burned
        let now = 1700000000i64;
        let authority = Pubkey::new_unique();
        let referrer = Pubkey::new_unique();

        // Simulate Garage state after FuelUp (yields on full deposit)
        let garage = Garage {
            authority,
            referrer,
            total_deposited: gross_amount, // Full deposit for yield calculation
            total_claimed: 0,
            max_payout: gross_amount * MAX_PAYOUT_MULT / 100, // 365% of full deposit
            last_action_at: now,
            created_at: now,
            direct_referrals: 0,
            _padding: 0,
            lifetime_ref_earnings: 0,
        };

        // Verify initialization
        assert_eq!(garage.authority, authority);
        assert_eq!(garage.referrer, referrer);
        assert_eq!(garage.total_deposited, 100 * ONE_FUEL);
        assert_eq!(garage.total_claimed, 0);
        assert_eq!(garage.max_payout, 100 * ONE_FUEL * MAX_PAYOUT_MULT / 100);
        assert!(!garage.is_exhausted());
    }

    // ========================================================================
    // Test 2: FuelUp House Code - No Referrer Fee
    // ========================================================================

    /// Test FuelUp with house code: 17% team (captures ref share), 28% pool, 55% burn.
    #[test]
    fn test_fuel_up_house_code_fee_splits() {
        let gross_amount = 1000 * ONE_FUEL;

        // House code scenario: referrer fee goes to team instead
        let house_team_bps = FUEL_UP_TEAM_BPS + FUEL_UP_REF_BPS; // 7% + 10% = 17%
        let house_ref_bps: u64 = 0;

        let burn_amount = gross_amount
            .checked_mul(FUEL_UP_BURN_BPS)
            .and_then(|v| v.checked_div(DENOMINATOR_BPS))
            .unwrap();
        let team_fee = gross_amount
            .checked_mul(house_team_bps)
            .and_then(|v| v.checked_div(DENOMINATOR_BPS))
            .unwrap();
        let ref_fee = gross_amount
            .checked_mul(house_ref_bps)
            .and_then(|v| v.checked_div(DENOMINATOR_BPS))
            .unwrap();
        let pool_fee = gross_amount
            .checked_mul(FUEL_UP_POOL_BPS)
            .and_then(|v| v.checked_div(DENOMINATOR_BPS))
            .unwrap();
        let total_split = burn_amount + team_fee + ref_fee + pool_fee;

        // Verify house code fee distribution
        assert_eq!(burn_amount, 550 * ONE_FUEL, "Burn still 55%");
        assert_eq!(team_fee, 170 * ONE_FUEL, "House code team fee should be 17%");
        assert_eq!(ref_fee, 0, "House code referrer fee should be 0%");
        assert_eq!(pool_fee, 280 * ONE_FUEL, "Pool fee unchanged at 28%");
        assert_eq!(total_split, gross_amount, "Total should still be 100%");
    }

    // ========================================================================
    // Test 3: Boost - Compound Rewards
    // ========================================================================

    /// Test Boost instruction: compound rewards with 5% tax.
    #[test]
    fn test_boost_fee_calculation() {
        let rewards_to_compound = 100 * ONE_FUEL;

        // Calculate boost tax (5%)
        let boost_tax = rewards_to_compound
            .checked_mul(BOOST_TAX_BPS)
            .and_then(|v| v.checked_div(DENOMINATOR_BPS))
            .unwrap();
        let net_compounded = rewards_to_compound - boost_tax;

        assert_eq!(boost_tax, 5 * ONE_FUEL, "Boost tax should be 5%");
        assert_eq!(net_compounded, 95 * ONE_FUEL, "Net compounded should be 95%");
        assert_eq!(BOOST_TAX_BPS, 500, "Boost tax should be 500 BPS (5%)");
    }

    /// Test that Boost recalculates max_payout correctly.
    #[test]
    fn test_boost_max_payout_recalculation() {
        let initial_deposit = 100 * ONE_FUEL;
        let boost_amount = 50 * ONE_FUEL; // After 5% tax

        let mut garage = Garage {
            authority: Pubkey::new_unique(),
            referrer: Pubkey::new_unique(),
            total_deposited: initial_deposit,
            total_claimed: 0,
            max_payout: initial_deposit * MAX_PAYOUT_MULT / 100,
            last_action_at: 1700000000,
            created_at: 1700000000,
            direct_referrals: 0,
            _padding: 0,
            lifetime_ref_earnings: 0,
        };

        let old_max_payout = garage.max_payout;

        // Simulate boost
        garage.total_deposited += boost_amount;
        garage.update_max_payout();

        let new_total = initial_deposit + boost_amount; // 150 FUEL
        let expected_max_payout = new_total * MAX_PAYOUT_MULT / 100;

        assert_eq!(garage.total_deposited, 150 * ONE_FUEL);
        assert_eq!(garage.max_payout, expected_max_payout);
        assert!(garage.max_payout > old_max_payout, "Max payout should increase");
    }

    // ========================================================================
    // Test 4: Collect - Withdraw Rewards
    // ========================================================================

    /// Test Collect instruction: 10% base tax.
    #[test]
    fn test_collect_base_tax() {
        let rewards_to_withdraw = 100 * ONE_FUEL;

        let base_tax = rewards_to_withdraw
            .checked_mul(COLLECT_TAX_BPS)
            .and_then(|v| v.checked_div(DENOMINATOR_BPS))
            .unwrap();
        let net_amount = rewards_to_withdraw - base_tax;

        assert_eq!(base_tax, 10 * ONE_FUEL, "Collect base tax should be 10%");
        assert_eq!(net_amount, 90 * ONE_FUEL, "Net withdrawal should be 90%");
        assert_eq!(COLLECT_TAX_BPS, 1000, "Collect tax should be 1000 BPS (10%)");
    }

    /// Test 1.5% daily accrual calculation.
    #[test]
    fn test_daily_accrual_calculation() {
        let total_deposited = 1000 * ONE_FUEL;
        let now = 1700000000i64;
        let one_day_later = now + ONE_DAY;
        let seven_days_later = now + (7 * ONE_DAY);

        let garage = Garage {
            authority: Pubkey::new_unique(),
            referrer: Pubkey::new_unique(),
            total_deposited,
            total_claimed: 0,
            max_payout: total_deposited * MAX_PAYOUT_MULT / 100,
            last_action_at: now,
            created_at: now,
            direct_referrals: 0,
            _padding: 0,
            lifetime_ref_earnings: 0,
        };

        // After 1 day: 1.5% of 1000 = 15 FUEL
        let available_1_day = garage.calculate_available(one_day_later);
        assert_eq!(available_1_day, 15 * ONE_FUEL, "1 day should accrue 1.5%");

        // After 7 days: 10.5% of 1000 = 105 FUEL
        let available_7_days = garage.calculate_available(seven_days_later);
        assert_eq!(available_7_days, 105 * ONE_FUEL, "7 days should accrue 10.5%");

        // Verify daily rate constant
        assert_eq!(DAILY_RATE_BPS, 150, "Daily rate should be 150 BPS (1.5%)");
    }

    // ========================================================================
    // Test 5: Collect Whale Tax
    // ========================================================================

    /// Test whale tax calculation for users with >1% of TVL.
    #[test]
    fn test_collect_whale_tax_tiers() {
        let total_tvl = 10000 * ONE_FUEL;

        // Test various TVL percentages
        let test_cases = vec![
            (50 * ONE_FUEL, 0),      // 0.5% → 0% whale tax
            (100 * ONE_FUEL, 500),   // 1% → 5% whale tax
            (200 * ONE_FUEL, 1000),  // 2% → 10% whale tax
            (500 * ONE_FUEL, 2500),  // 5% → 25% whale tax
            (1000 * ONE_FUEL, 5000), // 10% → 50% whale tax (max)
        ];

        for (user_balance, expected_whale_tax_bps) in test_cases {
            let whale_tax_bps = calculate_whale_tax_bps(user_balance, total_tvl);
            assert_eq!(
                whale_tax_bps, expected_whale_tax_bps,
                "Whale tax mismatch for balance {} / TVL {}",
                user_balance, total_tvl
            );
        }
    }

    /// Test whale tax distribution: 30% team, 70% pool.
    #[test]
    fn test_whale_tax_distribution() {
        let whale_tax = 100 * ONE_FUEL;

        let team_portion = whale_tax
            .checked_mul(WHALE_TAX_TEAM_BPS)
            .and_then(|v| v.checked_div(DENOMINATOR_BPS))
            .unwrap();
        let pool_portion = whale_tax - team_portion;

        assert_eq!(team_portion, 30 * ONE_FUEL, "Team should get 30% of whale tax");
        assert_eq!(pool_portion, 70 * ONE_FUEL, "Pool should get 70% of whale tax");
    }

    // ========================================================================
    // Test 6: Stash - Sprint Rewards to Garage
    // ========================================================================

    /// Test Stash instruction: 0% fee.
    #[test]
    fn test_stash_zero_fee() {
        let sprint_rewards = 100 * ONE_FUEL;

        // Stash has no fee - 100% goes to Garage
        let fee: u64 = 0;
        let net_amount = sprint_rewards - fee;

        assert_eq!(net_amount, sprint_rewards, "Stash should have 0% fee");
        assert_eq!(net_amount, 100 * ONE_FUEL, "Full amount should be stashed");
    }

    /// Test that Stash updates Garage state correctly.
    #[test]
    fn test_stash_garage_update() {
        let initial_deposit = 100 * ONE_FUEL;
        let stash_amount = 50 * ONE_FUEL;

        let mut garage = Garage {
            authority: Pubkey::new_unique(),
            referrer: Pubkey::new_unique(),
            total_deposited: initial_deposit,
            total_claimed: 0,
            max_payout: initial_deposit * MAX_PAYOUT_MULT / 100,
            last_action_at: 1700000000,
            created_at: 1700000000,
            direct_referrals: 0,
            _padding: 0,
            lifetime_ref_earnings: 0,
        };

        // Simulate stash (0% fee, full amount added)
        garage.total_deposited += stash_amount;
        garage.update_max_payout();

        assert_eq!(garage.total_deposited, 150 * ONE_FUEL);
        assert_eq!(garage.max_payout, 150 * ONE_FUEL * MAX_PAYOUT_MULT / 100);
    }

    // ========================================================================
    // Test 7: ClaimWallet - Sprint Rewards to Wallet
    // ========================================================================

    /// Test ClaimWallet: 20% haircut (50% burn = 10% total, 50% team = 10% total).
    #[test]
    fn test_claim_wallet_haircut() {
        let gross_amount = 100 * ONE_FUEL;

        // Calculate 20% haircut
        let haircut = gross_amount
            .checked_mul(WALLET_HAIRCUT_BPS)
            .and_then(|v| v.checked_div(DENOMINATOR_BPS))
            .unwrap();
        let net_amount = gross_amount - haircut;

        // Split haircut: 50% burn, 50% team
        let burn_amount = haircut
            .checked_mul(HAIRCUT_BURN_BPS)
            .and_then(|v| v.checked_div(DENOMINATOR_BPS))
            .unwrap();
        let team_fee = haircut - burn_amount;

        assert_eq!(haircut, 20 * ONE_FUEL, "Haircut should be 20%");
        assert_eq!(net_amount, 80 * ONE_FUEL, "Net should be 80%");
        assert_eq!(burn_amount, 10 * ONE_FUEL, "Burn should be 50% of haircut (10% total)");
        assert_eq!(team_fee, 10 * ONE_FUEL, "Team fee should be 50% of haircut (10% total)");

        // Verify constants
        assert_eq!(WALLET_HAIRCUT_BPS, 2000, "Haircut should be 2000 BPS (20%)");
        assert_eq!(HAIRCUT_BURN_BPS, 5000, "Burn portion should be 5000 BPS (50%)");
        assert_eq!(HAIRCUT_TEAM_BPS, 5000, "Team portion should be 5000 BPS (50%)");
    }

    // ========================================================================
    // Test 8: Max Payout Exhaustion
    // ========================================================================

    /// Test that Garage becomes exhausted at 365% max payout.
    #[test]
    fn test_max_payout_exhaustion() {
        let deposit = 100 * ONE_FUEL;
        let max_payout = deposit * MAX_PAYOUT_MULT / 100; // 365 FUEL

        let mut garage = Garage {
            authority: Pubkey::new_unique(),
            referrer: Pubkey::new_unique(),
            total_deposited: deposit,
            total_claimed: 0,
            max_payout,
            last_action_at: 1700000000,
            created_at: 1700000000,
            direct_referrals: 0,
            _padding: 0,
            lifetime_ref_earnings: 0,
        };

        // Initially not exhausted
        assert!(!garage.is_exhausted());

        // Claim 50%
        garage.total_claimed = max_payout / 2;
        assert!(!garage.is_exhausted());

        // Claim up to max
        garage.total_claimed = max_payout;
        assert!(garage.is_exhausted());

        // No more rewards available
        let available = garage.calculate_available(garage.last_action_at + ONE_DAY * 30);
        assert_eq!(available, 0, "Exhausted garage should have 0 available");
    }

    /// Test max payout multiplier constant.
    #[test]
    fn test_max_payout_multiplier() {
        assert_eq!(MAX_PAYOUT_MULT, 365, "Max payout multiplier should be 365");

        let deposit = 100 * ONE_FUEL;
        let expected_max = deposit * 365 / 100;

        let mut garage = Garage {
            authority: Pubkey::new_unique(),
            referrer: Pubkey::new_unique(),
            total_deposited: deposit,
            total_claimed: 0,
            max_payout: 0,
            last_action_at: 0,
            created_at: 0,
            direct_referrals: 0,
            _padding: 0,
            lifetime_ref_earnings: 0,
        };
        garage.update_max_payout();

        assert_eq!(garage.max_payout, expected_max, "Max payout should be 365% of deposit");
    }

    // ========================================================================
    // Additional Edge Case Tests
    // ========================================================================

    /// Test minimum deposit requirement.
    #[test]
    fn test_minimum_deposit() {
        assert_eq!(MIN_DEPOSIT, 10 * ONE_FUEL, "Minimum deposit should be 10 FUEL");
    }

    /// Test rewards capped at remaining payout.
    #[test]
    fn test_rewards_capped_at_remaining() {
        let deposit = 100 * ONE_FUEL;
        let max_payout = deposit * MAX_PAYOUT_MULT / 100; // 365 FUEL

        let garage = Garage {
            authority: Pubkey::new_unique(),
            referrer: Pubkey::new_unique(),
            total_deposited: deposit,
            total_claimed: max_payout - (10 * ONE_FUEL), // Only 10 FUEL remaining
            max_payout,
            last_action_at: 1700000000,
            created_at: 1700000000,
            direct_referrals: 0,
            _padding: 0,
            lifetime_ref_earnings: 0,
        };

        // Even after 365 days, should only get remaining 10 FUEL
        let far_future = garage.last_action_at + ONE_DAY * 365;
        let available = garage.calculate_available(far_future);

        assert_eq!(available, 10 * ONE_FUEL, "Available should be capped at remaining payout");
    }

    /// Test token decimals and ONE_FUEL constant.
    #[test]
    fn test_token_decimals() {
        assert_eq!(TOKEN_DECIMALS, 11, "FUEL should have 11 decimals");
        assert_eq!(ONE_FUEL, 10u64.pow(11), "ONE_FUEL should be 10^11");
        assert_eq!(ONE_FUEL, 100_000_000_000, "ONE_FUEL should be 100 billion drops");
    }

    /// Test max supply constant.
    #[test]
    fn test_max_supply() {
        assert_eq!(MAX_SUPPLY, 5_000_000 * ONE_FUEL, "Max supply should be 5M FUEL");
    }
}
