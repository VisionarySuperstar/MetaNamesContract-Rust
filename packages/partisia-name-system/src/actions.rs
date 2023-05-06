use contract_version_base::state::ContractVersionBase;
use pbc_contract_common::{context::ContractContext, events::EventGroup};

use mpc721_hierarchy::{actions as mpc721_actions, msg as mpc721_msg};

use crate::{
    msg::{
        ApproveForAllMsg, ApproveMsg, BurnMsg, InitMsg, MintMsg, RevokeForAllMsg,
        RevokeMsg, SetBaseUriMsg, TransferFromMsg, TransferMsg, UpdateMinterMsg, MultiMintMsg, CheckOwnerMsg,
    },
    state::PartisiaNameSystemState,
};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// ## Description
/// Inits contract state.
/// Returns [`(PartisiaNameSystemState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **_ctx** is an object of type [`ContractContext`]
///
/// * **msg** is an object of type [`InitMsg`]
pub fn execute_init(
    ctx: &ContractContext,
    msg: &InitMsg,
) -> (PartisiaNameSystemState, Vec<EventGroup>) {
    let mpc721_msg = mpc721_msg::InitMsg {
        owner: msg.owner,
        name: msg.name.clone(),
        symbol: msg.symbol.clone(),
        base_uri: msg.base_uri.clone(),
        minter: msg.minter,
    };

    let (mpc721, events) = mpc721_actions::execute_init(&ctx, &mpc721_msg);
    let state = PartisiaNameSystemState {
        mpc721,
        version: ContractVersionBase::new(CONTRACT_NAME, CONTRACT_VERSION),
    };

    (state, events)
}

/// ## Description
/// Transfer token to another account.
/// Returns [`Vec<EventGroup>`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`PartisiaNameSystemState`]
///
/// * **msg** is an object of type [`TransferMsg`]
pub fn execute_transfer(
    ctx: &ContractContext,
    state: &mut PartisiaNameSystemState,
    msg: &TransferMsg,
) -> Vec<EventGroup> {
    let events = mpc721_actions::execute_transfer(
        &ctx,
        &mut state.mpc721,
        &mpc721_msg::TransferMsg {
            to: msg.to,
            token_id: msg.token_id,
        },
    );

    events
}

/// ## Description
/// Only with approval extension. Transfer token from owner to spender.
/// Returns [`Vec<EventGroup>`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`PartisiaNameSystemState`]
///
/// * **msg** is an object of type [`TransferFromMsg`]
pub fn execute_transfer_from(
    ctx: &ContractContext,
    state: &mut PartisiaNameSystemState,
    msg: &TransferFromMsg,
) -> Vec<EventGroup> {
    let events = mpc721_actions::execute_transfer_from(
        &ctx,
        &mut state.mpc721,
        &mpc721_msg::TransferFromMsg {
            from: msg.from,
            to: msg.to,
            token_id: msg.token_id,
        },
    );

    events
}

/// ## Description
/// Allows spender to transfer token from the owner account.
/// Returns [`Vec<EventGroup>`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`PartisiaNameSystemState`]
///
/// * **msg** is an object of type [`ApproveMsg`]
pub fn execute_approve(
    ctx: &ContractContext,
    state: &mut PartisiaNameSystemState,
    msg: &ApproveMsg,
) -> Vec<EventGroup> {
    let events = mpc721_actions::execute_approve(
        &ctx,
        &mut state.mpc721,
        &mpc721_msg::ApproveMsg {
            spender: msg.spender,
            token_id: msg.token_id,
        },
    );

    events
}

/// ## Description
/// Set base uri for the tokens.
/// Returns [`Vec<EventGroup>`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`PartisiaNameSystemState`]
///
/// * **msg** is an object of type [`SetBaseUriMsg`]
pub fn execute_set_base_uri(
    ctx: &ContractContext,
    state: &mut PartisiaNameSystemState,
    msg: &SetBaseUriMsg,
) -> Vec<EventGroup> {
    let events = mpc721_actions::execute_set_base_uri(
        &ctx,
        &mut state.mpc721,
        &mpc721_msg::SetBaseUriMsg {
            new_base_uri: msg.new_base_uri.clone(),
        },
    );

    events
}

/// ## Description
/// Mint a new token. Can only be executed by minter account.
/// Returns [`Vec<EventGroup>`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`PartisiaNameSystemState`]
///
/// * **msg** is an object of type [`MintMsg`]
pub fn execute_mint(
    ctx: &ContractContext,
    state: &mut PartisiaNameSystemState,
    msg: &MintMsg,
) -> Vec<EventGroup> {
    let mut events = mpc721_actions::execute_mint(
        &ctx,
        &mut state.mpc721,
        &mpc721_msg::MintMsg {
            token_id: msg.token_id,
            to: msg.to,
            token_uri: msg.token_uri.clone(),
        },
    );

    let mut update_parent_events: Vec<EventGroup> = vec![];
    if let Some(parent) = msg.parent {
        // TODO: Do not mint if parent does not belong to the owner
        update_parent_events = mpc721_actions::execute_update_parent(
            &ctx,
            &mut state.mpc721,
            &mpc721_msg::UpdateParentMsg {
                token_id: msg.token_id,
                parent_id: Some(parent),
            },
        );
    }

    events.extend(update_parent_events);
    events
}

