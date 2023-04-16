use std::collections::BTreeMap;

use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::{address::Address, context::ContractContext};
use read_write_state_derive::ReadWriteState;

use crate::ContractError;

pub const DEFAULT_ADMIN_ROLE: u8 = 0x00;

/// ## Description
/// This structure describes access control extension state
#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug, Default)]
pub struct AccessControlBaseState {
    /// configured roles
    pub roles: BTreeMap<u8, Role>,
}

/// ## Description
/// This structure describes role with some granted access control
#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct Role {
    /// configured admin role
    pub admin_role: u8,
    /// whitelisted role members
    pub members: BTreeMap<Address, bool>,
}

impl AccessControlBaseState {
    /// ## Description
    /// Grants specified tole to specified account
    /// ## Params
    /// * **role** is an object of type [`u8`]
    ///
    /// * **account** is an object of type [`Address`]
    ///
    /// * **ctx** is an object of type [`ContractContext`]
    pub fn grant_role(&mut self, role: u8, account: &Address, ctx: &ContractContext) {
        self.assert_only_role(self.get_role_admin(role).unwrap(), ctx);
        self.set_role(role, account);
    }

    /// ## Description
    /// Setups new role
    /// ## Params
    /// * **role** is an object of type [`u8`]
    ///
    /// * **account** is an object of type [`Address`]
    pub fn setup_role(&mut self, role: u8, account: &Address) {
        self.set_role(role, account);
    }

    /// ## Description
    /// Removes role access for specified account
    /// ## Params
    /// * **role** is an object of type [`u8`]
    ///
    /// * **account** is an object of type [`Address`]
    ///
    /// * **ctx** is an object of type [`ContractContext`]
    pub fn revoke_role(&mut self, role: u8, account: &Address, ctx: &ContractContext) {
        self.assert_only_role(self.get_role_admin(role).unwrap(), ctx);

        if self.has_role(role, account) {
            self.roles.entry(role).and_modify(|role| {
                role.members.remove(account);
            });
        }
    }

    /// ## Description
    /// Removes sender access to role
    /// ## Params
    /// * **role** is an object of type [`u8`]
    ///
    /// * **ctx** is an object of type [`ContractContext`]
    pub fn renounce_role(&mut self, role: u8, ctx: &ContractContext) {
        if self.has_role(role, &ctx.sender) {
            self.roles.entry(role).and_modify(|role| {
                role.members.remove(&ctx.sender);
            });
        }
    }

    /// ## Description
    /// Sets new tole admin for role
    /// ## Params
    /// * **role** is an object of type [`u8`]
    ///
    /// * **admin_role** is an object of type [`u8`]
    pub fn set_role_admin(&mut self, role: u8, admin_role: u8) {
        self.roles
            .entry(role)
            .and_modify(|role| role.admin_role = admin_role)
            .or_insert(Role {
                admin_role,
                members: BTreeMap::new(),
            });
    }

    /// ## Description
    /// Validates that only specified role member can have access
    /// ## Params
    /// * **role** is an object of type [`u8`]
    ///
    /// * **ctx** is an object of type [`ContractContext`]
    pub fn assert_only_role(&self, role: u8, ctx: &ContractContext) {
        assert!(
            self.has_role(role, &ctx.sender),
            "{}",
            ContractError::MissingRole
        );
    }

    /// ## Description
    /// Returns either address has specified role or not
    pub fn has_role(&self, role: u8, account: &Address) -> bool {
        if let Some(role) = self.roles.get(&role) {
            return *role.members.get(account).unwrap_or(&false);
        }

        false
    }

    /// ## Description
    /// Returns admin role of specified role
    pub fn get_role_admin(&self, role: u8) -> Option<u8> {
        if let Some(role) = self.roles.get(&role) {
            return Some(role.admin_role);
        }

        None
    }

    fn set_role(&mut self, role: u8, account: &Address) {
        if !self.has_role(role, account) {
            self.roles
                .entry(role)
                .and_modify(|role| {
                    role.members.insert(*account, true);
                })
                .or_insert(Role {
                    admin_role: DEFAULT_ADMIN_ROLE,
                    members: BTreeMap::from([(*account, true)]),
                });
        }
    }
}

#[cfg(test)]
mod tests {
    use pbc_contract_common::address::AddressType;

    use super::*;

    const ROLE_A: u8 = 0x02;
    const ROLE_B: u8 = 0x03;

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
            contract_address: mock_address(20u8),
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
    fn proper_access_control() {
        let alice = mock_address(1u8);
        let bob = mock_address(2u8);
        let jack = mock_address(3u8);

        let mut access_control = AccessControlBaseState::default();

        assert_eq!(access_control.has_role(ROLE_A, &alice), false);
        assert_eq!(access_control.get_role_admin(ROLE_A), None);

        access_control.setup_role(DEFAULT_ADMIN_ROLE, &alice);
        assert_eq!(access_control.has_role(ROLE_A, &alice), false);
        assert_eq!(access_control.has_role(DEFAULT_ADMIN_ROLE, &alice), true);

        assert_eq!(access_control.has_role(ROLE_B, &bob), false);
        access_control.setup_role(ROLE_B, &bob);
        assert_eq!(access_control.has_role(ROLE_B, &bob), true);

        assert_eq!(access_control.has_role(ROLE_B, &jack), false);
        access_control.grant_role(ROLE_B, &jack, &mock_contract_context(1u8));
        assert_eq!(access_control.has_role(ROLE_B, &jack), true);

        assert_eq!(access_control.get_role_admin(ROLE_B), Some(0x00));

        access_control.assert_only_role(ROLE_B, &mock_contract_context(3u8));

        assert_eq!(access_control.has_role(ROLE_B, &jack), true);
        access_control.revoke_role(ROLE_B, &jack, &mock_contract_context(1u8));
        assert_eq!(access_control.has_role(ROLE_B, &jack), false);

        access_control.setup_role(ROLE_A, &bob);
        access_control.set_role_admin(ROLE_A, ROLE_B);

        assert_eq!(access_control.get_role_admin(ROLE_A), Some(0x03));

        access_control.renounce_role(DEFAULT_ADMIN_ROLE, &mock_contract_context(1u8));
        assert_eq!(access_control.has_role(DEFAULT_ADMIN_ROLE, &alice), false);
    }

    #[test]
    #[should_panic(expected = "AccessControl-base: Specified address is missing role")]
    fn test_role_mismatch() {
        let jack = mock_address(3u8);

        let mut access_control = AccessControlBaseState::default();

        access_control.setup_role(DEFAULT_ADMIN_ROLE, &jack);

        access_control.assert_only_role(ROLE_A, &mock_contract_context(3u8));
    }
}
