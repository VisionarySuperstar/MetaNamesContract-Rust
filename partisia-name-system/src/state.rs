use contract_version_base::state::ContractVersionBase;
use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::{avl_tree_map::AvlTreeMap, sorted_vec_map::SortedVecMap};
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

use crate::ContractError;

pub const MAX_RECORD_DATA_LENGTH: usize = 64;
pub const MAX_DOMAIN_LEN: usize = 32;
pub const MAX_CUSTOM_RECORDS: usize = 10;

/// This structure describes Partisia Name System state
#[repr(C)]
#[derive(ReadWriteState, CreateTypeSpec, Default, Debug)]
pub struct PartisiaNameSystemState {
    pub version: ContractVersionBase,
    pub domains: AvlTreeMap<String, Domain>,
}

#[repr(C)]
#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct Domain {
    pub token_id: u128,
    pub parent_id: Option<String>,
    pub minted_at: i64,
    /// Unix millis timestamp
    pub expires_at: Option<i64>,
    pub records: SortedVecMap<RecordClass, Record>,
    pub custom_records: SortedVecMap<String, Record>,
}

#[repr(C)]
#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct Record {
    pub data: Vec<u8>,
}

#[repr(u8)]
#[derive(
    Eq, PartialEq, Debug, Clone, Ord, PartialOrd, Copy, CreateTypeSpec, ReadWriteState, ReadWriteRPC,
)]
pub enum RecordClass {
    #[discriminant(0)]
    Bio {},
    #[discriminant(1)]
    Discord {},
    #[discriminant(2)]
    Twitter {},
    #[discriminant(3)]
    Uri {},
    #[discriminant(4)]
    Wallet {},
    #[discriminant(5)]
    Avatar {},
    #[discriminant(6)]
    Email {},
    #[discriminant(7)]
    Custom {},
    #[discriminant(8)]
    Custom2 {},
    #[discriminant(9)]
    Custom3 {},
    #[discriminant(10)]
    Custom4 {},
    #[discriminant(11)]
    Custom5 {},
}

impl Domain {
    /// Get record given class
    pub fn get_record(&self, class: &RecordClass) -> Option<&Record> {
        self.records.get(class)
    }

    /// Existence of record given class
    pub fn is_record_minted(&self, class: &RecordClass) -> bool {
        self.records.contains_key(class)
    }

    /// Get custom record given key
    pub fn get_custom_record(&self, key: &str) -> Option<&Record> {
        self.custom_records.get(key)
    }

    /// Existence of custom record given key
    pub fn is_custom_record_minted(&self, key: &str) -> bool {
        self.custom_records.contains_key(key)
    }

    /// Checks if domain is active
    /// Opposite of expired
    pub fn is_active(&self, unix_millis_now: i64) -> bool {
        match self.expires_at {
            Some(expires_at) => expires_at >= unix_millis_now,
            None => true,
        }
    }

    /// Mints record for token
    pub fn mint_record(&mut self, class: &RecordClass, data: &[u8]) {
        assert!(
            !self.is_record_minted(class),
            "{}",
            ContractError::RecordMinted
        );

        let record = Record {
            data: data.to_vec(),
        };
        self.records.insert(*class, record);
    }

    /// Update data of a record
    pub fn update_record_data(&mut self, class: &RecordClass, data: &[u8]) {
        assert!(
            self.is_record_minted(class),
            "{}",
            ContractError::RecordNotMinted
        );

        self.records.get_mut(class).map(|record| {
            record.data = data.to_vec();
            record
        });
    }

    /// Remove a record
    pub fn delete_record(&mut self, class: &RecordClass) {
        assert!(
            self.is_record_minted(class),
            "{}",
            ContractError::RecordNotMinted
        );

        if self.records.contains_key(class) {
            self.records.remove_entry(class);
        } else {
            panic!("{}", ContractError::NotFound);
        }
    }

    /// Mints custom record for token
    pub fn mint_custom_record(&mut self, key: &str, data: &[u8]) {
        assert!(
            !self.is_custom_record_minted(key),
            "{}",
            ContractError::RecordMinted
        );

        let record = Record {
            data: data.to_vec(),
        };
        self.custom_records.insert(String::from(key), record);
    }

    /// Update data of a custom record
    pub fn update_custom_record_data(&mut self, key: &str, data: &[u8]) {
        assert!(
            self.is_custom_record_minted(key),
            "{}",
            ContractError::RecordNotMinted
        );

        self.custom_records.get_mut(key).map(|record| {
            record.data = data.to_vec();
            record
        });
    }

    /// Remove a custom record
    pub fn delete_custom_record(&mut self, key: &str) {
        assert!(
            self.is_custom_record_minted(key),
            "{}",
            ContractError::RecordNotMinted
        );

        if self.custom_records.contains_key(key) {
            self.custom_records.remove_entry(key);
        } else {
            panic!("{}", ContractError::NotFound);
        }
    }
}

impl PartisiaNameSystemState {
    /// Returns info given domain
    pub fn get_domain(&self, domain_name: &str) -> Option<Domain> {
        self.domains.get(&String::from(domain_name))
    }

    /// Returns if the domain is active
    /// If the domain is a subdomain, it checks if the parent is active
    pub fn is_active(&self, domain_name: &str, unix_millis_now: i64) -> bool {
        match self.get_domain(domain_name) {
            Some(domain) => {
                domain.is_active(unix_millis_now)
                    && self
                        .get_root_parent(domain_name)
                        .map_or(true, |parent| parent.is_active(unix_millis_now))
            }
            None => false,
        }
    }

    pub fn get_domain_by_token_id(&self, token_id: u128) -> Option<(String, Domain)> {
        self.domains
            .iter()
            .find(|(_, domain)| domain.token_id == token_id)
    }

    /// Returns parent info by domain
    pub fn get_parent(&self, domain: &Domain) -> Option<Domain> {
        domain.parent_id.as_ref().and_then(|parent_id| {
            if !self.domains.contains_key(parent_id) {
                panic!("Expected parent domain not found")
            }

            self.domains.get(parent_id)
        })
    }

    /// Get all parents of a domain
    pub fn get_parents(&self, domain_name: &str) -> Vec<Domain> {
        let mut parents: Vec<Domain> = vec![];
        let mut current_domain = self.get_domain(domain_name);

        while let Some(domain) = current_domain {
            if let Some(parent) = self.get_parent(&domain) {
                parents.push(parent.clone());
                current_domain = Some(parent);
            } else {
                current_domain = None;
            }
        }

        parents
    }

    /// Get root parent of a domain
    pub fn get_root_parent(&self, domain_name: &str) -> Option<Domain> {
        let parents = self.get_parents(domain_name);

        match parents.last() {
            Some(parent) => {
                // By definition, the root parent has no parent
                assert!(
                    parent.parent_id.is_none(),
                    "Expected root parent to have no parent"
                );

                Some(parent.clone())
            }
            None => None,
        }
    }

    /// Says is token id minted or not
    pub fn is_minted(&self, domain_name: &str) -> bool {
        self.domains.contains_key(&String::from(domain_name))
    }

    /// This function returns token id for given domain
    pub fn get_token_id(&self, domain_name: &str) -> Option<u128> {
        self.domains
            .get(&String::from(domain_name))
            .map(|d| d.token_id)
    }
}
