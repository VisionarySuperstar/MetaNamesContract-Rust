use std::collections::BTreeMap;

use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::Address;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

use crate::{msg::InitialBalance, ContractError};

/// ## Description
/// This structure describes main mpc20 contract state.
#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct MPC20ContractState {
    /// mpc20 token information
    pub info: TokenInfo,
    /// total mpc20 supply
    pub total_supply: u128,
    /// optional address allowed to mint new tokens
    pub minter: Option<Minter>,
    /// token holders balance
    pub balances: BTreeMap<Address, u128>,
    /// token allowances
    pub allowances: BTreeMap<Address, BTreeMap<Address, u128>>,
}

/// ## Description
/// This structure describes mpc20 information
#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct TokenInfo {
    /// mpc20 token name
    pub name: String,
    /// mpc20 token symbol
    pub symbol: String,
    /// mpc20 token decimals
    pub decimals: u8,
}

/// ## Description
/// This structure describes mpc20 minter info
#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct Minter {
    /// allowed to mint new tokens address
    pub minter: Address,
    /// optionaly total supply limit
    pub capacity: Option<u128>,
}

impl MPC20ContractState {
    /// ## Description
    /// Creates new instance of [`MPC20ContractState`] with initial values
    /// ## Params
    /// * **info** is an object of type [`TokenInfo`]
    ///
    /// * **minter** is an optional object of type [`Option<Minter>`]
    pub fn new(info: &TokenInfo, minter: &Option<Minter>) -> Self {
        Self {
            info: info.clone(),
            total_supply: 0,
            minter: minter.clone(),
            balances: BTreeMap::new(),
            allowances: BTreeMap::new(),
        }
    }

    /// ## Description
    /// Set's initial token balances
    /// ## Params
    /// * **initial_balances** is an object of type [`&[InitialBalance]`]
    pub fn init_balances(&mut self, initial_balances: &[InitialBalance]) -> u128 {
        let mut total_supply: u128 = 0;
        for ib in initial_balances {
            self.balances.insert(ib.address, ib.amount);
            total_supply += ib.amount;
        }

        self.total_supply = total_supply;
        total_supply
    }

    /// ## Description
    /// Mints specified amount of tokens to specified address
    ///  ## Params
    /// * **to** is an object of type [`Address`]
    ///
    /// * **amount** is a field of type [`u128`]
    pub fn mint_to(&mut self, to: &Address, amount: u128) {
        self.increase_total_supply(amount);
        if let Some(limit) = self.get_capacity() {
            assert!(
                self.total_supply <= limit,
                "{}",
                ContractError::CapacityExceeded
            );
        }

        self.increase_balance(to, amount);
    }

    /// ## Description
    /// Increases balance of specified address
    ///  ## Params
    /// * **address** is an object of type [`Address`]
    ///
    /// * **amount** is a field of type [`u128`]
    pub fn increase_balance(&mut self, address: &Address, amount: u128) {
        Self::increase_or_set(&mut self.balances, address, amount);
    }

    /// ## Description
    /// Decreases balance of specified address
    ///  ## Params
    /// * **address** is an object of type [`Address`]
    ///
    /// * **amount** is a field of type [`u128`]
    pub fn decrease_balance(&mut self, address: &Address, amount: u128) {
        Self::decrease_or_remove(&mut self.balances, address, amount);
    }

    /// ## Description
    /// Increases total token supply. Usually invoked with 'increase_balance' function
    ///  ## Params
    /// * **amount** is a field of type [`u128`]
    pub fn increase_total_supply(&mut self, amount: u128) {
        self.total_supply += amount
    }

    /// ## Description
    /// Decreases total supply. Usually invoked with 'decrease_balance' function
    ///  ## Params
    /// * **amount** is a field of type [`u128`]
    pub fn decrease_total_supply(&mut self, amount: u128) {
        self.total_supply = self.total_supply.checked_sub(amount).unwrap();
    }

    /// ## Description
    /// Adds allowance for specified spender address to use owner tokens
    ///  ## Params
    /// * **owner** is an object of type [`Address`]
    ///
    /// * **spender** is an object of type [`Address`]
    ///
    /// * **amount** is a field of type [`u128`]
    pub fn set_allowance(&mut self, owner: &Address, spender: &Address, amount: u128) {
        let owner_allowances = self.allowances.entry(*owner).or_insert_with(BTreeMap::new);
        owner_allowances.insert(*spender, amount);
    }

    /// ## Description
    /// Increases token allowance
    ///  ## Params
    /// * **owner** is an object of type [`Address`]
    ///
    /// * **spender** is an object of type [`Address`]
    ///
    /// * **amount** is a field of type [`u128`]
    pub fn increase_allowance(&mut self, owner: &Address, spender: &Address, amount: u128) {
        let owner_allowances = self.allowances.entry(*owner).or_insert_with(BTreeMap::new);
        Self::increase_or_set(owner_allowances, spender, amount);
    }

    /// ## Description
    /// Decreases token allowance
    ///  ## Params
    /// * **owner** is an object of type [`Address`]
    ///
    /// * **spender** is an object of type [`Address`]
    ///
    /// * **amount** is a field of type [`u128`]
    pub fn decrease_allowance(&mut self, owner: &Address, spender: &Address, amount: u128) {
        let owner_allowances = self
            .allowances
            .get_mut(owner)
            .unwrap_or_else(|| panic!("{}", ContractError::NotFound.to_string()));

        Self::decrease_or_remove(owner_allowances, spender, amount);
    }

    /// ## Description
    /// Increases already existing entry otherwise creates new one
    /// ## Params
    /// * **map** is an object of type [`BTreeMap<Address, u128>`]
    ///
    /// * **address** is an object of type [`Address`]
    ///
    /// * **amount** is a field of type [`u128`]
    fn increase_or_set(map: &mut BTreeMap<Address, u128>, address: &Address, amount: u128) {
        map.entry(*address)
            .and_modify(|a| *a += amount)
            .or_insert(amount);
    }

    /// ## Description
    /// Decreases already existing entry otherwise removes it
    ///  ## Params
    /// * **map** is an object of type [`BTreeMap<Address, u128>`]
    ///
    /// * **address** is an object of type [`Address`]
    ///
    /// * **amount** is a field of type [`u128`]
    fn decrease_or_remove(map: &mut BTreeMap<Address, u128>, address: &Address, amount: u128) {
        let current = *map
            .get(address)
            .unwrap_or_else(|| panic!("{}", ContractError::NotFound.to_string()));

        assert!(current >= amount, "{}", ContractError::Overflow.to_string());

        if amount < current {
            map.entry(*address).and_modify(|a| *a -= amount);
        } else {
            map.remove(address);
        }
    }

    /// ## Description
    /// Returns token capacity
    pub fn get_capacity(&self) -> Option<u128> {
        self.minter.as_ref().and_then(|m| m.capacity)
    }

    /// ## Description
    /// Returns balance of specified address
    ///  ## Params
    /// * **address** is an object of type [`Address`]
    pub fn balance_of(&self, address: &Address) -> u128 {
        *self.balances.get(address).unwrap_or(&0)
    }

    /// ## Description
    /// Returns allowance for specified address pair
    ///  ## Params
    /// * **owner** is an object of type [`Address`]
    ///
    /// * **spender** is an object of type [`Address`]
    pub fn allowance(&self, owner: &Address, spender: &Address) -> u128 {
        *self
            .allowances
            .get(owner)
            .unwrap_or(&BTreeMap::new())
            .get(spender)
            .unwrap_or(&0)
    }
}
