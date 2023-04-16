//! This is an example non-fungible token (NFT) smart contract.
//!
//! The contract provides basic functionality to track and transfer NFTs.
//!
//! The contract works using a mint method for creating new bindings of NFTs to accounts.
//!
//! An NFT is identified via an u128 tokenID.
//!
//! Any token owner can then `transfer` their tokens to other accounts, or `approve` other accounts
//! to transfer their tokens.
//! If Alice has been approved an NFT from Bob, then Alice can use `transfer_from` to transfer Bob's tokens.
//!
//! Each token can only be approved to a single account.
//!
//! Any token owner can also make another account an operator of their tokens using `set_approval_for_all`.
//! An operator is approved to manage all NFTs owned by the owner, this includes setting approval on each
//! token and transfer.
//!
//!
//! The contract is inspired by the ERC721 NFT contract with extensions for Metadata and Burnable\
//! <https://github.com/ethereum/EIPs/blob/master/EIPS/eip-721.md>
#![allow(unused_variables)]

#[macro_use]
extern crate pbc_contract_codegen;

use std::collections::BTreeMap;

use pbc_contract_common::address::Address;
use pbc_contract_common::context::ContractContext;

/// Custom struct for the state of the contract.
///
/// The "state" attribute is attached.
///
/// ### Fields:
///
/// *`name`: [`String`], A descriptive name for a collection of NFTs in this contract.
///
/// * `symbol`: [`String`], An abbreviated name for NFTs in this contract.
///
/// * `contract_owner`: [`Address`], the owner of the contract.
///
/// * `owners`: [`BTreeMap<u128, Address>`], mapping from token_id to owner address.
///
/// * `token_approvals`: [`BTreeMap<u128, Option<Address>>`], mapping from token_id to the approved address
/// who can transfer the token.
///
/// * `operator_approvals`: [`BTreeMap<Address, Vec<Address>`], mapping from owner to
/// operator approvals. Operators can transfer and change approvals on all tokens owned by owner.
///
/// * `token_uris`: [`BTreeMap<u128, String>`], mapping for token URIs.
#[state]
pub struct NFTContractState {
    name: String,
    symbol: String,
    contract_owner: Address,
    owners: BTreeMap<u128, Address>,
    token_approvals: BTreeMap<u128, Address>,
    operator_approvals: BTreeMap<Address, Vec<Address>>,
    token_uris: BTreeMap<u128, String>,
}

impl NFTContractState {
    /// Find the owner of an NFT.
    /// Throws if no such token exists.
    ///
    /// ### Parameters:
    ///
    /// * `token_id`: [`u128`] The identifier for an NFT.
    ///
    /// ### Returns:
    ///
    /// An [`Address`] for the owner of the NFT.
    pub fn owner_of(&self, token_id: u128) -> Address {
        let owner_opt = self.owners.get(&token_id);
        match owner_opt {
            None => panic!("ERC721: owner query for nonexistent token"),
            Some(owner) => *owner,
        }
    }

    /// Get the approved address for a single NFT.
    ///
    /// ### Parameters:
    ///
    /// * `token_id`: [`u128`] The NFT to find the approved address for.
    ///
    /// ### Returns:
    ///
    /// An [`Option<Address>`] The approved address for this NFT, or none if there is none.
    pub fn get_approved(&self, token_id: u128) -> Option<Address> {
        self.token_approvals.get(&token_id).copied()
    }

    /// Query if an address is an authorized operator for another address.
    ///
    /// ### Parameters:
    ///
    /// * `owner`: [`Address`] The address that owns the NFTs.
    ///
    /// * `operator`: [`Address`] The address that acts on behalf of the owner.
    ///
    /// ### Returns:
    ///
    /// A [`bool`] True if `_operator` is an approved operator for `_owner`, false otherwise.
    pub fn is_approved_for_all(&self, owner: Address, operator: Address) -> bool {
        let approved_by_owner_opt = self.operator_approvals.get(&owner);
        if let Some(approved_by_owner) = approved_by_owner_opt {
            approved_by_owner.contains(&operator)
        } else {
            false
        }
    }

    /// Helper function to check whether a tokenId exists.
    ///
    /// Tokens start existing when they are minted (`mint`),
    /// and stop existing when they are burned (`burn`).
    ///
    /// ### Parameters:
    ///
    /// * `token_id`: [`u128`] The tokenId that is checked.
    ///
    /// ### Returns:
    ///
    /// A [`bool`] True if `token_id` is in use, false otherwise.
    pub fn exists(&self, token_id: u128) -> bool {
        let owner = self.owners.get(&token_id);
        owner.is_some()
    }

