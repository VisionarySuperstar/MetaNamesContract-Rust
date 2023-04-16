use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::{Address, Shortname};
use read_write_rpc_derive::ReadWriteRPC;

use rpc_msg_derive::IntoShortnameRPCEvent;
use utils::events::IntoShortnameRPCEvent;

/// ## Description
/// This structure describes fields for PNS initialize msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct InitMsg {
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
}

/// ## Description
/// This structure describes fields for PNS transfer msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x01)]
pub struct TransferMsg {
    /// receiver address
    pub to: Address,
    /// token id
    pub token_id: String,
}

/// ## Description
/// This structure describes fields for PNS transfer from msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x03)]
pub struct TransferFromMsg {
    /// owner address
    pub from: Address,
    /// receiver address
    pub to: Address,
    /// token id
    pub token_id: String,
}

/// ## Description
/// This structure describes fields for PNS approve msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x05)]
pub struct ApproveMsg {
    /// operator address to approve
    pub spender: Address,
    /// token id
    pub token_id: String,
}

/// ## Description
/// This structure describes fields for PNS set base uri msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x07)]
pub struct SetBaseUriMsg {
    /// new base uri
    pub new_base_uri: String,
}

/// ## Description
/// This structure describes fields for PNS mint msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x09)]
pub struct MintMsg {
    /// newly minted token id
    pub token_id: String,
    /// receiver address
    pub to: Address,
    /// parent
    pub parent: Option<String>,
}

/// ## Description
/// This structure describes fields for PNS approve for all msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x11)]
pub struct ApproveForAllMsg {
    /// operator address to approve
    pub operator: Address,
}

/// ## Description
/// This structure describes fields for PNS revoke msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x13)]
pub struct RevokeMsg {
    /// operator address to revoke
    pub spender: Address,
    /// token id
    pub token_id: String,
}

/// ## Description
/// This structure describes fields for PNS revoke for all msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x15)]
pub struct RevokeForAllMsg {
    /// operator address to revoke
    pub operator: Address,
}

/// ## Description
/// This structure describes fields for PNS burn msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x17)]
pub struct BurnMsg {
    /// token id to burn
    pub token_id: String,
}

/// ## Description
/// This structure describes fields for PNS check owner msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x18)]
pub struct CheckOwnerMsg {
    /// receiver address
    pub owner: Address,
    /// token id
    pub token_id: String,
}
/// ## Description
/// This structure describes fields for the Update Minter Msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x19)]
pub struct UpdateMinterMsg {
    /// operator address to approve
    pub new_minter: Address,
}

/// ## Description
/// This structure describes fields for the Multi Mint Msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x20)]
pub struct MultiMintMsg {
    /// array of MintMsgs to mint multiple nfts
    pub mints: Vec<MintMsg>,
}

/// ## Description
/// This structure describes fields for PNS Record Mint Msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x21)]
pub struct RecordMintMsg {
    /// Related domain
    pub token_id: String,
    /// Class type
    pub class: u8,
    /// Data
    pub data: String,
}

/// ## Description
/// This structure describes fields for the record update msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x22)]
pub struct RecordUpdateMsg {
    /// Related domain
    pub token_id: String,
    /// Class type
    pub class: u8,
    /// Data
    pub data: String,
}

/// ## Description
/// This structure describes fields for the Record Delete Msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x23)]
pub struct RecordDeleteMsg {
    /// Related domain
    pub token_id: String,
    /// Class type
    pub class: u8,
}
