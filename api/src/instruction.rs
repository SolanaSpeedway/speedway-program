use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum OreInstruction {
    // Miner
    Automate = 0,
    Checkpoint = 2,
    ClaimSOL = 3,
    ClaimORE = 4,
    Close = 5,
    Deploy = 6,
    Log = 8,
    Reset = 9,
    ReloadSOL = 21,

    // DEPRECATED: ORE Staker instructions (replaced by Garage system)
    // Deposit = 10,
    // Withdraw = 11,
    // ClaimYield = 12,
    // CompoundYield = 22,

    // Garage
    FuelUp = 30,
    Boost = 31,
    Collect = 32,
    Stash = 33,
    ClaimWallet = 34,

    // Admin
    Buyback = 13,
    Bury = 24,
    Wrap = 14,
    SetAdmin = 15,
    NewVar = 19,
    Liq = 25,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Automate {
    pub amount: [u8; 8],
    pub deposit: [u8; 8],
    pub fee: [u8; 8],
    pub mask: [u8; 8],
    pub strategy: u8,
    pub reload: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ClaimSOL {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ClaimORE {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Deploy {
    pub amount: [u8; 8],
    pub squares: [u8; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Log {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Reset {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Mine {
    pub nonce: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Swap {
    pub amount: [u8; 8],
    pub direction: u8,
    pub precision: u8,
    pub seed: [u8; 32],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Uncommit {
    pub amount: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct SetAdmin {
    pub admin: [u8; 32],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct SetFeeCollector {
    pub fee_collector: [u8; 32],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct SetFeeRate {
    pub fee_rate: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Wrap {
    pub amount: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Buyback {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Bury {
    pub amount: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ReloadSOL {}

// DEPRECATED: ORE Staking instruction structs (replaced by Garage system)
// #[repr(C)]
// #[derive(Clone, Copy, Debug, Pod, Zeroable)]
// pub struct Deposit {
//     pub amount: [u8; 8],
//     pub compound_fee: [u8; 8],
// }

// #[repr(C)]
// #[derive(Clone, Copy, Debug, Pod, Zeroable)]
// pub struct Withdraw {
//     pub amount: [u8; 8],
// }

// #[repr(C)]
// #[derive(Clone, Copy, Debug, Pod, Zeroable)]
// pub struct ClaimYield {
//     pub amount: [u8; 8],
// }

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Checkpoint {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Close {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct NewVar {
    pub id: [u8; 8],
    pub commit: [u8; 32],
    pub samples: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct SetAdminFee {
    pub admin_fee: [u8; 8],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct SetSwapProgram {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct SetVarAddress {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Liq {}

// DEPRECATED: ORE Staking (replaced by Garage system)
// #[repr(C)]
// #[derive(Clone, Copy, Debug, Pod, Zeroable)]
// pub struct CompoundYield {}

// ============================================================================
// Garage Instructions
// ============================================================================

/// FuelUp: Deposit FUEL tokens into Garage.
/// Burns tokens and credits to user's Garage position.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct FuelUp {
    /// Amount of FUEL to deposit (in drops).
    pub amount: [u8; 8],
}

/// Boost: Compound accrued rewards back into Garage.
/// Adds rewards to total_deposited, increasing max_payout.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Boost {}

/// Collect: Withdraw accrued rewards from Garage.
/// Subject to 10% collect tax.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Collect {}

/// Stash: Send Sprint FUEL rewards directly to Garage.
/// 0% fee - frictionless path from Sprint rewards to Garage.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Stash {}

/// ClaimWallet: Claim Sprint FUEL rewards directly to wallet.
/// 20% haircut: 75% burned (15% total), 25% to team (5% total).
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ClaimWallet {}

instruction!(OreInstruction, Automate);
instruction!(OreInstruction, Close);
instruction!(OreInstruction, Checkpoint);
instruction!(OreInstruction, ClaimSOL);
instruction!(OreInstruction, ClaimORE);
instruction!(OreInstruction, ReloadSOL);
instruction!(OreInstruction, Deploy);
instruction!(OreInstruction, Log);
instruction!(OreInstruction, Wrap);
instruction!(OreInstruction, Buyback);
instruction!(OreInstruction, Bury);
instruction!(OreInstruction, Reset);
instruction!(OreInstruction, SetAdmin);
// DEPRECATED: ORE Staking (replaced by Garage system)
// instruction!(OreInstruction, Deposit);
// instruction!(OreInstruction, Withdraw);
// instruction!(OreInstruction, ClaimYield);
instruction!(OreInstruction, NewVar);
instruction!(OreInstruction, Liq);
// instruction!(OreInstruction, CompoundYield);

// Garage instructions
instruction!(OreInstruction, FuelUp);
instruction!(OreInstruction, Boost);
instruction!(OreInstruction, Collect);
instruction!(OreInstruction, Stash);
instruction!(OreInstruction, ClaimWallet);
