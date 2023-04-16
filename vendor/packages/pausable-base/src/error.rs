use thiserror::Error;

/// ## Description
/// This enum describes pausable extension error
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("Pausable-base: paused")]
    Paused,

    #[error("Pausable-base: not paused")]
    NotPaused,
}
