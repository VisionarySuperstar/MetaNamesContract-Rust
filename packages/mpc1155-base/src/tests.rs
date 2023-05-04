use std::collections::BTreeMap;

use pbc_contract_common::{
    address::{Address, AddressType},
    context::ContractContext,
};

use crate::{
    actions::{
        execute_approve_for_all, execute_batch_burn, execute_batch_mint,
        execute_batch_transfer_from, execute_burn, execute_check_balances, execute_init,
        execute_mint, execute_revoke_for_all, execute_set_uri, execute_transfer_from,
    },
    msg::{
        ApproveForAllMsg, BatchBurnMsg, BatchMintMsg, BatchTransferFromMsg, BurnMsg,
        CheckBalancesMsg, InitMsg, MintMsg, RevokeForAllMsg, SetUriMsg, TokenMintInfoMsg,
        TokenTransferInfoMsg, TransferFromMsg,
    },
    state::{MPC1155ContractState, TokenInfo},
};

fn mock_address(le: u8) -> Address {
    Address {
        address_type: AddressType::Account,
        identifier: [
            le, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8,
        ],
    }
}

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
fn proper_execute_init() {
    let msg = InitMsg {
        owner: Some(mock_address(1)),
        uri: "ipfs://random".to_string(),
        minter: mock_address(3),
    };

    let (state, events) = execute_init(&mock_contract_context(2), &msg);
    assert_eq!(events.len(), 0);
    assert_eq!(
        state,
        MPC1155ContractState {
            owner: Some(mock_address(1)),
            uri: "ipfs://random".to_string(),
            minter: mock_address(3),
            balances: BTreeMap::new(),
            operator_approvals: BTreeMap::new(),
            tokens: BTreeMap::new(),
        }
    );
}

#[test]
fn proper_set_uri() {
    let owner = 1u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        uri: "ipfs://random".to_string(),
        minter: mock_address(3),
    };

    let (mut state, _) = execute_init(&mock_contract_context(2), &msg);

    let set_base_uri_msg = SetUriMsg {
        new_uri: "ipfs://new.new".to_string(),
    };

    let _ = execute_set_uri(&mock_contract_context(owner), &mut state, &set_base_uri_msg);
    assert_eq!(state.uri, "ipfs://new.new".to_string());
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn owner_is_not_set_on_set_base_uri() {
    let owner = 1u8;

    let msg = InitMsg {
        owner: None,
        uri: "ipfs://random".to_string(),
        minter: mock_address(3),
    };

    let (mut state, _) = execute_init(&mock_contract_context(2), &msg);

    let set_base_uri_msg = SetUriMsg {
        new_uri: "ipfs://new.new".to_string(),
    };

    let _ = execute_set_uri(&mock_contract_context(owner), &mut state, &set_base_uri_msg);
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn sender_is_not_owner_on_set_base_uri() {
    let owner = 1u8;
    let alice = 10u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        uri: "ipfs://random".to_string(),
        minter: mock_address(3),
    };

    let (mut state, _) = execute_init(&mock_contract_context(2), &msg);

    let set_base_uri_msg = SetUriMsg {
        new_uri: "ipfs://new.new".to_string(),
    };

    let _ = execute_set_uri(&mock_contract_context(alice), &mut state, &set_base_uri_msg);
}

