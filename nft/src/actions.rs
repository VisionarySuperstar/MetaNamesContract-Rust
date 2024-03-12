use pbc_contract_common::{avl_tree_map::AvlTreeMap, context::ContractContext, events::EventGroup};

use crate::{
    msg::{
        NFTApproveForAllMsg, NFTApproveMsg, NFTBurnMsg, NFTInitMsg, NFTMintMsg, NFTTransferFromMsg,
    },
    state::{NFTContractState, OperatorApproval, Unit},
    ContractError,
};

/// Inits contract state.
/// Returns [`NFTContractState`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
pub fn execute_init(ctx: &ContractContext, msg: &NFTInitMsg) -> NFTContractState {
    NFTContractState {
        contract_owner: Some(ctx.sender),
        name: msg.name.clone(),
        symbol: msg.symbol.clone(),
        supply: 0,
        operator_approvals: AvlTreeMap::new(),
        owners: AvlTreeMap::new(),
        token_approvals: AvlTreeMap::new(),
        owners_inventory: AvlTreeMap::new(),
        uri_template: msg.uri_template.clone(),
        token_uri_details: AvlTreeMap::new(),
    }
}

/// Mint a new token. Can only be executed by minter account.
/// Returns [`NFTContractState`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
pub fn execute_mint(
    ctx: &ContractContext,
    state: &mut NFTContractState,
    msg: &NFTMintMsg,
) -> Vec<EventGroup> {
    assert!(!state.exists(msg.token_id), "{}", ContractError::Minted);

    state.owners.insert(msg.token_id, msg.to);
    state._owner_inventory_add(msg.to, msg.token_id);

    if let Some(token_uri) = msg.token_uri.clone() {
        state.token_uri_details.insert(msg.token_id, token_uri);
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
    state: &mut NFTContractState,
    msg: &NFTApproveMsg,
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
    state: &mut NFTContractState,
    msg: &NFTApproveForAllMsg,
) -> Vec<EventGroup> {
    assert!(
        msg.operator != ctx.sender,
        "{}",
        ContractError::Unauthorized
    );

    let operator_approval = OperatorApproval {
        owner: ctx.sender,
        operator: msg.operator,
    };

    if msg.approved {
        state.operator_approvals.insert(operator_approval, Unit {});
    } else {
        state.operator_approvals.remove(&operator_approval)
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
    state: &mut NFTContractState,
    msg: &NFTTransferFromMsg,
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
pub fn execute_burn(
    ctx: &ContractContext,
    state: &mut NFTContractState,
    msg: &NFTBurnMsg,
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
    state._owner_inventory_remove(owner, token_id);
    state.token_uri_details.remove(&token_id);
    state.decrease_supply();

    vec![]
}
