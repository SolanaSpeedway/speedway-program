use serde::{Deserialize, Serialize};
use steel::*;

use super::OreAccount;

/// Treasury is a singleton account which is the mint authority for the FUEL token and the authority of
/// the program's global token account.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct Treasury {
    // The amount of SOL collected for buy-bury operations.
    pub balance: u64,

    /// Buffer a (placeholder)
    pub buffer_a: u64,

    /// The amount of FUEL in the motherlode rewards pool.
    pub motherlode: u64,

    /// The cumulative FUEL distributed to miners, divided by the total unclaimed FUEL at the time of distribution.
    pub miner_rewards_factor: Numeric,

    /// The cumulative FUEL distributed to stakers, divided by the total stake at the time of distribution.
    pub stake_rewards_factor: Numeric,

    /// Buffer b (placeholder)
    pub buffer_b: u64,

    /// The current total amount of refined FUEL mining rewards.
    pub total_refined: u64,

    /// The current total amount of FUEL staking deposits.
    pub total_staked: u64,

    /// The current total amount of unclaimed FUEL mining rewards.
    pub total_unclaimed: u64,

    // ============================================================================
    // Garage System Fields
    // ============================================================================

    /// FUEL pool for Garage withdrawals (funded by Fuel Up fees and protocol).
    pub garage_pool: u64,

    /// Total FUEL deposited across all Garages (for whale tax calculation).
    pub total_garage_tvl: u64,
}

account!(OreAccount, Treasury);