#[test]
fn proper_mint() {
    let owner = 1u8;
    let minter = 2u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        uri: "ipfs://random".to_string(),
        minter: mock_address(minter),
    };

    let (mut state, _) = execute_init(&mock_contract_context(owner), &msg);

    let mint_msg = MintMsg {
        to: mock_address(alice),
        token_info: TokenMintInfoMsg {
            token_id: 1,
            amount: 10,
            token_uri: Some("1.json".to_string()),
        },
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);
    assert_eq!(
        state.tokens,
        BTreeMap::from([(
            1,
            TokenInfo {
                token_uri: Some("1.json".to_string()),
            }
        )])
    );
    assert_eq!(
        state.balances,
        BTreeMap::from([(1, BTreeMap::from([(mock_address(alice), 10)]))])
    );

    let mut state = state;
    for msg in vec![
        MintMsg {
            to: mock_address(alice),
            token_info: TokenMintInfoMsg {
                token_id: 2,
                amount: 20,
                token_uri: Some("2.json".to_string()),
            },
        },
        MintMsg {
            to: mock_address(alice),
            token_info: TokenMintInfoMsg {
                token_id: 1,
                amount: 50,
                token_uri: None,
            },
        },
        MintMsg {
            to: mock_address(bob),
            token_info: TokenMintInfoMsg {
                token_id: 1,
                amount: 1,
                token_uri: None,
            },
        },
    ]
    .into_iter()
    {
        let _ = execute_mint(&mock_contract_context(minter), &mut state, &msg);
    }

    assert_eq!(
        state.tokens,
        BTreeMap::from([
            (
                1,
                TokenInfo {
                    token_uri: Some("1.json".to_string()),
                }
            ),
            (
                2,
                TokenInfo {
                    token_uri: Some("2.json".to_string()),
                }
            )
        ])
    );
    assert_eq!(
        state.balances,
        BTreeMap::from([
            (
                1,
                BTreeMap::from([(mock_address(alice), 60), (mock_address(bob), 1)])
            ),
            (2, BTreeMap::from([(mock_address(alice), 20)]))
        ])
    );
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn sender_is_not_minter_on_mint() {
    let owner = 1u8;
    let minter = 2u8;
    let alice = 10u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        uri: "ipfs://random".to_string(),
        minter: mock_address(minter),
    };

    let (mut state, _) = execute_init(&mock_contract_context(owner), &msg);

    let mint_msg = MintMsg {
        to: mock_address(alice),
        token_info: TokenMintInfoMsg {
            token_id: 1,
            amount: 10,
            token_uri: Some("1.json".to_string()),
        },
    };

    let _ = execute_mint(&mock_contract_context(alice), &mut state, &mint_msg);
}

#[test]
fn proper_batch_mint() {
    let owner = 1u8;
    let minter = 2u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        uri: "ipfs://random".to_string(),
        minter: mock_address(minter),
    };

    let (mut state, _) = execute_init(&mock_contract_context(owner), &msg);

    for msg in vec![
        BatchMintMsg {
            to: mock_address(alice),
            token_infos: vec![
                TokenMintInfoMsg {
                    token_id: 1,
                    amount: 10,
                    token_uri: None,
                },
                TokenMintInfoMsg {
                    token_id: 2,
                    amount: 20,
                    token_uri: Some("2.json".to_string()),
                },
            ],
        },
        BatchMintMsg {
            to: mock_address(bob),
            token_infos: vec![
                TokenMintInfoMsg {
                    token_id: 1,
                    amount: 100,
                    token_uri: None,
                },
                TokenMintInfoMsg {
                    token_id: 3,
                    amount: 30,
                    token_uri: Some("3.json".to_string()),
                },
            ],
        },
    ]
    .into_iter()
    {
        let _ = execute_batch_mint(&mock_contract_context(minter), &mut state, &msg);
    }

    assert_eq!(
        state.tokens,
        BTreeMap::from([
            (1, TokenInfo { token_uri: None }),
            (
                2,
                TokenInfo {
                    token_uri: Some("2.json".to_string()),
                }
            ),
            (
                3,
                TokenInfo {
                    token_uri: Some("3.json".to_string()),
                }
            )
        ])
    );
    assert_eq!(
        state.balances,
        BTreeMap::from([
            (
                1,
                BTreeMap::from([(mock_address(alice), 10), (mock_address(bob), 100)])
            ),
            (2, BTreeMap::from([(mock_address(alice), 20)])),
            (3, BTreeMap::from([(mock_address(bob), 30)]))
        ])
    );
}

