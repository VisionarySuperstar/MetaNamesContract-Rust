use thiserror::Error;

/// ## Description
/// This enum describes PNS contract errors
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("Token with specified id is already minted")]
    Minted,

    #[error("Record with specified token id and class is already minted")]
    RecordMinted,

    #[error("Token with specified id is not minted")]
    NotMinted,

    #[error("Record with specified token id and class is not minted")]
    RecordNotMinted,

    #[error("Not found")]
    NotFound,
}
