use std::collections::BTreeMap;

use contract_version_base::state::ContractVersionBase;
use create_type_spec_derive::CreateTypeSpec;
use mpc721_hierarchy::state::MPC721ContractState;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

/// ## Description
/// This structure describes partisia name system state
#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct PartisiaNameSystemState {
    pub mpc721: MPC721ContractState,
    pub version: ContractVersionBase,
    pub domains: BTreeMap<String, Domain>
}

#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct Domain {
    pub token_id: u128,
}

#[repr(u8)]
#[derive(
    Eq, PartialEq, Debug, Clone, Ord, PartialOrd, Copy, CreateTypeSpec, ReadWriteState, ReadWriteRPC,
)]
pub enum RecordClass {
    /// Wallet
    #[discriminant(0)]
    Wallet {},
    /// Website
    #[discriminant(1)]
    Uri {},
    /// Twitter
    #[discriminant(2)]
    Twitter {},
}

impl PartisiaNameSystemState {
    /// ## Description
    /// This function returns token id for given domain
    /// ## Params
    /// * `domain` is an object of type [`String`]
    pub fn get_token_id(&self, domain: &String) -> Option<u128> {
        self.domains.get(domain).map(|d| d.token_id)
    }

    /// ## Description
    /// Says is token id minted or not
    /// ## Params
    /// * **token_id** is an object of type [`String`]
    pub fn is_minted(&self, token_id: &String) -> bool {
        self.domains.contains_key(token_id)
    }
}
