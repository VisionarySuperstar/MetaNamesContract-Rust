use std::collections::BTreeMap;

use mpc20_base::{
    msg::{TransferFromMsg, TransferMsg},
    state::{MPC20ContractState, Minter, TokenInfo},
};
use pbc_contract_common::{
    address::{Address, AddressType, Shortname},
    context::ContractContext,
    events::EventGroup,
};
use utils::{decimal::DecimalRatio, events::IntoShortnameRPCEvent};

use crate::{
    actions::{execute_claim, execute_compound, execute_init, execute_stake, execute_unstake},
    msg::{ClaimMsg, CompoundMsg, Mpc20StakingInitMsg, StakeMsg, UnstakeMsg},
    state::{MPC20StakingContractState, Staker},
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

fn mock_contract_context(sender: u8, block_time: i64) -> ContractContext {
    ContractContext {
        contract_address: mock_address(1u8),
        sender: mock_address(sender),
        block_time,
        block_production_time: block_time,
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
fn test_staking() {
    const DEPOSIT_TOKEN: u8 = 1;
    const MINTER: u8 = 9;
    const ALICE: u8 = 10;
    const BOB: u8 = 11;
    const JACK: u8 = 12;

    let mut block_production_time = 100;

    let msg = Mpc20StakingInitMsg {
        deposit_token: None,
        distribution_amount: 1_000,
        distribution_epoch: 10,
        compound_frequency: 100,
        info: TokenInfo {
            name: "Staking Token".to_string(),
            symbol: "STKN".to_string(),
            decimals: 18,
        },
        initial_balances: vec![],
        minter: Some(mock_address(MINTER)),
    };
    let (mut state, events) =
        execute_init(&mock_contract_context(MINTER, block_production_time), &msg);
    assert_eq!(events, vec![]);
    assert_eq!(
        state,
        MPC20StakingContractState {
            deposit_token: mock_address(DEPOSIT_TOKEN),
            distribution_amount: 1_000,
            distribution_epoch: 10,
            global_index: DecimalRatio::new(0, 0),
            total_staked: 0,
            last_distributed: 100,
            stakers: BTreeMap::new(),
            compound_frequency: 100,
            mpc20: MPC20ContractState {
                info: TokenInfo {
                    name: "Staking Token".to_string(),
                    symbol: "STKN".to_string(),
                    decimals: 18,
                },
                total_supply: 0,
                minter: Some(Minter {
                    minter: mock_address(MINTER),
                    capacity: None,
                }),
                balances: BTreeMap::new(),
                allowances: BTreeMap::new(),
            },
        }
    );

    // no distribution yet, total_stake is zero
    block_production_time = 105;

    let msg = StakeMsg { amount: 100 };
    let events = execute_stake(
        &mock_contract_context(ALICE, block_production_time),
        &mut state,
        &msg,
    );

    assert_eq!(events.len(), 1);

    let transfer_from_msg = TransferFromMsg {
        from: mock_address(ALICE),
        to: mock_address(1u8),
        amount: 100,
    };

    let mut eg = EventGroup::builder();
    eg.call(
        mock_address(DEPOSIT_TOKEN),
        Shortname::from_u32(transfer_from_msg.action_shortname()),
    )
    .argument(mock_address(ALICE))
    .argument(mock_address(1u8))
    .argument(100u128)
    .done();

    assert_eq!(events[0], eg.build());

    assert_eq!(
        state,
        MPC20StakingContractState {
            deposit_token: mock_address(DEPOSIT_TOKEN),
            distribution_amount: 1_000,
            distribution_epoch: 10,
            global_index: DecimalRatio::new(0, 0),
            total_staked: 100,
            last_distributed: 105,
            stakers: BTreeMap::from([(
                mock_address(ALICE),
                Staker {
                    reward_index: DecimalRatio::zero(),
                    staked_amount: 100,
                    pending_reward: 0,
                    last_compound: 0,
                }
            )]),
            compound_frequency: 100,
            mpc20: MPC20ContractState {
                info: TokenInfo {
                    name: "Staking Token".to_string(),
                    symbol: "STKN".to_string(),
                    decimals: 18,
                },
                total_supply: 0,
                minter: Some(Minter {
                    minter: mock_address(MINTER),
                    capacity: None,
                }),
                balances: BTreeMap::new(),
                allowances: BTreeMap::new(),
            },
        }
    );

    // no distribution yet, total stake 100
    block_production_time = 114;

    let msg = StakeMsg { amount: 100 };
    let events = execute_stake(
        &mock_contract_context(BOB, block_production_time),
        &mut state,
        &msg,
    );
    assert_eq!(
        state,
        MPC20StakingContractState {
            deposit_token: mock_address(DEPOSIT_TOKEN),
            distribution_amount: 1_000,
            distribution_epoch: 10,
            global_index: DecimalRatio::new(0, 0),
            total_staked: 200,
            last_distributed: 105,
            stakers: BTreeMap::from([
                (
                    mock_address(ALICE),
                    Staker {
                        reward_index: DecimalRatio::zero(),
                        staked_amount: 100,
                        pending_reward: 0,
                        last_compound: 0,
                    }
                ),
                (
                    mock_address(BOB),
                    Staker {
                        reward_index: DecimalRatio::zero(),
                        staked_amount: 100,
                        pending_reward: 0,
                        last_compound: 0,
                    }
                )
            ]),
            compound_frequency: 100,
            mpc20: MPC20ContractState {
                info: TokenInfo {
                    name: "Staking Token".to_string(),
                    symbol: "STKN".to_string(),
                    decimals: 18,
                },
                total_supply: 0,
                minter: Some(Minter {
                    minter: mock_address(MINTER),
                    capacity: None,
                }),
                balances: BTreeMap::new(),
                allowances: BTreeMap::new(),
            },
        }
    );

    // first distribution, ALICE and BOB must claim and receive equal rewards
    block_production_time = 115;

    let msg = ClaimMsg { amount: None };
    let _ = execute_claim(
        &mock_contract_context(ALICE, block_production_time),
        &mut state,
        &msg,
    );
    assert_eq!(
        state,
        MPC20StakingContractState {
            deposit_token: mock_address(DEPOSIT_TOKEN),
            distribution_amount: 1_000,
            distribution_epoch: 10,
            global_index: DecimalRatio::new(5, 0),
            total_staked: 200,
            last_distributed: 115,
            stakers: BTreeMap::from([
                (
                    mock_address(ALICE),
                    Staker {
                        reward_index: DecimalRatio::new(5, 0),
                        staked_amount: 100,
                        pending_reward: 0, // pending reward claimed(minted)
                        last_compound: 0,
                    }
                ),
                (
                    mock_address(BOB),
                    Staker {
                        reward_index: DecimalRatio::zero(),
                        staked_amount: 100,
                        pending_reward: 0,
                        last_compound: 0,
                    }
                )
            ]),
            compound_frequency: 100,
            mpc20: MPC20ContractState {
                info: TokenInfo {
                    name: "Staking Token".to_string(),
                    symbol: "STKN".to_string(),
                    decimals: 18,
                },
                total_supply: 500,
                minter: Some(Minter {
                    minter: mock_address(MINTER),
                    capacity: None,
                }),
                balances: BTreeMap::from([(mock_address(ALICE), 500),]),
                allowances: BTreeMap::new(),
            },
        }
    );

    block_production_time = 116;
    let msg = ClaimMsg { amount: None };
    let _ = execute_claim(
        &mock_contract_context(BOB, block_production_time),
        &mut state,
        &msg,
    );
    assert_eq!(
        state,
        MPC20StakingContractState {
            deposit_token: mock_address(DEPOSIT_TOKEN),
            distribution_amount: 1_000,
            distribution_epoch: 10,
            global_index: DecimalRatio::new(5, 0),
            total_staked: 200,
            last_distributed: 115,
            stakers: BTreeMap::from([
                (
                    mock_address(ALICE),
                    Staker {
                        reward_index: DecimalRatio::new(5, 0),
                        staked_amount: 100,
                        pending_reward: 0, // pending reward claimed(minted)
                        last_compound: 0,
                    }
                ),
                (
                    mock_address(BOB),
                    Staker {
                        reward_index: DecimalRatio::new(5, 0),
                        staked_amount: 100,
                        pending_reward: 0,
                        last_compound: 0,
                    }
                )
            ]),
            compound_frequency: 100,
            mpc20: MPC20ContractState {
                info: TokenInfo {
                    name: "Staking Token".to_string(),
                    symbol: "STKN".to_string(),
                    decimals: 18,
                },
                total_supply: 1_000,
                minter: Some(Minter {
                    minter: mock_address(MINTER),
                    capacity: None,
                }),
                balances: BTreeMap::from([(mock_address(ALICE), 500), (mock_address(BOB), 500)]),
                allowances: BTreeMap::new(),
            },
        }
    );

    // BOB unstakes half
    block_production_time = 120;
    let msg = UnstakeMsg { amount: 50 };
    let unstake_events = execute_unstake(
        &mock_contract_context(BOB, block_production_time),
        &mut state,
        &msg,
    );

    assert_eq!(unstake_events.len(), 1);

    let transfer_msg = TransferMsg {
        to: mock_address(BOB),
        amount: 50,
    };

    let mut eg = EventGroup::builder();
    eg.call(
        mock_address(DEPOSIT_TOKEN),
        Shortname::from_u32(transfer_msg.action_shortname()),
    )
    .argument(mock_address(BOB))
    .argument(50u128)
    .done();

    assert_eq!(unstake_events[0], eg.build());

    assert_eq!(
        state,
        MPC20StakingContractState {
            deposit_token: mock_address(DEPOSIT_TOKEN),
            distribution_amount: 1_000,
            distribution_epoch: 10,
            global_index: DecimalRatio::new(5, 0),
            total_staked: 150,
            last_distributed: 115,
            stakers: BTreeMap::from([
                (
                    mock_address(ALICE),
                    Staker {
                        reward_index: DecimalRatio::new(5, 0),
                        staked_amount: 100,
                        pending_reward: 0, // pending reward claimed(minted)
                        last_compound: 0,
                    }
                ),
                (
                    mock_address(BOB),
                    Staker {
                        reward_index: DecimalRatio::new(5, 0),
                        staked_amount: 50,
                        pending_reward: 0,
                        last_compound: 0,
                    }
                )
            ]),
            compound_frequency: 100,
            mpc20: MPC20ContractState {
                info: TokenInfo {
                    name: "Staking Token".to_string(),
                    symbol: "STKN".to_string(),
                    decimals: 18,
                },
                total_supply: 1_000,
                minter: Some(Minter {
                    minter: mock_address(MINTER),
                    capacity: None,
                }),
                balances: BTreeMap::from([(mock_address(ALICE), 500), (mock_address(BOB), 500)]),
                allowances: BTreeMap::new(),
            },
        }
    );

    // next distribution, ALICE share 66.7, BOB share 33.3
    block_production_time = 125;
    let msg = ClaimMsg { amount: Some(100) };
    let _ = execute_claim(
        &mock_contract_context(ALICE, block_production_time),
        &mut state,
        &msg,
    );
    let msg = ClaimMsg { amount: None };
    let _ = execute_claim(
        &mock_contract_context(BOB, block_production_time),
        &mut state,
        &msg,
    );
    assert_eq!(
        state,
        MPC20StakingContractState {
            deposit_token: mock_address(DEPOSIT_TOKEN),
            distribution_amount: 1_000,
            distribution_epoch: 10,
            global_index: DecimalRatio::new(11666666666666666666666666667, 27),
            total_staked: 150,
            last_distributed: 125,
            stakers: BTreeMap::from([
                (
                    mock_address(ALICE),
                    Staker {
                        reward_index: DecimalRatio::new(11666666666666666666666666667, 27),
                        staked_amount: 100,
                        pending_reward: 566, // 666 - claim_amount
                        last_compound: 0,
                    }
                ),
                (
                    mock_address(BOB),
                    Staker {
                        reward_index: DecimalRatio::new(11666666666666666666666666667, 27),
                        staked_amount: 50,
                        pending_reward: 0,
                        last_compound: 0,
                    }
                )
            ]),
            compound_frequency: 100,
            mpc20: MPC20ContractState {
                info: TokenInfo {
                    name: "Staking Token".to_string(),
                    symbol: "STKN".to_string(),
                    decimals: 18,
                },
                total_supply: 1_433,
                minter: Some(Minter {
                    minter: mock_address(MINTER),
                    capacity: None,
                }),
                balances: BTreeMap::from([(mock_address(ALICE), 600), (mock_address(BOB), 833)]),
                allowances: BTreeMap::new(),
            },
        }
    );

    // JACK stakes 50, ALICE share 50, BOB and JACK 25 resp
    block_production_time = 134;

    let msg = StakeMsg { amount: 50 };
    let events = execute_stake(
        &mock_contract_context(JACK, block_production_time),
        &mut state,
        &msg,
    );
    assert_eq!(
        state,
        MPC20StakingContractState {
            deposit_token: mock_address(DEPOSIT_TOKEN),
            distribution_amount: 1_000,
            distribution_epoch: 10,
            global_index: DecimalRatio::new(11666666666666666666666666667, 27),
            total_staked: 200,
            last_distributed: 125,
            stakers: BTreeMap::from([
                (
                    mock_address(ALICE),
                    Staker {
                        reward_index: DecimalRatio::new(11666666666666666666666666667, 27),
                        staked_amount: 100,
                        pending_reward: 566, // 666 - claim_amount
                        last_compound: 0,
                    }
                ),
                (
                    mock_address(BOB),
                    Staker {
                        reward_index: DecimalRatio::new(11666666666666666666666666667, 27),
                        staked_amount: 50,
                        pending_reward: 0,
                        last_compound: 0,
                    }
                ),
                (
                    mock_address(JACK),
                    Staker {
                        reward_index: DecimalRatio::new(11666666666666666666666666667, 27),
                        staked_amount: 50,
                        pending_reward: 0,
                        last_compound: 0,
                    }
                )
            ]),
            compound_frequency: 100,
            mpc20: MPC20ContractState {
                info: TokenInfo {
                    name: "Staking Token".to_string(),
                    symbol: "STKN".to_string(),
                    decimals: 18,
                },
                total_supply: 1_433,
                minter: Some(Minter {
                    minter: mock_address(MINTER),
                    capacity: None,
                }),
                balances: BTreeMap::from([(mock_address(ALICE), 600), (mock_address(BOB), 833)]),
                allowances: BTreeMap::new(),
            },
        }
    );

    // everyone claims 1 token
    block_production_time = 140;

    let msg = ClaimMsg { amount: Some(1) };
    let _ = execute_claim(
        &mock_contract_context(ALICE, block_production_time),
        &mut state,
        &msg,
    );
    let _ = execute_claim(
        &mock_contract_context(BOB, block_production_time),
        &mut state,
        &msg,
    );
    let _ = execute_claim(
        &mock_contract_context(JACK, block_production_time),
        &mut state,
        &msg,
    );
    assert_eq!(
        state,
        MPC20StakingContractState {
            deposit_token: mock_address(DEPOSIT_TOKEN),
            distribution_amount: 1_000,
            distribution_epoch: 10,
            global_index: DecimalRatio::new(16666666666666666666666666667, 27),
            total_staked: 200,
            last_distributed: 135,
            stakers: BTreeMap::from([
                (
                    mock_address(ALICE),
                    Staker {
                        reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                        staked_amount: 100,
                        pending_reward: 1065,
                        last_compound: 0,
                    }
                ),
                (
                    mock_address(BOB),
                    Staker {
                        reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                        staked_amount: 50,
                        pending_reward: 249,
                        last_compound: 0,
                    }
                ),
                (
                    mock_address(JACK),
                    Staker {
                        reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                        staked_amount: 50,
                        pending_reward: 249,
                        last_compound: 0,
                    }
                )
            ]),
            compound_frequency: 100,
            mpc20: MPC20ContractState {
                info: TokenInfo {
                    name: "Staking Token".to_string(),
                    symbol: "STKN".to_string(),
                    decimals: 18,
                },
                total_supply: 1_436,
                minter: Some(Minter {
                    minter: mock_address(MINTER),
                    capacity: None,
                }),
                balances: BTreeMap::from([
                    (mock_address(ALICE), 601),
                    (mock_address(BOB), 834),
                    (mock_address(JACK), 1)
                ]),
                allowances: BTreeMap::new(),
            },
        }
    );

    // JACK compounds
    block_production_time = 144;

    let msg = CompoundMsg { amount: Some(100) };
    let events = execute_compound(
        &mock_contract_context(JACK, block_production_time),
        &mut state,
        &msg,
    );
    assert_eq!(
        state,
        MPC20StakingContractState {
            deposit_token: mock_address(DEPOSIT_TOKEN),
            distribution_amount: 1_000,
            distribution_epoch: 10,
            global_index: DecimalRatio::new(16666666666666666666666666667, 27),
            total_staked: 300,
            last_distributed: 135,
            stakers: BTreeMap::from([
                (
                    mock_address(ALICE),
                    Staker {
                        reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                        staked_amount: 100,
                        pending_reward: 1065,
                        last_compound: 0,
                    }
                ),
                (
                    mock_address(BOB),
                    Staker {
                        reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                        staked_amount: 50,
                        pending_reward: 249,
                        last_compound: 0,
                    }
                ),
                (
                    mock_address(JACK),
                    Staker {
                        reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                        staked_amount: 150,
                        pending_reward: 149,
                        last_compound: 144,
                    }
                )
            ]),
            compound_frequency: 100,
            mpc20: MPC20ContractState {
                info: TokenInfo {
                    name: "Staking Token".to_string(),
                    symbol: "STKN".to_string(),
                    decimals: 18,
                },
                total_supply: 1_536,
                minter: Some(Minter {
                    minter: mock_address(MINTER),
                    capacity: None,
                }),
                balances: BTreeMap::from([
                    (mock_address(ALICE), 601),
                    (mock_address(BOB), 834),
                    (mock_address(JACK), 1),
                    (mock_address(DEPOSIT_TOKEN), 100) // compound - claim + stake
                ]),
                allowances: BTreeMap::new(),
            },
        }
    );
}

#[test]
#[should_panic(expected = "Distribution amount must be higher then 0")]
fn invalid_distribution_amount() {
    const MINTER: u8 = 9;

    let block_production_time = 100;

    let msg = Mpc20StakingInitMsg {
        deposit_token: None,
        distribution_amount: 0,
        distribution_epoch: 10,
        compound_frequency: 100,
        info: TokenInfo {
            name: "Staking Token".to_string(),
            symbol: "STKN".to_string(),
            decimals: 18,
        },
        initial_balances: vec![],
        minter: Some(mock_address(MINTER)),
    };
    let (_, _) = execute_init(&mock_contract_context(MINTER, block_production_time), &msg);
}

#[test]
#[should_panic(expected = "Distribution epoch must be higher then 0")]
fn invalid_distribution_epoch() {
    const MINTER: u8 = 9;

    let block_production_time = 100;

    let msg = Mpc20StakingInitMsg {
        deposit_token: None,
        distribution_amount: 1_000,
        distribution_epoch: 0,
        compound_frequency: 100,
        info: TokenInfo {
            name: "Staking Token".to_string(),
            symbol: "STKN".to_string(),
            decimals: 18,
        },
        initial_balances: vec![],
        minter: Some(mock_address(MINTER)),
    };
    let (_, _) = execute_init(&mock_contract_context(MINTER, block_production_time), &msg);
}

#[test]
#[should_panic(expected = "Cannot unstake more then staked")]
fn unstake_more_then_staked() {
    const DEPOSIT_TOKEN: u8 = 1;
    const MINTER: u8 = 9;
    const ALICE: u8 = 10;
    const BOB: u8 = 11;
    const JACK: u8 = 12;

    let block_production_time = 150;

    let mut state = MPC20StakingContractState {
        deposit_token: mock_address(DEPOSIT_TOKEN),
        distribution_amount: 1_000,
        distribution_epoch: 10,
        global_index: DecimalRatio::new(16666666666666666666666666667, 27),
        total_staked: 300,
        last_distributed: 135,
        stakers: BTreeMap::from([
            (
                mock_address(ALICE),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 100,
                    pending_reward: 1065,
                    last_compound: 0,
                },
            ),
            (
                mock_address(BOB),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 50,
                    pending_reward: 249,
                    last_compound: 0,
                },
            ),
            (
                mock_address(JACK),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 150,
                    pending_reward: 149,
                    last_compound: 144,
                },
            ),
        ]),
        compound_frequency: 100,
        mpc20: MPC20ContractState {
            info: TokenInfo {
                name: "Staking Token".to_string(),
                symbol: "STKN".to_string(),
                decimals: 18,
            },
            total_supply: 1_536,
            minter: Some(Minter {
                minter: mock_address(MINTER),
                capacity: None,
            }),
            balances: BTreeMap::from([
                (mock_address(ALICE), 601),
                (mock_address(BOB), 834),
                (mock_address(JACK), 1),
                (mock_address(DEPOSIT_TOKEN), 100),
            ]),
            allowances: BTreeMap::new(),
        },
    };

    let msg = UnstakeMsg { amount: 151 };
    let _ = execute_unstake(
        &mock_contract_context(JACK, block_production_time),
        &mut state,
        &msg,
    );
}

