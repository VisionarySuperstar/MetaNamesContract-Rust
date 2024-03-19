use thiserror::Error;

/// This enum describes PNS contract errors
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("Token with specified id is already minted")]
    Minted,

    #[error("Record is already minted")]
    RecordMinted,

    #[error("Token with specified id is not minted")]
    NotMinted,

    #[error("Record with specified token id and class is not minted")]
    RecordNotMinted,

    #[error("The maximum amount of custom records has been reached")]
    MaxCustomRecords,

    #[error("Not found")]
    NotFound,

    #[error("The specified domain is not valid with the parent domain")]
    InvalidDomainWithParent,

    #[error("The specified domain is not valid")]
    InvalidDomain,

    #[error("The record data is too long")]
    RecordDataTooLong,

    #[error("The specified domain is expired")]
    DomainExpired,
}
