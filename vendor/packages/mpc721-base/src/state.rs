use std::collections::BTreeMap;

use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::Address;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

use crate::ContractError;

/// ## Description
/// This structure describes main mpc721 contract state.
#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct MPC721ContractState {
    /// optional owner address
    pub owner: Option<Address>,
    /// token name
    pub name: String,
    /// token symbol
    pub symbol: String,
    /// optional base uri
    pub base_uri: Option<String>,
    /// minter address
    pub minter: Address,
    /// current supply
    pub supply: u128,
    /// token info by token id
    pub tokens: BTreeMap<u128, TokenInfo>,
    /// token approvals
    pub operator_approvals: BTreeMap<Address, BTreeMap<Address, bool>>,
}

/// ## Description
/// This structure describes minted mpc721 token information
#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct TokenInfo {
    /// token owner
    pub owner: Address,
    /// token approvals
    pub approvals: Vec<Address>,
    /// optional token uri
    pub token_uri: Option<String>,
}

impl MPC721ContractState {
    /// ## Description
    /// Sets new base uri
    /// ## Params
    /// * **base_uri** is an object of type [`str`]
    pub fn set_base_uri(&mut self, base_uri: &str) {
        self.base_uri = Some(base_uri.to_string())
    }

    /// ## Description
    /// Mints new token id to specified address
    /// ## Params
    /// * **token_id** is a field of type [`u128`]
    ///
    /// * **to** is an object of type [`Address`]
    ///
    /// * **token_uri** is an object of type [`Option<String>`]
    pub fn mint(&mut self, token_id: u128, to: &Address, token_uri: &Option<String>) {
        let token = TokenInfo {
            owner: *to,
            approvals: vec![],
            token_uri: token_uri.clone(),
        };

        self.tokens.insert(token_id, token);
    }

    /// ## Description
    /// Increases total supply
    pub fn increase_supply(&mut self) {
        self.supply = self.supply.checked_add(1).unwrap()
    }

    /// ## Description
    /// Decreases total supply
    pub fn decrease_supply(&mut self) {
        self.supply = self.supply.checked_sub(1).unwrap()
    }

    /// ## Description
    /// Transfers specified token id to the new owner
    /// ## Params
    /// * **from** is an object of type [`Address`]
    ///
    /// * **to** is an object of type [`Address`]
    ///
    /// * **token_id** is an object of type [`u128`]
    pub fn transfer(&mut self, from: &Address, to: &Address, token_id: u128) {
        let token = self.tokens.get(&token_id).unwrap();
        assert!(
            Self::allowed_to_transfer(from, token, &self.operator_approvals),
            "{}",
            ContractError::Unauthorized
        );

        self.tokens.entry(token_id).and_modify(|t| {
            t.owner = *to;
            t.approvals = vec![];
        });
    }

    /// ## Description
    /// Updates token approvals
    /// ## Params
    /// * **from** is an object of type [`Address`]
    ///
    /// * **spender** is an object of type [`Address`]
    ///
    /// * **token_id** is an object of type [`u128`]
    ///
    /// * **approved** is an object of type [`bool`]
    pub fn update_approvals(
        &mut self,
        from: &Address,
        spender: &Address,
        token_id: u128,
        approved: bool,
    ) {
        let token = self.tokens.get(&token_id).unwrap().to_owned();
        assert!(
            Self::allowed_to_approve(from, &token, &self.operator_approvals),
            "{}",
            ContractError::Unauthorized,
        );

        let mut approvals = token
            .approvals
            .into_iter()
            .filter(|account| account != spender)
            .collect::<Vec<Address>>();

        if approved {
            approvals.push(*spender);
        }

        self.tokens
            .entry(token_id)
            .and_modify(|t| t.approvals = approvals);
    }

    /// ## Description
    /// Adds operator approval
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
    /// Removes information about token
    /// ## Params
    /// * **owner** is an object of type [`Address`]
    ///
    /// * **token_id** is an object of type [`u128`]
    pub fn remove_token(&mut self, owner: &Address, token_id: u128) {
        let token = self.tokens.get(&token_id).unwrap();
        assert!(
            Self::allowed_to_transfer(owner, token, &self.operator_approvals),
            "{}",
            ContractError::Unauthorized
        );

        self.tokens.remove(&token_id);
    }

    /// ## Description
    /// Says is token id minted or not
    /// ## Params
    /// * **token_id** is an object of type [`u128`]
    pub fn is_minted(&self, token_id: u128) -> bool {
        self.tokens.contains_key(&token_id)
    }

    /// ## Description
    /// Checks that address is owner or not
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
    /// Returns token info by token id
    /// ## Params
    /// * **token_id** is an object of type [`u128`]
    pub fn token_info(&self, token_id: u128) -> Option<&TokenInfo> {
        self.tokens.get(&token_id)
    }

    /// ## Description
    /// Returns address token balance
    /// ## Params
    /// * **owner** is an object of type [`Address`]
    pub fn balance_of(&self, owner: &Address) -> u128 {
        self.tokens
            .values()
            .into_iter()
            .filter(|ti| ti.owner == *owner)
            .count() as u128
    }

    /// ## Description
    /// Returns owner of specified token id
    /// ## Params
    /// * **token_id** is an object of type [`u128`]
    pub fn owner_of(&self, token_id: u128) -> Address {
        self.tokens.get(&token_id).unwrap().owner
    }

    fn allowed_to_transfer(
        account: &Address,
        token: &TokenInfo,
        operator_approvals: &BTreeMap<Address, BTreeMap<Address, bool>>,
    ) -> bool {
        if token.owner == *account {
            return true;
        }

        if token.approvals.iter().any(|spender| spender == account) {
            return true;
        }

        if let Some(owner_approvals) = operator_approvals.get(&token.owner) {
            if let Some(approved) = owner_approvals.get(account) {
                return *approved;
            }
        }

        false
    }

    fn allowed_to_approve(
        account: &Address,
        token: &TokenInfo,
        operator_approvals: &BTreeMap<Address, BTreeMap<Address, bool>>,
    ) -> bool {
        if token.owner == *account {
            return true;
        }

        if let Some(owner_approvals) = operator_approvals.get(&token.owner) {
            if let Some(approved) = owner_approvals.get(account) {
                return *approved;
            }
        }

        false
    }
}
