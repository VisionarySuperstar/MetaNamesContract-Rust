use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::{Address, Shortname};
use read_write_rpc_derive::ReadWriteRPC;

use rpc_msg_derive::IntoShortnameRPCEvent;
use utils::events::IntoShortnameRPCEvent;

use crate::state::ContractConfig;

/// This structure describes fields for PNS initialize msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct InitMsg {
    /// token name
    pub name: String,
    /// token symbol
    pub symbol: String,
    pub uri_template: String,
    pub admin_addresses: Vec<Address>,
    pub config: ContractConfig,
}

/// This structure describes fields for mint msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x09)]
pub struct MintMsg {
    pub domain: String,
    /// receiver address
    pub to: Address,
    /// BYOC token id
    pub payment_coin_id: u64,
    /// optional token_uri
    pub token_uri: Option<String>,
    /// optional parent
    pub parent_id: Option<String>,
    /// Subscription years
    pub subscription_years: Option<u32>,
}

#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x03)]
pub struct MPC20TransferFromMsg {
    /// token owner
    pub from: Address,
    /// token receiver
    pub to: Address,
    /// amount to receive
    pub amount: u128,
}

/// This structure describes fields for renew msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x26)]
pub struct RenewDomainMsg {
    pub domain: String,
    pub payer: Address,
    /// BYOC token id
    pub payment_coin_id: u64,
    pub subscription_years: u32,
}

// Events structs

/// Struct for owner info event
/// Copy this struct to the external contracts that are using this action
/// to be able to decode the event data
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct OwnerInfoEvent {
    pub owner: Address,
    pub domain_count: u128,
    pub total_supply: u128,
}
