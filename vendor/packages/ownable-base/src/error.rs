use thiserror::Error;

/// ## Description
/// This enum describes ownable extension error
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("Ownable-base: caller is not the owner")]
    CallerIsNotTheOwner,
}