    /// Helper function to check whether a spender is owner or approved for a given token.
    /// Throws if token_id does not exist.
    ///
    /// ### Parameters:
    ///
    /// * `spender`: [`Address`] The address to check ownership for.
    ///
    /// * `token_id`: [`u128`] The tokenId which is checked.
    ///
    /// ### Returns:
    ///
    /// A [`bool`] True if `token_id` is owned or approved for `spender`, false otherwise.
    pub fn is_approved_or_owner(&self, spender: Address, token_id: u128) -> bool {
        let owner = self.owner_of(token_id);
        spender == owner
            || self.is_approved_for_all(owner, spender)
            || self.get_approved(token_id) == Some(spender)
    }

    /// Mutates the state by approving `to` to operate on `token_id`.
    /// None indicates there is no approved address.
    ///
    /// ### Parameters:
    ///
    /// * `approved`: [`Option<Address>`], The new approved NFT controller.
    ///
    /// * `token_id`: [`u128`], The NFT to approve.
    pub fn _approve(&mut self, approved: Option<Address>, token_id: u128) {
        if let Some(appr) = approved {
            self.token_approvals.insert(token_id, appr);
        } else {
            self.token_approvals.remove(&token_id);
        }
    }

    /// Mutates the state by transferring `token_id` from `from` to `to`.
    /// As opposed to {transfer_from}, this imposes no restrictions on `ctx.sender`.
    ///
    /// Throws if `from` is not the owner of `token_id`.
    ///
    /// ### Parameters:
    ///
    /// * `from`: [`Address`], The current owner of the NFT
    ///
    /// * `to`: [`Address`], The new owner
    ///
    /// * `token_id`: [`u128`], The NFT to transfer
    pub fn _transfer(&mut self, from: Address, to: Address, token_id: u128) {
        if self.owner_of(token_id) != from {
            panic!("ERC721: transfer from incorrect owner")
        } else {
            // clear approvals from the previous owner
            self._approve(None, token_id);
            self.owners.insert(token_id, to);
        }
    }
}

/// Initial function to bootstrap the contracts state.
///
/// ### Parameters:
///
/// * `ctx`: [`ContractContext`], initial context.
///
/// * `name`: [`String`], A descriptive name for a collection of NFTs in this contract.
///
/// * `symbol`: [`String`], An abbreviated name for NFTs in this contract.
///
/// ### Returns:
///
/// The new state object of type [`NFTContractState`].
#[init]
pub fn initialize(ctx: ContractContext, name: String, symbol: String) -> NFTContractState {
    NFTContractState {
        name,
        symbol,
        contract_owner: ctx.sender,
        owners: BTreeMap::new(),
        token_approvals: BTreeMap::new(),
        operator_approvals: BTreeMap::new(),
        token_uris: BTreeMap::new(),
    }
}

/// Change or reaffirm the approved address for an NFT.
/// None indicates there is no approved address.
/// Throws unless `ctx.sender` is the current NFT owner, or an authorized
/// operator of the current owner.
///
/// ### Parameters:
///
/// * `ctx`: [`ContractContext`], the context for the action call.
///
/// * `state`: [`NFTContractState`], the current state of the contract.
///
/// * `approved`: [`Option<Address>`], The new approved NFT controller.
///
/// * `token_id`: [`u128`], The NFT to approve.
///
/// ### Returns
///
/// The new state object of type [`NFTContractState`] with an updated ledger.
#[action]
pub fn approve(
    ctx: ContractContext,
    state: NFTContractState,
    approved: Option<Address>,
    token_id: u128,
) -> NFTContractState {
    let mut new_state = state;
    let owner = new_state.owner_of(token_id);
    if ctx.sender != owner && !new_state.is_approved_for_all(owner, ctx.sender) {
        panic!("ERC721: approve caller is not owner nor approved for all")
    }
    new_state._approve(approved, token_id);
    new_state
}

