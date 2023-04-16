use thiserror::Error;

/// ## Description
/// This enum describes mpc20-staking contract errors
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("Cannot unstake more then staked")]
    CannotUnstakeMoreThenStaked,

    #[error("Cannot claim more then rewarded")]
    CannotClaimMoreThenRewarded,

    #[error("Cannot compound more then rewarded")]
    CannotCompoundMoreThenRewarded,

    #[error("Nothing to claim")]
    NothingToClaim,

    #[error("Compound only enabled when deposit token is reward token")]
    CompoundOnlyWorksWithSelfToken,

    #[error("Forbidden to compound to often")]
    ForbiddenToCompoundToOften,
}
