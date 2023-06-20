use create_type_spec_derive::CreateTypeSpec;
use read_write_rpc_derive::ReadWriteRPC;

/// ## Description
/// This structure describes fields for PNS initialize msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct InitMsg {
    /// token name
    pub name: String,
    /// token symbol
    pub symbol: String,
    pub uri_template: String,
}
