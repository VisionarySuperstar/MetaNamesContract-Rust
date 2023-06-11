use std::{panic::catch_unwind, rc::Rc, sync::Mutex};

use crate::{
    actions::{
        execute_init, execute_mint, execute_record_delete, execute_record_mint,
        execute_record_update,
    },
    msg::{
        PnsApproveForAllMsg, PnsApproveMsg, PnsBurnMsg, PnsCheckOwnerMsg, PnsInitMsg, PnsMintMsg,
        PnsMultiMintMsg, PnsRevokeForAllMsg, PnsRevokeMsg, PnsSetBaseUriMsg, PnsTransferFromMsg,
        PnsTransferMsg, PnsUpdateMinterMsg, RecordDeleteMsg, RecordMintMsg, RecordUpdateMsg,
    },
    state::{Record, RecordClass},
};

use pbc_contract_common::{address::Shortname, context::ContractContext, events::EventGroup};

use utils::{events::IntoShortnameRPCEvent, tests::{mock_address, string_to_bytes, mock_empty_transaction_hash}};

// TODO: DRY up tests

const TRANSFER: u32 = 0x01;
const TRANSFER_FROM: u32 = 0x03;
const APPROVE: u32 = 0x05;
const SET_BASE_URI: u32 = 0x07;
const MINT: u32 = 0x09;
const APPROVE_FOR_ALL: u32 = 0x11;
const REVOKE: u32 = 0x13;
const REVOKE_FOR_ALL: u32 = 0x15;
const BURN: u32 = 0x17;

const MULTI_MINT: u32 = 0x20;
const CHECKOWNER: u32 = 0x18;
const UPDATE_MINTER: u32 = 0x19;

