use pbc_contract_common::address::Address;

pub struct ACInitMsg<RoleEnum> {
    pub admin_role: RoleEnum,
    pub admin_addresses: Vec<Address>,
    pub additional_roles: Vec<RoleEnum>,
}

pub struct ACRoleMsg<RoleEnum> {
    pub role: RoleEnum,
    pub account: Address,
}

pub struct ACSetAdminRoleMsg<RoleEnum> {
    pub role: RoleEnum,
    pub new_admin_role: RoleEnum,
}
