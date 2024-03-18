use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::{address::Address, avl_tree_map::AvlTreeMap};
use read_write_state_derive::ReadWriteState;

#[repr(C)]
#[derive(ReadWriteState, CreateTypeSpec, Default, Debug)]
pub struct AirdropState {
    pub inventory: AvlTreeMap<Address, u128>,
}

impl AirdropState {
    /// Check if the address has an airdrop
    pub fn has_airdrop(&self, address: &Address) -> bool {
        self.inventory.contains_key(address)
    }

    /// Use airdrop from the address
    pub fn _use_airdrop(&mut self, address: &Address) {
        if let Some(airdrop) = self.inventory.get(address) {
            let remaining = airdrop - 1;
            if remaining == 0 {
                self.inventory.remove(address);
            } else {
                self.inventory.insert(*address, remaining);
            }
        }
    }

    /// Add airdrop to the address
    pub fn _add_airdrop(&mut self, address: &Address) {
        let airdrop = self.inventory.get(address).unwrap_or(0);
        self.inventory.insert(*address, airdrop + 1);
    }
}
