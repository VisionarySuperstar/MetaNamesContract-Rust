use pbc_contract_common::{address::Address, avl_tree_map::AvlTreeMap};

use crate::state::AirdropState;

pub fn execute_init() -> AirdropState {
    AirdropState {
        inventory: AvlTreeMap::new(),
    }
}

pub fn execute_airdrop(state: &mut AirdropState, address: &Address) {
    assert!(state.has_airdrop(address), "No airdrop for the address");

    state._use_airdrop(address);
}

pub fn execute_add_airdrop(state: &mut AirdropState, address: &Address) {
    state._add_airdrop(address);
}
