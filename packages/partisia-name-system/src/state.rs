use contract_version_base::state::ContractVersionBase;
use create_type_spec_derive::CreateTypeSpec;
use mpc721_hierarchy::state::MPC721ContractState;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

/// ## Description
/// This structure describes partisia name system state
#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct PartisiaNameSystemState {
    pub mpc721: MPC721ContractState,
    pub version: ContractVersionBase,
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
