use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::{Address, Shortname};
use read_write_rpc_derive::ReadWriteRPC;

use rpc_msg_derive::IntoShortnameRPCEvent;
use utils::events::IntoShortnameRPCEvent;

use crate::state::ContractConfig;

/// ## Description
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

/// ## Description
/// This structure describes fields for mint msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x09)]
pub struct MintMsg {
    pub domain: String,
    /// receiver address
    pub to: Address,
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

/// ## Description
/// This structure describes fields for renew msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x26)]
pub struct RenewDomainMsg {
    pub domain: String,
    pub payer: Address,
    pub subscription_years: u32,
}