#[test]
fn check_balances() {
    let owner = 1u8;
    let minter = 2u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        uri: "ipfs://random".to_string(),
        minter: mock_address(minter),
    };

    let (mut state, _) = execute_init(&mock_contract_context(owner), &msg);

    for msg in vec![
        BatchMintMsg {
            to: mock_address(alice),
            token_infos: vec![
                TokenMintInfoMsg {
                    token_id: 1,
                    amount: 10,
                    token_uri: None,
                },
                TokenMintInfoMsg {
                    token_id: 2,
                    amount: 20,
                    token_uri: Some("2.json".to_string()),
                },
            ],
        },
        BatchMintMsg {
            to: mock_address(bob),
            token_infos: vec![
                TokenMintInfoMsg {
                    token_id: 1,
                    amount: 100,
                    token_uri: None,
                },
                TokenMintInfoMsg {
                    token_id: 3,
                    amount: 30,
                    token_uri: Some("3.json".to_string()),
                },
            ],
        },
    ]
    .into_iter()
    {
        let _ = execute_batch_mint(&mock_contract_context(minter), &mut state, &msg);
    }
    let tokens = vec![1, 2];
    let amounts = vec![10, 20];
    let msg = CheckBalancesMsg {
        owner: mock_address(alice),
        token_ids: tokens,
        amounts: amounts,
    };
    execute_check_balances(&mock_contract_context(minter), &mut state, msg);
    let msg2 = CheckBalancesMsg {
        owner: mock_address(bob),
        token_ids: vec![1, 3],
        amounts: vec![100, 30],
    };
    execute_check_balances(&mock_contract_context(minter), &mut state, msg2);
}
#[test]
#[should_panic(expected = "Balance check has failed")]
fn check_balances_fail() {
    let owner = 1u8;
    let minter = 2u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        uri: "ipfs://random".to_string(),
        minter: mock_address(minter),
    };

    let (mut state, _) = execute_init(&mock_contract_context(owner), &msg);

    for msg in vec![
        BatchMintMsg {
            to: mock_address(alice),
            token_infos: vec![
                TokenMintInfoMsg {
                    token_id: 1,
                    amount: 10,
                    token_uri: None,
                },
                TokenMintInfoMsg {
                    token_id: 2,
                    amount: 20,
                    token_uri: Some("2.json".to_string()),
                },
            ],
        },
        BatchMintMsg {
            to: mock_address(bob),
            token_infos: vec![
                TokenMintInfoMsg {
                    token_id: 1,
                    amount: 100,
                    token_uri: None,
                },
                TokenMintInfoMsg {
                    token_id: 3,
                    amount: 30,
                    token_uri: Some("3.json".to_string()),
                },
            ],
        },
    ]
    .into_iter()
    {
        let _ = execute_batch_mint(&mock_contract_context(minter), &mut state, &msg);
    }
    let tokens = vec![1, 2];
    let amounts = vec![11, 21];
    let msg = CheckBalancesMsg {
        owner: mock_address(alice),
        token_ids: tokens,
        amounts: amounts,
    };
    execute_check_balances(&mock_contract_context(minter), &mut state, msg);
}
#[test]
#[should_panic(expected = "Unauthorized")]
fn sender_is_not_minter_on_batch_mint() {
    let owner = 1u8;
    let minter = 2u8;
    let alice = 10u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        uri: "ipfs://random".to_string(),
        minter: mock_address(minter),
    };

    let (mut state, _) = execute_init(&mock_contract_context(owner), &msg);

    let batch_mint_msg = BatchMintMsg {
        to: mock_address(alice),
        token_infos: vec![
            TokenMintInfoMsg {
                token_id: 1,
                amount: 100,
                token_uri: None,
            },
            TokenMintInfoMsg {
                token_id: 3,
                amount: 30,
                token_uri: Some("3.json".to_string()),
            },
        ],
    };

    let _ = execute_batch_mint(&mock_contract_context(alice), &mut state, &batch_mint_msg);
}

#[test]
fn proper_approve_for_all() {
    let owner = 1u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        uri: "ipfs://random".to_string(),
        minter: mock_address(1),
    };

    let (mut state, _) = execute_init(&mock_contract_context(2), &msg);

    let approve_all_msg = ApproveForAllMsg {
        operator: mock_address(bob),
    };
    let _ = execute_approve_for_all(&mock_contract_context(alice), &mut state, &approve_all_msg);
    assert_eq!(
        state.operator_approvals,
        BTreeMap::from([(
            mock_address(alice),
            BTreeMap::from([(mock_address(bob), true)])
        )])
    );

    let approve_all_msg = ApproveForAllMsg {
        operator: mock_address(alice),
    };
    let _ = execute_approve_for_all(&mock_contract_context(bob), &mut state, &approve_all_msg);
    assert_eq!(
        state.operator_approvals,
        BTreeMap::from([
            (
                mock_address(alice),
                BTreeMap::from([(mock_address(bob), true)])
            ),
            (
                mock_address(bob),
                BTreeMap::from([(mock_address(alice), true)])
            )
        ])
    );
}

