use std::collections::BTreeMap;

use pbc_contract_common::{
    address::{Address, AddressType},
    context::ContractContext,
};

use crate::{
    actions::{
        execute_approve, execute_burn, execute_burn_from, execute_decrease_allowance,
        execute_increase_allowance, execute_init, execute_mint, execute_transfer,
        execute_transfer_from,
    },
    msg::{
        ApproveMsg, BurnFromMsg, BurnMsg, DecreaseAllowanceMsg, IncreaseAllowanceMsg,
        InitialBalance, MintMsg, Mpc20InitMsg, TransferFromMsg, TransferMsg,
    },
    state::{MPC20ContractState, Minter, TokenInfo},
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
    let msg = Mpc20InitMsg {
        info: TokenInfo {
            name: "Token".to_string(),
            symbol: "TKN".to_string(),
            decimals: 9,
        },
        initial_balances: vec![InitialBalance {
            address: mock_address(10u8),
            amount: 100,
        }],
        minter: Some(Minter {
            minter: mock_address(3u8),
            capacity: Some(1_000),
        }),
    };

    let (state, events) = execute_init(&mock_contract_context(2u8), &msg);
    assert_eq!(events.len(), 0);
    assert_eq!(
        state,
        MPC20ContractState {
            info: TokenInfo {
                name: "Token".to_string(),
                symbol: "TKN".to_string(),
                decimals: 9,
            },
            total_supply: 100,
            minter: Some(Minter {
                minter: mock_address(3u8),
                capacity: Some(1_000),
            }),
            balances: BTreeMap::from([(mock_address(10u8), 100)]),
            allowances: BTreeMap::new(),
        }
    );
}

fn mock_token_info() -> TokenInfo {
    TokenInfo {
        name: "Token".to_string(),
        symbol: "TKN".to_string(),
        decimals: 9,
    }
}

#[test]
#[should_panic(expected = "Name is not in the expected length. Must be 3-50")]
fn invalid_token_name_on_init() {
    let msg = Mpc20InitMsg {
        info: TokenInfo {
            name: "TO".to_string(),
            symbol: "TKN".to_string(),
            decimals: 9,
        },
        initial_balances: vec![InitialBalance {
            address: mock_address(10u8),
            amount: 100,
        }],
        minter: Some(Minter {
            minter: mock_address(3u8),
            capacity: Some(1_000),
        }),
    };

    let (_, _) = execute_init(&mock_contract_context(2u8), &msg);
}

#[test]
#[should_panic(expected = "Ticker symbol is not in expected length. Must be 3-12")]
fn invalid_symbol_on_init() {
    let msg = Mpc20InitMsg {
        info: TokenInfo {
            name: "Token".to_string(),
            symbol: "TKKTKKTKKTKKR".to_string(),
            decimals: 9,
        },
        initial_balances: vec![InitialBalance {
            address: mock_address(10u8),
            amount: 100,
        }],
        minter: Some(Minter {
            minter: mock_address(3u8),
            capacity: Some(1_000),
        }),
    };

    let (_, _) = execute_init(&mock_contract_context(2u8), &msg);
}

#[test]
#[should_panic(expected = "Ticker symbol is not in expected format. Must be [a-zA-Z\\-]")]
fn invalid_symbol_character_on_init() {
    let msg = Mpc20InitMsg {
        info: TokenInfo {
            name: "Token".to_string(),
            symbol: "!@#TKN".to_string(),
            decimals: 9,
        },
        initial_balances: vec![InitialBalance {
            address: mock_address(10u8),
            amount: 100,
        }],
        minter: Some(Minter {
            minter: mock_address(3u8),
            capacity: Some(1_000),
        }),
    };

    let (_, _) = execute_init(&mock_contract_context(2u8), &msg);
}

#[test]
#[should_panic(expected = "")]
fn invalid_decimals_on_init() {
    let msg = Mpc20InitMsg {
        info: TokenInfo {
            name: "Token".to_string(),
            symbol: "TKN".to_string(),
            decimals: 100,
        },
        initial_balances: vec![InitialBalance {
            address: mock_address(10u8),
            amount: 100,
        }],
        minter: Some(Minter {
            minter: mock_address(3u8),
            capacity: Some(1_000),
        }),
    };

    let (_, _) = execute_init(&mock_contract_context(2u8), &msg);
}