#[test]
fn proper_transfer_action_call() {
    let dest = mock_address(30u8);

    let msg = PnsTransferMsg {
        to: mock_address(1u8),
        token_id: string_to_bytes("name"),
    };
    let mut event_group = EventGroup::builder();
    let mut test_event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    test_event_group
        .call(dest, Shortname::from_u32(TRANSFER))
        .argument(mock_address(1u8))
        .argument(string_to_bytes("name"))
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_transfer_from_action_call() {
    let dest = mock_address(30u8);

    let msg = PnsTransferFromMsg {
        from: mock_address(1u8),
        to: mock_address(2u8),
        token_id: string_to_bytes("name"),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest, Shortname::from_u32(TRANSFER_FROM))
        .argument(mock_address(1u8))
        .argument(mock_address(2u8))
        .argument(string_to_bytes("name"))
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_approve_action_call() {
    let dest = mock_address(30u8);

    let msg = PnsApproveMsg {
        spender: mock_address(1u8),
        token_id: string_to_bytes("name"),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest, Shortname::from_u32(APPROVE))
        .argument(mock_address(1u8))
        .argument(string_to_bytes("name"))
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_set_base_uri_action_call() {
    let dest = mock_address(30u8);

    let msg = PnsSetBaseUriMsg {
        new_base_uri: "new".to_string(),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest, Shortname::from_u32(SET_BASE_URI))
        .argument("new".to_string())
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_mint_action_call() {
    let dest = mock_address(30u8);

    let msg = PnsMintMsg {
        token_id: string_to_bytes("name"),
        to: mock_address(1u8),
        token_uri: Some("uri".to_string()),
        parent_id: Some(string_to_bytes("meta")),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest, Shortname::from_u32(MINT))
        .argument(string_to_bytes("name"))
        .argument(mock_address(1u8))
        .argument(Some("uri".to_string()))
        .argument(Some("meta".to_string()))
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_ownership_check_call() {
    let dest = mock_address(30u8);

    let msg = PnsCheckOwnerMsg {
        owner: mock_address(1u8),
        token_id: string_to_bytes("name"),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest, Shortname::from_u32(CHECKOWNER))
        .argument(mock_address(1u8))
        .argument(string_to_bytes("name"))
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_approve_for_all_action_call() {
    let dest = mock_address(30u8);

    let msg = PnsApproveForAllMsg {
        operator: mock_address(1u8),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest, Shortname::from_u32(APPROVE_FOR_ALL))
        .argument(mock_address(1u8))
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_revoke_action_call() {
    let dest = mock_address(30u8);

    let msg = PnsRevokeMsg {
        spender: mock_address(1u8),
        token_id: string_to_bytes("name"),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest, Shortname::from_u32(REVOKE))
        .argument(mock_address(1u8))
        .argument(string_to_bytes("name"))
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_revoke_for_all_action_call() {
    let dest = mock_address(30u8);

    let msg = PnsRevokeForAllMsg {
        operator: mock_address(1u8),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest, Shortname::from_u32(REVOKE_FOR_ALL))
        .argument(mock_address(1u8))
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_burn_action_call() {
    let dest = mock_address(30u8);

    let msg = PnsBurnMsg {
        token_id: string_to_bytes("name"),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest, Shortname::from_u32(BURN))
        .argument(string_to_bytes("name"))
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}
#[test]
fn proper_minter_update_action_call() {
    let dest = mock_address(30u8);

    let msg = PnsUpdateMinterMsg {
        new_minter: mock_address(19u8),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest, Shortname::from_u32(UPDATE_MINTER))
        .argument(mock_address(19u8))
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}
#[test]
fn proper_multi_mint_action_call() {
    let dest = mock_address(30u8);

    let mints = vec![
        PnsMintMsg {
            token_id: string_to_bytes("name"),
            to: mock_address(4),
            token_uri: Some(String::from("Token1")),
            parent_id: Some(string_to_bytes("meta")),
        },
        PnsMintMsg {
            token_id: string_to_bytes("name2"),
            to: mock_address(4),
            token_uri: Some(String::from("Token2")),
            parent_id: Some(string_to_bytes("meta")),
        },
        PnsMintMsg {
            token_id: string_to_bytes("name3"),
            to: mock_address(5),
            token_uri: Some(String::from("Token3")),
            parent_id: Some(string_to_bytes("meta")),
        },
        PnsMintMsg {
            token_id: string_to_bytes("name4"),
            to: mock_address(5),
            token_uri: Some(String::from("Token4")),
            parent_id: Some(string_to_bytes("meta")),
        },
        PnsMintMsg {
            token_id: string_to_bytes("name5"),
            to: mock_address(6),
            token_uri: Some(String::from("Token5")),
            parent_id: Some(string_to_bytes("meta")),
        },
    ];
    let msg = PnsMultiMintMsg {
        mints: mints.clone(),
    };
    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest, Shortname::from_u32(MULTI_MINT))
        .argument(mints)
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

    let msg = PnsInitMsg {
        owner: None,
        name: "Meta Names".to_string(),
        symbol: "META".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
        tld: Some(string_to_bytes("meta")),
        tld_uri: Some("meta".to_string()),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let token_id = string_to_bytes("name");
    let mint_msg = PnsMintMsg {
        token_id: token_id.clone(),
        to: mock_address(alice),
        token_uri: Some(String::from("name")),
        parent_id: Some(string_to_bytes("meta")),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let num_token_id = state.token_id(&token_id).unwrap();
    assert_eq!(num_token_id, 2);
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn sender_is_not_minter_on_mint() {
    let minter = 1u8;
    let alice = 10u8;

    let msg = PnsInitMsg {
        owner: None,
        name: "Meta Names".to_string(),
        symbol: "META".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
        tld: Some(string_to_bytes("meta")),
        tld_uri: Some("meta".to_string()),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = PnsMintMsg {
        token_id: string_to_bytes("name"),
        to: mock_address(alice),
        token_uri: Some("name".to_string()),
        parent_id: Some(string_to_bytes("meta")),
    };

    let _ = execute_mint(&mock_contract_context(alice), &mut state, &mint_msg);
}

#[test]
#[should_panic(expected = "Token with specified id is already minted")]
fn token_already_minted_on_mint() {
    let minter = 1u8;
    let alice = 10u8;

    let msg = PnsInitMsg {
        owner: None,
        name: "Meta Names".to_string(),
        symbol: "META".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
        tld: Some(string_to_bytes("meta")),
        tld_uri: Some("meta".to_string()),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = PnsMintMsg {
        token_id: string_to_bytes("name"),
        token_uri: Some("name".to_string()),
        to: mock_address(alice),
        parent_id: Some(string_to_bytes("meta")),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let mint_msg = PnsMintMsg {
        token_id: string_to_bytes("name"),
        token_uri: Some("name".to_string()),
        to: mock_address(alice),
        parent_id: Some(string_to_bytes("meta")),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);
}

#[test]
#[should_panic(expected = "Not found")]
fn mint_fails_when_parent_does_not_exist() {
    let minter = 1u8;
    let alice = 10u8;

    let msg = PnsInitMsg {
        owner: None,
        name: "Meta Names".to_string(),
        symbol: "META".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
        tld: Some(string_to_bytes("meta")),
        tld_uri: Some("meta".to_string()),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let token_id = &string_to_bytes("name");
    let mint_msg = PnsMintMsg {
        token_id: token_id.clone(),
        to: mock_address(alice),
        token_uri: Some(String::from("name")),
        parent_id: Some(string_to_bytes("not.existing.meta")),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn mint_fails_when_parent_is_not_owned() {
    let minter = 1u8;
    let alice = 10u8;
    let bob = 20u8;

    let msg = PnsInitMsg {
        owner: None,
        name: "Meta Names".to_string(),
        symbol: "META".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
        tld: Some(string_to_bytes("meta")),
        tld_uri: Some("meta".to_string()),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let token_id = &string_to_bytes("name");
    let mint_msg = PnsMintMsg {
        token_id: token_id.clone(),
        to: mock_address(alice),
        token_uri: Some(String::from("name")),
        parent_id: Some(string_to_bytes("meta")),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let mint_msg = PnsMintMsg {
        token_id: string_to_bytes("bob.name"),
        to: mock_address(alice),
        token_uri: Some(String::from("bob.name")),
        parent_id: Some(string_to_bytes("name")),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);
}

#[test]
fn when_parent_is_not_owned_no_mint() {
    let minter = 1u8;
    let alice = 10u8;
    let bob = 20u8;

    let msg = PnsInitMsg {
        owner: None,
        name: "Meta Names".to_string(),
        symbol: "META".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
        tld: Some(string_to_bytes("meta")),
        tld_uri: Some("meta".to_string()),
    };

    let (state, events) = execute_init(&mock_contract_context(2), &msg);

    let token_id = &string_to_bytes("name");
    let mint_msg = PnsMintMsg {
        token_id: token_id.clone(),
        to: mock_address(alice),
        token_uri: Some(String::from("name")),
        parent_id: Some(string_to_bytes("not.existing.meta")),
    };

    let state_mutex = Rc::new(Mutex::new(state));
    let _ = catch_unwind(|| {
        let mut state_mut = state_mutex.lock().unwrap();
        let _ = execute_mint(&mock_contract_context(minter), &mut state_mut, &mint_msg);
    });

    let err = state_mutex.lock().err().unwrap();

    let mpc721 = &err.get_ref().mpc721;
    // The only domain present is the TLD
    assert_eq!(mpc721.tokens.len(), 1);

    // Check that the TLD is present
    let (_, token) = mpc721.tokens.first_key_value().unwrap();
    assert_eq!(token.owner, mock_address(minter));
    assert_eq!(token.token_uri, Some("meta".to_string()));
    assert_eq!(token.parent_id, None);
}

#[test]
fn proper_record_mint() {
    let minter = 1u8;
    let alice = 10u8;

    let msg = PnsInitMsg {
        owner: None,
        name: "Meta Names".to_string(),
        symbol: "META".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
        tld: Some(string_to_bytes("meta")),
        tld_uri: Some("meta".to_string()),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let token_id = &string_to_bytes("name");
    let mint_msg = PnsMintMsg {
        token_id: token_id.clone(),
        to: mock_address(alice),
        token_uri: Some(String::from("name")),
        parent_id: Some(string_to_bytes("meta")),
    };
    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let record_class = RecordClass::Twitter {};
    let record_mint_msg = RecordMintMsg {
        token_id: token_id.clone(),
        class: record_class,
        data: "data".to_string(),
    };
    let _ = execute_record_mint(&mock_contract_context(alice), &mut state, &record_mint_msg);

    let record = state.record_info(token_id, &record_class).unwrap();
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

    let msg = PnsInitMsg {
        owner: None,
        name: "Meta Names".to_string(),
        symbol: "META".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
        tld: Some(string_to_bytes("meta")),
        tld_uri: Some("meta".to_string()),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = PnsMintMsg {
        token_id: string_to_bytes("name"),
        to: mock_address(alice),
        token_uri: Some("name".to_string()),
        parent_id: Some(string_to_bytes("meta")),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let record_mint = RecordMintMsg {
        token_id: string_to_bytes("not-existing.meta"),
        class: RecordClass::Wallet {},
        data: "some data".to_string(),
    };

    let _ = execute_record_mint(&mock_contract_context(alice), &mut state, &record_mint);
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn when_token_not_owned_record_mint_fails() {
    let minter = 1u8;
    let alice = 10u8;
    let bob = 20u8;

    let msg = PnsInitMsg {
        owner: None,
        name: "Meta Names".to_string(),
        symbol: "META".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
        tld: Some(string_to_bytes("meta")),
        tld_uri: Some("meta".to_string()),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = PnsMintMsg {
        token_id: string_to_bytes("name"),
        to: mock_address(alice),
        token_uri: Some("name".to_string()),
        parent_id: Some(string_to_bytes("meta")),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let record_mint = RecordMintMsg {
        token_id: string_to_bytes("name"),
        class: RecordClass::Wallet {},
        data: "some data".to_string(),
    };

    let _ = execute_record_mint(&mock_contract_context(bob), &mut state, &record_mint);
}

#[test]
#[should_panic(expected = "Record with specified token id and class is already minted")]
fn record_already_minted_on_record_mint() {
    let minter = 1u8;
    let alice = 10u8;

    let msg = PnsInitMsg {
        owner: None,
        name: "Meta Names".to_string(),
        symbol: "META".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
        tld: Some(string_to_bytes("meta")),
        tld_uri: Some("meta".to_string()),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = PnsMintMsg {
        token_id: string_to_bytes("name"),
        to: mock_address(alice),
        token_uri: Some("name".to_string()),
        parent_id: Some(string_to_bytes("meta")),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let record_mint = RecordMintMsg {
        token_id: string_to_bytes("name"),
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

    let msg = PnsInitMsg {
        owner: None,
        name: "Meta Names".to_string(),
        symbol: "META".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
        tld: Some(string_to_bytes("meta")),
        tld_uri: Some("meta".to_string()),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let token_id = &string_to_bytes("name");
    let mint_msg = PnsMintMsg {
        token_id: token_id.clone(),
        to: mock_address(alice),
        token_uri: Some(String::from("name")),
        parent_id: Some(string_to_bytes("meta")),
    };
    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let record_class = RecordClass::Twitter {};
    let record_mint_msg = RecordMintMsg {
        token_id: token_id.clone(),
        class: record_class,
        data: "data".to_string(),
    };
    let _ = execute_record_mint(&mock_contract_context(alice), &mut state, &record_mint_msg);

    let record_update_msg = RecordUpdateMsg {
        token_id: token_id.clone(),
        class: record_class,
        data: "new data".to_string(),
    };

    let _ = execute_record_update(
        &mock_contract_context(alice),
        &mut state,
        &record_update_msg,
    );

    let record = state.record_info(token_id, &record_class).unwrap();
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

    let msg = PnsInitMsg {
        owner: None,
        name: "Meta Names".to_string(),
        symbol: "META".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
        tld: Some(string_to_bytes("meta")),
        tld_uri: Some("meta".to_string()),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let record_update_msg = RecordUpdateMsg {
        token_id: string_to_bytes("name"),
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
#[should_panic(expected = "Unauthorized")]
fn when_record_is_not_owned_record_update_fails() {
    let minter = 1u8;
    let alice = 10u8;
    let bob = 20u8;

    let msg = PnsInitMsg {
        owner: None,
        name: "Meta Names".to_string(),
        symbol: "META".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
        tld: Some(string_to_bytes("meta")),
        tld_uri: Some("meta".to_string()),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let token_id = &string_to_bytes("name");
    let mint_msg = PnsMintMsg {
        token_id: token_id.clone(),
        to: mock_address(alice),
        token_uri: Some(String::from("name")),
        parent_id: Some(string_to_bytes("meta")),
    };
    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let record_class = RecordClass::Twitter {};
    let record_mint_msg = RecordMintMsg {
        token_id: token_id.clone(),
        class: record_class,
        data: "data".to_string(),
    };
    let _ = execute_record_mint(&mock_contract_context(alice), &mut state, &record_mint_msg);

    let record_update_msg = RecordUpdateMsg {
        token_id: token_id.clone(),
        class: record_class,
        data: "new data".to_string(),
    };

    let _ = execute_record_update(&mock_contract_context(bob), &mut state, &record_update_msg);

    let record = state.record_info(token_id, &record_class).unwrap();
    assert_eq!(
        *record,
        Record {
            data: "new data".to_string(),
        }
    );
}

#[test]
fn proper_record_delete() {
    let minter = 1u8;
    let alice = 10u8;

    let msg = PnsInitMsg {
        owner: None,
        name: "Meta Names".to_string(),
        symbol: "META".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
        tld: Some(string_to_bytes("meta")),
        tld_uri: Some("meta".to_string()),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let token_id = &string_to_bytes("name");
    let mint_msg = PnsMintMsg {
        token_id: token_id.clone(),
        to: mock_address(alice),
        token_uri: Some(String::from("name")),
        parent_id: Some(string_to_bytes("meta")),
    };
    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let record_class = RecordClass::Twitter {};
    let record_mint_msg = RecordMintMsg {
        token_id: token_id.clone(),
        class: record_class,
        data: "data".to_string(),
    };
    let _ = execute_record_mint(&mock_contract_context(alice), &mut state, &record_mint_msg);

    let record_delete_msg = RecordDeleteMsg {
        token_id: token_id.clone(),
        class: record_class,
    };

    let _ = execute_record_delete(
        &mock_contract_context(alice),
        &mut state,
        &record_delete_msg,
    );

    let record = state.record_info(token_id, &record_class);

    assert!(record.is_none());
}

#[test]
#[should_panic(expected = "Not found")]
fn when_record_does_not_exist_record_delete_fails() {
    let minter = 1u8;
    let alice = 10u8;

    let msg = PnsInitMsg {
        owner: None,
        name: "Meta Names".to_string(),
        symbol: "META".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
        tld: Some(string_to_bytes("meta")),
        tld_uri: Some("meta".to_string()),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let record_delete_msg = RecordDeleteMsg {
        token_id: string_to_bytes("name"),
        class: RecordClass::Twitter {},
    };

    let _ = execute_record_delete(
        &mock_contract_context(alice),
        &mut state,
        &record_delete_msg,
    );
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn when_record_is_not_owned_record_delete_fails() {
    let minter = 1u8;
    let alice = 10u8;
    let bob = 20u8;

    let msg = PnsInitMsg {
        owner: None,
        name: "Meta Names".to_string(),
        symbol: "META".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
        tld: Some(string_to_bytes("meta")),
        tld_uri: Some("meta".to_string()),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let token_id = &string_to_bytes("name");
    let mint_msg = PnsMintMsg {
        token_id: token_id.clone(),
        to: mock_address(alice),
        token_uri: Some(String::from("name")),
        parent_id: Some(string_to_bytes("meta")),
    };
    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let record_class = RecordClass::Twitter {};
    let record_mint_msg = RecordMintMsg {
        token_id: token_id.clone(),
        class: record_class,
        data: "data".to_string(),
    };
    let _ = execute_record_mint(&mock_contract_context(alice), &mut state, &record_mint_msg);

    let record_delete_msg = RecordDeleteMsg {
        token_id: token_id.clone(),
        class: record_class,
    };

    let _ = execute_record_delete(&mock_contract_context(bob), &mut state, &record_delete_msg);
}