#[test]
fn proper_revoke_for_all() {
    let owner = 1u8;
    let alice = 10u8;
    let bob = 11u8;
    let jack = 12u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        uri: "ipfs://random".to_string(),
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
        BTreeMap::from([(
            mock_address(alice),
            BTreeMap::from([(mock_address(jack), true)])
        )])
    );

    let revoke_all_msg = RevokeForAllMsg {
        operator: mock_address(jack),
    };
    let _ = execute_revoke_for_all(&mock_contract_context(alice), &mut state, &revoke_all_msg);
    assert_eq!(state.operator_approvals, BTreeMap::new());
}

#[test]
#[should_panic(expected = "Not found")]
fn revoke_not_existing_operator() {
    let owner = 1u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        uri: "ipfs://random".to_string(),
        minter: mock_address(1),
    };

    let (mut state, _) = execute_init(&mock_contract_context(2), &msg);

    let revoke_all_msg = RevokeForAllMsg {
        operator: mock_address(bob),
    };
    let _ = execute_revoke_for_all(&mock_contract_context(alice), &mut state, &revoke_all_msg);
}

#[test]
fn proper_transfer_from() {
    let owner = 1u8;
    let minter = 2u8;
    let alice = 10u8;
    let bob = 11u8;
    let jack = 12u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        uri: "ipfs://random".to_string(),
        minter: mock_address(minter),
    };

    let (mut state, _) = execute_init(&mock_contract_context(owner), &msg);

    for msg in vec![
        BatchMintMsg {
            to: mock_address(alice),
            token_infos: vec![
                TokenMintInfoMsg {
                    token_id: 1,
                    amount: 10,
                    token_uri: None,
                },
                TokenMintInfoMsg {
                    token_id: 2,
                    amount: 20,
                    token_uri: Some("2.json".to_string()),
                },
            ],
        },
        BatchMintMsg {
            to: mock_address(bob),
            token_infos: vec![
                TokenMintInfoMsg {
                    token_id: 1,
                    amount: 100,
                    token_uri: None,
                },
                TokenMintInfoMsg {
                    token_id: 3,
                    amount: 30,
                    token_uri: Some("3.json".to_string()),
                },
            ],
        },
    ]
    .into_iter()
    {
        let _ = execute_batch_mint(&mock_contract_context(minter), &mut state, &msg);
    }

    for (sender, msg) in vec![
        (
            alice,
            TransferFromMsg {
                from: mock_address(alice),
                to: mock_address(bob),
                token_info: TokenTransferInfoMsg {
                    token_id: 1,
                    amount: 5,
                },
            },
        ),
        (
            bob,
            TransferFromMsg {
                from: mock_address(bob),
                to: mock_address(jack),
                token_info: TokenTransferInfoMsg {
                    token_id: 3,
                    amount: 15,
                },
            },
        ),
        (
            alice,
            TransferFromMsg {
                from: mock_address(alice),
                to: mock_address(jack),
                token_info: TokenTransferInfoMsg {
                    token_id: 1,
                    amount: 4,
                },
            },
        ),
        (
            alice,
            TransferFromMsg {
                from: mock_address(alice),
                to: mock_address(jack),
                token_info: TokenTransferInfoMsg {
                    token_id: 2,
                    amount: 10,
                },
            },
        ),
    ]
    .into_iter()
    {
        let _ = execute_transfer_from(&mock_contract_context(sender), &mut state, &msg);
    }

    assert_eq!(
        state.balances,
        BTreeMap::from([
            (
                1,
                BTreeMap::from([
                    (mock_address(alice), 1),
                    (mock_address(bob), 105),
                    (mock_address(jack), 4)
                ])
            ),
            (
                2,
                BTreeMap::from([(mock_address(alice), 10), (mock_address(jack), 10)])
            ),
            (
                3,
                BTreeMap::from([(mock_address(bob), 15), (mock_address(jack), 15)])
            )
        ])
    );

    // allow jack to transfer alice tokens
    let approve_all_msg = ApproveForAllMsg {
        operator: mock_address(jack),
    };
    let _ = execute_approve_for_all(&mock_contract_context(alice), &mut state, &approve_all_msg);

    let transfer_from_msg = TransferFromMsg {
        from: mock_address(alice),
        to: mock_address(bob),
        token_info: TokenTransferInfoMsg {
            token_id: 1,
            amount: 1,
        },
    };

    let _ = execute_transfer_from(&mock_contract_context(jack), &mut state, &transfer_from_msg);
    assert_eq!(
        state.balances,
        BTreeMap::from([
            (
                1,
                BTreeMap::from([
                    (mock_address(alice), 0),
                    (mock_address(bob), 106),
                    (mock_address(jack), 4)
                ])
            ),
            (
                2,
                BTreeMap::from([(mock_address(alice), 10), (mock_address(jack), 10)])
            ),
            (
                3,
                BTreeMap::from([(mock_address(bob), 15), (mock_address(jack), 15)])
            )
        ])
    );
}

