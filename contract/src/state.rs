use access_control::state::AccessControlState;
use airdrop::state::AirdropState;
use contract_version_base::state::ContractVersionBase;
use create_type_spec_derive::CreateTypeSpec;
use nft::state::NFTContractState;
use partisia_name_system::state::PartisiaNameSystemState;
use pbc_contract_common::{address::Address, avl_tree_map::AvlTreeMap};
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

#[allow(unused_imports)]
use crate::contract::__PBC_IS_ZK_CONTRACT;

#[state]
#[derive(Default, Debug)]
pub struct ContractState {
    pub access_control: AccessControlState,
    pub airdrop: AirdropState,
    pub config: ContractConfig,
    pub nft: NFTContractState,
    pub pns: PartisiaNameSystemState,
    pub stats: ContractStats,
    pub version: ContractVersionBase,
}

#[repr(C)]
#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, PartialEq, Eq, Default, Clone, Debug)]
pub struct PaymentInfo {
    // Those are required but need to be optional for Default trait to work
    pub id: u64,
    pub token: Option<Address>,
    pub receiver: Option<Address>,
    pub fees: Fees,
}

#[repr(u8)]
#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, PartialEq, Eq, Copy, Clone, Debug)]
pub enum UserRole {
    #[discriminant(0)]
    Admin {},
    #[discriminant(1)]
    Whitelist {},
    #[discriminant(2)]
    Airdrop {},
}

#[repr(C)]
#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, PartialEq, Eq, Default, Clone, Debug)]
pub struct Fee {
    pub chars_count: u32,
    pub amount: u128,
}

#[repr(C)]
#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, PartialEq, Eq, Default, Clone, Debug)]
pub struct Fees {
    pub mapping: Vec<Fee>,
    pub default_fee: u128,
    pub decimals: u32,
}

#[repr(C)]
#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, PartialEq, Eq, Default, Clone, Debug)]
pub struct ContractConfig {
    pub contract_enabled: bool,
    pub mint_count_limit_enabled: bool,
    pub mint_count_limit: u32,
    pub payment_info: Vec<PaymentInfo>,
    pub whitelist_enabled: bool,
}

#[repr(C)]
#[derive(ReadWriteState, CreateTypeSpec, Default, Debug)]
pub struct ContractStats {
    pub mint_count: AvlTreeMap<Address, u32>,
}

impl ContractConfig {
    pub fn get_payment_info(&self, id: u64) -> Option<PaymentInfo> {
        for info in &self.payment_info {
            if info.id == id {
                return Some(info.clone());
            }
        }

        None
    }
}

impl ContractStats {
    pub fn increase_mint_count(&mut self, address: Address) {
        let count = self.mint_count.get(&address).unwrap_or(0);
        self.mint_count.insert(address, count + 1);
    }
}

impl Fees {
    pub fn get(&self, domain: &str) -> u128 {
        let decimals = 10_u128.pow(self.decimals);

        let chars_count = domain.chars().count() as u32;
        for fee in &self.mapping {
            if fee.chars_count == chars_count {
                return fee.amount * decimals;
            }
        }

        self.default_fee * decimals
    }
}
