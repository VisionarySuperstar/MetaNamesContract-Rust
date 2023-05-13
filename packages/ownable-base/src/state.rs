use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::{address::Address, context::ContractContext};
use read_write_state_derive::ReadWriteState;

use crate::ContractError;

/// ## Description
/// This structure describes ownable extension state
#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct OwnableBaseState {
    /// owner address
    owner: Address,
}

impl OwnableBaseState {
    /// ## Description
    /// Creates ownable extension state
    /// ## Params
    /// * **ctx** is an object of type [`ContractContext`]
    pub fn new(ctx: &ContractContext) -> Self {
        Self { owner: ctx.sender }
    }

    /// ## Description
    /// Transfers ownership to the new address
    /// ## Params
    /// * **ctx** is an object of type [`ContractContext`]
    ///
    /// * **new_owner** is an object of type [`Address`]
    pub fn transfer_ownership(&mut self, ctx: &ContractContext, new_owner: Address) {
        self.assert_only_owner(ctx);
        self.change_owner(new_owner)
    }

    fn change_owner(&mut self, new_owner: Address) {
        self.owner = new_owner;
    }

    /// ## Description
    /// Verifies that sender is an owner
    /// ## Params
    /// * **ctx** is an object of type [`ContractContext`]
    pub fn assert_only_owner(&self, ctx: &ContractContext) {
        assert!(
            self.owner == ctx.sender,
            "{}",
            ContractError::CallerIsNotTheOwner
        );
    }

    /// ## Description
    /// Returns current owner address
    pub fn get_owner(&self) -> Address {
        self.owner
    }
}

#[cfg(test)]
mod tests {
    use pbc_contract_common::address::AddressType;

    use super::*;

    const OWNER: u8 = 1u8;
    const NEW_OWNER: u8 = 2u8;

    fn mock_address(le: u8) -> Address {
        Address {
            address_type: AddressType::Account,
            identifier: [
                le, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8,
            ],
        }
    }

    fn mock_contract_context(sender: u8) -> ContractContext {
        ContractContext {
            contract_address: mock_address(1u8),
            sender: mock_address(sender),
            block_time: 100,
            block_production_time: 100,
            current_transaction: [
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            ],
            original_transaction: [
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
                0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            ],
        }
    }

    #[test]
    fn test_proper_ownable() {
        let mut ownable_state = OwnableBaseState::new(&mock_contract_context(OWNER));
        assert_eq!(ownable_state.get_owner(), mock_address(OWNER));

        ownable_state.assert_only_owner(&mock_contract_context(OWNER));

        ownable_state.transfer_ownership(&mock_contract_context(OWNER), mock_address(NEW_OWNER));
        assert_eq!(ownable_state.get_owner(), mock_address(NEW_OWNER));
    }

    #[test]
    #[should_panic(expected = "Ownable-base: caller is not the owner")]
    fn test_not_owner_transferes_ownership() {
        let mut ownable_state = OwnableBaseState::new(&mock_contract_context(OWNER));
        ownable_state.transfer_ownership(&mock_contract_context(NEW_OWNER), mock_address(NEW_OWNER))
    }
}