#[test]
#[should_panic(expected = "Duplicate addresses in initial balances list")]
fn invalid_initial_balances_on_init() {
    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![
            InitialBalance {
                address: mock_address(10u8),
                amount: 100,
            },
            InitialBalance {
                address: mock_address(10u8),
                amount: 50,
            },
        ],
        minter: Some(Minter {
            minter: mock_address(3u8),
            capacity: Some(1_000),
        }),
    };

    let (_, _) = execute_init(&mock_contract_context(2u8), &msg);
}

#[test]
#[should_panic(expected = "Initial supply is greater than capacity")]
fn exceed_total_supply_on_init() {
    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![InitialBalance {
            address: mock_address(10u8),
            amount: 1001,
        }],
        minter: Some(Minter {
            minter: mock_address(3u8),
            capacity: Some(1_000),
        }),
    };

    let (_, _) = execute_init(&mock_contract_context(2u8), &msg);
}

#[test]
fn proper_mint() {
    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![],
        minter: Some(Minter {
            minter: mock_address(2u8),
            capacity: Some(1_000),
        }),
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let mint_msg = MintMsg {
        recipient: mock_address(10u8),
        amount: 400,
    };

    let _ = execute_mint(&mock_contract_context(2u8), &mut state, &mint_msg.clone());
    assert_eq!(state.balances, BTreeMap::from([(mock_address(10u8), 400)]));

    let _ = execute_mint(&mock_contract_context(2u8), &mut state, &mint_msg);
    assert_eq!(state.balances, BTreeMap::from([(mock_address(10u8), 800)]));
}

#[test]
#[should_panic(expected = "Amount must be higher then zero")]
fn zero_amount_on_mint() {
    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![],
        minter: Some(Minter {
            minter: mock_address(2u8),
            capacity: Some(1_000),
        }),
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let mint_msg = MintMsg {
        recipient: mock_address(10u8),
        amount: 0,
    };

    let _ = execute_mint(&mock_contract_context(2u8), &mut state, &mint_msg.clone());
}

