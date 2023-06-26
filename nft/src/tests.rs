use pbc_contract_common::sorted_vec_map::SortedVecMap;

use utils::tests::{mock_address, mock_contract_context};

use crate::state::{OperatorApproval, URL_LENGTH};
use crate::{
    actions::{
        execute_approve, execute_burn, execute_init, execute_mint, execute_set_approval_for_all,
        execute_transfer_from,
    },
    msg::{
        NFTApproveForAllMsg, NFTApproveMsg, NFTBurnMsg, NFTInitMsg, NFTMintMsg, NFTTransferFromMsg,
    },
    state::NFTContractState,
};

#[test]
fn proper_execute_init() {
    let msg = NFTInitMsg {
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        uri_template: "ipfs://some.some".to_string(),
    };

    let state = execute_init(&mock_contract_context(2), &msg);
    assert_eq!(
        state,
        NFTContractState {
            name: "Cool Token".to_string(),
            symbol: "CTC".to_string(),
            uri_template: "ipfs://some.some".to_string(),
            supply: 0,
            owners: SortedVecMap::new(),
            token_approvals: SortedVecMap::new(),
            token_uri_details: SortedVecMap::new(),
            operator_approvals: vec![],
        }
    );
}

#[test]
fn proper_mint() {
    let minter = 1u8;
    let alice = 10u8;

    let msg = NFTInitMsg {
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        uri_template: "ipfs://some.some/".to_string(),
    };

    let mut state = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = NFTMintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: Some("token".to_string()),
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);
    assert_eq!(state.supply, 1);

    assert_eq!(state.owners, SortedVecMap::from([(1, mock_address(alice))]));
    let mut expected_bytes: [u8; URL_LENGTH] = [0; URL_LENGTH];
    let bytes = "ipfs://some.some/token".to_string().into_bytes();
    expected_bytes[..bytes.len()].copy_from_slice(&bytes);

    assert_eq!(
        state.token_uri_details,
        SortedVecMap::from([(1, expected_bytes)]),
    );
}

#[test]
#[should_panic(expected = "Token with specified id is already minted")]
fn token_already_minted_on_mint() {
    let minter = 1u8;
    let alice = 10u8;

    let msg = NFTInitMsg {
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        uri_template: "ipfs://some.some".to_string(),
    };

    let mut state = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = NFTMintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let mint_msg = NFTMintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);
}

#[test]
fn proper_set_approve_for_all() {
    let owner = 1u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = NFTInitMsg {
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        uri_template: "ipfs://some.some".to_string(),
    };

    let mut state = execute_init(&mock_contract_context(2), &msg);

    let approve_all_msg = NFTApproveForAllMsg {
        operator: mock_address(bob),
        approved: true,
    };
    let _ =
        execute_set_approval_for_all(&mock_contract_context(alice), &mut state, &approve_all_msg);
    assert_eq!(
        state.operator_approvals,
        vec![OperatorApproval {
            operator: mock_address(bob),
            owner: mock_address(alice)
        }]
    );

    let approve_all_msg = NFTApproveForAllMsg {
        operator: mock_address(alice),
        approved: true,
    };
    let _ = execute_set_approval_for_all(&mock_contract_context(bob), &mut state, &approve_all_msg);
    assert_eq!(
        state.operator_approvals,
        vec![
            OperatorApproval {
                operator: mock_address(bob),
                owner: mock_address(alice)
            },
            OperatorApproval {
                operator: mock_address(alice),
                owner: mock_address(bob)
            },
        ]
    );
}

#[test]
fn proper_token_operator_approve() {
    let minter = 1u8;
    let alice = 10u8;
    let bob = 11u8;
    let jack = 12u8;

    let msg = NFTInitMsg {
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        uri_template: "ipfs://some.some".to_string(),
    };

    let mut state = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = NFTMintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);
    assert_eq!(state.owners, SortedVecMap::from([(1, mock_address(alice))]),);

    let approve_msg = NFTApproveMsg {
        approved: Some(mock_address(jack)),
        token_id: 1,
    };

    let _ = execute_approve(&mock_contract_context(alice), &mut state, &approve_msg);
    assert_eq!(
        state.token_approvals,
        SortedVecMap::from([(1, mock_address(jack))]),
    );
}

