use thiserror::Error;

/// ## Description
/// This enum describes mpc20 contract errors
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("Amount must be higher then zero")]
    AmountMustBeHigherThenZero,

    #[error("Overflow")]
    Overflow,

    #[error("Not found")]
    NotFound,

    #[error("Minting is disabled")]
    MintingIsDisabled,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Capacity exceeded")]
    CapacityExceeded,

    #[error("Cannot approve to yourself")]
    CannotApproveToYourself,
}
