use contract_version_base::state::ContractVersionBase;
use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::sorted_vec_map::SortedVecMap;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

use crate::ContractError;

pub const MAX_RECORD_DATA_LENGTH: usize = 64;
pub const MAX_DOMAIN_LEN: usize = 32;

/// ## Description
/// This structure describes Partisia Name System state
#[derive(ReadWriteState, CreateTypeSpec, Clone, Default, PartialEq, Eq, Debug)]
pub struct PartisiaNameSystemState {
    pub version: ContractVersionBase,
    pub domains: SortedVecMap<String, Domain>,
    pub records: SortedVecMap<String, Record>,
}

#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct Domain {
    pub token_id: u128,
    pub parent_id: Option<String>,
    pub records: SortedVecMap<RecordClass, Record>,
}

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
    // Customizables
    #[discriminant(5)]
    Custom {},
    #[discriminant(6)]
    Custom2 {},
    #[discriminant(7)]
    Custom3 {},
    #[discriminant(8)]
    Custom4 {},
    #[discriminant(9)]
    Custom5 {},
}

impl Domain {
    /// ## Description
    /// Get record given class
    pub fn get_record(&self, class: &RecordClass) -> Option<&Record> {
        self.records.get(class)
    }

    /// ## Description
    /// Existence of record given class
    pub fn is_record_minted(&self, class: &RecordClass) -> bool {
        self.records.contains_key(class)
    }

    /// ## Description
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

    /// ## Description
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

    /// ## Description
    /// Remove a record
    pub fn delete_record(&mut self, class: &RecordClass) {
        assert!(self.is_record_minted(class), "{}", ContractError::NotMinted);

        if self.records.contains_key(class) {
            self.records.remove_entry(class);
        } else {
            panic!("{}", ContractError::NotFound);
        }
    }
}

impl PartisiaNameSystemState {
    /// ## Description
    /// Returns info given domain
    pub fn get_domain(&self, domain: &str) -> Option<&Domain> {
        self.domains.get(&String::from(domain))
    }

    pub fn get_domain_by_token_id(&self, token_id: u128) -> Option<(&String, &Domain)> {
        self.domains
            .iter()
            .find(|(_, domain)| domain.token_id == token_id)
    }

    /// ## Description
    /// Returns parent info by domain
    pub fn get_parent(&self, domain: &str) -> Option<&Domain> {
        self.domains
            .get(&String::from(domain))
            .and_then(|d| d.parent_id.as_ref())
            .and_then(|parent_id| self.domains.get(parent_id))
    }

    /// ## Description
    /// Says is token id minted or not
    pub fn is_minted(&self, domain: &str) -> bool {
        self.domains.contains_key(&String::from(domain))
    }

    /// ## Description
    /// This function returns token id for given domain
    pub fn get_token_id(&self, domain: &str) -> Option<u128> {
        self.domains.get(&String::from(domain)).map(|d| d.token_id)
    }
}
