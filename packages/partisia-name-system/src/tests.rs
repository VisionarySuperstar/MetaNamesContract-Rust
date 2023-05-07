use crate::{
    actions::{
        execute_record_delete, execute_init, execute_mint, execute_record_mint,
        execute_record_update,
    },
    msg::{
        ApproveForAllMsg, ApproveMsg, BurnMsg, CheckOwnerMsg, InitMsg, MintMsg, MultiMintMsg,
        RecordDeleteMsg, RecordMintMsg, RecordUpdateMsg, RevokeForAllMsg, RevokeMsg, SetBaseUriMsg,
        TransferFromMsg, TransferMsg, UpdateMinterMsg,
    },
    state::{Record, RecordClass},
};

use pbc_contract_common::{
    address::{Address, AddressType, Shortname},
    context::ContractContext,
    events::EventGroup,
};

use utils::events::IntoShortnameRPCEvent;

// TODO: DRY up tests

// TODO: Add tests for functionality
// TODO: Test parent

fn mock_address(le: u8) -> Address {
    Address {
        address_type: AddressType::Account,
        identifier: [
            le, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8,
        ],
    }
}

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

    let msg = TransferMsg {
        to: mock_address(1u8),
        token_id: "name.meta".to_string(),
    };
    let mut event_group = EventGroup::builder();
    let mut test_event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    test_event_group
        .call(dest.clone(), Shortname::from_u32(TRANSFER))
        .argument(mock_address(1u8))
        .argument("name.meta".to_string())
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_transfer_from_action_call() {
    let dest = mock_address(30u8);

    let msg = TransferFromMsg {
        from: mock_address(1u8),
        to: mock_address(2u8),
        token_id: "name.meta".to_string(),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(TRANSFER_FROM))
        .argument(mock_address(1u8))
        .argument(mock_address(2u8))
        .argument("name.meta".to_string())
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_approve_action_call() {
    let dest = mock_address(30u8);

    let msg = ApproveMsg {
        spender: mock_address(1u8),
        token_id: "name.meta".to_string(),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(APPROVE))
        .argument(mock_address(1u8))
        .argument("name.meta".to_string())
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_set_base_uri_action_call() {
    let dest = mock_address(30u8);

    let msg = SetBaseUriMsg {
        new_base_uri: "new".to_string(),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(SET_BASE_URI))
        .argument("new".to_string())
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_mint_action_call() {
    let dest = mock_address(30u8);

    let msg = MintMsg {
        token_id: "name.meta".to_string(),
        to: mock_address(1u8),
        token_uri: Some("uri".to_string()),
        parent_id: None,
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(MINT))
        .argument("name.meta".to_string())
        .argument(mock_address(1u8))
        .argument(Some("uri".to_string()))
        .argument(None::<String>)
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_ownership_check_call() {
    let dest = mock_address(30u8);

    let msg = CheckOwnerMsg {
        owner: mock_address(1u8),
        token_id: "name.meta".to_string(),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(CHECKOWNER))
        .argument(mock_address(1u8))
        .argument("name.meta".to_string())
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_approve_for_all_action_call() {
    let dest = mock_address(30u8);

    let msg = ApproveForAllMsg {
        operator: mock_address(1u8),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(APPROVE_FOR_ALL))
        .argument(mock_address(1u8))
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_revoke_action_call() {
    let dest = mock_address(30u8);

    let msg = RevokeMsg {
        spender: mock_address(1u8),
        token_id: "name.meta".to_string(),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(REVOKE))
        .argument(mock_address(1u8))
        .argument("name.meta".to_string())
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_revoke_for_all_action_call() {
    let dest = mock_address(30u8);

    let msg = RevokeForAllMsg {
        operator: mock_address(1u8),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(REVOKE_FOR_ALL))
        .argument(mock_address(1u8))
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_burn_action_call() {
    let dest = mock_address(30u8);

    let msg = BurnMsg {
        token_id: "name.meta".to_string(),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(BURN))
        .argument("name.meta".to_string())
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}
#[test]
fn proper_minter_update_action_call() {
    let dest = mock_address(30u8);

    let msg = UpdateMinterMsg {
        new_minter: mock_address(19u8),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(UPDATE_MINTER))
        .argument(mock_address(19u8))
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}
#[test]
fn proper_multi_mint_action_call() {
    let dest = mock_address(30u8);

    let mints = vec![
        MintMsg {
            token_id: "name.meta".to_string(),
            to: mock_address(4),
            token_uri: Some(String::from("Token1")),
            parent_id: None,
        },
        MintMsg {
            token_id: "name2.meta".to_string(),
            to: mock_address(4),
            token_uri: Some(String::from("Token2")),
            parent_id: None,
        },
        MintMsg {
            token_id: "name3.meta".to_string(),
            to: mock_address(5),
            token_uri: Some(String::from("Token3")),
            parent_id: None,
        },
        MintMsg {
            token_id: "name4.meta".to_string(),
            to: mock_address(5),
            token_uri: Some(String::from("Token4")),
            parent_id: None,
        },
        MintMsg {
            token_id: "name5.meta".to_string(),
            to: mock_address(6),
            token_uri: Some(String::from("Token5")),
            parent_id: None,
        },
    ];
    let msg = MultiMintMsg {
        mints: mints.clone(),
    };
    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest.clone(), Shortname::from_u32(MULTI_MINT))
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
        current_transaction: [
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        ],
        original_transaction: [
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        ],
    }
}

#[test]
fn proper_mint() {
    let minter = 1u8;
    let alice = 10u8;

    let msg = InitMsg {
        owner: None,
        name: "Meta Names".to_string(),
        symbol: "META".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let ref token_id = "name.meta".to_string();
    let mint_msg = MintMsg {
        token_id: token_id.clone(),
        to: mock_address(alice),
        token_uri: Some(String::from("name.meta")),
        parent_id: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let num_token_id = state.token_id(token_id).unwrap();
    assert_eq!(num_token_id, 1);
}

// TODO: test if parent doesn't exist
// TODO: test if parent is not owned by sender

#[test]
fn proper_record_mint() {
    let minter = 1u8;
    let alice = 10u8;

    let msg = InitMsg {
        owner: None,
        name: "Meta Names".to_string(),
        symbol: "META".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let ref token_id = "name.meta".to_string();
    let mint_msg = MintMsg {
        token_id: token_id.clone(),
        to: mock_address(alice),
        token_uri: Some(String::from("name.meta")),
        parent_id: None,
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

// TODO: Test if record doesn't exist
// TODO: Test if record is not owned by sender

#[test]
fn proper_record_update() {
    let minter = 1u8;
    let alice = 10u8;

    let msg = InitMsg {
        owner: None,
        name: "Meta Names".to_string(),
        symbol: "META".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let ref token_id = "name.meta".to_string();
    let mint_msg = MintMsg {
        token_id: token_id.clone(),
        to: mock_address(alice),
        token_uri: Some(String::from("name.meta")),
        parent_id: None,
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

// TODO: Test if record doesn't exist
// TODO: Test if record is not owned by sender

#[test]
fn proper_record_delete() {
    let minter = 1u8;
    let alice = 10u8;

    let msg = InitMsg {
        owner: None,
        name: "Meta Names".to_string(),
        symbol: "META".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let ref token_id = "name.meta".to_string();
    let mint_msg = MintMsg {
        token_id: token_id.clone(),
        to: mock_address(alice),
        token_uri: Some(String::from("name.meta")),
        parent_id: None,
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

// TODO: Test if record doesn't exist
// TODO: Test if record is not owned by sender
