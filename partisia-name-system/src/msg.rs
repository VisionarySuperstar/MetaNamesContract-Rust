use create_type_spec_derive::CreateTypeSpec;
use read_write_rpc_derive::ReadWriteRPC;

use crate::state::RecordClass;

/// This structure describes fields for PNS mint msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct PnsMintMsg {
    pub domain: String,
    /// NFT token id
    pub token_id: u128,
    /// Unix timestamp
    pub expires_at: Option<i64>,
    /// optional parent
    pub parent_id: Option<String>,
}

/// This structure describes fields for PNS Record Mint Msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct PnsRecordMintMsg {
    pub domain: String,
    /// Class type
    pub class: RecordClass,
    /// Data
    pub data: Vec<u8>,
}

/// This structure describes fields for the record update msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct PnsRecordUpdateMsg {
    pub domain: String,
    /// Class type
    pub class: RecordClass,
    /// Data
    pub data: Vec<u8>,
}

/// This structure describes fields for the Record Delete Msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct PnsRecordDeleteMsg {
    pub domain: String,
    /// Class type
    pub class: RecordClass,
}

/// This structure describes fields for the Record Delete All Msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct PnsRecordDeleteAllMsg {
    pub domain: String,
}

/// This structure describes fields for the Domain Update Expiration Msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct PnsDomainUpdateExpirationMsg {
    pub domain: String,
    pub expires_at: Option<i64>,
}
