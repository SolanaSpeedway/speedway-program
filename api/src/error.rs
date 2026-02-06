use steel::*;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum OreError {
    #[error("Amount too small")]
    AmountTooSmall = 0,

    #[error("Not authorized")]
    NotAuthorized = 1,

    #[error("Invalid executor")]
    InvalidExecutor = 2,

    // ============================================================================
    // Garage Errors
    // ============================================================================

    #[error("Deposit below minimum (10 FUEL)")]
    DepositBelowMinimum = 100,

    #[error("Garage is exhausted (max payout reached)")]
    GarageExhausted = 101,

    #[error("No rewards available to claim")]
    NoRewardsAvailable = 102,

    #[error("Invalid referrer (cannot self-refer)")]
    InvalidReferrer = 103,

    #[error("Referrer has no Garage account")]
    ReferrerNoGarage = 104,

    #[error("Insufficient pool balance")]
    InsufficientPoolBalance = 105,

    #[error("Garage account required (use FuelUp first)")]
    GarageRequired = 106,
}

error!(OreError);
