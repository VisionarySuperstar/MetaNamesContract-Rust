use pbc_contract_common::address::{Address, AddressType};
use pbc_contract_common::Hash;
use pbc_contract_common::context::ContractContext;

pub fn mock_address(le: u8) -> Address {
    Address {
        address_type: AddressType::Account,
        identifier: [
            le, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8,
        ],
    }
}

pub fn mock_empty_transaction_hash() -> Hash {
    Hash {
        bytes: [
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        ],
    }
}

pub fn mock_contract_context(sender: u8) -> ContractContext {
    ContractContext {
        contract_address: mock_address(1u8),
        sender: mock_address(sender),
        block_time: 100,
        block_production_time: 100,
        current_transaction: mock_empty_transaction_hash(),
        original_transaction: mock_empty_transaction_hash(),
    }
}

pub fn string_to_bytes(s: &str) -> Vec<u8> {
    s.to_string().into_bytes()
}