#[test]
#[should_panic]
fn transfer_not_owned_token() {
    let owner = 1u8;
    let minter = 2u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        uri: "ipfs://random".to_string(),
        minter: mock_address(minter),
    };

    let (mut state, _) = execute_init(&mock_contract_context(owner), &msg);

    let mint_msg = MintMsg {
        to: mock_address(alice),
        token_info: TokenMintInfoMsg {
            token_id: 1,
            amount: 10,
            token_uri: Some("1.json".to_string()),
        },
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let transfer_from_msg = TransferFromMsg {
        from: mock_address(alice),
        to: mock_address(bob),
        token_info: TokenTransferInfoMsg {
            token_id: 1,
            amount: 1,
        },
    };
    let _ = execute_transfer_from(&mock_contract_context(bob), &mut state, &transfer_from_msg);
}

#[test]
#[should_panic]
fn transfer_more_than_balance() {
    let owner = 1u8;
    let minter = 2u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        uri: "ipfs://random".to_string(),
        minter: mock_address(minter),
    };

    let (mut state, _) = execute_init(&mock_contract_context(owner), &msg);

    let mint_msg = MintMsg {
        to: mock_address(alice),
        token_info: TokenMintInfoMsg {
            token_id: 1,
            amount: 10,
            token_uri: Some("1.json".to_string()),
        },
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let transfer_from_msg = TransferFromMsg {
        from: mock_address(alice),
        to: mock_address(bob),
        token_info: TokenTransferInfoMsg {
            token_id: 1,
            amount: 11,
        },
    };
    let _ = execute_transfer_from(
        &mock_contract_context(alice),
        &mut state,
        &transfer_from_msg,
    );
}

#[test]
fn proper_batch_transfer_from() {
    let owner = 1u8;
    let minter = 2u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        uri: "ipfs://random".to_string(),
        minter: mock_address(minter),
    };

    let (mut state, _) = execute_init(&mock_contract_context(owner), &msg);

    let batch_mint_msg = BatchMintMsg {
        to: mock_address(alice),
        token_infos: vec![
            TokenMintInfoMsg {
                token_id: 1,
                amount: 10,
                token_uri: None,
            },
            TokenMintInfoMsg {
                token_id: 2,
                amount: 20,
                token_uri: Some("2.json".to_string()),
            },
        ],
    };
    let _ = execute_batch_mint(&mock_contract_context(minter), &mut state, &batch_mint_msg);

    let batch_transfer_from_msg = BatchTransferFromMsg {
        from: mock_address(alice),
        to: mock_address(bob),
        token_infos: vec![
            TokenTransferInfoMsg {
                token_id: 1,
                amount: 5,
            },
            TokenTransferInfoMsg {
                token_id: 2,
                amount: 5,
            },
        ],
    };
    let _ = execute_batch_transfer_from(
        &mock_contract_context(alice),
        &mut state,
        &batch_transfer_from_msg,
    );

    assert_eq!(
        state.balances,
        BTreeMap::from([
            (
                1,
                BTreeMap::from([(mock_address(alice), 5), (mock_address(bob), 5),])
            ),
            (
                2,
                BTreeMap::from([(mock_address(alice), 15), (mock_address(bob), 5)])
            )
        ])
    );
}

