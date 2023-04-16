use std::collections::BTreeMap;

use pbc_contract_common::{context::ContractContext, events::EventGroup};

use crate::{
    msg::{
        ApproveForAllMsg, ApproveMsg, BurnMsg, CheckOwnerMsg, InitMsg, MintMsg, MultiMintMsg,
        RevokeForAllMsg, RevokeMsg, SetBaseUriMsg, TransferFromMsg, TransferMsg, UpdateMinterMsg,
    },
    state::MPC721ContractState,
    ContractError,
};

/// ## Description
/// Inits contract state.
/// Returns [`(MPC721ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **_ctx** is an object of type [`ContractContext`]
///
/// * **msg** is an object of type [`InitMsg`]
pub fn execute_init(
    _ctx: &ContractContext,
    msg: &InitMsg,
) -> (MPC721ContractState, Vec<EventGroup>) {
    let state = MPC721ContractState {
        owner: msg.owner,
        name: msg.name.clone(),
        symbol: msg.symbol.clone(),
        base_uri: msg.base_uri.clone(),
        minter: msg.minter,
        supply: 0,
        tokens: BTreeMap::new(),
        operator_approvals: BTreeMap::new(),
    };

    (state, vec![])
}

/// ## Description
/// Set base uri for the tokens.
/// Returns [`(MPC721ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC721ContractState`]
///
/// * **msg** is an object of type [`SetBaseUriMsg`]
pub fn execute_set_base_uri(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: &SetBaseUriMsg,
) -> Vec<EventGroup> {
    assert!(
        state.is_owner(&ctx.sender),
        "{}",
        ContractError::Unauthorized
    );

    state.set_base_uri(&msg.new_base_uri);
    vec![]
}

/// ## Description
/// Mint a new token. Can only be executed by minter account.
/// Returns [`(MPC721ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC721ContractState`]
///
/// * **msg** is an object of type [`MintMsg`]
pub fn execute_mint(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: &MintMsg,
) -> Vec<EventGroup> {
    assert!(
        state.minter == ctx.sender,
        "{}",
        ContractError::Unauthorized
    );

    assert!(!state.is_minted(msg.token_id), "{}", ContractError::Minted);

    state.mint(msg.token_id, &msg.to, &msg.token_uri);
    state.increase_supply();

    vec![]
}
/// ## Description
/// Updates the minter address checking that the sender is the contract owner address
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC721ContractState`]
///
/// * **msg** is an object of type [`UpdateMinterMsg`]
pub fn execute_update_minter(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: UpdateMinterMsg,
) -> Vec<EventGroup> {
    assert!(
        state.owner.is_some() && state.owner.unwrap() == ctx.sender,
        "{}",
        ContractError::Unauthorized
    );

    state.minter = msg.new_minter;

    vec![]
}

/// ## Description
/// Transfer token to another account.
/// Returns [`(MPC721ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC721ContractState`]
///
/// * **msg** is an object of type [`TransferMsg`]
pub fn execute_transfer(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: &TransferMsg,
) -> Vec<EventGroup> {
    assert!(state.is_minted(msg.token_id), "{}", ContractError::NotFound);

    state.transfer(&ctx.sender, &msg.to, msg.token_id);
    vec![]
}

/// ## Description
/// Only with approval extension. Transfer token from owner to spender.
/// Returns [`(MPC721ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC721ContractState`]
///
/// * **msg** is an object of type [`TransferFromMsg`]
pub fn execute_transfer_from(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: &TransferFromMsg,
) -> Vec<EventGroup> {
    assert!(state.is_minted(msg.token_id), "{}", ContractError::NotFound);

    state.transfer(&msg.from, &msg.to, msg.token_id);
    vec![]
}

/// ## Description
/// Allows spender to transfer token from the owner account.
/// Returns [`(MPC721ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC721ContractState`]
///
/// * **msg** is an object of type [`ApproveMsg`]
pub fn execute_approve(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: &ApproveMsg,
) -> Vec<EventGroup> {
    assert!(state.is_minted(msg.token_id), "{}", ContractError::NotFound);

    state.update_approvals(&ctx.sender, &msg.spender, msg.token_id, true);
    vec![]
}

/// ## Description
/// Allows operator to transfer any owner tokens from his account.
/// Returns [`(MPC721ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC721ContractState`]
///
/// * **msg** is an object of type [`ApproveForAllMsg`]
pub fn execute_approve_for_all(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: &ApproveForAllMsg,
) -> Vec<EventGroup> {
    state.add_operator(&ctx.sender, &msg.operator);
    vec![]
}

/// ## Description
/// Remove approval.
/// Returns [`(MPC721ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC721ContractState`]
///
/// * **msg** is an object of type [`RevokeMsg`]
pub fn execute_revoke(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: &RevokeMsg,
) -> Vec<EventGroup> {
    assert!(state.is_minted(msg.token_id), "{}", ContractError::NotFound);

    state.update_approvals(&ctx.sender, &msg.spender, msg.token_id, false);
    vec![]
}

/// ## Description
/// Remove operator.
/// Returns [`(MPC721ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC721ContractState`]
///
/// * **msg** is an object of type [`RevokeForAllMsg`]
pub fn execute_revoke_for_all(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: &RevokeForAllMsg,
) -> Vec<EventGroup> {
    state.remove_operator(&ctx.sender, &msg.operator);
    vec![]
}

/// ## Description
/// Destroy your token forever.
/// Returns [`(MPC721ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC721ContractState`]
///
/// * **msg** is an object of type [`BurnMsg`]
pub fn execute_burn(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: &BurnMsg,
) -> Vec<EventGroup> {
    assert!(state.is_minted(msg.token_id), "{}", ContractError::NotFound);

    state.remove_token(&ctx.sender, msg.token_id);
    state.decrease_supply();

    vec![]
}

/// ## Description
/// Check if a user owns a particular token. Will revert otherwise
/// Returns [`(MPC721ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC721ContractState`]
///
/// * **msg** is an object of type [`CheckOwnerMsg`]
pub fn execute_ownership_check(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: &CheckOwnerMsg,
) -> Vec<EventGroup> {
    let token_info = state.token_info(msg.token_id);
    match token_info {
        Some(token_info) => assert!(
            token_info.owner == msg.owner,
            "{}",
            ContractError::IncorrectOwner
        ),
        None => panic!("{}", ContractError::NotFound),
    };
    vec![]
}

/// ## Description
/// Mint Multiple NFTs in a single function call
/// Returns [` Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC721ContractState`]
///
/// * **msg** is an object of type [`MultiMintMsg`]
pub fn execute_multi_mint(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: &MultiMintMsg,
) -> Vec<EventGroup> {
    for mint in msg.mints.iter() {
        execute_mint(ctx, state, mint);
    }

    vec![]
}
