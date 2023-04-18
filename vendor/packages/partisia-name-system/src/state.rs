use std::collections::BTreeMap;

use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::Address;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

use crate::ContractError;

/// ## Description
/// This structure describes main mpc721 contract state.
#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct PartisiaNameSystemContractState {
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
    /// Token supply
    pub supply: u128,
    /// domains are token id
    /// Token id is currently a string (the domain name)
    pub tokens: BTreeMap<String, Domain>,
    /// record info by token id
    /// Token id is currently a string (the domain name)
    pub records: BTreeMap<String, Record>,
    /// token approvals
    pub operator_approvals: BTreeMap<Address, BTreeMap<Address, bool>>,
}

/// ## Description
/// This structure describes minted PNS information
#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct Domain {
    /// token owner
    pub owner: Address,
    /// Parent
    pub parent: Option<String>,
    /// token approvals
    pub approvals: Vec<Address>,
}

/// ## Description
/// This structure describes minted PNS information
#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct Record {
    /// Related domain
    pub domain: String,
    /// Class type
    pub class: RecordClass,
    /// Data
    pub data: String,
}

#[repr(u8)]
#[derive(
    Eq,
    PartialEq,
    Debug,
    Clone,
    Ord,
    PartialOrd,
    Copy,
    CreateTypeSpec,
    ReadWriteState,
    ReadWriteRPC,
)]
pub enum RecordClass {
    /// Wallet
    #[discriminant(0)]
    Wallet {},
    /// Website
    #[discriminant(1)]
    Uri {},
    /// Twitter
    #[discriminant(2)]
    Twitter {},
}

impl PartisiaNameSystemContractState {
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
    /// * **token_id** is a field of type [`String`]
    ///
    /// * **to** is an object of type [`Address`]
    ///
    /// * **data** is an object of type [`String`]
    ///
    /// * **parent** is an object of type [`String`]
    pub fn mint(&mut self, token_id: String, to: &Address, parent: &Option<String>) {
        let token = Domain {
            owner: *to,
            parent: parent.clone(),
            approvals: vec![],
        };

        self.tokens.insert(token_id, token);
        self.supply += 1;
    }

    /// ## Description
    /// Mints record for token
    /// ## Params
    /// * **actor** is an object of type [`Address`]
    ///
    /// * **token_id** is a field of type [`String`]
    ///
    /// * **data** is an object of type [`String`]
    ///
    /// * **class** is an object of type [`RecordClass`]
    pub fn mint_record(&mut self, token_id: String, data: String, class: RecordClass) {
        let record = Record {
            domain: token_id.to_string(),
            class: class,
            data: data,
        };
        let qualified_name = Self::fully_qualified_name(token_id.to_string(), class);

        self.records.insert(qualified_name, record);
    }

    /// ## Description
    /// Transfers specified token id to the new owner
    /// ## Params
    /// * **from** is an object of type [`Address`]
    ///
    /// * **to** is an object of type [`Address`]
    ///
    /// * **token_id** is an object of type [`String`]
    pub fn transfer(&mut self, from: &Address, to: &Address, token_id: String) {
        let token = self.tokens.get(&token_id).unwrap();
        // TODO: Investigate transfer with parent
        if let Some(parent) = &token.parent {
            assert!(
                Self::allowed_parent(parent.clone()),
                "{}",
                ContractError::ParentError
            );
        }

        assert!(
            Self::allowed_to_transfer(from, token, &self.operator_approvals),
            "{}",
            ContractError::Unauthorized
        );

        // TODO: Remove all records related to this mint
        self.tokens.entry(token_id).and_modify(|t| {
            t.owner = *to;
            t.approvals = vec![];
        });
    }

