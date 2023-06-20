use thiserror::Error;

/// ## Description
/// This enum describes mpc721 contract errors
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("The specified domain is already minted")]
    Minted,
}
