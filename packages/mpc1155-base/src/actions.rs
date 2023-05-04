use std::collections::BTreeMap;

use pbc_contract_common::{context::ContractContext, events::EventGroup};

use crate::{
    msg::{
        ApproveForAllMsg, BatchBurnMsg, BatchMintMsg, BatchTransferFromMsg, BurnMsg,
        CheckBalancesMsg, InitMsg, MintMsg, RevokeForAllMsg, SetUriMsg, TransferFromMsg,
    },
    state::{MPC1155ContractState, TokenInfo},
    ContractError,
};

/// ## Description
/// Inits contract state.
/// Returns [`(MPC1155ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **_ctx** is an object of type [`ContractContext`]
///
/// * **msg** is an object of type [`InitMsg`]
pub fn execute_init(
    _ctx: &ContractContext,
    msg: &InitMsg,
) -> (MPC1155ContractState, Vec<EventGroup>) {
    let state = MPC1155ContractState {
        owner: msg.owner,
        uri: msg.uri.clone(),
        minter: msg.minter,
        balances: BTreeMap::new(),
        operator_approvals: BTreeMap::new(),
        tokens: BTreeMap::new(),
    };

    (state, vec![])
}

/// ## Description
/// Set uri for the tokens.
/// Returns [`(MPC1155ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC1155ContractState`]
///
/// * **msg** is an object of type [`SetUriMsg`]
pub fn execute_set_uri(
    ctx: &ContractContext,
    state: &mut MPC1155ContractState,
    msg: &SetUriMsg,
) -> Vec<EventGroup> {
    assert!(
        state.is_owner(&ctx.sender),
        "{}",
        ContractError::Unauthorized
    );

    state.set_uri(&msg.new_uri);
    vec![]
}

/// ## Description
/// Mint a new token. Can only be executed by minter account.
/// Returns [`(MPC1155ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC1155ContractState`]
///
/// * **msg** is an object of type [`MintMsg`]
pub fn execute_mint(
    ctx: &ContractContext,
    state: &mut MPC1155ContractState,
    msg: &MintMsg,
) -> Vec<EventGroup> {
    assert!(
        state.minter == ctx.sender,
        "{}",
        ContractError::Unauthorized
    );

    state.store_token(
        msg.token_info.token_id,
        &TokenInfo {
            token_uri: msg.token_info.token_uri.clone(),
        },
    );
    state.transfer(
        None,
        Some(&msg.to),
        msg.token_info.token_id,
        msg.token_info.amount,
    );

    vec![]
}

/// ## Description
/// Batch mint a new token. Can only be executed by minter account.
/// Returns [`(MPC1155ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC1155ContractState`]
///
/// * **msg** is an object of type [`BatchMintMsg`]
pub fn execute_batch_mint(
    ctx: &ContractContext,
    state: &mut MPC1155ContractState,
    msg: &BatchMintMsg,
) -> Vec<EventGroup> {
    assert!(
        state.minter == ctx.sender,
        "{}",
        ContractError::Unauthorized
    );

    for token_info in msg.token_infos.iter() {
        state.store_token(
            token_info.token_id,
            &TokenInfo {
                token_uri: token_info.token_uri.clone(),
            },
        );
        state.transfer(None, Some(&msg.to), token_info.token_id, token_info.amount);
    }

    vec![]
}

/// ## Description
/// Only with approval extension. Transfer token from owner to spender.
/// Returns [`(MPC1155ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC1155ContractState`]
///
/// * **msg** is an object of type [`TransferFromMsg`]
pub fn execute_transfer_from(
    ctx: &ContractContext,
    state: &mut MPC1155ContractState,
    msg: &TransferFromMsg,
) -> Vec<EventGroup> {
    assert!(
        state.is_token_owner_or_operator(&msg.from, &ctx.sender),
        "{}",
        ContractError::Unauthorized,
    );

    state.transfer(
        Some(&msg.from),
        Some(&msg.to),
        msg.token_info.token_id,
        msg.token_info.amount,
    );

    vec![]
}

/// ## Description
/// Only with approval extension. Batch transfer token from owner to spender.
/// Returns [`(MPC1155ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC1155ContractState`]
///
/// * **msg** is an object of type [`BatchTransferFromMsg`]
pub fn execute_batch_transfer_from(
    ctx: &ContractContext,
    state: &mut MPC1155ContractState,
    msg: &BatchTransferFromMsg,
) -> Vec<EventGroup> {
    assert!(
        state.is_token_owner_or_operator(&msg.from, &ctx.sender),
        "{}",
        ContractError::Unauthorized,
    );

    for token_info in msg.token_infos.iter() {
        state.transfer(
            Some(&msg.from),
            Some(&msg.to),
            token_info.token_id,
            token_info.amount,
        );
    }

    vec![]
}

/// ## Description
/// Destroy your token forever.
/// Returns [`(MPC1155ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC1155ContractState`]
///
/// * **msg** is an object of type [`BurnMsg`]
pub fn execute_burn(
    ctx: &ContractContext,
    state: &mut MPC1155ContractState,
    msg: &BurnMsg,
) -> Vec<EventGroup> {
    assert!(
        state.is_token_owner_or_operator(&msg.from, &ctx.sender),
        "{}",
        ContractError::Unauthorized,
    );

    state.transfer(
        Some(&msg.from),
        None,
        msg.token_info.token_id,
        msg.token_info.amount,
    );

    vec![]
}

/// ## Description
/// Batch destroy your token forever.
/// Returns [`(MPC1155ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC1155ContractState`]
///
/// * **msg** is an object of type [`BatchBurnMsg`]
pub fn execute_batch_burn(
    ctx: &ContractContext,
    state: &mut MPC1155ContractState,
    msg: &BatchBurnMsg,
) -> Vec<EventGroup> {
    assert!(
        state.is_token_owner_or_operator(&msg.from, &ctx.sender),
        "{}",
        ContractError::Unauthorized,
    );

    for token_info in msg.token_infos.iter() {
        state.transfer(
            Some(&msg.from),
            None,
            token_info.token_id,
            token_info.amount,
        );
    }

    vec![]
}

/// ## Description
/// Allows operator to transfer any owner tokens from his account.
/// Returns [`(MPC1155ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC1155ContractState`]
///
/// * **msg** is an object of type [`ApproveForAllMsg`]
pub fn execute_approve_for_all(
    ctx: &ContractContext,
    state: &mut MPC1155ContractState,
    msg: &ApproveForAllMsg,
) -> Vec<EventGroup> {
    state.add_operator(&ctx.sender, &msg.operator);
    vec![]
}

/// ## Description
/// Remove operator.
/// Returns [`(MPC1155ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC1155ContractState`]
///
/// * **msg** is an object of type [`RevokeForAllMsg`]
pub fn execute_revoke_for_all(
    ctx: &ContractContext,
    state: &mut MPC1155ContractState,
    msg: &RevokeForAllMsg,
) -> Vec<EventGroup> {
    state.remove_operator(&ctx.sender, &msg.operator);
    vec![]
}
/// ## Description
/// Assert that a user owns a given amount of nfts of a different id.
/// Returns [ Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`InadequateBalance`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC1155ContractState`]
///
/// * **msg** is an object of type [`CheckBalancesMsg`]
pub fn execute_check_balances(
    ctx: &ContractContext,
    state: &mut MPC1155ContractState,
    msg: CheckBalancesMsg,
) -> Vec<EventGroup> {
    state.check_balances(msg.owner, msg.token_ids, msg.amounts);
    vec![]
}
