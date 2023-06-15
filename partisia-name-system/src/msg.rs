use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::{Address, Shortname};
use read_write_rpc_derive::ReadWriteRPC;

use rpc_msg_derive::IntoShortnameRPCEvent;
use utils::events::IntoShortnameRPCEvent;

use crate::state::RecordClass;

/// ## Description
/// This structure describes fields for PNS initialize msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct PnsInitMsg {
    /// optional owner address
    pub owner: Option<Address>,
    /// token name
    pub name: String,
    /// token symbol
    pub symbol: String,
    /// optional base uri
    pub base_uri: Option<String>,
    /// token minter address
    pub minter: Address,
    /// optional top level domain
    pub tld: Option<Vec<u8>>,
    /// optional top level domain uri
    pub tld_uri: Option<String>,
}

/// ## Description
/// This structure describes fields for PNS mint msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x09)]
pub struct PnsMintMsg {
    pub domain: Vec<u8>,
    /// NFT token id
    pub token_id: u128,
    /// receiver address
    pub to: Address,
    /// optional token_uri
    pub token_uri: Option<String>,
    /// optional parent
    pub parent_id: Option<Vec<u8>>,
}

/// ## Description
/// This structure describes fields for PNS Record Mint Msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x21)]
pub struct RecordMintMsg {
    pub domain: Vec<u8>,
    /// Class type
    pub class: RecordClass,
    /// Data
    pub data: String,
}

/// ## Description
/// This structure describes fields for the record update msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x22)]
pub struct RecordUpdateMsg {
    pub domain: Vec<u8>,
    /// Class type
    pub class: RecordClass,
    /// Data
    pub data: String,
}

/// ## Description
/// This structure describes fields for the Record Delete Msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x23)]
pub struct RecordDeleteMsg {
    pub domain: Vec<u8>,
    /// Class type
    pub class: RecordClass,
}
