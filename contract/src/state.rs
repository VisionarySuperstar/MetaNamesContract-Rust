use contract_version_base::state::ContractVersionBase;
use mpc721_base::state::MPC721ContractState;
use partisia_name_system::state::PartisiaNameSystemState;

#[state]
#[derive(PartialEq, Eq, Debug)]
pub struct ContractState {
    pub pns: PartisiaNameSystemState,
    pub nft: MPC721ContractState,
    pub version: ContractVersionBase,
}
