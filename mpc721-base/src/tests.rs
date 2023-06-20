use pbc_contract_common::sorted_vec_map::SortedVecMap;

use utils::tests::{mock_address, mock_contract_context};

use crate::{
    actions::{
        execute_approve, execute_approve_for_all, execute_burn, execute_init, execute_mint,
        execute_multi_mint, execute_ownership_check, execute_revoke, execute_revoke_for_all,
        execute_set_base_uri, execute_transfer, execute_transfer_from, execute_update_minter,
        execute_update_parent,
    },
    msg::{
        ApproveForAllMsg, ApproveMsg, BurnMsg, CheckOwnerMsg, NFTInitMsg, MintMsg, MultiMintMsg,
        RevokeForAllMsg, RevokeMsg, SetBaseUriMsg, TransferFromMsg, TransferMsg, UpdateMinterMsg,
        UpdateParentMsg,
    },
    state::{MPC721ContractState, TokenInfo},
};

#[test]
fn proper_execute_init() {
    let msg = NFTInitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(1),
    };

    let (state, events) = execute_init(&mock_contract_context(2), &msg);
    assert_eq!(events.len(), 0);
    assert_eq!(
        state,
        MPC721ContractState {
            owner: None,
            name: "Cool Token".to_string(),
            symbol: "CTC".to_string(),
            base_uri: Some("ipfs://some.some".to_string()),
            minter: mock_address(1),
            supply: 0,
            tokens: SortedVecMap::new(),
            operator_approvals: SortedVecMap::new(),
        }
    );
}

#[test]
fn proper_set_base_uri() {
    let owner = 1u8;

    let msg = NFTInitMsg {
        owner: Some(mock_address(owner)),
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(1),
    };

    let (mut state, _) = execute_init(&mock_contract_context(2), &msg);

    let set_base_uri_msg = SetBaseUriMsg {
        new_base_uri: "ipfs://new.new".to_string(),
    };

    let _ = execute_set_base_uri(&mock_contract_context(owner), &mut state, &set_base_uri_msg);
    assert_eq!(state.base_uri, Some("ipfs://new.new".to_string()));
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn owner_is_not_set_on_set_base_uri() {
    let owner = 1u8;

    let msg = NFTInitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(1),
    };

    let (mut state, _) = execute_init(&mock_contract_context(2), &msg);

    let set_base_uri_msg = SetBaseUriMsg {
        new_base_uri: "ipfs://new.new".to_string(),
    };

    let _ = execute_set_base_uri(&mock_contract_context(owner), &mut state, &set_base_uri_msg);
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn sender_is_not_owner_on_set_base_uri() {
    let owner = 1u8;
    let alice = 10u8;

    let msg = NFTInitMsg {
        owner: Some(mock_address(owner)),
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(1),
    };

    let (mut state, _) = execute_init(&mock_contract_context(2), &msg);

    let set_base_uri_msg = SetBaseUriMsg {
        new_base_uri: "ipfs://new.new".to_string(),
    };

    let _ = execute_set_base_uri(&mock_contract_context(alice), &mut state, &set_base_uri_msg);
}

#[test]
fn proper_mint() {
    let minter = 1u8;
    let alice = 10u8;

    let msg = NFTInitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = MintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);
    assert_eq!(state.supply, 1);

    let token = state.token_info(1).unwrap();
    assert_eq!(
        *token,
        TokenInfo {
            owner: mock_address(alice),
            approvals: vec![],
            token_uri: None,
            parent_id: None,
        }
    );
}

