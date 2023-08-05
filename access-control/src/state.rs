use std::vec;

use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::{
    address::Address, context::ContractContext, sorted_vec_map::SortedVecMap,
};
use read_write_state_derive::ReadWriteState;

pub const DEFAULT_ADMIN_ROLE: u8 = 0x00;

/// ## Description
/// This structure describes access control extension state
#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug, Default)]
pub struct AccessControlState {
    /// configured roles
    pub roles: SortedVecMap<u8, Role>,
}

/// ## Description
/// This structure describes role with some granted access control
#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct Role {
    /// configured admin role
    pub admin_role: u8,
    /// whitelisted role members
    pub members: Vec<Address>,
}

impl AccessControlState {
    /// ## Description
    /// Returns either address has specified role or not
    pub fn has_role(&self, role: u8, account: &Address) -> bool {
        if let Some(role) = self.roles.get(&role) {
            return role.members.contains(account);
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

    /// ## Description
    /// Setups new role
    pub fn _setup_role(&mut self, role: u8, accounts: &Vec<Address>) {
        for account in accounts {
            self._set_role(role, account);
        }
    }

    /// ## Description
    /// Removes role access for specified account
    pub fn _revoke_role(&mut self, role: u8, account: &Address) {
        if self.has_role(role, account) {
            let role = self.roles.get_mut(&role).unwrap();
            role.members.retain(|addr| addr != account)
        }
    }

    /// ## Description
    /// Removes sender access to role
    pub fn _renounce_role(&mut self, role: u8, ctx: &ContractContext) {
        if self.has_role(role, &ctx.sender) {
            let role = self.roles.get_mut(&role).unwrap();
            role.members.retain(|addr| addr != &ctx.sender)
        }
    }

    /// ## Description
    /// Sets new tole admin for role
    pub fn _set_role_admin(&mut self, role: u8, admin_role: u8) {
        match self.roles.get_mut(&role) {
            Some(role) => role.admin_role = admin_role,
            None => {
                self.roles.insert(
                    role,
                    Role {
                        admin_role,
                        members: vec![],
                    },
                );
            }
        }
    }

    pub fn _set_role(&mut self, role: u8, account: &Address) {
        if !self.has_role(role, account) {
            match self.roles.get_mut(&role) {
                Some(role) => role.members.push(*account),
                None => {
                    self.roles.insert(
                        role,
                        Role {
                            admin_role: DEFAULT_ADMIN_ROLE,
                            members: vec![*account],
                        },
                    );
                }
            }
        }
    }
}