#[test]
#[should_panic(expected = "Nothing to claim")]
fn claim_with_zero_rewards() {
    const DEPOSIT_TOKEN: u8 = 1;
    const MINTER: u8 = 9;
    const ALICE: u8 = 10;
    const BOB: u8 = 11;
    const JACK: u8 = 12;

    let block_production_time = 135;

    let mut state = MPC20StakingContractState {
        deposit_token: mock_address(DEPOSIT_TOKEN),
        distribution_amount: 1_000,
        distribution_epoch: 10,
        global_index: DecimalRatio::new(16666666666666666666666666667, 27),
        total_staked: 300,
        last_distributed: 135,
        stakers: BTreeMap::from([
            (
                mock_address(ALICE),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 100,
                    pending_reward: 1065,
                    last_compound: 0,
                },
            ),
            (
                mock_address(BOB),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 50,
                    pending_reward: 249,
                    last_compound: 0,
                },
            ),
            (
                mock_address(JACK),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 150,
                    pending_reward: 0,
                    last_compound: 144,
                },
            ),
        ]),
        compound_frequency: 100,
        mpc20: MPC20ContractState {
            info: TokenInfo {
                name: "Staking Token".to_string(),
                symbol: "STKN".to_string(),
                decimals: 18,
            },
            total_supply: 1_536,
            minter: Some(Minter {
                minter: mock_address(MINTER),
                capacity: None,
            }),
            balances: BTreeMap::from([
                (mock_address(ALICE), 601),
                (mock_address(BOB), 834),
                (mock_address(JACK), 1),
                (mock_address(DEPOSIT_TOKEN), 100),
            ]),
            allowances: BTreeMap::new(),
        },
    };

    let msg = ClaimMsg { amount: None };
    let _ = execute_claim(
        &mock_contract_context(JACK, block_production_time),
        &mut state,
        &msg,
    );
}

