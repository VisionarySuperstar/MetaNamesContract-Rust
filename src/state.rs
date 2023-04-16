use contract_version_base::state::ContractVersionBase;
use mpc721_base::state::MPC721ContractState;

#[state]
#[derive(PartialEq, Eq, Debug)]
pub struct ContractState {
    pub mpc721: MPC721ContractState,
    pub version: ContractVersionBase,
}
