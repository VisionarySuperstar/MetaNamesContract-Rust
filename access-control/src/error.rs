use thiserror::Error;

/// This enum describes access control extension error
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("AccessControl-base: Specified address is missing role")]
    MissingRole,
    #[error("AccessControl-base: Specified role is missing the admin role")]
    MissingAdminRole,
}