#[test]
#[should_panic(expected = "Cannot claim more then rewarded")]
fn claim_more_then_rewarded() {
    const DEPOSIT_TOKEN: u8 = 1;
    const MINTER: u8 = 9;
    const ALICE: u8 = 10;
    const BOB: u8 = 11;
    const JACK: u8 = 12;

    let block_production_time = 135;

    let mut state = MPC20StakingContractState {
        deposit_token: mock_address(DEPOSIT_TOKEN),
        distribution_amount: 1_000,
        distribution_epoch: 10,
        global_index: DecimalRatio::new(16666666666666666666666666667, 27),
        total_staked: 300,
        last_distributed: 135,
        stakers: BTreeMap::from([
            (
                mock_address(ALICE),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 100,
                    pending_reward: 1065,
                    last_compound: 0,
                },
            ),
            (
                mock_address(BOB),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 50,
                    pending_reward: 249,
                    last_compound: 0,
                },
            ),
            (
                mock_address(JACK),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 150,
                    pending_reward: 10,
                    last_compound: 144,
                },
            ),
        ]),
        compound_frequency: 100,
        mpc20: MPC20ContractState {
            info: TokenInfo {
                name: "Staking Token".to_string(),
                symbol: "STKN".to_string(),
                decimals: 18,
            },
            total_supply: 1_536,
            minter: Some(Minter {
                minter: mock_address(MINTER),
                capacity: None,
            }),
            balances: BTreeMap::from([
                (mock_address(ALICE), 601),
                (mock_address(BOB), 834),
                (mock_address(JACK), 1),
                (mock_address(DEPOSIT_TOKEN), 100),
            ]),
            allowances: BTreeMap::new(),
        },
    };

    let msg = ClaimMsg { amount: Some(11) };
    let _ = execute_claim(
        &mock_contract_context(JACK, block_production_time),
        &mut state,
        &msg,
    );
}

