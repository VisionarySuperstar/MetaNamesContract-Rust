use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::{Address, Shortname};
use read_write_rpc_derive::ReadWriteRPC;

use rpc_msg_derive::IntoShortnameRPCEvent;
use utils::events::IntoShortnameRPCEvent;

/// ## Description
/// This structure describes fields for mpc721 initialize msg
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
/// This structure describes fields for mpc721 transfer msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x01)]
pub struct TransferMsg {
    /// receiver address
    pub to: Address,
    /// token id
    pub token_id: u128,
}

/// ## Description
/// This structure describes fields for mpc721 transfer from msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x03)]
pub struct TransferFromMsg {
    /// owner address
    pub from: Address,
    /// receiver address
    pub to: Address,
    /// token id
    pub token_id: u128,
}

/// ## Description
/// This structure describes fields for mpc721 approve msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x05)]
pub struct ApproveMsg {
    /// operator address to approve
    pub spender: Address,
    /// token id
    pub token_id: u128,
}

/// ## Description
/// This structure describes fields for mpc721 set base uri msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x07)]
pub struct SetBaseUriMsg {
    /// new base uri
    pub new_base_uri: String,
}

/// ## Description
/// This structure describes fields for mpc721 mint msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x09)]
pub struct MintMsg {
    /// newly minted token id
    pub token_id: u128,
    /// receiver address
    pub to: Address,
    /// optional token uri
    pub token_uri: Option<String>,
}

/// ## Description
/// This structure describes fields for mpc721 approve for all msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x11)]
pub struct ApproveForAllMsg {
    /// operator address to approve
    pub operator: Address,
}

/// ## Description
/// This structure describes fields for mpc721 revoke msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x13)]
pub struct RevokeMsg {
    /// operator address to revoke
    pub spender: Address,
    /// token id
    pub token_id: u128,
}

/// ## Description
/// This structure describes fields for mpc721 revoke for all msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x15)]
pub struct RevokeForAllMsg {
    /// operator address to revoke
    pub operator: Address,
}

/// ## Description
/// This structure describes fields for mpc721 burn msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x17)]
pub struct BurnMsg {
    /// token id to burn
    pub token_id: u128,
}

/// ## Description
/// This structure describes fields for mpc721 check owner msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x18)]
pub struct CheckOwnerMsg {
    /// receiver address
    pub owner: Address,
    /// token id
    pub token_id: u128,
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
