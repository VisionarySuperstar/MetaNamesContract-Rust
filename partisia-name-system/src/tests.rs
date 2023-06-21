use crate::{
    actions::{
        execute_init, execute_mint, execute_record_delete, execute_record_mint,
        execute_record_update,
    },
    msg::{PnsMintMsg, RecordDeleteMsg, RecordMintMsg, RecordUpdateMsg},
    state::{Record, RecordClass},
};

use pbc_contract_common::{address::Shortname, context::ContractContext, events::EventGroup};

use utils::{
    events::IntoShortnameRPCEvent,
    tests::{mock_address, mock_empty_transaction_hash, string_to_bytes},
};

// TODO: DRY up tests

// TODO: Review action calls

const MINT: u32 = 0x09;

#[test]
fn proper_mint_action_call() {
    let dest = mock_address(30u8);

    let msg = PnsMintMsg {
        domain: string_to_bytes("name"),
        token_id: 1,
        to: mock_address(1u8),
        token_uri: Some("uri".to_string()),
        parent_id: None,
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest, Shortname::from_u32(MINT))
        .argument(string_to_bytes("name"))
        .argument(1u128)
        .argument(mock_address(1u8))
        .argument(Some("uri".to_string()))
        .argument(None::<Vec<u8>>)
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

// Test actions

fn mock_contract_context(sender: u8) -> ContractContext {
    ContractContext {
        contract_address: mock_address(1u8),
        sender: mock_address(sender),
        block_time: 100,
        block_production_time: 100,
        current_transaction: mock_empty_transaction_hash(),
        original_transaction: mock_empty_transaction_hash(),
    }
}

#[test]
fn proper_mint() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let domain = string_to_bytes("name");
    let mint_msg = PnsMintMsg {
        token_id: 1,
        domain: domain.clone(),
        to: mock_address(alice),
        token_uri: Some(String::from("name")),
        parent_id: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let num_token_id = state.get_token_id(&domain).unwrap();
    assert_eq!(num_token_id, 1);
}

#[test]
fn proper_mint_with_parent() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let domain = string_to_bytes("meta");
    let mint_msg = PnsMintMsg {
        token_id: 1,
        domain,
        to: mock_address(alice),
        token_uri: Some(String::from("meta")),
        parent_id: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let domain = string_to_bytes("name");
    let mint_msg = PnsMintMsg {
        token_id: 2,
        domain,
        to: mock_address(alice),
        token_uri: Some(String::from("name")),
        parent_id: Some(string_to_bytes("meta")),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let domains_length = state.domains.len();
    assert_eq!(domains_length, 2);
}

#[test]
#[should_panic(expected = "Not found")]
fn when_parent_does_not_exist_mint_fails() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let domain = string_to_bytes("meta");
    let mint_msg = PnsMintMsg {
        token_id: 1,
        domain,
        to: mock_address(alice),
        token_uri: Some(String::from("meta")),
        parent_id: Some(string_to_bytes("notfound")),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);
}

#[test]
#[should_panic(expected = "Token with specified id is already minted")]
fn token_already_minted_on_mint() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let mint_msg = PnsMintMsg {
        domain: string_to_bytes("name"),
        token_id: 1,
        token_uri: Some("name".to_string()),
        to: mock_address(alice),
        parent_id: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let mint_msg = PnsMintMsg {
        domain: string_to_bytes("name"),
        token_id: 2,
        token_uri: Some("name".to_string()),
        to: mock_address(alice),
        parent_id: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);
}

#[test]
#[should_panic(expected = "Not found")]
fn mint_fails_when_parent_does_not_exist() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let domain = &string_to_bytes("name");
    let mint_msg = PnsMintMsg {
        domain: domain.clone(),
        token_id: 1,
        to: mock_address(alice),
        token_uri: Some(String::from("name")),
        parent_id: Some(string_to_bytes("not.existing.meta")),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);
}

#[test]
fn proper_record_mint() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let domain = &string_to_bytes("name");
    let mint_msg = PnsMintMsg {
        domain: domain.clone(),
        token_id: 1,
        to: mock_address(alice),
        token_uri: Some(String::from("name")),
        parent_id: None,
    };
    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let record_class = RecordClass::Twitter {};
    let record_mint_msg = RecordMintMsg {
        domain: domain.clone(),
        class: record_class,
        data: "data".to_string(),
    };
    let _ = execute_record_mint(&mock_contract_context(alice), &mut state, &record_mint_msg);

    let domain = state.get_domain(domain).unwrap();
    let record = domain.get_record(&record_class).unwrap();
    assert_eq!(
        *record,
        Record {
            data: "data".to_string(),
        }
    );
}

