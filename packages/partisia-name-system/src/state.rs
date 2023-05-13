use std::collections::BTreeMap;

use contract_version_base::state::ContractVersionBase;
use create_type_spec_derive::CreateTypeSpec;
use mpc721_hierarchy::state::{MPC721ContractState, TokenInfo};
use pbc_contract_common::address::Address;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

use crate::ContractError;

/// ## Description
/// This structure describes partisia name system state
#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct PartisiaNameSystemState {
    pub mpc721: MPC721ContractState,
    pub version: ContractVersionBase,
    pub domains: BTreeMap<String, Domain>,
    pub records: BTreeMap<String, Record>,
}

#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct Domain {
    pub token_id: u128,
}

#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct Record {
    pub data: String,
}

#[repr(u8)]
#[derive(
    Eq, PartialEq, Debug, Clone, Ord, PartialOrd, Copy, CreateTypeSpec, ReadWriteState, ReadWriteRPC,
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

impl PartisiaNameSystemState {
    /// ## Description
    /// Returns domain info by token id
    /// ## Params
    /// * **domain** is an object of type [`String`]
    pub fn domain_info(&self, domain: &String) -> Option<&Domain> {
        self.domains.get(domain)
    }

    /// ## Description
    /// Says is token id minted or not
    /// ## Params
    /// * **token_id** is an object of type [`String`]
    pub fn is_minted(&self, token_id: &String) -> bool {
        self.domains.contains_key(token_id)
    }

    /// ## Description
    /// Returns token info by domain
    /// ## Params
    /// * **domain** is an object of type [`String`]
    pub fn token_info(&self, domain: &String) -> Option<&TokenInfo> {
        let domain = self.domain_info(domain);
        if domain.is_none() {
            return None;
        }

        self.mpc721.token_info(domain.unwrap().token_id)
    }

    /// ## Description
    /// This function returns token id for given domain
    /// ## Params
    /// * `domain` is an object of type [`String`]
    pub fn token_id(&self, domain: &String) -> Option<u128> {
        self.domains.get(domain).map(|d| d.token_id)
    }

    /// ## Description
    /// Returns record info by token id
    /// ## Params
    /// * **token_id** is an object of type [`String`]
    ///
    /// * **class** is an object of type [`RecordClass`]
    pub fn record_info(&self, token_id: &String, class: &RecordClass) -> Option<&Record> {
        let qualified_name = Self::fully_qualified_name(token_id, class);
        self.records.get(&qualified_name)
    }

    /// ## Description
    /// Returns boolean if account is allowed to manage domain
    /// ## Params
    /// * **account** is an object of type [`Address`]
    ///
    /// * **domain** is an object of type [`String`]
    pub fn allowed_to_manage(&self, account: &Address, domain: &String) -> bool {
        let domain = self.domain_info(domain);
        if domain.is_none() {
            return false;
        }

        self.mpc721
            .allowed_to_manage(account, domain.unwrap().token_id)
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
    pub fn mint_record(&mut self, token_id: &String, class: &RecordClass, data: &String) {
        let record = Record { data: data.clone() };
        let qualified_name = Self::fully_qualified_name(token_id, class);
        assert!(
            !self.records.contains_key(&qualified_name),
            "{}",
            ContractError::RecordMinted
        );

        self.records.insert(qualified_name, record);
    }

    /// ## Description
    /// Update data of a record
    /// ## Params
    /// * **token_id** is an object of type [`String`]
    ///
    /// * **class** is an object of type [`RecordClass`]
    ///
    /// * **data** is an object of type [`String`]
    pub fn update_record_data(&mut self, token_id: &String, class: &RecordClass, data: &String) {
        assert!(self.is_minted(token_id), "{}", ContractError::NotMinted);

        let qualified_name = Self::fully_qualified_name(token_id, class);
        self.records.entry(qualified_name).and_modify(|t| {
            t.data = data.clone();
        });
    }

    /// ## Description
    /// Remove a record
    /// ## Params
    /// * **token_id** is an object of type [`String`]
    ///
    /// * **class** is an object of type [`RecordClass`]
    pub fn delete_record(&mut self, token_id: &String, class: &RecordClass) {
        assert!(self.is_minted(token_id), "{}", ContractError::NotMinted);

        let qualified_name = Self::fully_qualified_name(token_id, class);
        if self.records.contains_key(&qualified_name) {
            self.records.remove_entry(&qualified_name);
        } else {
            panic!("{}", ContractError::NotFound);
        }
    }

    /// ## Description
    /// Says if record minted or not
    /// ## Params
    /// * **token_id** is an object of type [`String`]
    ///
    /// * **class** is an object of type [`RecordClass`]
    pub fn is_record_minted(&self, token_id: &String, class: &RecordClass) -> bool {
        let qualified_name = Self::fully_qualified_name(token_id, class);
        self.records.contains_key(&qualified_name)
    }

    /// ## Description
    /// Get fully qualified name for token and record class
    fn fully_qualified_name(token_id: &String, class: &RecordClass) -> String {
        // TODO: Optimize memory usage
        let class_name = match class {
            RecordClass::Wallet {} => "wallet",
            RecordClass::Uri {} => "uri",
            RecordClass::Twitter {} => "twitter",
        };

        format!("{}.{}", class_name, token_id)
    }
}