#[test]
#[should_panic(expected = "Not found")]
fn approve_not_minted_token() {
    let minter = 1u8;
    let alice = 10u8;
    let bob = 11u8;
    let jack = 12u8;

    let msg = NFTInitMsg {
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        uri_template: "ipfs://some.some".to_string(),
    };

    let mut state = execute_init(&mock_contract_context(2), &msg);

    let approve_msg = NFTApproveMsg {
        approved: Some(mock_address(jack)),
        token_id: 1,
    };

    let _ = execute_approve(&mock_contract_context(bob), &mut state, &approve_msg);
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn not_owner_or_operator_approve() {
    let minter = 1u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = NFTInitMsg {
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        uri_template: "ipfs://some.some".to_string(),
    };

    let mut state = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = NFTMintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let approve_msg = NFTApproveMsg {
        approved: Some(mock_address(bob)),
        token_id: 1,
    };

    let _ = execute_approve(&mock_contract_context(bob), &mut state, &approve_msg);
}

#[test]
fn proper_owner_transfer_from() {
    let minter = 1u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = NFTInitMsg {
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        uri_template: "ipfs://some.some".to_string(),
    };

    let mut state = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = NFTMintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let transfer_msg = NFTTransferFromMsg {
        from: mock_address(alice),
        to: mock_address(bob),
        token_id: 1,
    };

    let _ = execute_transfer_from(&mock_contract_context(alice), &mut state, &transfer_msg);
    assert_eq!(state.owners, SortedVecMap::from([(1, mock_address(bob))]),);
}

#[test]
fn proper_approved_transfer_from() {
    let minter = 1u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = NFTInitMsg {
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        uri_template: "ipfs://some.some".to_string(),
    };

    let mut state = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = NFTMintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let approve_msg = NFTApproveMsg {
        approved: Some(mock_address(bob)),
        token_id: 1,
    };

    let _ = execute_approve(&mock_contract_context(alice), &mut state, &approve_msg);

    let transfer_msg = NFTTransferFromMsg {
        from: mock_address(alice),
        to: mock_address(bob),
        token_id: 1,
    };

    let _ = execute_transfer_from(&mock_contract_context(bob), &mut state, &transfer_msg);
    assert_eq!(state.owners, SortedVecMap::from([(1, mock_address(bob))]),);
    assert_eq!(state.token_approvals, SortedVecMap::new(),);
}

#[test]
fn proper_operator_transfer_from() {
    let minter = 1u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = NFTInitMsg {
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        uri_template: "ipfs://some.some".to_string(),
    };

    let mut state = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = NFTMintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let approve_all_msg = NFTApproveForAllMsg {
        operator: mock_address(bob),
        approved: true,
    };
    let _ =
        execute_set_approval_for_all(&mock_contract_context(alice), &mut state, &approve_all_msg);

    let transfer_msg = NFTTransferFromMsg {
        from: mock_address(alice),
        to: mock_address(bob),
        token_id: 1,
    };

    let _ = execute_transfer_from(&mock_contract_context(bob), &mut state, &transfer_msg);
    assert_eq!(state.owners, SortedVecMap::from([(1, mock_address(bob))]),);
    assert_eq!(state.token_approvals, SortedVecMap::new(),);
    assert_eq!(
        state.operator_approvals,
        vec![OperatorApproval {
            owner: mock_address(alice),
            operator: mock_address(bob)
        }]
    );
}

#[test]
#[should_panic(expected = "Not found")]
fn transfer_from_not_minted_token() {
    let minter = 1u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = NFTInitMsg {
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        uri_template: "ipfs://some.some".to_string(),
    };

    let mut state = execute_init(&mock_contract_context(2), &msg);

    let transfer_msg = NFTTransferFromMsg {
        from: mock_address(alice),
        to: mock_address(bob),
        token_id: 1,
    };

    let _ = execute_transfer_from(&mock_contract_context(bob), &mut state, &transfer_msg);
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn transfer_from_not_owner_or_approved_token() {
    let minter = 1u8;
    let alice = 10u8;
    let bob = 11u8;
    let jack = 12u8;

    let msg = NFTInitMsg {
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        uri_template: "ipfs://some.some".to_string(),
    };

    let mut state = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = NFTMintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let transfer_msg = NFTTransferFromMsg {
        from: mock_address(alice),
        to: mock_address(jack),
        token_id: 1,
    };

    let _ = execute_transfer_from(&mock_contract_context(jack), &mut state, &transfer_msg);
}

#[test]
fn proper_burn() {
    let minter = 1u8;
    let alice = 10u8;

    let msg = NFTInitMsg {
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        uri_template: "ipfs://some.some".to_string(),
    };

    let mut state = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = NFTMintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let burn_msg = NFTBurnMsg { token_id: 1 };

    let _ = execute_burn(&mock_contract_context(alice), &mut state, &burn_msg);
    assert_eq!(state.supply, 0);
    assert!(!state.exists(1));
}

#[test]
#[should_panic(expected = "Not found")]
fn burn_not_minted_token() {
    let minter = 1u8;
    let alice = 10u8;

    let msg = NFTInitMsg {
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        uri_template: "ipfs://some.some".to_string(),
    };

    let mut state = execute_init(&mock_contract_context(2), &msg);

    let burn_msg = NFTBurnMsg { token_id: 1 };
    let _ = execute_burn(&mock_contract_context(alice), &mut state, &burn_msg);
}
