use pbc_contract_common::{context::ContractContext, events::EventGroup};

use crate::{
    msg::{
        ApproveMsg, BurnFromMsg, BurnMsg, DecreaseAllowanceMsg, IncreaseAllowanceMsg, MintMsg,
        Mpc20InitMsg, TransferFromMsg, TransferMsg,
    },
    state::MPC20ContractState,
    ContractError,
};

/// ## Description
/// Inits contract state.
/// Returns [`(MPC20ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **_ctx** is an object of type [`ContractContext`]
///
/// * **msg** is an object of type [`Mpc20InitMsg`]
pub fn execute_init(
    _ctx: &ContractContext,
    msg: &Mpc20InitMsg,
) -> (MPC20ContractState, Vec<EventGroup>) {
    msg.validate();

    let mut state = MPC20ContractState::new(&msg.info, &msg.minter);

    let total_supply = state.init_balances(&msg.initial_balances);
    if let Some(limit) = msg.capacity() {
        assert!(
            total_supply <= limit,
            "Initial supply is greater than capacity"
        );
    }

    (state, vec![])
}

/// ## Description
/// Mint specified amount of tokens to provided address.
/// Only works when minter option is enabled.
/// Returns [`(MPC20ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC20ContractState`]
///
/// * **msg** is an object of type [`MintMsg`]
pub fn execute_mint(
    ctx: &ContractContext,
    state: &mut MPC20ContractState,
    msg: &MintMsg,
) -> Vec<EventGroup> {
    assert!(
        msg.amount > 0,
        "{}",
        ContractError::AmountMustBeHigherThenZero,
    );
    assert!(
        state.minter.is_some(),
        "{}",
        ContractError::MintingIsDisabled
    );
    assert!(
        state.minter.as_ref().unwrap().minter == ctx.sender,
        "{}",
        ContractError::Unauthorized
    );

    state.mint_to(&msg.recipient, msg.amount);
    vec![]
}

/// ## Description
/// Moves amount tokens from the msg sender account to specified `to` account.
/// Returns [`(MPC20ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC20ContractState`]
///
/// * **msg** is an object of type [`TransferMsg`]
pub fn execute_transfer(
    ctx: &ContractContext,
    state: &mut MPC20ContractState,
    msg: &TransferMsg,
) -> Vec<EventGroup> {
    assert!(
        msg.amount > 0,
        "{}",
        ContractError::AmountMustBeHigherThenZero,
    );

    state.decrease_balance(&ctx.sender, msg.amount);
    state.increase_balance(&msg.to, msg.amount);

    vec![]
}

/// ## Description
/// Only with approval extension.
/// Transfers amount tokens from owner -> recipient if sender has sufficient pre-approval.
/// Returns [`(MPC20ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **_ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC20ContractState`]
///
/// * **msg** is an object of type [TransferFromMsg``]
pub fn execute_transfer_from(
    _ctx: &ContractContext,
    state: &mut MPC20ContractState,
    msg: &TransferFromMsg,
) -> Vec<EventGroup> {
    assert!(
        msg.amount > 0,
        "{}",
        ContractError::AmountMustBeHigherThenZero,
    );

    state.decrease_allowance(&msg.from, &msg.to, msg.amount);
    state.decrease_balance(&msg.from, msg.amount);
    state.increase_balance(&msg.to, msg.amount);

    vec![]
}

/// ## Description
/// Burn is a method to destroy your tokens forever.
/// Returns [`(MPC20ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC20ContractState`]
///
/// * **msg** is an object of type [`BurnMsg`]
pub fn execute_burn(
    ctx: &ContractContext,
    state: &mut MPC20ContractState,
    msg: &BurnMsg,
) -> Vec<EventGroup> {
    assert!(
        msg.amount > 0,
        "{}",
        ContractError::AmountMustBeHigherThenZero,
    );

    state.decrease_balance(&ctx.sender, msg.amount);
    state.decrease_total_supply(msg.amount);

    vec![]
}

/// ## Description
/// Only with approval extension. Destroys your tokens forever.
/// Returns [`(MPC20ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC20ContractState`]
///
/// * **msg** is an object of type [`BurnFromMsg`]
pub fn execute_burn_from(
    ctx: &ContractContext,
    state: &mut MPC20ContractState,
    msg: &BurnFromMsg,
) -> Vec<EventGroup> {
    assert!(
        msg.amount > 0,
        "{}",
        ContractError::AmountMustBeHigherThenZero,
    );

    state.decrease_allowance(&msg.owner, &ctx.sender, msg.amount);
    state.decrease_balance(&msg.owner, msg.amount);
    state.decrease_total_supply(msg.amount);

    vec![]
}

/// ## Description
/// Sets amount as the allowance of spender over the caller's tokens.
/// Returns [`(MPC20ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC20ContractState`]
///
/// * **msg** is an object of type [`ApproveMsg`]
pub fn execute_approve(
    ctx: &ContractContext,
    state: &mut MPC20ContractState,
    msg: &ApproveMsg,
) -> Vec<EventGroup> {
    assert!(
        ctx.sender != msg.spender,
        "{}",
        ContractError::CannotApproveToYourself
    );

    assert!(
        msg.amount > 0,
        "{}",
        ContractError::AmountMustBeHigherThenZero,
    );

    state.set_allowance(&ctx.sender, &msg.spender, msg.amount);
    vec![]
}

/// ## Description
/// Allows spender to access an additional amount tokens from the owner's account.
/// Returns [`(MPC20ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC20ContractState`]
///
/// * **msg** is an object of type [`IncreaseAllowanceMsg`]
pub fn execute_increase_allowance(
    ctx: &ContractContext,
    state: &mut MPC20ContractState,
    msg: &IncreaseAllowanceMsg,
) -> Vec<EventGroup> {
    assert!(
        ctx.sender != msg.spender,
        "{}",
        ContractError::CannotApproveToYourself
    );

    assert!(
        msg.amount > 0,
        "{}",
        ContractError::AmountMustBeHigherThenZero,
    );

    state.increase_allowance(&ctx.sender, &msg.spender, msg.amount);
    vec![]
}

/// ## Description
/// Lowers the spender's access of tokens from the owner's account by amount.
/// Returns [`(MPC20ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC20ContractState`]
///
/// * **msg** is an object of type [`DecreaseAllowanceMsg`]
pub fn execute_decrease_allowance(
    ctx: &ContractContext,
    state: &mut MPC20ContractState,
    msg: &DecreaseAllowanceMsg,
) -> Vec<EventGroup> {
    assert!(
        ctx.sender != msg.spender,
        "{}",
        ContractError::CannotApproveToYourself
    );

    assert!(
        msg.amount > 0,
        "{}",
        ContractError::AmountMustBeHigherThenZero,
    );

    state.decrease_allowance(&ctx.sender, &msg.spender, msg.amount);
    vec![]
}
