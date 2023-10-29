use thiserror::Error;

/// This enum describes NFT contract errors
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

    #[error("Already presesent")]
    AlreadyPresent,

    #[error("URI is too long")]
    UriTooLong,
}
