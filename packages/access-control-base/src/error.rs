use thiserror::Error;

/// ## Description
/// This enum describes access control extension error
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("AccessControl-base: Specified address is missing role")]
    MissingRole,
}
