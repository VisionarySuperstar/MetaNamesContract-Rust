use std::vec;

use pbc_contract_common::{
    context::ContractContext, events::EventGroup, sorted_vec_map::SortedVecMap,
};

use crate::{
    msg::{ApproveForAllMsg, ApproveMsg, BurnMsg, MintMsg, NFTInitMsg, TransferFromMsg},
    state::{MPC721ContractState, OperatorApproval, URL_LENGTH},
    ContractError,
};

/// ## Description
/// Inits contract state.
/// Returns [`(MPC721ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
pub fn execute_init(ctx: &ContractContext, msg: &NFTInitMsg) -> MPC721ContractState {
    MPC721ContractState {
        name: msg.name.clone(),
        symbol: msg.symbol.clone(),
        supply: 0,
        operator_approvals: vec![],
        owners: SortedVecMap::new(),
        token_approvals: SortedVecMap::new(),
        uri_template: msg.uri_template.clone(),
        token_uri_details: SortedVecMap::new(),
    }
}

/// ## Description
/// Mint a new token. Can only be executed by minter account.
/// Returns [`(MPC721ContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
pub fn execute_mint(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: &MintMsg,
) -> Vec<EventGroup> {
    assert!(!state.exists(msg.token_id), "{}", ContractError::Minted);

    state.owners.insert(msg.token_id, ctx.sender);
    if let Some(token_uri) = msg.token_uri.clone() {
        let formatted_uri = format!("{}{}", state.uri_template, token_uri).into_bytes();
        assert!(
            formatted_uri.len() <= URL_LENGTH,
            "{}",
            ContractError::UriTooLong
        );

        let array: [u8; 128] = formatted_uri.try_into().unwrap();
        state.token_uri_details.insert(msg.token_id, array);
    }

    state.increase_supply();

    vec![]
}

/// Change or reaffirm the approved address for an NFT.
/// None indicates there is no approved address.
/// Throws unless `ctx.sender` is the current NFT owner, or an authorized
/// operator of the current owner.
pub fn execute_approve(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: &ApproveMsg,
) -> Vec<EventGroup> {
    let owner = state.owner_of(msg.token_id);
    assert!(
        ctx.sender == owner || state.is_approved_for_all(owner, ctx.sender),
        "{}",
        ContractError::Unauthorized
    );
    state._approve(msg.approved, msg.token_id);

    vec![]
}

/// Enable or disable approval for a third party ("operator") to manage all of
/// `ctx.sender`'s assets.
/// Throws if `operator` == `ctx.sender`.
pub fn execute_set_approval_for_all(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: &ApproveForAllMsg,
) -> Vec<EventGroup> {
    assert!(
        msg.operator != ctx.sender,
        "{}",
        ContractError::Unauthorized
    );

    if msg.approved {
        let already_present = state
            .operator_approvals
            .clone()
            .into_iter()
            .any(|approval| approval.owner == ctx.sender && approval.operator == msg.operator);
        if !already_present {
            state.operator_approvals.push(OperatorApproval {
                owner: ctx.sender,
                operator: msg.operator,
            });
        }
    } else {
        state.operator_approvals.retain(|approval| {
            !(approval.owner == ctx.sender && approval.operator == msg.operator)
        });
    }

    vec![]
}

/// Transfer ownership of an NFT -- THE CALLER IS RESPONSIBLE
/// TO CONFIRM THAT `to` IS CAPABLE OF RECEIVING NFTS OR ELSE
/// THEY MAY BE PERMANENTLY LOST
///
/// Throws unless `ctx.sender` is the current owner, an authorized
/// operator, or the approved address for this NFT. Throws if `from` is
/// not the current owner. Throws if `token_id` is not a valid NFT.
pub fn execute_transfer_from(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: &TransferFromMsg,
) -> Vec<EventGroup> {
    assert!(
        state.is_approved_or_owner(ctx.sender, msg.token_id),
        "{}",
        ContractError::Unauthorized
    );

    state._transfer(msg.from, msg.to, msg.token_id);

    vec![]
}

/// Destroys `token_id`.
/// The approval is cleared when the token is burned.
/// Requires that the `token_id` exists and `ctx.sender` is approved or owner of the token.
pub fn burn(
    ctx: &ContractContext,
    state: &mut MPC721ContractState,
    msg: &BurnMsg,
) -> Vec<EventGroup> {
    let token_id = msg.token_id;
    assert!(
        state.is_approved_or_owner(ctx.sender, token_id),
        "{}",
        ContractError::Unauthorized
    );

    let owner = state.owner_of(token_id);
    // Clear approvals
    state._approve(None, token_id);

    state.owners.remove(&token_id);
    state.token_uri_details.remove(&token_id);
    state.decrease_supply();

    vec![]
}
