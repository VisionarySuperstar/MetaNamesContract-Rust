use std::collections::BTreeMap;

use pbc_contract_common::{context::ContractContext, events::EventGroup};
use rust_decimal::prelude::*;

use crate::{
    msg::{ClaimMsg, CompoundMsg, Mpc20StakingInitMsg, StakeMsg, UnstakeMsg},
    state::MPC20StakingContractState,
    ContractError,
};

use mpc20_base::{
    actions::execute_init as mpc20_execute_init,
    msg::{Mpc20InitMsg, TransferFromMsg as Mpc20TransferFromMsg, TransferMsg as Mpc20TransferMsg},
    state::Minter as Mpc20Minter,
};
use utils::{decimal::DecimalRatio, events::IntoShortnameRPCEvent};

/// ## Description
/// Inits contract state.
/// Returns [`(MPC20StakingContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **_ctx** is an object of type [`ContractContext`]
///
/// * **msg** is an object of type [`Mpc20StakingInitMsg`]
pub fn execute_init(
    ctx: &ContractContext,
    msg: &Mpc20StakingInitMsg,
) -> (MPC20StakingContractState, Vec<EventGroup>) {
    msg.validate();

    let deposit_token = if let Some(token) = msg.deposit_token {
        token
    } else {
        ctx.contract_address
    };

    let last_distributed = ctx.block_production_time as u64;

    let minter = msg.minter.map(|minter_addr| Mpc20Minter {
        minter: minter_addr,
        capacity: None,
    });

    let (mpc20, _) = mpc20_execute_init(
        ctx,
        &Mpc20InitMsg {
            info: msg.info.clone(),
            initial_balances: msg.initial_balances.clone(),
            minter,
        },
    );

    let state = MPC20StakingContractState {
        deposit_token,
        distribution_amount: msg.distribution_amount,
        distribution_epoch: msg.distribution_epoch,
        global_index: DecimalRatio::zero(),
        total_staked: 0,
        last_distributed,
        stakers: BTreeMap::new(),
        compound_frequency: msg.compound_frequency,
        mpc20,
    };

    (state, vec![])
}

/// ## Description
/// Stake specified amount of tokens to earn rewards.
/// Returns [`(MPC20StakingContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC20StakingContractState`]
///
/// * **msg** is an object of type [`StakeMsg`]
pub fn execute_stake(
    ctx: &ContractContext,
    state: &mut MPC20StakingContractState,
    msg: &StakeMsg,
) -> Vec<EventGroup> {
    let mut staker = state.get_staker(&ctx.sender);

    state.distribute_rewards(ctx.block_production_time as u64);
    staker.compute_reward(state.global_index);
    state.increase_stake_amount(&ctx.sender, &mut staker, msg.amount);

    let mut event_group = EventGroup::builder();
    Mpc20TransferFromMsg {
        from: ctx.sender,
        to: ctx.contract_address,
        amount: msg.amount,
    }
    .as_interaction(&mut event_group, &state.deposit_token);

    vec![event_group.build()]
}

/// ## Description
/// Withdraw staked tokens.
/// Returns [`(MPC20StakingContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC20StakingContractState`]
///
/// * **msg** is an object of type [`UnstakeMsg`]
pub fn execute_unstake(
    ctx: &ContractContext,
    state: &mut MPC20StakingContractState,
    msg: &UnstakeMsg,
) -> Vec<EventGroup> {
    let mut staker = state.get_staker(&ctx.sender);

    assert!(
        staker.staked_amount >= msg.amount,
        "{}",
        ContractError::CannotUnstakeMoreThenStaked,
    );

    state.distribute_rewards(ctx.block_production_time as u64);
    staker.compute_reward(state.global_index);
    state.decrease_stake_amount(&ctx.sender, &mut staker, msg.amount);

    let mut event_group = EventGroup::builder();
    Mpc20TransferMsg {
        to: ctx.sender,
        amount: msg.amount,
    }
    .as_interaction(&mut event_group, &state.deposit_token);

    vec![event_group.build()]
}

/// ## Description
/// Claim earned rewards.
/// Returns [`(MPC20StakingContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC20StakingContractState`]
///
/// * **msg** is an object of type [`ClaimMsg`]
pub fn execute_claim(
    ctx: &ContractContext,
    state: &mut MPC20StakingContractState,
    msg: &ClaimMsg,
) -> Vec<EventGroup> {
    let mut staker = state.get_staker(&ctx.sender);

    state.distribute_rewards(ctx.block_production_time as u64);
    staker.compute_reward(state.global_index);

    assert!(
        !staker.pending_reward.is_zero(),
        "{}",
        ContractError::NothingToClaim
    );

    let claim_amount = if let Some(amount) = msg.amount {
        assert!(
            amount <= staker.pending_reward && !amount.is_zero(),
            "{}",
            ContractError::CannotClaimMoreThenRewarded
        );
        amount
    } else {
        staker.pending_reward
    };

    staker.pending_reward = staker.pending_reward.checked_sub(claim_amount).unwrap();
    state.store_staker(&ctx.sender, &staker);
    state.mpc20.mint_to(&ctx.sender, claim_amount);

    vec![]
}

/// ## Description
/// Compound earned rewards(e.g. stake them).
/// Only works when deposit token is reward token.
/// Returns [`(MPC20StakingContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC20StakingContractState`]
///
/// * **msg** is an object of type [`CompoundMsg`]
pub fn execute_compound(
    ctx: &ContractContext,
    state: &mut MPC20StakingContractState,
    msg: &CompoundMsg,
) -> Vec<EventGroup> {
    let mut staker = state.get_staker(&ctx.sender);

    state.distribute_rewards(ctx.block_production_time as u64);
    staker.compute_reward(state.global_index);

    assert!(
        state.deposit_token == ctx.contract_address,
        "{}",
        ContractError::CompoundOnlyWorksWithSelfToken
    );

    assert!(
        (staker.last_compound + state.compound_frequency) < (ctx.block_production_time as u64),
        "{}",
        ContractError::ForbiddenToCompoundToOften,
    );

    let compound_amount = if let Some(amount) = msg.amount {
        assert!(
            amount <= staker.pending_reward && !amount.is_zero(),
            "{}",
            ContractError::CannotCompoundMoreThenRewarded
        );
        amount
    } else {
        staker.pending_reward
    };

    staker.last_compound = ctx.block_production_time as u64;
    staker.pending_reward = staker.pending_reward.checked_sub(compound_amount).unwrap();
    state.increase_stake_amount(&ctx.sender, &mut staker, compound_amount);

    state.mpc20.mint_to(&ctx.contract_address, compound_amount);

    vec![]
}