#[test]
#[should_panic(expected = "Compound only enabled when deposit token is reward token")]
fn compound_when_disabled() {
    const DEPOSIT_TOKEN: u8 = 2;
    const MINTER: u8 = 9;
    const ALICE: u8 = 10;
    const BOB: u8 = 11;
    const JACK: u8 = 12;

    let block_production_time = 135;

    let mut state = MPC20StakingContractState {
        deposit_token: mock_address(DEPOSIT_TOKEN),
        distribution_amount: 1_000,
        distribution_epoch: 10,
        global_index: DecimalRatio::new(16666666666666666666666666667, 27),
        total_staked: 300,
        last_distributed: 135,
        stakers: BTreeMap::from([
            (
                mock_address(ALICE),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 100,
                    pending_reward: 1065,
                    last_compound: 0,
                },
            ),
            (
                mock_address(BOB),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 50,
                    pending_reward: 249,
                    last_compound: 0,
                },
            ),
            (
                mock_address(JACK),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 150,
                    pending_reward: 10,
                    last_compound: 144,
                },
            ),
        ]),
        compound_frequency: 100,
        mpc20: MPC20ContractState {
            info: TokenInfo {
                name: "Staking Token".to_string(),
                symbol: "STKN".to_string(),
                decimals: 18,
            },
            total_supply: 1_536,
            minter: Some(Minter {
                minter: mock_address(MINTER),
                capacity: None,
            }),
            balances: BTreeMap::from([
                (mock_address(ALICE), 601),
                (mock_address(BOB), 834),
                (mock_address(JACK), 1),
                (mock_address(DEPOSIT_TOKEN), 100),
            ]),
            allowances: BTreeMap::new(),
        },
    };

    let msg = CompoundMsg { amount: None };
    let _ = execute_compound(
        &mock_contract_context(BOB, block_production_time),
        &mut state,
        &msg,
    );
}

