use pbc_contract_common::address::{Address, AddressType};
use pbc_contract_common::context::{CallbackContext, ContractContext};
use pbc_contract_common::Hash;
use std::time::SystemTime;

pub const SYSTEM_ADDRESS: u8 = 0;
pub const ALICE_ADDRESS: u8 = 1;
pub const BOB_ADDRESS: u8 = 2;
pub const PAYMENT_TOKEN_ADDRESS: u8 = 10;

pub fn get_address_for_user(user: String) -> u8 {
    match user.to_lowercase().as_str() {
        "alice" => ALICE_ADDRESS,
        "bob" => BOB_ADDRESS,
        "contract" => SYSTEM_ADDRESS,
        _ => panic!("Unknown user"),
    }
}

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
        block_time: unix_epoch_now(),
        block_production_time: unix_epoch_now(),
        current_transaction: mock_empty_transaction_hash(),
        original_transaction: mock_empty_transaction_hash(),
    }
}

pub fn mock_successful_callback_context() -> CallbackContext {
    CallbackContext {
        success: true,
        results: vec![],
    }
}

pub fn string_to_bytes(s: &str) -> Vec<u8> {
    s.to_string().into_bytes()
}

pub fn tomorrow_timestamp() -> i64 {
    unix_epoch_now() + (60 * 60 * 24 * 1000)
}

pub fn yesterday_timestamp() -> i64 {
    unix_epoch_now() - (60 * 60 * 24 * 1000)
}

pub fn unix_epoch_now() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}