#[test]
#[should_panic]
fn batch_transfer_not_owned_token() {
    let owner = 1u8;
    let minter = 2u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        uri: "ipfs://random".to_string(),
        minter: mock_address(minter),
    };

    let (mut state, _) = execute_init(&mock_contract_context(owner), &msg);

    let mint_msg = MintMsg {
        to: mock_address(alice),
        token_info: TokenMintInfoMsg {
            token_id: 1,
            amount: 10,
            token_uri: Some("1.json".to_string()),
        },
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let batch_transfer_from_msg = BatchTransferFromMsg {
        from: mock_address(alice),
        to: mock_address(bob),
        token_infos: vec![TokenTransferInfoMsg {
            token_id: 1,
            amount: 1,
        }],
    };
    let _ = execute_batch_transfer_from(
        &mock_contract_context(bob),
        &mut state,
        &batch_transfer_from_msg,
    );
}

#[test]
#[should_panic]
fn batch_transfer_more_than_balance() {
    let owner = 1u8;
    let minter = 2u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        uri: "ipfs://random".to_string(),
        minter: mock_address(minter),
    };

    let (mut state, _) = execute_init(&mock_contract_context(owner), &msg);

    let mint_msg = MintMsg {
        to: mock_address(alice),
        token_info: TokenMintInfoMsg {
            token_id: 1,
            amount: 10,
            token_uri: Some("1.json".to_string()),
        },
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let batch_transfer_from_msg = BatchTransferFromMsg {
        from: mock_address(alice),
        to: mock_address(bob),
        token_infos: vec![TokenTransferInfoMsg {
            token_id: 1,
            amount: 11,
        }],
    };
    let _ = execute_batch_transfer_from(
        &mock_contract_context(alice),
        &mut state,
        &batch_transfer_from_msg,
    );
}

#[test]
fn proper_burn() {
    let owner = 1u8;
    let minter = 2u8;
    let alice = 10u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        uri: "ipfs://random".to_string(),
        minter: mock_address(minter),
    };

    let (mut state, _) = execute_init(&mock_contract_context(owner), &msg);

    let mint_msg = MintMsg {
        to: mock_address(alice),
        token_info: TokenMintInfoMsg {
            token_id: 1,
            amount: 10,
            token_uri: Some("1.json".to_string()),
        },
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let burn_msg = BurnMsg {
        from: mock_address(alice),
        token_info: TokenTransferInfoMsg {
            token_id: 1,
            amount: 1,
        },
    };
    let _ = execute_burn(&mock_contract_context(alice), &mut state, &burn_msg);
    assert_eq!(
        state.balances,
        BTreeMap::from([(1, BTreeMap::from([(mock_address(alice), 9)]))])
    );
}

#[test]
#[should_panic]
fn burn_not_owned_token() {
    let owner = 1u8;
    let minter = 2u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        uri: "ipfs://random".to_string(),
        minter: mock_address(minter),
    };

    let (mut state, _) = execute_init(&mock_contract_context(owner), &msg);

    let mint_msg = MintMsg {
        to: mock_address(alice),
        token_info: TokenMintInfoMsg {
            token_id: 1,
            amount: 10,
            token_uri: Some("1.json".to_string()),
        },
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let burn_msg = BurnMsg {
        from: mock_address(alice),
        token_info: TokenTransferInfoMsg {
            token_id: 1,
            amount: 1,
        },
    };
    let _ = execute_burn(&mock_contract_context(bob), &mut state, &burn_msg);
}

