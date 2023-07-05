use nft::msg::{NFTApproveForAllMsg, NFTApproveMsg, NFTTransferFromMsg};
use partisia_name_system::{
    msg::{PnsRecordDeleteMsg, PnsRecordMintMsg, PnsRecordUpdateMsg},
    state::RecordClass,
};

use pbc_contract_common::{
    address::{Address, AddressType, Shortname},
    events::EventGroup,
};

use utils::events::IntoShortnameRPCEvent;

use crate::msg::MintMsg;

fn mock_address(le: u8) -> Address {
    Address {
        address_type: AddressType::Account,
        identifier: [
            le, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8,
        ],
    }
}

fn string_to_bytes(s: &str) -> Vec<u8> {
    s.to_string().into_bytes()
}

const TRANSFER_FROM: u32 = 0x03;
const APPROVE: u32 = 0x05;
const APPROVE_FOR_ALL: u32 = 0x07;
const MINT: u32 = 0x09;

const RECORD_MINT: u32 = 0x21;
const RECORD_UPDATE: u32 = 0x22;
const RECORD_DELETE: u32 = 0x23;

#[test]
fn proper_transfer_from_action_call() {
    let dest = mock_address(30u8);

    let msg = NFTTransferFromMsg {
        from: mock_address(1u8),
        to: mock_address(2u8),
        token_id: 1u128,
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest, Shortname::from_u32(TRANSFER_FROM))
        .argument(mock_address(1u8))
        .argument(mock_address(2u8))
        .argument(1u128)
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_approve_action_call() {
    let dest = mock_address(30u8);

    let msg = NFTApproveMsg {
        approved: Some(mock_address(1u8)),
        token_id: 1u128,
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest, Shortname::from_u32(APPROVE))
        .argument(Some(mock_address(1u8)))
        .argument(1u128)
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_mint_action_call() {
    let dest = mock_address(30u8);

    let msg = MintMsg {
        domain: string_to_bytes("meta.name"),
        to: mock_address(1u8),
        token_uri: None,
        parent_id: Some(string_to_bytes("")),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest, Shortname::from_u32(MINT))
        .argument(string_to_bytes("meta.name"))
        .argument(mock_address(1u8))
        .argument(None::<String>)
        .argument(Some(string_to_bytes("")))
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_record_mint_action_call() {
    let dest = mock_address(30u8);

    let msg = PnsRecordMintMsg {
        domain: string_to_bytes("meta.name"),
        class: RecordClass::Wallet {},
        data: string_to_bytes(""),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest, Shortname::from_u32(RECORD_MINT))
        .argument(string_to_bytes("meta.name"))
        .argument(RecordClass::Wallet {})
        .argument(string_to_bytes(""))
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_record_update_action_call() {
    let dest = mock_address(30u8);

    let msg = PnsRecordUpdateMsg {
        domain: string_to_bytes("meta.name"),
        class: RecordClass::Wallet {},
        data: string_to_bytes(""),
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest, Shortname::from_u32(RECORD_UPDATE))
        .argument(string_to_bytes("meta.name"))
        .argument(RecordClass::Wallet {})
        .argument(string_to_bytes(""))
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_record_delete_action_call() {
    let dest = mock_address(30u8);

    let msg = PnsRecordDeleteMsg {
        domain: string_to_bytes("meta.name"),
        class: RecordClass::Wallet {},
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest, Shortname::from_u32(RECORD_DELETE))
        .argument(string_to_bytes("meta.name"))
        .argument(RecordClass::Wallet {})
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}

#[test]
fn proper_approve_for_all_action_call() {
    let dest = mock_address(30u8);

    let msg = NFTApproveForAllMsg {
        operator: mock_address(1u8),
        approved: true,
    };

    let mut event_group = EventGroup::builder();
    msg.as_interaction(&mut event_group, &dest);

    let mut test_event_group = EventGroup::builder();
    test_event_group
        .call(dest, Shortname::from_u32(APPROVE_FOR_ALL))
        .argument(mock_address(1u8))
        .argument(true)
        .done();

    assert_eq!(event_group.build(), test_event_group.build());
}