#[test]
fn proper_ownership_check() {
    let minter = 1u8;
    let alice = 10u8;

    let msg = NFTInitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(1),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);
    let mint_msg = MintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);
    assert_eq!(state.supply, 1);
    let ownership_msg: CheckOwnerMsg = CheckOwnerMsg {
        owner: mock_address(alice),
        token_id: 1,
    };
    let _ = execute_ownership_check(&mock_contract_context(2), &mut state, &ownership_msg);
}
#[test]
#[should_panic(expected = "Incorrect Owner")]
fn proper_ownership_check_fail() {
    let minter = 1u8;
    let alice = 10u8;
    let bob = 11u8;
    let msg = NFTInitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(1),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);
    let mint_msg = MintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);
    assert_eq!(state.supply, 1);

    let ownership_msg: CheckOwnerMsg = CheckOwnerMsg {
        owner: mock_address(bob),
        token_id: 1,
    };
    let _ = execute_ownership_check(&mock_contract_context(2), &mut state, &ownership_msg);
}
#[test]
#[should_panic(expected = "Not found")]
fn proper_ownership_check_fail_not_found() {
    let minter = 1u8;
    let alice = 10u8;
    let bob = 11u8;
    let msg = NFTInitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(1),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);
    let mint_msg = MintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);
    assert_eq!(state.supply, 1);

    let ownership_msg: CheckOwnerMsg = CheckOwnerMsg {
        owner: mock_address(bob),
        token_id: 3,
    };
    let _ = execute_ownership_check(&mock_contract_context(2), &mut state, &ownership_msg);
}
#[test]
#[should_panic(expected = "Unauthorized")]
fn sender_is_not_minter_on_mint() {
    let minter = 1u8;
    let alice = 10u8;

    let msg = NFTInitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = MintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(alice), &mut state, &mint_msg);
}

#[test]
#[should_panic(expected = "Token with specified id is already minted")]
fn token_already_minted_on_mint() {
    let minter = 1u8;
    let alice = 10u8;

    let msg = NFTInitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = MintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let mint_msg = MintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);
}

#[test]
fn proper_approve_for_all() {
    let owner = 1u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = NFTInitMsg {
        owner: Some(mock_address(owner)),
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(1),
    };

    let (mut state, _) = execute_init(&mock_contract_context(2), &msg);

    let approve_all_msg = ApproveForAllMsg {
        operator: mock_address(bob),
    };
    let _ = execute_approve_for_all(&mock_contract_context(alice), &mut state, &approve_all_msg);
    assert_eq!(
        state.operator_approvals,
        SortedVecMap::from([(mock_address(alice), vec![mock_address(bob)])])
    );

    let approve_all_msg = ApproveForAllMsg {
        operator: mock_address(alice),
    };
    let _ = execute_approve_for_all(&mock_contract_context(bob), &mut state, &approve_all_msg);
    assert_eq!(
        state.operator_approvals,
        SortedVecMap::from([
            (mock_address(alice), vec![mock_address(bob)]),
            (mock_address(bob), vec![mock_address(alice)])
        ])
    );
}

#[test]
fn proper_revoke_for_all() {
    let owner = 1u8;
    let alice = 10u8;
    let bob = 11u8;
    let jack = 12u8;

    let msg = NFTInitMsg {
        owner: Some(mock_address(owner)),
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(1),
    };

    let (mut state, _) = execute_init(&mock_contract_context(2), &msg);

    let approve_all_msg = ApproveForAllMsg {
        operator: mock_address(bob),
    };
    let _ = execute_approve_for_all(&mock_contract_context(alice), &mut state, &approve_all_msg);
    let approve_all_msg = ApproveForAllMsg {
        operator: mock_address(jack),
    };
    let _ = execute_approve_for_all(&mock_contract_context(alice), &mut state, &approve_all_msg);

    let revoke_all_msg = RevokeForAllMsg {
        operator: mock_address(bob),
    };
    let _ = execute_revoke_for_all(&mock_contract_context(alice), &mut state, &revoke_all_msg);
    assert_eq!(
        state.operator_approvals,
        SortedVecMap::from([(mock_address(alice), vec![mock_address(jack)])])
    );

    let revoke_all_msg = RevokeForAllMsg {
        operator: mock_address(jack),
    };
    let _ = execute_revoke_for_all(&mock_contract_context(alice), &mut state, &revoke_all_msg);
    assert_eq!(state.operator_approvals, SortedVecMap::new());
}

#[test]
#[should_panic(expected = "Not found")]
fn revoke_not_existing_operator() {
    let owner = 1u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = NFTInitMsg {
        owner: Some(mock_address(owner)),
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(1),
    };

    let (mut state, _) = execute_init(&mock_contract_context(2), &msg);

    let revoke_all_msg = RevokeForAllMsg {
        operator: mock_address(bob),
    };
    let _ = execute_revoke_for_all(&mock_contract_context(alice), &mut state, &revoke_all_msg);
}

#[test]
fn proper_token_owner_approve() {
    let minter = 1u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = NFTInitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = MintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let approve_msg = ApproveMsg {
        spender: mock_address(bob),
        token_id: 1,
    };

    let _ = execute_approve(&mock_contract_context(alice), &mut state, &approve_msg);
    assert_eq!(
        *state.token_info(1).unwrap(),
        TokenInfo {
            owner: mock_address(alice),
            approvals: vec![mock_address(bob)],
            token_uri: None,
            parent_id: None,
        }
    );
}

