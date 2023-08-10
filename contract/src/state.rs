use access_control::state::AccessControlState;
use contract_version_base::state::ContractVersionBase;
use create_type_spec_derive::CreateTypeSpec;
use nft::state::NFTContractState;
use partisia_name_system::state::PartisiaNameSystemState;
use pbc_contract_common::{address::Address, sorted_vec_map::SortedVecMap};
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

#[allow(unused_imports)]
use crate::contract::__PBC_IS_ZK_CONTRACT;

#[state]
#[derive(PartialEq, Eq, Default, Clone, Debug)]
pub struct ContractState {
    pub access_control: AccessControlState,
    pub config: ContractConfig,
    pub nft: NFTContractState,
    pub pns: PartisiaNameSystemState,
    pub stats: ContractStats,
    pub version: ContractVersionBase,
}

#[derive(
    ReadWriteRPC, ReadWriteState, CreateTypeSpec, PartialEq, Eq, Default, Copy, Clone, Debug,
)]
pub struct PayableMintInfo {
    // Those are required but need to be optional for Default trait to work
    pub token: Option<Address>,
    pub receiver: Option<Address>,
}

#[repr(u8)]
#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, PartialEq, Eq, Copy, Clone, Debug)]
pub enum UserRole {
    #[discriminant(0)]
    Admin {},
    #[discriminant(1)]
    Whitelist {},
}

#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, PartialEq, Eq, Default, Clone, Debug)]
pub struct ContractConfig {
    pub contract_enabled: bool,
    pub mint_count_limit_enabled: bool,
    pub mint_count_limit: u32,
    pub payable_mint_info: PayableMintInfo,
    pub whitelist_enabled: bool,
}

#[derive(ReadWriteState, CreateTypeSpec, PartialEq, Eq, Default, Clone, Debug)]
pub struct ContractStats {
    pub mint_count: SortedVecMap<Address, u32>,
}

impl ContractStats {
    pub fn increate_mint_count(&mut self, address: Address) {
        let count = self.mint_count.get(&address).unwrap_or(&0);
        self.mint_count.insert(address, count + 1);
    }
}
