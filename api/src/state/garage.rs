use serde::{Deserialize, Serialize};
use steel::*;

use crate::consts::*;
use crate::state::garage_pda;

use super::OreAccount;

/// Garage is a user's faucet position in the Garage yield system.
/// Users deposit FUEL tokens and earn 1% daily returns up to 365% max payout.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct Garage {
    /// Owner wallet (authority of this garage account).
    pub authority: Pubkey,

    /// Referrer wallet (set on first deposit, immutable after).
    pub referrer: Pubkey,

    /// Sum of all deposits + boosts (for max payout calculation).
    pub total_deposited: u64,

    /// Running total of claims.
    pub total_claimed: u64,

    /// Maximum payout (365% of total_deposited).
    pub max_payout: u64,

    /// Timestamp of last claim/boost action.
    pub last_action_at: i64,

    /// Account creation timestamp.
    pub created_at: i64,

    /// Count of direct referrals.
    pub direct_referrals: u32,

    /// Padding for alignment.
    pub _padding: u32,

    /// Total FUEL earned from referrals over lifetime.
    pub lifetime_ref_earnings: u64,
}

impl Garage {
    /// Returns the PDA for this garage account.
    pub fn pda(&self) -> (Pubkey, u8) {
        garage_pda(self.authority)
    }

    /// Check if account has reached max payout (exhausted).
    pub fn is_exhausted(&self) -> bool {
        self.total_claimed >= self.max_payout
    }

    /// Calculate available rewards based on time elapsed.
    /// Returns the amount of FUEL that can be claimed.
    pub fn calculate_available(&self, now: i64) -> u64 {
        // If exhausted, no rewards available
        if self.is_exhausted() {
            return 0;
        }

        let seconds_elapsed = now.saturating_sub(self.last_action_at) as u64;
        let days_elapsed = seconds_elapsed / (ONE_DAY as u64);

        // 1.5% daily = total_deposited * days * 150 / 10000
        // Using checked arithmetic as per project conventions
        let accrued = self
            .total_deposited
            .checked_mul(days_elapsed)
            .and_then(|v| v.checked_mul(DAILY_RATE_BPS))
            .and_then(|v| v.checked_div(DENOMINATOR_BPS))
            .unwrap_or(0);

        // Cap at remaining payout
        let remaining = self.max_payout.saturating_sub(self.total_claimed);
        accrued.min(remaining)
    }

    /// Update max_payout when deposits or boosts occur.
    /// max_payout = total_deposited * 365 / 100 = total_deposited * MAX_PAYOUT_MULT / 100
    pub fn update_max_payout(&mut self) {
        self.max_payout = self
            .total_deposited
            .checked_mul(MAX_PAYOUT_MULT)
            .and_then(|v| v.checked_div(100))
            .unwrap_or(u64::MAX);
    }
}

account!(OreAccount, Garage);