#[test]
#[should_panic(expected = "Minting is disabled")]
fn minting_is_disabled_on_mint() {
    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![],
        minter: None,
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let mint_msg = MintMsg {
        recipient: mock_address(10u8),
        amount: 100,
    };

    let _ = execute_mint(&mock_contract_context(2u8), &mut state, &mint_msg.clone());
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn mint_from_different_address_on_mint() {
    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![],
        minter: Some(Minter {
            minter: mock_address(11u8),
            capacity: Some(1_000),
        }),
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let mint_msg = MintMsg {
        recipient: mock_address(10u8),
        amount: 100,
    };

    let _ = execute_mint(&mock_contract_context(2u8), &mut state, &mint_msg.clone());
}

#[test]
#[should_panic(expected = "Capacity exceeded")]
fn exceed_total_supply_on_mint() {
    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![],
        minter: Some(Minter {
            minter: mock_address(2u8),
            capacity: Some(1_000),
        }),
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let mint_msg = MintMsg {
        recipient: mock_address(10u8),
        amount: 1_001,
    };

    let _ = execute_mint(&mock_contract_context(2u8), &mut state, &mint_msg.clone());
}

#[test]
fn proper_transfer() {
    let alice = 10u8;
    let bob = 11u8;

    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![InitialBalance {
            address: mock_address(alice),
            amount: 1_000,
        }],
        minter: None,
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let transfer_msg = TransferMsg {
        to: mock_address(bob),
        amount: 100,
    };

    let _ = execute_transfer(&mock_contract_context(alice), &mut state, &transfer_msg);
    assert_eq!(
        state.balances,
        BTreeMap::from([(mock_address(alice), 900), (mock_address(bob), 100)])
    );

    let transfer_msg = TransferMsg {
        to: mock_address(bob),
        amount: 900,
    };

    let _ = execute_transfer(&mock_contract_context(alice), &mut state, &transfer_msg);
    assert_eq!(state.balances, BTreeMap::from([(mock_address(bob), 1000)]));
    assert_eq!(state.balance_of(&mock_address(alice)), 0);
    assert_eq!(state.balance_of(&mock_address(bob)), 1000);
}

#[test]
#[should_panic(expected = "Amount must be higher then zero")]
fn transfer_zero_amount() {
    let alice = 10u8;
    let bob = 11u8;

    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![],
        minter: None,
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let transfer_msg = TransferMsg {
        to: mock_address(bob),
        amount: 0,
    };

    let _ = execute_transfer(&mock_contract_context(alice), &mut state, &transfer_msg);
}

#[test]
#[should_panic(expected = "Not found")]
fn transfer_with_zero_balance() {
    let alice = 10u8;
    let bob = 11u8;

    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![],
        minter: None,
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let transfer_msg = TransferMsg {
        to: mock_address(bob),
        amount: 100,
    };

    let _ = execute_transfer(&mock_contract_context(alice), &mut state, &transfer_msg);
}

#[test]
#[should_panic(expected = "Overflow")]
fn insufficient_funds_on_transfer() {
    let alice = 10u8;
    let bob = 11u8;

    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![InitialBalance {
            address: mock_address(alice),
            amount: 99,
        }],
        minter: None,
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let transfer_msg = TransferMsg {
        to: mock_address(bob),
        amount: 100,
    };

    let _ = execute_transfer(&mock_contract_context(alice), &mut state, &transfer_msg);
}

#[test]
fn proper_transfer_from() {
    let alice = 10u8;
    let bob = 11u8;

    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![InitialBalance {
            address: mock_address(alice),
            amount: 1_000,
        }],
        minter: None,
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let increase_allowance_msg = IncreaseAllowanceMsg {
        spender: mock_address(bob),
        amount: 100,
    };

    let _ = execute_increase_allowance(
        &mock_contract_context(alice),
        &mut state,
        &increase_allowance_msg,
    );

    let transfer_from_msg = TransferFromMsg {
        from: mock_address(alice),
        to: mock_address(bob),
        amount: 100,
    };

    let _ = execute_transfer_from(&mock_contract_context(20u8), &mut state, &transfer_from_msg);
    assert_eq!(
        state.balances,
        BTreeMap::from([(mock_address(alice), 900), (mock_address(bob), 100)])
    );
}

#[test]
#[should_panic(expected = "Amount must be higher then zero")]
fn zero_amount_on_transfer_from() {
    let alice = 10u8;
    let bob = 11u8;

    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![InitialBalance {
            address: mock_address(alice),
            amount: 1_000,
        }],
        minter: None,
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let transfer_from_msg = TransferFromMsg {
        from: mock_address(alice),
        to: mock_address(bob),
        amount: 0,
    };

    let _ = execute_transfer_from(&mock_contract_context(20u8), &mut state, &transfer_from_msg);
}

#[test]
fn proper_burn() {
    let alice = 10u8;

    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![InitialBalance {
            address: mock_address(alice),
            amount: 1_000,
        }],
        minter: None,
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let burn_msg = BurnMsg { amount: 100 };
    let _ = execute_burn(&mock_contract_context(alice), &mut state, &burn_msg);
    assert_eq!(state.total_supply, 900);
    assert_eq!(state.balances, BTreeMap::from([(mock_address(alice), 900)]));
}

#[test]
#[should_panic(expected = "Amount must be higher then zero")]
fn burn_zero_amount() {
    let alice = 10u8;

    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![InitialBalance {
            address: mock_address(alice),
            amount: 1_000,
        }],
        minter: None,
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let burn_msg = BurnMsg { amount: 0 };
    let _ = execute_burn(&mock_contract_context(alice), &mut state, &burn_msg);
}

#[test]
#[should_panic(expected = "Not found")]
fn burn_with_zero_balance() {
    let alice = 10u8;

    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![],
        minter: None,
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let burn_msg = BurnMsg { amount: 100 };
    let _ = execute_burn(&mock_contract_context(alice), &mut state, &burn_msg);
}

#[test]
#[should_panic(expected = "Overflow")]
fn insufficient_funds_on_burn() {
    let alice = 10u8;

    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![InitialBalance {
            address: mock_address(alice),
            amount: 100,
        }],
        minter: None,
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let burn_msg = BurnMsg { amount: 101 };
    let _ = execute_burn(&mock_contract_context(alice), &mut state, &burn_msg);
}

#[test]
fn proper_burn_from() {
    let alice = 10u8;
    let bob = 11u8;

    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![InitialBalance {
            address: mock_address(alice),
            amount: 1_000,
        }],
        minter: None,
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let increase_allowance_msg = IncreaseAllowanceMsg {
        spender: mock_address(bob),
        amount: 100,
    };

    let _ = execute_increase_allowance(
        &mock_contract_context(alice),
        &mut state,
        &increase_allowance_msg,
    );

    let burn_from_msg = BurnFromMsg {
        owner: mock_address(alice),
        amount: 100,
    };

    let _ = execute_burn_from(&mock_contract_context(bob), &mut state, &burn_from_msg);
    assert_eq!(state.total_supply, 900);
    assert_eq!(state.balances, BTreeMap::from([(mock_address(alice), 900)]));
}

#[test]
#[should_panic(expected = "Amount must be higher then zero")]
fn zero_amount_on_burn_from() {
    let alice = 10u8;
    let bob = 11u8;

    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![InitialBalance {
            address: mock_address(alice),
            amount: 1_000,
        }],
        minter: None,
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let burn_from_msg = BurnFromMsg {
        owner: mock_address(alice),
        amount: 0,
    };

    let _ = execute_burn_from(&mock_contract_context(bob), &mut state, &burn_from_msg);
}

#[test]
fn proper_approve() {
    let alice = 10u8;
    let bob = 11u8;

    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![],
        minter: None,
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let approve_msg = ApproveMsg {
        spender: mock_address(bob),
        amount: 100,
    };
    let _ = execute_approve(&mock_contract_context(alice), &mut state, &approve_msg);
    assert_eq!(
        state.allowances,
        BTreeMap::from([(
            mock_address(alice),
            BTreeMap::from([(mock_address(bob), 100,)])
        )])
    );
    assert_eq!(
        state.allowance(&mock_address(alice), &mock_address(bob)),
        100
    );
    assert_eq!(state.allowance(&mock_address(bob), &mock_address(alice)), 0);
}

#[test]
#[should_panic(expected = "Cannot approve to yourself")]
fn approve_to_yourself() {
    let alice = 10u8;

    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![],
        minter: None,
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let approve_msg = ApproveMsg {
        spender: mock_address(alice),
        amount: 100,
    };
    let _ = execute_approve(&mock_contract_context(alice), &mut state, &approve_msg);
}

#[test]
#[should_panic(expected = "Amount must be higher then zero")]
pub fn zero_amount_on_approve() {
    let alice = 10u8;
    let bob = 11u8;

    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![],
        minter: None,
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let approve_msg = ApproveMsg {
        spender: mock_address(bob),
        amount: 0,
    };
    let _ = execute_approve(&mock_contract_context(alice), &mut state, &approve_msg);
}

#[test]
fn proper_increase_allowance() {
    let alice = 10u8;
    let bob = 11u8;
    let joe = 12u8;

    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![],
        minter: None,
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let increase_allowance_msg = IncreaseAllowanceMsg {
        spender: mock_address(bob),
        amount: 100,
    };

    let _ = execute_increase_allowance(
        &mock_contract_context(alice),
        &mut state,
        &increase_allowance_msg,
    );

    assert_eq!(
        state.allowances,
        BTreeMap::from([(
            mock_address(alice),
            BTreeMap::from([(mock_address(bob), 100,)])
        )])
    );

    let increase_allowance_msg = IncreaseAllowanceMsg {
        spender: mock_address(joe),
        amount: 500,
    };

    let _ = execute_increase_allowance(
        &mock_contract_context(bob),
        &mut state,
        &increase_allowance_msg,
    );
    assert_eq!(
        state.allowances,
        BTreeMap::from([
            (
                mock_address(alice),
                BTreeMap::from([(mock_address(bob), 100)])
            ),
            (
                mock_address(bob),
                BTreeMap::from([(mock_address(joe), 500)])
            )
        ])
    );
}

#[test]
#[should_panic(expected = "Cannot approve to yourself")]
fn increase_allowance_to_yourself() {
    let alice = 10u8;

    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![],
        minter: None,
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let increase_allowance_msg = IncreaseAllowanceMsg {
        spender: mock_address(alice),
        amount: 100,
    };

    let _ = execute_increase_allowance(
        &mock_contract_context(alice),
        &mut state,
        &increase_allowance_msg,
    );
}

#[test]
#[should_panic(expected = "Amount must be higher then zero")]
fn zero_amount_on_increase_allowance() {
    let alice = 10u8;
    let bob = 11u8;

    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![],
        minter: None,
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let increase_allowance_msg = IncreaseAllowanceMsg {
        spender: mock_address(bob),
        amount: 0,
    };

    let _ = execute_increase_allowance(
        &mock_contract_context(alice),
        &mut state,
        &increase_allowance_msg,
    );
}

#[test]
fn proper_decrease_allowance() {
    let alice = 10u8;
    let bob = 11u8;

    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![],
        minter: None,
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let increase_allowance_msg = IncreaseAllowanceMsg {
        spender: mock_address(bob),
        amount: 1_000,
    };

    let _ = execute_increase_allowance(
        &mock_contract_context(alice),
        &mut state,
        &increase_allowance_msg,
    );

    let decrease_allowance_msg = DecreaseAllowanceMsg {
        spender: mock_address(bob),
        amount: 450,
    };

    let _ = execute_decrease_allowance(
        &mock_contract_context(alice),
        &mut state,
        &decrease_allowance_msg,
    );

    assert_eq!(
        state.allowances,
        BTreeMap::from([(
            mock_address(alice),
            BTreeMap::from([(mock_address(bob), 550)])
        )])
    );
}

#[test]
#[should_panic(expected = "Cannot approve to yourself")]
fn decrease_allowance_to_yourself() {
    let alice = 10u8;
    let bob = 11u8;

    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![],
        minter: None,
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let decrease_allowance_msg = DecreaseAllowanceMsg {
        spender: mock_address(alice),
        amount: 450,
    };

    let _ = execute_decrease_allowance(
        &mock_contract_context(alice),
        &mut state,
        &decrease_allowance_msg,
    );
}

#[test]
#[should_panic(expected = "Amount must be higher then zero")]
fn zero_amount_on_decrease_allowance() {
    let alice = 10u8;
    let bob = 11u8;

    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![],
        minter: None,
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let decrease_allowance_msg = DecreaseAllowanceMsg {
        spender: mock_address(bob),
        amount: 0,
    };

    let _ = execute_decrease_allowance(
        &mock_contract_context(alice),
        &mut state,
        &decrease_allowance_msg,
    );
}

#[test]
#[should_panic(expected = "Not found")]
fn decrease_with_zero_approved() {
    let alice = 10u8;
    let bob = 11u8;

    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![],
        minter: None,
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let decrease_allowance_msg = DecreaseAllowanceMsg {
        spender: mock_address(bob),
        amount: 450,
    };

    let _ = execute_decrease_allowance(
        &mock_contract_context(alice),
        &mut state,
        &decrease_allowance_msg,
    );
}

#[test]
#[should_panic(expected = "Overflow")]
fn decrease_more_than_approved() {
    let alice = 10u8;
    let bob = 11u8;

    let msg = Mpc20InitMsg {
        info: mock_token_info(),
        initial_balances: vec![],
        minter: None,
    };

    let (mut state, _) = execute_init(&mock_contract_context(2u8), &msg);

    let increase_allowance_msg = IncreaseAllowanceMsg {
        spender: mock_address(bob),
        amount: 1_000,
    };

    let _ = execute_increase_allowance(
        &mock_contract_context(alice),
        &mut state,
        &increase_allowance_msg,
    );

    let decrease_allowance_msg = DecreaseAllowanceMsg {
        spender: mock_address(bob),
        amount: 1_001,
    };

    let _ = execute_decrease_allowance(
        &mock_contract_context(alice),
        &mut state,
        &decrease_allowance_msg,
    );
}