/// Enable or disable approval for a third party ("operator") to manage all of
/// `ctx.sender`'s assets.
/// Throws if `operator` == `ctx.sender`.
///
/// ### Parameters:
///
/// * `context`: [`ContractContext`], the context for the action call.
///
/// * `state`: [`NFTContractState`], the current state of the contract.
///
/// * `operator`: [`Address`], Address to add to the set of authorized operators.
///
/// * `approved`: [`bool`], True if the operator is approved, false to revoke approval.
///
/// ### Returns
///
/// The new state object of type [`NFTContractState`] with an updated ledger.
#[action]
pub fn set_approval_for_all(
    ctx: ContractContext,
    state: NFTContractState,
    operator: Address,
    approved: bool,
) -> NFTContractState {
    if operator == ctx.sender {
        panic!("ERC721: approve to caller")
    } else {
        let mut new_state = state;
        let owner_approvals_entry = new_state.operator_approvals.entry(ctx.sender);
        let owner_approvals = owner_approvals_entry.or_insert_with(Vec::new);
        if approved {
            if !owner_approvals.contains(&operator) {
                owner_approvals.push(operator);
            }
        } else {
            owner_approvals.retain(|&x| x != operator);
        }
        new_state
    }
}

/// Transfer ownership of an NFT -- THE CALLER IS RESPONSIBLE
/// TO CONFIRM THAT `to` IS CAPABLE OF RECEIVING NFTS OR ELSE
/// THEY MAY BE PERMANENTLY LOST
///
/// Throws unless `ctx.sender` is the current owner, an authorized
/// operator, or the approved address for this NFT. Throws if `from` is
/// not the current owner. Throws if `token_id` is not a valid NFT.
///
/// ### Parameters:
///
/// * `ctx`: [`ContractContext`], the context for the action call.
///
/// * `state`: [`NFTContractState`], the current state of the contract.
///
/// * `from`: [`Address`], The current owner of the NFT
///
/// * `to`: [`Address`], The new owner
///
/// * `token_id`: [`u128`], The NFT to transfer
///
/// ### Returns
///
/// The new state object of type [`NFTContractState`] with an updated ledger.
#[action]
pub fn transfer_from(
    ctx: ContractContext,
    state: NFTContractState,
    from: Address,
    to: Address,
    token_id: u128,
) -> NFTContractState {
    let mut new_state = state;
    if !new_state.is_approved_or_owner(ctx.sender, token_id) {
        panic!("ERC721: transfer caller is not owner nor approved")
    } else {
        new_state._transfer(from, to, token_id);
        new_state
    }
}

/// Mints `token_id` and transfers it to an owner.
///
/// Requirements:
///
/// - `token_id` must not exist
/// - `ctx.sender` owns the contract
///
/// ### Parameters:
///
/// * `ctx`: [`ContractContext`], the context for the action call.
///
/// * `state`: [`NFTContractState`], the current state of the contract.
///
/// * `to`: [`Address`], the owner of the minted token.
///
/// * `token_id`: [`u128`], The new id for the minted token.
///
/// ### Returns
///
/// The new state object of type [`NFTContractState`] with an updated ledger.
#[action]
pub fn mint(
    ctx: ContractContext,
    state: NFTContractState,
    to: Address,
    token_id: u128,
    token_uri: String,
) -> NFTContractState {
    if ctx.sender != state.contract_owner {
        panic!("ERC721: mint only callable by the contract owner")
    } else if state.exists(token_id) {
        panic!("ERC721: token already minted")
    } else {
        let mut new_state = state;
        new_state.owners.insert(token_id, to);
        new_state.token_uris.insert(token_id, token_uri);
        new_state
    }
}

/// Destroys `token_id`.
/// The approval is cleared when the token is burned.
/// Requires that the `token_id` exists and `ctx.sender` is approved or owner of the token.
///
/// ### Parameters:
///
/// * `ctx`: [`ContractContext`], the context for the action call.
///
/// * `state`: [`NFTContractState`], the current state of the contract.
///
/// * `token_id`: [`u128`], The id of the NFT to be burned.
///
/// ### Returns
///
/// The new state object of type [`NFTContractState`] with an updated ledger.
#[action]
pub fn burn(ctx: ContractContext, state: NFTContractState, token_id: u128) -> NFTContractState {
    let mut new_state = state;
    if !new_state.is_approved_or_owner(ctx.sender, token_id) {
        panic!("ERC721: burn caller is not owner nor approved")
    } else {
        let owner = new_state.owner_of(token_id);
        // Clear approvals
        new_state._approve(None, token_id);

        new_state.owners.remove(&token_id);
        new_state.token_uris.remove(&token_id);
        new_state
    }
}