#[test]
#[should_panic(expected = "Forbidden to compound to often")]
fn compound_to_often() {
    const DEPOSIT_TOKEN: u8 = 1;
    const MINTER: u8 = 9;
    const ALICE: u8 = 10;
    const BOB: u8 = 11;
    const JACK: u8 = 12;

    let block_production_time = 135;

    let mut state = MPC20StakingContractState {
        deposit_token: mock_address(DEPOSIT_TOKEN),
        distribution_amount: 1_000,
        distribution_epoch: 10,
        global_index: DecimalRatio::new(16666666666666666666666666667, 27),
        total_staked: 300,
        last_distributed: 135,
        stakers: BTreeMap::from([
            (
                mock_address(ALICE),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 100,
                    pending_reward: 1065,
                    last_compound: 0,
                },
            ),
            (
                mock_address(BOB),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 50,
                    pending_reward: 249,
                    last_compound: 36,
                },
            ),
            (
                mock_address(JACK),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 150,
                    pending_reward: 10,
                    last_compound: 144,
                },
            ),
        ]),
        compound_frequency: 100,
        mpc20: MPC20ContractState {
            info: TokenInfo {
                name: "Staking Token".to_string(),
                symbol: "STKN".to_string(),
                decimals: 18,
            },
            total_supply: 1_536,
            minter: Some(Minter {
                minter: mock_address(MINTER),
                capacity: None,
            }),
            balances: BTreeMap::from([
                (mock_address(ALICE), 601),
                (mock_address(BOB), 834),
                (mock_address(JACK), 1),
                (mock_address(DEPOSIT_TOKEN), 100),
            ]),
            allowances: BTreeMap::new(),
        },
    };

    let msg = CompoundMsg { amount: None };
    let _ = execute_compound(
        &mock_contract_context(BOB, block_production_time),
        &mut state,
        &msg,
    );
}

