use pbc_contract_common::address::Address;

pub struct ACInitMsg {
    pub admin_addresses: Vec<Address>,
}

pub struct ACRoleMsg {
    pub role: u8,
    pub account: Address,
}

pub struct ACSetAdminRoleMsg {
    pub role: u8,
    pub new_admin_role: u8,
}
