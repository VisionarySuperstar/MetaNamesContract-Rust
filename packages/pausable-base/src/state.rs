use create_type_spec_derive::CreateTypeSpec;
use read_write_state_derive::ReadWriteState;

use crate::ContractError;

/// ## Description
/// This structure describes pausable extension state
#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct PausableBaseState {
    /// paused or not
    paused: bool,
}

impl PausableBaseState {
    /// ## Description
    /// Creates pausable extension state
    pub fn new() -> Self {
        Self { paused: false }
    }

    /// ## Description
    /// Pauses contract
    pub fn pause(&mut self) {
        self.assert_not_paused();
        self.paused = true
    }

    /// ## Description
    /// Unpauses contract
    pub fn unpause(&mut self) {
        self.assert_paused();
        self.paused = false
    }

    /// ## Description
    /// Returns current pausable state
    pub fn paused(&self) -> bool {
        self.paused
    }

    /// ## Description
    /// Verifies that contract is paused
    pub fn assert_paused(&self) {
        assert!(self.paused(), "{}", ContractError::NotPaused)
    }

    /// ## Description
    /// Verifies that contract is not paused
    pub fn assert_not_paused(&self) {
        assert!(!self.paused(), "{}", ContractError::Paused)
    }
}

impl Default for PausableBaseState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_proper_pausable() {
        let mut pausable_state = PausableBaseState::new();
        assert_eq!(pausable_state.paused(), false);

        pausable_state.pause();
        assert_eq!(pausable_state.paused(), true);
        pausable_state.assert_paused();

        pausable_state.unpause();
        assert_eq!(pausable_state.paused(), false);
        pausable_state.assert_not_paused()
    }

    #[test]
    #[should_panic(expected = "Pausable-base: paused")]
    fn test_pause_when_already_paused() {
        let mut pausable_state = PausableBaseState::new();
        pausable_state.pause();

        pausable_state.pause()
    }

    #[test]
    #[should_panic(expected = "Pausable-base: not paused")]
    fn test_unpause_when_already_unpaused() {
        let mut pausable_state = PausableBaseState::new();
        pausable_state.unpause()
    }
}
