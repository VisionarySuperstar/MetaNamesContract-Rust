use contract_version_base::state::ContractVersionBase;
use partisia_name_system::state::PartisiaNameSystemState;

#[state]
#[derive(PartialEq, Eq, Debug)]
pub struct ContractState {
    pub pns: PartisiaNameSystemState,
    pub version: ContractVersionBase,
}
