use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::{Address, Shortname};
use read_write_rpc_derive::ReadWriteRPC;

use rpc_msg_derive::IntoShortnameRPCEvent;
use utils::events::IntoShortnameRPCEvent;

/// ## Description
/// This structure describes fields for mpc1155 initialize msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct InitMsg {
    /// optional owner address
    pub owner: Option<Address>,
    /// base uri
    pub uri: String,
    /// minter address
    pub minter: Address,
}

/// ## Description
/// This structure describes fields for mpc1155 transfer from msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x01)]
pub struct TransferFromMsg {
    /// owner address
    pub from: Address,
    /// receiver address
    pub to: Address,
    /// token info for transfer
    pub token_info: TokenTransferInfoMsg,
}

/// ## Description
/// This structure describes fields for mpc1155 batch transfer from msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x03)]
pub struct BatchTransferFromMsg {
    /// owner address
    pub from: Address,
    /// receiver address
    pub to: Address,
    /// list of token infos for transfer
    pub token_infos: Vec<TokenTransferInfoMsg>,
}

/// ## Description
/// This structure describes fields for mpc1155 approve for all msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x05)]
pub struct ApproveForAllMsg {
    /// operator address to approve
    pub operator: Address,
}

/// ## Description
/// This structure describes fields for mpc1155 set uri msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x07)]
pub struct SetUriMsg {
    /// new base uri
    pub new_uri: String,
}

/// ## Description
/// This structure describes fields for mpc1155 mint msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct TokenMintInfoMsg {
    /// token id
    pub token_id: u128,
    /// amount of token to mint
    pub amount: u128,
    /// optional token uri
    pub token_uri: Option<String>,
}

/// ## Description
/// This structure describes fields for mpc1155 mint msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x09)]
pub struct MintMsg {
    pub to: Address,
    pub token_info: TokenMintInfoMsg,
}

/// ## Description
/// This structure describes fields for mpc1155 batch mint msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x11)]
pub struct BatchMintMsg {
    /// receiver address
    pub to: Address,
    /// list of tokens to mint
    pub token_infos: Vec<TokenMintInfoMsg>,
}

/// ## Description
/// This structure describes fields for mpc1155 transfer msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct TokenTransferInfoMsg {
    /// token id
    pub token_id: u128,
    /// amount of tokens to transfer
    pub amount: u128,
}

/// ## Description
/// This structure describes fields for mpc1155 burn msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x13)]
pub struct BurnMsg {
    /// owner address
    pub from: Address,
    /// token info for burn
    pub token_info: TokenTransferInfoMsg,
}

/// ## Description
/// This structure describes fields for mpc1155 batch burn msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x15)]
pub struct BatchBurnMsg {
    /// owner address
    pub from: Address,
    /// list of token infos for burn
    pub token_infos: Vec<TokenTransferInfoMsg>,
}

/// ## Description
/// This structure describes fields for mpc1155 revoke for all msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x17)]
pub struct RevokeForAllMsg {
    /// operator address to revoke
    pub operator: Address,
}
/// ## Description
/// This structure descibes fields for the mpc1155 balance check function
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x18)]
pub struct CheckBalancesMsg {
    /// operator address to revoke
    pub owner: Address,
    // vector of ids being checked
    pub token_ids: Vec<u128>,
    // vectore of token amounts being checked
    pub amounts: Vec<u128>,
}
