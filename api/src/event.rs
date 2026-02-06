use serde::{Deserialize, Serialize};
use steel::*;

pub enum OreEvent {
    Reset = 0,
    Bury = 1,
    Deploy = 2,
    Liq = 3,
    // Garage events
    FuelUp = 10,
    Boost = 11,
    Collect = 12,
    Stash = 13,
    ClaimWallet = 14,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct ResetEvent {
    /// The event discriminator.
    pub disc: u64,

    /// The block that was opened for trading.
    pub round_id: u64,

    /// The start slot of the next block.
    pub start_slot: u64,

    /// The end slot of the next block.
    pub end_slot: u64,

    /// The winning square of the round.
    pub winning_square: u64,

    /// The top miner of the round.
    pub top_miner: Pubkey,

    /// The number of miners on the winning square.
    pub num_winners: u64,

    /// The amount of ORE payout for the motherlode.
    pub motherlode: u64,

    /// The total amount of SOL prospected in the round.
    pub total_deployed: u64,

    /// The total amount of SOL put in the ORE vault.
    pub total_vaulted: u64,

    /// The total amount of SOL won by miners for the round.
    pub total_winnings: u64,

    /// The total amount of ORE minted for the round.
    pub total_minted: u64,

    /// The timestamp of the event.
    pub ts: i64,

    /// The rng value of the round.
    pub rng: u64,

    /// The amount deployed on the winning square.
    pub deployed_winning_square: u64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct BuryEvent {
    /// The event discriminator.
    pub disc: u64,

    /// The amount of ORE buried.
    pub ore_buried: u64,

    /// The amount of ORE shared with stakers.
    pub ore_shared: u64,

    /// The amount of SOL swapped.
    pub sol_amount: u64,

    /// The new circulating supply of ORE.
    pub new_circulating_supply: u64,

    /// The timestamp of the event.
    pub ts: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct DeployEvent {
    /// The event discriminator.
    pub disc: u64,

    /// The authority of the deployer.
    pub authority: Pubkey,

    /// The amount of SOL deployed per square.
    pub amount: u64,

    /// The mask of the squares deployed to.
    pub mask: u64,

    /// The round id.
    pub round_id: u64,

    /// The signer of the deployer.
    pub signer: Pubkey,

    /// The strategy used by the autominer (u64::MAX if manual).
    pub strategy: u64,

    /// The total number of squares deployed to.
    pub total_squares: u64,

    /// The timestamp of the event.
    pub ts: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct LiqEvent {
    /// The event discriminator.
    pub disc: u64,

    /// The amount of SOL sent to the liq manager.
    pub sol_amount: u64,

    /// The recipient of the SOL.
    pub recipient: Pubkey,

    /// The timestamp of the event.
    pub ts: i64,
}

event!(ResetEvent);
event!(BuryEvent);
event!(DeployEvent);
event!(LiqEvent);

// ============================================================================
// Garage Events
// ============================================================================

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct FuelUpEvent {
    /// The event discriminator.
    pub disc: u64,

    /// The authority (user) who deposited.
    pub authority: Pubkey,

    /// The gross deposit amount (before tax).
    pub gross_amount: u64,

    /// The net amount credited to Garage (after 10% tax).
    pub net_amount: u64,

    /// The team fee (2% of gross).
    pub team_fee: u64,

    /// The referral fee (5% of gross).
    pub ref_fee: u64,

    /// The pool fee (3% of gross).
    pub pool_fee: u64,

    /// The referrer who received the referral fee.
    pub referrer: Pubkey,

    /// The new total_deposited in user's Garage.
    pub new_total_deposited: u64,

    /// The new max_payout in user's Garage.
    pub new_max_payout: u64,

    /// The timestamp of the event.
    pub ts: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct BoostEvent {
    /// The event discriminator.
    pub disc: u64,

    /// The authority (user) who boosted.
    pub authority: Pubkey,

    /// The gross rewards compounded (before 5% tax).
    pub gross_amount: u64,

    /// The net amount added to deposits (after tax).
    pub net_amount: u64,

    /// The boost tax (5% of gross).
    pub tax: u64,

    /// The new total_deposited in user's Garage.
    pub new_total_deposited: u64,

    /// The new max_payout in user's Garage.
    pub new_max_payout: u64,

    /// The timestamp of the event.
    pub ts: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct CollectEvent {
    /// The event discriminator.
    pub disc: u64,

    /// The authority (user) who collected.
    pub authority: Pubkey,

    /// The gross rewards withdrawn (before taxes).
    pub gross_amount: u64,

    /// The net amount received (after all taxes).
    pub net_amount: u64,

    /// The base collect tax (10% of gross).
    pub base_tax: u64,

    /// The whale tax (0-50% based on TVL %).
    pub whale_tax: u64,

    /// Whale tax portion sent to team (10% of whale_tax).
    pub whale_tax_team: u64,

    /// Whale tax portion sent to pool (90% of whale_tax).
    pub whale_tax_pool: u64,

    /// The new total_claimed in user's Garage.
    pub new_total_claimed: u64,

    /// Whether the Garage is now exhausted.
    pub is_exhausted: u8,

    /// Padding for alignment.
    pub _padding: [u8; 7],

    /// The timestamp of the event.
    pub ts: i64,
}

event!(FuelUpEvent);
event!(BoostEvent);
event!(CollectEvent);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct StashEvent {
    /// The event discriminator.
    pub disc: u64,

    /// The authority (user) who stashed.
    pub authority: Pubkey,

    /// The amount of FUEL stashed from Sprint rewards.
    pub amount: u64,

    /// The new total_deposited in user's Garage.
    pub new_total_deposited: u64,

    /// The new max_payout in user's Garage.
    pub new_max_payout: u64,

    /// The timestamp of the event.
    pub ts: i64,
}

event!(StashEvent);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct ClaimWalletEvent {
    /// The event discriminator.
    pub disc: u64,

    /// The authority (user) who claimed.
    pub authority: Pubkey,

    /// The gross amount before haircut.
    pub gross_amount: u64,

    /// The net amount received (80% of gross).
    pub net_amount: u64,

    /// The amount burned (15% of gross).
    pub burn_amount: u64,

    /// The team fee (5% of gross).
    pub team_fee: u64,

    /// The timestamp of the event.
    pub ts: i64,
}

event!(ClaimWalletEvent);
