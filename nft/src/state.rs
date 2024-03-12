use core::default::Default;
use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::{address::Address, avl_tree_map::AvlTreeMap};
use read_write_state_derive::ReadWriteState;

use crate::ContractError;

/// This structure describes main NFT contract state.
/// A permission to transfer and approve NFTs given from an NFT owner to a separate address, called an operator.
#[repr(C)]
#[derive(ReadWriteState, CreateTypeSpec, PartialEq, Copy, Clone, Ord, PartialOrd, Eq, Debug)]
pub struct OperatorApproval {
    /// NFT owner.
    pub owner: Address,
    /// Operator of the owner's tokens.
    pub operator: Address,
}

/// Unit
#[repr(C)]
#[derive(CreateTypeSpec, ReadWriteState, Debug)]
pub struct Unit {}

/// State of the contract.
#[derive(ReadWriteState, CreateTypeSpec, Default, Debug)]
pub struct NFTContractState {
    /// Descriptive name for the collection of NFTs in this contract.
    pub name: String,
    /// Abbreviated name for NFTs in this contract.
    pub symbol: String,
    /// Mapping from token_id to the owner of the token.
    pub owners: AvlTreeMap<u128, Address>,
    /// Mapping from token_id to the approved address who can transfer the token.
    pub token_approvals: AvlTreeMap<u128, Address>,
    /// Containing approved operators of owners. Operators can transfer and change approvals on all tokens owned by owner.
    pub operator_approvals: AvlTreeMap<OperatorApproval, Unit>,
    /// owners inverse lookup
    pub owners_inventory: AvlTreeMap<Address, Vec<u128>>,
    /// Template which the uri's of the NFTs fit into.
    pub uri_template: String,
    /// Mapping from token_id to the URI of the token.
    pub token_uri_details: AvlTreeMap<u128, String>,
    /// Owner of the contract. Is allowed to mint new NFTs.
    pub contract_owner: Option<Address>,
    /// Total supply of the NFTs.
    pub supply: u128,
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
        assert!(owner_opt.is_some(), "{}", ContractError::NotFound);

        owner_opt.unwrap()
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
        self.token_approvals.get(&token_id)
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
    /// A [`bool`] true if `operator` is an approved operator for `owner`, false otherwise.
    pub fn is_approved_for_all(&self, owner: Address, operator: Address) -> bool {
        let as_operator_approval: OperatorApproval = OperatorApproval { owner, operator };
        self.operator_approvals.contains_key(&as_operator_approval)
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
        self.owners.contains_key(&token_id)
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
        let contract_owner = self.contract_owner.unwrap();
        let owner = self.owner_of(token_id);

        spender == owner
            || spender == contract_owner
            || self.is_approved_for_all(owner, spender)
            || self.get_approved(token_id) == Some(spender)
    }

    /// Increase the supply of the token by 1
    pub fn increase_supply(&mut self) {
        self.supply += 1;
    }

    /// Decrease the supply of the token by 1
    pub fn decrease_supply(&mut self) {
        self.supply -= 1;
    }

    /// Get the next token id
    pub fn get_next_token_id(&self) -> u128 {
        self.supply
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

    /// Add token id to owner inventory
    pub fn _owner_inventory_add(&mut self, owner: Address, token_id: u128) {
        let mut inventory = self.owners_inventory.get(&owner).unwrap_or(vec![]);
        inventory.push(token_id);
        self.owners_inventory.insert(owner, inventory);
    }

    /// Remove token id from owner inventory
    ///
    /// Throws if token id is not found
    pub fn _owner_inventory_remove(&mut self, owner: Address, token_id: u128) {
        let inventory = self.owners_inventory.get(&owner);
        assert!(inventory.is_some(), "{}", ContractError::NotFound);

        let mut new_inventory = inventory.unwrap();
        if let Some(pos) = new_inventory.iter().position(|&id| id == token_id) {
            new_inventory.swap_remove(pos);
        }
        self.owners_inventory.insert(owner, new_inventory);
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
        assert!(
            self.owner_of(token_id) == from,
            "{}",
            ContractError::IncorrectOwner
        );

        // clear approvals from the previous owner
        self._approve(None, token_id);
        self.owners.insert(token_id, to);
        self._owner_inventory_remove(from, token_id);
        self._owner_inventory_add(to, token_id);
    }
}
