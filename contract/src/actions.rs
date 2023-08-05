use nft::{actions as nft_actions, msg as nft_msg};
use partisia_name_system::{actions as pns_actions, msg as pns_msg};
use pbc_contract_common::{address::Address, context::ContractContext, events::EventGroup};

use crate::{
    msg::{MPC20TransferFromMsg, MintMsg},
    state::{ContractState, PayableMintInfo},
    ContractError,
};
use utils::events::{build_msg_callback, IntoShortnameRPCEvent};

/// Action to mint contract
pub fn action_mint(
    ctx: ContractContext,
    state: ContractState,
    domain: String,
    to: Address,
    token_uri: Option<String>,
    parent_id: Option<String>,
) -> (ContractState, Vec<EventGroup>) {
    assert!(!state.pns.is_minted(&domain), "{}", ContractError::Minted);

    pns_actions::validate_domain(&domain);

    // Parent validations
    if let Some(parent_id) = parent_id.clone() {
        let parent = state.pns.get_domain(&parent_id);
        assert!(parent.is_some(), "{}", ContractError::DomainNotMinted);

        pns_actions::validate_domain_with_parent(&domain, &parent_id);

        let parent_token_id = parent.unwrap().token_id;
        assert!(
            state.nft.is_approved_or_owner(ctx.sender, parent_token_id),
            "{}",
            ContractError::Unauthorized
        );
    }

    let mut state = state;
    let token_id = state.nft.get_next_token_id();
    let nft_events = nft_actions::execute_mint(
        &ctx,
        &mut state.nft,
        &nft_msg::NFTMintMsg {
            to,
            token_id,
            token_uri,
        },
    );

    let pns_events = pns_actions::execute_mint(
        &ctx,
        &mut state.pns,
        &pns_msg::PnsMintMsg {
            domain,
            parent_id,
            token_id,
        },
    );

    let events = nft_events
        .into_iter()
        .chain(pns_events.into_iter())
        .collect();

    (state, events)
}

pub fn action_build_mint_callback(
    ctx: ContractContext,
    payable_mint_info: PayableMintInfo,
    mint_msg: &MintMsg,
    callback_byte: u32,
) -> Vec<EventGroup> {
    let mut payout_transfer_events = EventGroup::builder();

    MPC20TransferFromMsg {
        from: mint_msg.to,
        to: payable_mint_info.receiver.unwrap(),
        amount: calculate_mint_fees(mint_msg.domain.as_str()),
    }
    .as_interaction(
        &mut payout_transfer_events,
        &payable_mint_info.token.unwrap(),
    );

    build_msg_callback(&mut payout_transfer_events, callback_byte, mint_msg);

    vec![payout_transfer_events.build()]
}

pub fn calculate_mint_fees(domain_name: &str) -> u128 {
    let length = domain_name.len();
    match length {
        1 => 200,
        2 => 150,
        3 => 100,
        4 => 50,
        _ => 5,
    }
}