/// ## Description
/// Allows operator to transfer any owner tokens from his account.
/// Returns [`Vec<EventGroup>`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`PartisiaNameSystemState`]
///
/// * **msg** is an object of type [`ApproveForAllMsg`]
pub fn execute_approve_for_all(
    ctx: &ContractContext,
    state: &mut PartisiaNameSystemState,
    msg: &ApproveForAllMsg,
) -> Vec<EventGroup> {
    let events = mpc721_actions::execute_approve_for_all(
        &ctx,
        &mut state.mpc721,
        &mpc721_msg::ApproveForAllMsg {
            operator: msg.operator,
        },
    );

    events
}

/// ## Description
/// Remove approval.
/// Returns [`Vec<EventGroup>`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`PartisiaNameSystemState`]
///
/// * **msg** is an object of type [`RevokeMsg`]
pub fn execute_revoke(
    ctx: &ContractContext,
    state: &mut PartisiaNameSystemState,
    msg: &RevokeMsg,
) -> Vec<EventGroup> {
    let events = mpc721_actions::execute_revoke(
        &ctx,
        &mut state.mpc721,
        &mpc721_msg::RevokeMsg {
            spender: msg.spender,
            token_id: msg.token_id,
        },
    );

    events
}

/// ## Description
/// Remove operator.
/// Returns [`Vec<EventGroup>`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`PartisiaNameSystemState`]
///
/// * **msg** is an object of type [`RevokeForAllMsg`]
pub fn execute_revoke_for_all(
    ctx: &ContractContext,
    state: &mut PartisiaNameSystemState,
    msg: &RevokeForAllMsg,
) -> Vec<EventGroup> {
    let events = mpc721_actions::execute_revoke_for_all(
        &ctx,
        &mut state.mpc721,
        &mpc721_msg::RevokeForAllMsg {
            operator: msg.operator,
        },
    );

    events
}

/// ## Description
/// Destroy your token forever.
/// Returns [`Vec<EventGroup>`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`PartisiaNameSystemState`]
///
/// * **msg** is an object of type [`BurnMsg`]
pub fn execute_burn(
    ctx: &ContractContext,
    state: &mut PartisiaNameSystemState,
    msg: &BurnMsg,
) -> Vec<EventGroup> {
    let events = mpc721_actions::execute_burn(
        &ctx,
        &mut state.mpc721,
        &mpc721_msg::BurnMsg {
            token_id: msg.token_id,
        },
    );

    events
}

/// ## Description
/// Updates the minter address checking that the sender is the contract owner address
/// Returns [`Vec<EventGroup>`] if operation was successful,
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`PartisiaNameSystemState`]
///
/// * **msg** is an object of type [`UpdateMinterMsg`]
pub fn execute_update_minter(
    ctx: &ContractContext,
    state: &mut PartisiaNameSystemState,
    msg: UpdateMinterMsg,
) -> Vec<EventGroup> {
    let events = mpc721_actions::execute_update_minter(
        &ctx,
        &mut state.mpc721,
        mpc721_msg::UpdateMinterMsg {
            new_minter: msg.new_minter,
        },
    );

    events
}

/// ## Description
/// Check if a user owns a particular token. Will revert otherwise
/// Returns [`Vec<EventGroup>`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`PartisiaNameSystemState`]
///
/// * **msg** is an object of type [`CheckOwnerMsg`]
pub fn execute_ownership_check(
    ctx: &ContractContext,
    state: &mut PartisiaNameSystemState,
    msg: &CheckOwnerMsg,
) -> Vec<EventGroup> {
    let events = mpc721_actions::execute_ownership_check(
        &ctx,
        &mut state.mpc721,
        &mpc721_msg::CheckOwnerMsg {
            token_id: msg.token_id,
            owner: msg.owner,
        },
    );

    events
}


/// ## Description
/// Mint Multiple NFTs in a single function call
/// Returns [`Vec<EventGroup>`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`PartisiaNameSystemState`]
///
/// * **msg** is an object of type [`MultiMintMsg`]
pub fn execute_multi_mint(
    ctx: &ContractContext,
    state: &mut PartisiaNameSystemState,
    msg: &MultiMintMsg,
) -> Vec<EventGroup> {
    let mut events: Vec<EventGroup> = vec![];
    for mint in msg.mints.iter() {
        let event = execute_mint(ctx, state, mint);

        events.extend(event)
    }

    events
}

// TODO: add mint record
// TODO: add update record
// TODO: add delete record
