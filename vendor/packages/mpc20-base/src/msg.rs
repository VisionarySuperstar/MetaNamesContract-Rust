use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::{Address, Shortname};
use read_write_rpc_derive::ReadWriteRPC;

use crate::state::{Minter, TokenInfo};

use rpc_msg_derive::IntoShortnameRPCEvent;
use utils::events::IntoShortnameRPCEvent;

/// ## Description
/// This structure describes fields for mpc20 initial balances
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct InitialBalance {
    /// initial holder address
    pub address: Address,
    /// initial amount
    pub amount: u128,
}

/// ## Description
/// This structure describes fields for mpc20 initialize msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct Mpc20InitMsg {
    /// mpc20 token information
    pub info: TokenInfo,
    /// initial balances
    pub initial_balances: Vec<InitialBalance>,
    /// optional address allowed to mint new tokens
    pub minter: Option<Minter>,
}

impl Mpc20InitMsg {
    pub fn capacity(&self) -> Option<u128> {
        self.minter.as_ref().and_then(|m| m.capacity)
    }

    pub fn validate(&self) {
        self.validate_name();
        self.validate_symbol();
        assert!(self.info.decimals <= 18, "Decimals must not exceed 18");

        self.validate_initial_balances();
    }

    fn validate_name(&self) {
        let bytes = self.info.name.as_bytes();
        assert!(
            bytes.len() >= 3 && bytes.len() <= 50,
            "Name is not in the expected length. Must be 3-50"
        );
    }

    fn validate_symbol(&self) {
        let bytes = self.info.symbol.as_bytes();
        assert!(
            bytes.len() >= 3 && bytes.len() <= 12,
            "Ticker symbol is not in expected length. Must be 3-12"
        );

        for byte in bytes.iter() {
            if (*byte != 45) && (*byte < 65 || *byte > 90) && (*byte < 97 || *byte > 122) {
                panic!("Ticker symbol is not in expected format. Must be [a-zA-Z\\-]")
            }
        }
    }

    fn validate_initial_balances(&self) {
        let mut addrs = self
            .initial_balances
            .clone()
            .into_iter()
            .map(|b| b.address)
            .collect::<Vec<_>>();
        addrs.sort();
        addrs.dedup();

        assert!(
            addrs.len() == self.initial_balances.len(),
            "Duplicate addresses in initial balances list"
        );
    }
}

/// ## Description
/// This structure describes fields for mpc20 transfer msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x01)]
pub struct TransferMsg {
    /// token receiver
    pub to: Address,
    /// amount to receive
    pub amount: u128,
}

/// ## Description
/// This structure describes fields for mpc20 trasnfer from msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x03)]
pub struct TransferFromMsg {
    /// token owner
    pub from: Address,
    /// token receiver
    pub to: Address,
    /// amount to receive
    pub amount: u128,
}

/// ## Description
/// This structure describes fields for mpc20 approve msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x05)]
pub struct ApproveMsg {
    /// approved address
    pub spender: Address,
    /// approved amount
    pub amount: u128,
}

/// ## Description
/// This structure describes fields for mpc20 mint msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x07)]
pub struct MintMsg {
    /// token receiver
    pub recipient: Address,
    /// amount to receive
    pub amount: u128,
}

/// ## Description
/// This structure describes fields for mpc20 burn msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x09)]
pub struct BurnMsg {
    /// amount of tokens to burn
    pub amount: u128,
}

/// ## Description
/// This structure describes fields for mpc20 burn from msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x11)]
pub struct BurnFromMsg {
    /// token owner
    pub owner: Address,
    /// amount of tokens to burn
    pub amount: u128,
}

/// ## Description
/// This structure describes fields for mpc20 increase allowance msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x13)]
pub struct IncreaseAllowanceMsg {
    /// approved spender
    pub spender: Address,
    /// approved amount to increase
    pub amount: u128,
}

/// ## Description
/// This structure describes fields for mpc20 decrease allowance msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x15)]
pub struct DecreaseAllowanceMsg {
    /// approved spender
    pub spender: Address,
    /// approved amount to decrease
    pub amount: u128,
}
