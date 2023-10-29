use thiserror::Error;

/// This enum describes nft contract errors
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("The contract is disabled")]
    ContractDisabled,

    #[error("The specified domain is not minted")]
    DomainNotMinted,

    #[error("The specified domain is already minted")]
    Minted,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Payable token is not set")]
    PayableTokenNotSet,

    #[error("Payable receiver is not set")]
    PayableReceiverNotSet,

    #[error("User is not whitelisted")]
    UserNotWhitelisted,

    #[error("Mint count limit reached")]
    MintCountLimitReached,

    #[error("The specified domain is not active")]
    DomainNotActive,

    #[error("The given subscription years value is not valid")]
    InvalidSubscriptionYears,
}
