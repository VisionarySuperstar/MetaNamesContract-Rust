use thiserror::Error;

/// ## Description
/// This enum describes nft contract errors
#[derive(Error, Debug)]
pub enum ContractError {
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
}
