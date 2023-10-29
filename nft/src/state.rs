use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::{address::Address, sorted_vec_map::SortedVecMap};
use read_write_state_derive::ReadWriteState;

use crate::ContractError;

pub const URL_LENGTH: usize = 64;

/// This structure describes main NFT contract state.
#[repr(C)]
#[derive(ReadWriteState, CreateTypeSpec, Clone, Default, PartialEq, Eq, Debug)]
pub struct NFTContractState {
    pub contract_owner: Option<Address>,
    pub name: String,
    pub symbol: String,
    pub owners: SortedVecMap<u128, Address>,
    pub token_approvals: SortedVecMap<u128, Address>,
    pub operator_approvals: Vec<OperatorApproval>,
    pub supply: u128,
    pub uri_template: String,
    pub token_uri_details: SortedVecMap<u128, [u8; URL_LENGTH]>,
}

#[repr(C)]
#[derive(ReadWriteState, CreateTypeSpec, Copy, Clone, PartialEq, Eq, Debug)]
pub struct OperatorApproval {
    pub owner: Address,
    pub operator: Address,
}

impl NFTContractState {
    /// Find the owner of an NFT.
    /// Throws if no such token exists.
    /// ### Returns:
    ///
    /// An [`Address`] for the owner of the NFT.
    pub fn owner_of(&self, token_id: u128) -> Address {
        let owner_opt = self.owners.get(&token_id);
        assert!(owner_opt.is_some(), "{}", ContractError::NotFound);

        *owner_opt.unwrap()
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
        let as_operator_approval: OperatorApproval = OperatorApproval { owner, operator };
        self.operator_approvals.contains(&as_operator_approval)
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
    }
}