    /// ## Description
    /// Update data of a record
    /// ## Params
    /// * **token_id** is an object of type [`String`]
    ///
    /// * **class** is an object of type [`RecordClass`]
    ///
    /// * **data** is an object of type [`String`]
    pub fn update_record_data(&mut self, token_id: String, class: RecordClass, data: String) {
        assert!(
            !self.is_minted(token_id.to_string()),
            "{}",
            ContractError::Minted
        );

        let qualified_name = Self::fully_qualified_name(token_id, class);
        self.records.entry(qualified_name).and_modify(|t| {
            t.data = data;
        });
    }

    /// ## Description
    /// Remove a record
    /// ## Params
    /// * **token_id** is an object of type [`String`]
    ///
    /// * **class** is an object of type [`RecordClass`]
    pub fn delete_record(&mut self, token_id: String, class: RecordClass) {
        assert!(
            !self.is_minted(token_id.to_string()),
            "{}",
            ContractError::Minted
        );

        let qualified_name = Self::fully_qualified_name(token_id, class);
        if self.records.contains_key(&qualified_name) {
            self.records.remove_entry(&qualified_name);
        } else {
            panic!("{}", ContractError::NotFound);
        }
    }

    /// ## Description
    /// Updates token approvals
    /// ## Params
    /// * **from** is an object of type [`Address`]
    ///
    /// * **spender** is an object of type [`Address`]
    ///
    /// * **token_id** is an object of type [`String`]
    ///
    /// * **approved** is an object of type [`bool`]
    pub fn update_approvals(
        &mut self,
        from: &Address,
        spender: &Address,
        token_id: String,
        approved: bool,
    ) {
        let token = self.tokens.get(&token_id).unwrap().to_owned();
        if let Some(parent) = &token.parent {
            assert!(
                Self::allowed_parent(parent.clone()),
                "{}",
                ContractError::ParentError
            );
        }

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
    /// * **token_id** is an object of type [`String`]
    pub fn remove_token(&mut self, owner: &Address, token_id: String) {
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
    /// * **token_id** is an object of type [`String`]
    pub fn is_minted(&self, token_id: String) -> bool {
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
    /// Checks that address is owner or not of token
    /// ## Params
    /// * **token_id** is an object of type [`String`]
    ///
    /// * **address** is an object of type [`Address`]
    pub fn is_token_owner(&self, token_id: String, address: &Address) -> bool {
        if let Some(token) = self.tokens.get(&token_id) {
            token.owner.eq(address)
        } else {
            false
        }
    }

    /// ## Description
    /// Returns token info by token id
    /// ## Params
    /// * **token_id** is an object of type [`String`]
    pub fn token_info(&self, token_id: String) -> Option<&Domain> {
        self.tokens.get(&token_id)
    }

    /// ## Description
    /// Returns records of token by token id
    /// ## Params
    /// * **token_id** is an object of type [`String`]
    pub fn records(&self, actor: &Address, token_id: String) -> Vec<&Record> {
        let token = self.tokens.get(&token_id).unwrap();
        assert!(
            Self::allowed_to_transfer(actor, token, &self.operator_approvals),
            "{}",
            ContractError::Unauthorized
        );

        self.records
            .values()
            .into_iter()
            .filter(|r| r.domain == token_id)
            .collect()
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
    /// * **token_id** is an object of type [`String`]
    pub fn owner_of(&self, token_id: String) -> Address {
        self.tokens.get(&token_id).unwrap().owner
    }

    fn allowed_to_transfer(
        account: &Address,
        token: &Domain,
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
        token: &Domain,
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

    // TODO: Improve parent management
    fn allowed_parent(parent: String) -> bool {
        if parent.is_empty() {
            return true;
        }

        false
    }

    /// ## Description
    /// Get fully qualified name for token and record class
    fn fully_qualified_name(token_id: String, class: RecordClass) -> String {
        let class_name = match class {
            RecordClass::Wallet {} => "wallet",
            RecordClass::Uri {} => "uri",
            RecordClass::Twitter {} => "twitter",
        };

        format!("{}.{}", class_name, token_id)
    }
}
