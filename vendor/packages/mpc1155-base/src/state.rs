use std::collections::BTreeMap;

use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::Address;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

use crate::ContractError;

/// ## Description
/// This structure describes main mpc1155 contract state.
#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct MPC1155ContractState {
    /// optional owner address
    pub owner: Option<Address>,
    /// base uri for the tokens
    pub uri: String,
    /// minter address
    pub minter: Address,
    /// token holders balance
    pub balances: BTreeMap<u128, BTreeMap<Address, u128>>,
    /// token approvals
    pub operator_approvals: BTreeMap<Address, BTreeMap<Address, bool>>,
    /// token info by token id
    pub tokens: BTreeMap<u128, TokenInfo>,
}

/// ## Description
/// This structure describes minted mpc1155 token information
#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct TokenInfo {
    /// optional token uri
    pub token_uri: Option<String>,
}

impl MPC1155ContractState {
    /// ## Description
    /// Sets new base uri
    /// ## Params
    /// * **uri** is an object of type [`str`]
    pub fn set_uri(&mut self, uri: &str) {
        self.uri = uri.to_string()
    }

    /// ## Description
    /// Stores new token at specified token id
    /// ## Params
    /// * **token_id** is an object of type [`u128`]
    ///
    /// * **info** is an object of type [`TokenInfo`]
    pub fn store_token(&mut self, token_id: u128, info: &TokenInfo) {
        self.tokens.entry(token_id).or_insert_with(|| info.clone());
    }

    /// ## Description
    /// Transfers token from owner to spender
    /// ## Params
    /// * **from** is an object of type [`Option<Address>`]
    ///
    /// * **to** is an object of type [`Option<Address>`]
    ///
    /// * **token_id** is a field of type [`u128`]
    ///
    /// * **amount** is a field of type [`u128`]
    pub fn transfer(
        &mut self,
        from: Option<&Address>,
        to: Option<&Address>,
        token_id: u128,
        amount: u128,
    ) {
        if let Some(from) = from {
            self.balances.entry(token_id).and_modify(|token_balances| {
                token_balances
                    .entry(*from)
                    .and_modify(|balance| *balance = balance.checked_sub(amount).unwrap());
            });
        }

        if let Some(to) = to {
            self.balances
                .entry(token_id)
                .and_modify(|token_balances| {
                    token_balances
                        .entry(*to)
                        .and_modify(|balance| *balance = balance.checked_add(amount).unwrap())
                        .or_insert(amount);
                })
                .or_insert_with(|| BTreeMap::from([(*to, amount)]));
        }
    }
    /// ## Description
    /// checks if an address possesses at least a given balance of tokens
    pub fn check_balances(&mut self, owner: Address, token_ids: Vec<u128>, amounts: Vec<u128>) {
        token_ids.into_iter().enumerate().for_each(|(n, id)| {
            assert!(
                self.balances.get(&id).unwrap().get(&owner).unwrap() >= &amounts[n],
                "{}",
                ContractError::InadequateBalance
            )
        });
    }
    /// ## Description
    /// Adds new operator approval
    /// ## Params
    /// * **owner** is an object of type [`Address`]
    ///
    /// * **operator** is an object of type [`Address`]
    pub fn add_operator(&mut self, owner: &Address, operator: &Address) {
        let owner_operators = self
            .operator_approvals
            .entry(*owner)
            .or_insert_with(BTreeMap::new);

        owner_operators.insert(*operator, true);
    }

    /// ## Description
    /// Removes operator approval
    /// ## Params
    /// * **owner** is an object of type [`Address`]
    ///
    /// * **operator** is an object of type [`Address`]
    pub fn remove_operator(&mut self, owner: &Address, operator: &Address) {
        let owner_operators = self
            .operator_approvals
            .get_mut(owner)
            .unwrap_or_else(|| panic!("{}", ContractError::NotFound.to_string()));

        owner_operators.remove(operator);

        if owner_operators.is_empty() {
            self.operator_approvals.remove(owner);
        }
    }

    /// ## Description
    /// Checks that specified address is an owner or not
    /// ## Params
    /// * **address** is an object of type [`Address`]
    pub fn is_owner(&self, address: &Address) -> bool {
        if let Some(owner) = self.owner {
            owner.eq(address)
        } else {
            false
        }
    }

    /// ## Description
    /// Checks approval
    /// ## Params
    /// * **owner** is an object of type [`Address`]
    ///
    /// * **sender** is an object of type [`Address`]
    pub fn is_token_owner_or_operator(&self, owner: &Address, sender: &Address) -> bool {
        if owner == sender {
            return true;
        }

        if let Some(owner_approvals) = self.operator_approvals.get(owner) {
            if let Some(approved) = owner_approvals.get(sender) {
                return *approved;
            }
        }

        false
    }

    /// ## Description
    /// Returns token info by specified token id
    /// ## Params
    /// * **token_id** is an object of type [`u128`]
    pub fn token_info(&self, token_id: u128) -> Option<&TokenInfo> {
        self.tokens.get(&token_id)
    }
}