#[test]
fn proper_token_operator_approve() {
    let minter = 1u8;
    let alice = 10u8;
    let bob = 11u8;
    let jack = 12u8;

    let msg = NFTInitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = MintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let approve_all_msg = ApproveForAllMsg {
        operator: mock_address(bob),
    };
    let _ = execute_approve_for_all(&mock_contract_context(alice), &mut state, &approve_all_msg);

    let approve_msg = ApproveMsg {
        spender: mock_address(jack),
        token_id: 1,
    };

    let _ = execute_approve(&mock_contract_context(bob), &mut state, &approve_msg);
    assert_eq!(
        *state.token_info(1).unwrap(),
        TokenInfo {
            owner: mock_address(alice),
            approvals: vec![mock_address(jack)],
            token_uri: None,
            parent_id: None,
        }
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
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let approve_msg = ApproveMsg {
        spender: mock_address(jack),
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
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = MintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let approve_msg = ApproveMsg {
        spender: mock_address(bob),
        token_id: 1,
    };

    let _ = execute_approve(&mock_contract_context(bob), &mut state, &approve_msg);
}

#[test]
fn proper_revoke() {
    let minter = 1u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = NFTInitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = MintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let approve_msg = ApproveMsg {
        spender: mock_address(bob),
        token_id: 1,
    };

    let _ = execute_approve(&mock_contract_context(alice), &mut state, &approve_msg);

    let revoke_msg = RevokeMsg {
        spender: mock_address(bob),
        token_id: 1,
    };

    let _ = execute_revoke(&mock_contract_context(alice), &mut state, &revoke_msg);
    assert_eq!(
        *state.token_info(1).unwrap(),
        TokenInfo {
            owner: mock_address(alice),
            approvals: vec![],
            token_uri: None,
            parent_id: None,
        }
    );
}

#[test]
#[should_panic(expected = "Not found")]
fn revoke_not_minted_token() {
    let minter = 1u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = NFTInitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let revoke_msg = RevokeMsg {
        spender: mock_address(bob),
        token_id: 1,
    };

    let _ = execute_revoke(&mock_contract_context(alice), &mut state, &revoke_msg);
}

#[test]
fn proper_owner_transfer() {
    let minter = 1u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = NFTInitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = MintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let transfer_msg = TransferMsg {
        to: mock_address(bob),
        token_id: 1,
    };

    let _ = execute_transfer(&mock_contract_context(alice), &mut state, &transfer_msg);
    assert_eq!(
        *state.token_info(1).unwrap(),
        TokenInfo {
            owner: mock_address(bob),
            approvals: vec![],
            token_uri: None,
            parent_id: None,
        }
    );
}

#[test]
fn proper_approved_transfer() {
    let minter = 1u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = NFTInitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = MintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let approve_msg = ApproveMsg {
        spender: mock_address(bob),
        token_id: 1,
    };

    let _ = execute_approve(&mock_contract_context(alice), &mut state, &approve_msg);

    let transfer_msg = TransferMsg {
        to: mock_address(bob),
        token_id: 1,
    };

    let _ = execute_transfer(&mock_contract_context(bob), &mut state, &transfer_msg);
    assert_eq!(
        *state.token_info(1).unwrap(),
        TokenInfo {
            owner: mock_address(bob),
            approvals: vec![],
            token_uri: None,
            parent_id: None,
        }
    );
}

#[test]
fn proper_operator_transfer() {
    let minter = 1u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = NFTInitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = MintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let approve_all_msg = ApproveForAllMsg {
        operator: mock_address(bob),
    };
    let _ = execute_approve_for_all(&mock_contract_context(alice), &mut state, &approve_all_msg);

    let transfer_msg = TransferMsg {
        to: mock_address(bob),
        token_id: 1,
    };

    let _ = execute_transfer(&mock_contract_context(bob), &mut state, &transfer_msg);
    assert_eq!(
        *state.token_info(1).unwrap(),
        TokenInfo {
            owner: mock_address(bob),
            approvals: vec![],
            token_uri: None,
            parent_id: None,
        }
    );
}

#[test]
#[should_panic(expected = "Not found")]
fn transfer_not_minted_token() {
    let minter = 1u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = NFTInitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let transfer_msg = TransferMsg {
        to: mock_address(bob),
        token_id: 1,
    };

    let _ = execute_transfer(&mock_contract_context(bob), &mut state, &transfer_msg);
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn transfer_not_owner_or_approved_token() {
    let minter = 1u8;
    let alice = 10u8;
    let bob = 11u8;
    let jack = 12u8;

    let msg = NFTInitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = MintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let transfer_msg = TransferMsg {
        to: mock_address(jack),
        token_id: 1,
    };

    let _ = execute_transfer(&mock_contract_context(jack), &mut state, &transfer_msg);
}

#[test]
fn proper_transfer_from() {
    let minter = 1u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = NFTInitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = MintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let transfer_msg = TransferFromMsg {
        from: mock_address(alice),
        to: mock_address(bob),
        token_id: 1,
    };

    let _ = execute_transfer_from(&mock_contract_context(alice), &mut state, &transfer_msg);
    assert_eq!(
        *state.token_info(1).unwrap(),
        TokenInfo {
            owner: mock_address(bob),
            approvals: vec![],
            token_uri: None,
            parent_id: None,
        }
    );
}

#[test]
#[should_panic(expected = "Not found")]
fn transfer_from_not_minted_token() {
    let minter = 1u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = NFTInitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let transfer_msg = TransferFromMsg {
        from: mock_address(alice),
        to: mock_address(bob),
        token_id: 1,
    };

    let _ = execute_transfer_from(&mock_contract_context(alice), &mut state, &transfer_msg);
}

#[test]
fn proper_burn() {
    let minter = 1u8;
    let alice = 10u8;

    let msg = NFTInitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = MintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let burn_msg = BurnMsg { token_id: 1 };

    let _ = execute_burn(&mock_contract_context(alice), &mut state, &burn_msg);
    assert_eq!(state.supply, 0);
    assert!(!state.is_minted(1));
}

#[test]
#[should_panic(expected = "Not found")]
fn burn_not_minted_token() {
    let minter = 1u8;
    let alice = 10u8;

    let msg = NFTInitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let burn_msg = BurnMsg { token_id: 1 };
    let _ = execute_burn(&mock_contract_context(alice), &mut state, &burn_msg);
}
#[test]
fn test_multi_mint() {
    let msg = NFTInitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(1),
    };

    let (state, events) = execute_init(&mock_contract_context(2), &msg);
    let mut test_state = MPC721ContractState {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(1),
        supply: 0,
        tokens: SortedVecMap::new(),
        operator_approvals: SortedVecMap::new(),
    };
    test_state.tokens.insert(
        1,
        TokenInfo {
            /// token owner
            owner: mock_address(4),
            /// token approvals
            approvals: vec![],
            /// optional token uri
            token_uri: Some(String::from("Token1")),
            parent_id: None,
        },
    );
    test_state.tokens.insert(
        2,
        TokenInfo {
            /// token owner
            owner: mock_address(4),
            /// token approvals
            approvals: vec![],
            /// optional token uri
            token_uri: Some(String::from("Token2")),
            parent_id: None,
        },
    );
    test_state.tokens.insert(
        3,
        TokenInfo {
            /// token owner
            owner: mock_address(5),
            /// token approvals
            approvals: vec![],
            /// optional token uri
            token_uri: Some(String::from("Token3")),
            parent_id: None,
        },
    );
    test_state.tokens.insert(
        4,
        TokenInfo {
            /// token owner
            owner: mock_address(5),
            /// token approvals
            approvals: vec![],
            /// optional token uri
            token_uri: Some(String::from("Token4")),
            parent_id: None,
        },
    );
    test_state.tokens.insert(
        5,
        TokenInfo {
            /// token owner
            owner: mock_address(6),
            /// token approvals
            approvals: vec![],
            /// optional token uri
            token_uri: Some(String::from("Token5")),
            parent_id: None,
        },
    );
    test_state.supply = 5;
    let mut state = state;
    let mint = vec![
        MintMsg {
            token_id: 1,
            to: mock_address(4),
            token_uri: Some(String::from("Token1")),
        },
        MintMsg {
            token_id: 2,
            to: mock_address(4),
            token_uri: Some(String::from("Token2")),
        },
        MintMsg {
            token_id: 3,
            to: mock_address(5),
            token_uri: Some(String::from("Token3")),
        },
        MintMsg {
            token_id: 4,
            to: mock_address(5),
            token_uri: Some(String::from("Token4")),
        },
        MintMsg {
            token_id: 5,
            to: mock_address(6),
            token_uri: Some(String::from("Token5")),
        },
    ];
    execute_multi_mint(
        &mock_contract_context(1),
        &mut state,
        &MultiMintMsg { mints: mint },
    );

    assert_eq!(state, test_state);
}
#[test]
fn can_update_minter() {
    let minter = 1u8;
    let new_minter = 6u8;
    let alice = 10u8;

    let msg = NFTInitMsg {
        owner: Some(mock_address(alice)),
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let _ = execute_update_minter(
        &mock_contract_context(alice),
        &mut state,
        UpdateMinterMsg {
            new_minter: mock_address(new_minter),
        },
    );
    assert_eq!(mock_address(new_minter), state.minter);
}
#[test]
#[should_panic(expected = "Unauthorized")]
fn update_minter_fails_not_owner() {
    let minter = 1u8;
    let new_minter = 6u8;
    let alice = 10u8;

    let msg = NFTInitMsg {
        owner: Some(mock_address(alice)),
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let _ = execute_update_minter(
        &mock_contract_context(minter),
        &mut state,
        UpdateMinterMsg {
            new_minter: mock_address(new_minter),
        },
    );
}
#[test]
#[should_panic(expected = "Unauthorized")]
fn update_minter_fails_no_owner() {
    let minter = 1u8;
    let new_minter = 6u8;
    let alice = 10u8;

    let msg = NFTInitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let _ = execute_update_minter(
        &mock_contract_context(minter),
        &mut state,
        UpdateMinterMsg {
            new_minter: mock_address(new_minter),
        },
    );
}

#[test]
fn proper_update_parent() {
    let minter = 1u8;
    let alice = 10u8;

    let msg = NFTInitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = MintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };
    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let mint2_msg = MintMsg {
        token_id: 2,
        to: mock_address(alice),
        token_uri: None,
    };
    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint2_msg);

    assert_eq!(state.supply, 2);

    let update_parent_msg = UpdateParentMsg {
        token_id: 1,
        parent_id: Some(2),
    };

    let _ = execute_update_parent(
        &mock_contract_context(minter),
        &mut state,
        &update_parent_msg,
    );

    let token = state.token_info(1).unwrap();
    assert_eq!(
        *token,
        TokenInfo {
            owner: mock_address(alice),
            approvals: vec![],
            token_uri: None,
            parent_id: Some(2)
        }
    );
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn proper_update_parent_fail_check_minter() {
    let minter = 1u8;
    let alice = 10u8;

    let msg = NFTInitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = MintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);
    assert_eq!(state.supply, 1);

    let update_parent_msg = UpdateParentMsg {
        token_id: 1,
        parent_id: None,
    };

    let _ = execute_update_parent(
        &mock_contract_context(alice),
        &mut state,
        &update_parent_msg,
    );
}

#[test]
#[should_panic(expected = "Not found")]
fn proper_update_parent_fail_not_found() {
    let minter = 1u8;
    let alice = 10u8;

    let msg = NFTInitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = MintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };
    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);
    assert_eq!(state.supply, 1);

    let update_parent_msg = UpdateParentMsg {
        token_id: 2,
        parent_id: None,
    };

    let _ = execute_update_parent(
        &mock_contract_context(minter),
        &mut state,
        &update_parent_msg,
    );
}

#[test]
#[should_panic(expected = "Not found")]
fn proper_update_parent_fail_parent_not_found() {
    let minter = 1u8;
    let alice = 10u8;

    let msg = NFTInitMsg {
        owner: None,
        name: "Cool Token".to_string(),
        symbol: "CTC".to_string(),
        base_uri: Some("ipfs://some.some".to_string()),
        minter: mock_address(minter),
    };

    let (mut state, events) = execute_init(&mock_contract_context(2), &msg);

    let mint_msg = MintMsg {
        token_id: 1,
        to: mock_address(alice),
        token_uri: None,
    };
    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);
    assert_eq!(state.supply, 1);

    let update_parent_msg = UpdateParentMsg {
        token_id: 1,
        parent_id: Some(2),
    };

    let _ = execute_update_parent(
        &mock_contract_context(minter),
        &mut state,
        &update_parent_msg,
    );
}
