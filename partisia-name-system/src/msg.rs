use create_type_spec_derive::CreateTypeSpec;
use read_write_rpc_derive::ReadWriteRPC;


use crate::state::RecordClass;

/// ## Description
/// This structure describes fields for PNS mint msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct PnsMintMsg {
    pub domain: Vec<u8>,
    /// NFT token id
    pub token_id: u128,
    /// optional parent
    pub parent_id: Option<Vec<u8>>,
}

/// ## Description
/// This structure describes fields for PNS Record Mint Msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct PnsRecordMintMsg {
    pub domain: Vec<u8>,
    /// Class type
    pub class: RecordClass,
    /// Data
    pub data: Vec<u8>,
}

/// ## Description
/// This structure describes fields for the record update msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct PnsRecordUpdateMsg {
    pub domain: Vec<u8>,
    /// Class type
    pub class: RecordClass,
    /// Data
    pub data: Vec<u8>,
}

/// ## Description
/// This structure describes fields for the Record Delete Msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct PnsRecordDeleteMsg {
    pub domain: Vec<u8>,
    /// Class type
    pub class: RecordClass,
}
