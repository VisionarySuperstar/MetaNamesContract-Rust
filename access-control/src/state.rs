use pbc_contract_common::{
    address::Address,
    context::ContractContext,
    sorted_vec_map::{SortedVecMap, SortedVecSet},
};
use pbc_traits::CreateTypeSpec;
use pbc_traits::ReadWriteState;
use read_write_state_derive::ReadWriteState;

use std::collections::BTreeMap;

/// This structure describes access control extension state
#[repr(C)]
#[derive(ReadWriteState, Clone, PartialEq, Eq, Debug, Default)]
pub struct AccessControlState<RoleEnum: Ord + Clone> {
    pub roles_admin: SortedVecMap<RoleEnum, RoleEnum>,
    pub roles_addresses: SortedVecSet<RoleAddress<RoleEnum>>,
}

/// This structure describes role with some granted access control
#[repr(C)]
#[derive(ReadWriteState, Clone, PartialEq, PartialOrd, Eq, Ord, Debug)]
pub struct RoleAddress<RoleEnum: Ord + Clone> {
    pub role: RoleEnum,
    pub address: Address,
}

impl<RoleEnum: Ord + Clone> AccessControlState<RoleEnum> {
    pub fn new() -> Self {
        AccessControlState {
            roles_admin: Default::default(),
            roles_addresses: Default::default(),
        }
    }

    /// Returns either address has specified role or not
    pub fn has_role(&self, role: RoleEnum, address: &Address) -> bool {
        let role_address = &RoleAddress {
            role,
            address: *address,
        };

        self.roles_addresses.contains(role_address)
    }

    /// Returns admin role of specified role
    pub fn get_role_admin(&self, role: RoleEnum) -> Option<&RoleEnum> {
        self.roles_admin.get(&role)
    }

    /// Setups new role
    pub fn _setup_role(&mut self, role: RoleEnum, admin_role: RoleEnum, accounts: &[Address]) {
        self._set_role_admin(role.clone(), admin_role);

        for account in accounts {
            self._set_role(role.clone(), account);
        }
    }

    /// Removes role access for specified account
    pub fn _revoke_role(&mut self, role: RoleEnum, address: &Address) {
        if self.has_role(role.clone(), address) {
            self.roles_addresses.remove(&RoleAddress {
                role,
                address: *address,
            });
        }
    }

    /// Removes sender access to role
    pub fn _renounce_role(&mut self, role: RoleEnum, ctx: &ContractContext) {
        if self.has_role(role.clone(), &ctx.sender) {
            self.roles_addresses.remove(&RoleAddress {
                role,
                address: ctx.sender,
            });
        }
    }

    /// Sets new tole admin for role
    pub fn _set_role_admin(&mut self, role: RoleEnum, admin_role: RoleEnum) {
        self.roles_admin.insert(role, admin_role);
    }

    pub fn _set_role(&mut self, role: RoleEnum, address: &Address) {
        if !self.has_role(role.clone(), address) {
            self.roles_addresses.insert(RoleAddress {
                role,
                address: *address,
            });
        }
    }
}

// Implementations to fix the CreateTypeSpec bug
#[cfg(feature = "abi")]
impl<RoleEnum: CreateTypeSpec + Ord + Clone> CreateTypeSpec for AccessControlState<RoleEnum> {
    fn __ty_name() -> String {
        SortedVecMap::<RoleEnum, RoleEnum>::__ty_name()
    }
    fn __ty_identifier() -> String {
        SortedVecMap::<RoleEnum, RoleEnum>::__ty_identifier()
    }
    fn __ty_spec_write(w: &mut Vec<u8>, lut: &BTreeMap<String, u8>) {
        SortedVecMap::<RoleEnum, RoleEnum>::__ty_spec_write(w, lut)
    }
}

#[cfg(feature = "abi")]
impl<RoleEnum: CreateTypeSpec + Ord + Clone> CreateTypeSpec for RoleAddress<RoleEnum> {
    fn __ty_name() -> String {
        SortedVecMap::<RoleEnum, RoleEnum>::__ty_name()
    }
    fn __ty_identifier() -> String {
        SortedVecMap::<RoleEnum, RoleEnum>::__ty_identifier()
    }
    fn __ty_spec_write(w: &mut Vec<u8>, lut: &BTreeMap<String, u8>) {
        SortedVecMap::<RoleEnum, RoleEnum>::__ty_spec_write(w, lut)
    }
}