#[test]
#[should_panic(expected = "Cannot compound more then rewarded")]
fn compound_more_then_rewarded() {
    const DEPOSIT_TOKEN: u8 = 1;
    const MINTER: u8 = 9;
    const ALICE: u8 = 10;
    const BOB: u8 = 11;
    const JACK: u8 = 12;

    let block_production_time = 135;

    let mut state = MPC20StakingContractState {
        deposit_token: mock_address(DEPOSIT_TOKEN),
        distribution_amount: 1_000,
        distribution_epoch: 10,
        global_index: DecimalRatio::new(16666666666666666666666666667, 27),
        total_staked: 300,
        last_distributed: 135,
        stakers: BTreeMap::from([
            (
                mock_address(ALICE),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 100,
                    pending_reward: 1065,
                    last_compound: 0,
                },
            ),
            (
                mock_address(BOB),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 50,
                    pending_reward: 249,
                    last_compound: 0,
                },
            ),
            (
                mock_address(JACK),
                Staker {
                    reward_index: DecimalRatio::new(16666666666666666666666666667, 27),
                    staked_amount: 150,
                    pending_reward: 10,
                    last_compound: 144,
                },
            ),
        ]),
        compound_frequency: 100,
        mpc20: MPC20ContractState {
            info: TokenInfo {
                name: "Staking Token".to_string(),
                symbol: "STKN".to_string(),
                decimals: 18,
            },
            total_supply: 1_536,
            minter: Some(Minter {
                minter: mock_address(MINTER),
                capacity: None,
            }),
            balances: BTreeMap::from([
                (mock_address(ALICE), 601),
                (mock_address(BOB), 834),
                (mock_address(JACK), 1),
                (mock_address(DEPOSIT_TOKEN), 100),
            ]),
            allowances: BTreeMap::new(),
        },
    };

    let msg = CompoundMsg { amount: Some(250) };
    let _ = execute_compound(
        &mock_contract_context(BOB, block_production_time),
        &mut state,
        &msg,
    );
}
