use thiserror::Error;

/// ## Description
/// This enum describes PNS contract errors
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Token with specified id is already minted")]
    Minted,

    #[error("Not found")]
    NotFound,

    #[error("Incorrect Owner")]
    IncorrectOwner,
}
