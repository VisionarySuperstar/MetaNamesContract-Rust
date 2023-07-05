use contract_version_base::state::ContractVersionBase;
use nft::state::NFTContractState;
use partisia_name_system::state::PartisiaNameSystemState;

#[allow(unused_imports)]
use crate::contract::__PBC_IS_ZK_CONTRACT;

#[state]
#[derive(PartialEq, Eq, Default, Clone, Debug)]
pub struct ContractState {
    pub pns: PartisiaNameSystemState,
    pub nft: NFTContractState,
    pub version: ContractVersionBase,
}