#[test]
#[should_panic(expected = "Not found")]
fn when_token_not_present_record_mint_fails() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let mint_msg = PnsMintMsg {
        domain: string_to_bytes("name"),
        token_id: 1,
        to: mock_address(alice),
        token_uri: Some("name".to_string()),
        parent_id: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let record_mint = RecordMintMsg {
        domain: string_to_bytes("not-existing.meta"),
        class: RecordClass::Wallet {},
        data: "some data".to_string(),
    };

    let _ = execute_record_mint(&mock_contract_context(alice), &mut state, &record_mint);
}

#[test]
#[should_panic(expected = "Record with specified token id and class is already minted")]
fn record_already_minted_on_record_mint() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let mint_msg = PnsMintMsg {
        domain: string_to_bytes("name"),
        token_id: 1,
        to: mock_address(alice),
        token_uri: Some("name".to_string()),
        parent_id: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let record_mint = RecordMintMsg {
        domain: string_to_bytes("name"),
        class: RecordClass::Wallet {},
        data: "some data".to_string(),
    };

    let _ = execute_record_mint(&mock_contract_context(alice), &mut state, &record_mint);
    let _ = execute_record_mint(&mock_contract_context(alice), &mut state, &record_mint);
}

#[test]
fn proper_record_update() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let domain = &string_to_bytes("name");
    let mint_msg = PnsMintMsg {
        domain: domain.clone(),
        token_id: 1,
        to: mock_address(alice),
        token_uri: Some(String::from("name")),
        parent_id: None,
    };
    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let record_class = RecordClass::Twitter {};
    let record_mint_msg = RecordMintMsg {
        domain: domain.clone(),
        class: record_class,
        data: "data".to_string(),
    };
    let _ = execute_record_mint(&mock_contract_context(alice), &mut state, &record_mint_msg);

    let record_update_msg = RecordUpdateMsg {
        domain: domain.clone(),
        class: record_class,
        data: "new data".to_string(),
    };

    let _ = execute_record_update(
        &mock_contract_context(alice),
        &mut state,
        &record_update_msg,
    );

    let domain = state.get_domain(domain).unwrap();
    let record = domain.get_record(&record_class).unwrap();
    assert_eq!(
        *record,
        Record {
            data: "new data".to_string(),
        }
    );
}

#[test]
#[should_panic(expected = "Not found")]
fn when_record_does_not_exist_record_update_fails() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let record_update_msg = RecordUpdateMsg {
        domain: string_to_bytes("name"),
        class: RecordClass::Twitter {},
        data: "new data".to_string(),
    };

    let _ = execute_record_update(
        &mock_contract_context(alice),
        &mut state,
        &record_update_msg,
    );
}

#[test]
fn proper_record_delete() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let domain = &string_to_bytes("name");
    let mint_msg = PnsMintMsg {
        domain: domain.clone(),
        token_id: 1,
        to: mock_address(alice),
        token_uri: Some(String::from("name")),
        parent_id: None,
    };
    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let record_class = RecordClass::Twitter {};
    let record_mint_msg = RecordMintMsg {
        domain: domain.clone(),
        class: record_class,
        data: "data".to_string(),
    };
    let _ = execute_record_mint(&mock_contract_context(alice), &mut state, &record_mint_msg);

    let record_delete_msg = RecordDeleteMsg {
        domain: domain.clone(),
        class: record_class,
    };

    let _ = execute_record_delete(
        &mock_contract_context(alice),
        &mut state,
        &record_delete_msg,
    );

    let domain = state.get_domain(domain).unwrap();
    let record = domain.get_record(&record_class);

    assert!(record.is_none());
}

#[test]
#[should_panic(expected = "Not found")]
fn when_record_does_not_exist_record_delete_fails() {
    let minter = 1u8;
    let alice = 10u8;

    let mut state = execute_init(&mock_contract_context(2));

    let record_delete_msg = RecordDeleteMsg {
        domain: string_to_bytes("name"),
        class: RecordClass::Twitter {},
    };

    let _ = execute_record_delete(
        &mock_contract_context(alice),
        &mut state,
        &record_delete_msg,
    );
}
