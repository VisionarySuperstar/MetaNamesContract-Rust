use contract_version_base::state::ContractVersionBase;
use partisia_name_system::state::PartisiaNameSystemContractState;

#[state]
#[derive(PartialEq, Eq, Debug)]
pub struct ContractState {
    pub pns: PartisiaNameSystemContractState,
    pub version: ContractVersionBase,
}