#[test]
#[should_panic]
fn burn_more_than_balance() {
    let owner = 1u8;
    let minter = 2u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        uri: "ipfs://random".to_string(),
        minter: mock_address(minter),
    };

    let (mut state, _) = execute_init(&mock_contract_context(owner), &msg);

    let mint_msg = MintMsg {
        to: mock_address(alice),
        token_info: TokenMintInfoMsg {
            token_id: 1,
            amount: 10,
            token_uri: Some("1.json".to_string()),
        },
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let burn_msg = BurnMsg {
        from: mock_address(alice),
        token_info: TokenTransferInfoMsg {
            token_id: 1,
            amount: 11,
        },
    };
    let _ = execute_burn(&mock_contract_context(alice), &mut state, &burn_msg);
}

#[test]
fn proper_batch_burn() {
    let owner = 1u8;
    let minter = 2u8;
    let alice = 10u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        uri: "ipfs://random".to_string(),
        minter: mock_address(minter),
    };

    let (mut state, _) = execute_init(&mock_contract_context(owner), &msg);

    let mint_msg = MintMsg {
        to: mock_address(alice),
        token_info: TokenMintInfoMsg {
            token_id: 1,
            amount: 10,
            token_uri: Some("1.json".to_string()),
        },
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let mint_msg = MintMsg {
        to: mock_address(alice),
        token_info: TokenMintInfoMsg {
            token_id: 2,
            amount: 10,
            token_uri: None,
        },
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let batch_burn_msg = BatchBurnMsg {
        from: mock_address(alice),
        token_infos: vec![
            TokenTransferInfoMsg {
                token_id: 1,
                amount: 1,
            },
            TokenTransferInfoMsg {
                token_id: 2,
                amount: 2,
            },
        ],
    };
    let _ = execute_batch_burn(&mock_contract_context(alice), &mut state, &batch_burn_msg);
    assert_eq!(
        state.balances,
        BTreeMap::from([
            (1, BTreeMap::from([(mock_address(alice), 9)])),
            (2, BTreeMap::from([(mock_address(alice), 8)]))
        ])
    );
}

#[test]
#[should_panic]
fn batch_burn_not_owned_token() {
    let owner = 1u8;
    let minter = 2u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        uri: "ipfs://random".to_string(),
        minter: mock_address(minter),
    };

    let (mut state, _) = execute_init(&mock_contract_context(owner), &msg);

    let mint_msg = MintMsg {
        to: mock_address(alice),
        token_info: TokenMintInfoMsg {
            token_id: 1,
            amount: 10,
            token_uri: Some("1.json".to_string()),
        },
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let mint_msg = MintMsg {
        to: mock_address(alice),
        token_info: TokenMintInfoMsg {
            token_id: 2,
            amount: 10,
            token_uri: None,
        },
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let batch_burn_msg = BatchBurnMsg {
        from: mock_address(alice),
        token_infos: vec![
            TokenTransferInfoMsg {
                token_id: 1,
                amount: 1,
            },
            TokenTransferInfoMsg {
                token_id: 2,
                amount: 2,
            },
        ],
    };
    let _ = execute_batch_burn(&mock_contract_context(bob), &mut state, &batch_burn_msg);
}

#[test]
#[should_panic]
fn batch_burn_more_than_balance() {
    let owner = 1u8;
    let minter = 2u8;
    let alice = 10u8;
    let bob = 11u8;

    let msg = InitMsg {
        owner: Some(mock_address(owner)),
        uri: "ipfs://random".to_string(),
        minter: mock_address(minter),
    };

    let (mut state, _) = execute_init(&mock_contract_context(owner), &msg);

    let mint_msg = MintMsg {
        to: mock_address(alice),
        token_info: TokenMintInfoMsg {
            token_id: 1,
            amount: 10,
            token_uri: Some("1.json".to_string()),
        },
    };

    let _ = execute_mint(&mock_contract_context(minter), &mut state, &mint_msg);

    let batch_burn_msg = BatchBurnMsg {
        from: mock_address(alice),
        token_infos: vec![TokenTransferInfoMsg {
            token_id: 1,
            amount: 11,
        }],
    };
    let _ = execute_batch_burn(&mock_contract_context(bob), &mut state, &batch_burn_msg);
}
