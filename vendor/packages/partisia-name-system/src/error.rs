use thiserror::Error;

/// ## Description
/// This enum describes PNS contract errors
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Cannot transfer record belonging to this parent")]
    ParentError,

    #[error("Token with specified id is already minted")]
    Minted,

    #[error("Token with specified id is not minted")]
    NotMinted,

    #[error("Not found")]
    NotFound,

    #[error("Incorrect Owner")]
    IncorrectOwner,
}
