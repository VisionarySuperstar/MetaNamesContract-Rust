#![doc = include_str!("../README.md")]

#[macro_use]
extern crate pbc_contract_codegen;
extern crate contract_version_base;
extern crate pbc_contract_common;

use contract_version_base::state::ContractVersionBase;

use pbc_contract_common::{address::Address, context::ContractContext};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// State of the contract
#[state]
struct ContractState {
    pub address: Address,
    pub owner: Address,
    pub version: ContractVersionBase,
}

/// Initialize a new Nickname contract.
///
/// # Arguments
///
/// * `_ctx` - the contract context containing information about the sender and the blockchain.
///
/// # Returns
///
/// The initial state of the contract
#[init]
fn initialize(ctx: ContractContext, address: Address) -> ContractState {
    ContractState {
        owner: ctx.sender,
        address,
        version: ContractVersionBase::new(CONTRACT_NAME, CONTRACT_VERSION),
    }
}

/// Update contract state
///
/// # Arguments
///
/// * `_ctx` - the contract context containing information about the sender and the blockchain.
/// * `state` - the current state of the contract
/// * `address` - new contract address
///
/// # Returns
/// Updated contract state
#[action(shortname = 0x01)]
fn update_address(
    ctx: ContractContext,
    mut state: ContractState,
    address: Address,
) -> ContractState {
    assert_eq!(
        state.owner, ctx.sender,
        "Only the owner can update the address"
    );

    state.address = address;

    state
}
