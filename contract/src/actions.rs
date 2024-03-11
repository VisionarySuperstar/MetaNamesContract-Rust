use crate::{
    msg::{MPC20TransferFromMsg, MintMsg, RenewDomainMsg},
    state::ContractState,
    ContractError,
};
use nft::{actions as nft_actions, msg as nft_msg};
use partisia_name_system::{
    actions::{self as pns_actions, execute_update_expiration},
    msg::{self as pns_msg, PnsDomainUpdateExpirationMsg},
};
use pbc_contract_common::{
    address::Address,
    context::ContractContext,
    events::{EventGroup, EventGroupBuilder},
};
use utils::{
    events::{build_msg_callback, IntoShortnameRPCEvent},
    time::milliseconds_in_years,
};

pub struct PaymentIntent {
    pub id: u64,
    pub token: Address,
    pub receiver: Address,
    pub total_fees: u128,
}

/// Action to mint contract
pub fn action_mint(
    ctx: &ContractContext,
    mut state: ContractState,
    domain: &String,
    to: &Address,
    token_uri: &Option<String>,
    parent_id: &Option<String>,
    subscription_years: &Option<u32>,
) -> (ContractState, Vec<EventGroup>) {
    assert!(!state.pns.is_minted(&domain), "{}", ContractError::Minted);

    pns_actions::validate_domain(&domain);

    let mut expires_at: Option<i64> = None;

    // Parent validations
    if let Some(parent_id) = parent_id.clone() {
        let parent = state.pns.get_domain(&parent_id);
        assert!(parent.is_some(), "{}", ContractError::DomainNotMinted);

        let parent = parent.unwrap();
        assert!(
            parent.is_active(ctx.block_production_time),
            "{}",
            ContractError::DomainNotActive
        );

        pns_actions::validate_domain_with_parent(&domain, &parent_id);

        let parent_token_id = parent.token_id;
        assert!(
            state.nft.is_approved_or_owner(ctx.sender, parent_token_id),
            "{}",
            ContractError::Unauthorized
        );
    } else if let Some(years_active) = subscription_years {
        let date = ctx.block_production_time + milliseconds_in_years(*years_active as i64);
        expires_at = Some(date);
    }

    let token_id = state.nft.get_next_token_id();
    let nft_events = nft_actions::execute_mint(
        &ctx,
        &mut state.nft,
        &nft_msg::NFTMintMsg {
            to: to.clone(),
            token_id,
            token_uri: token_uri.clone(),
        },
    );

    let pns_events = pns_actions::execute_mint(
        &ctx,
        &mut state.pns,
        &pns_msg::PnsMintMsg {
            domain: domain.clone(),
            expires_at,
            parent_id: parent_id.clone(),
            token_id,
        },
    );

    state.stats.increase_mint_count(ctx.sender);

    let events = nft_events
        .into_iter()
        .chain(pns_events.into_iter())
        .collect();

    (state, events)
}

pub fn action_build_mint_callback(
    payment_intent: &PaymentIntent,
    mint_msg: &MintMsg,
    callback_byte: u32,
) -> Vec<EventGroup> {
    assert!(
        payment_intent.id == mint_msg.payment_coin_id,
        "{}",
        ContractError::PaymentInfoNotValid
    );

    let subscription_years = mint_msg.subscription_years.unwrap_or(1);
    let mut payout_transfer_events = build_payout_fees_event_group(&mint_msg.to, payment_intent);

    build_msg_callback(&mut payout_transfer_events, callback_byte, mint_msg);

    vec![payout_transfer_events.build()]
}

pub fn action_build_renew_callback(
    payment_intent: &PaymentIntent,
    renew_msg: &RenewDomainMsg,
    callback_byte: u32,
) -> Vec<EventGroup> {
    assert!(
        payment_intent.id == renew_msg.payment_coin_id,
        "{}",
        ContractError::PaymentInfoNotValid
    );

    let mut payout_transfer_events =
        build_payout_fees_event_group(&renew_msg.payer, payment_intent);

    build_msg_callback(&mut payout_transfer_events, callback_byte, renew_msg);

    vec![payout_transfer_events.build()]
}

pub fn action_renew_subscription(
    ctx: ContractContext,
    mut state: ContractState,
    domain_name: String,
    subscription_years: u32,
) -> (ContractState, Vec<EventGroup>) {
    let domain = state.pns.get_domain(&domain_name).unwrap();

    let mut new_expiration_at = match domain.expires_at {
        Some(expires_at) => expires_at,
        None => ctx.block_production_time,
    };
    new_expiration_at += milliseconds_in_years(subscription_years as i64);

    execute_update_expiration(
        &ctx,
        &mut state.pns,
        &PnsDomainUpdateExpirationMsg {
            domain: domain_name,
            expires_at: Some(new_expiration_at),
        },
    );

    (state, vec![])
}

fn build_payout_fees_event_group(
    payer: &Address,
    payment_intent: &PaymentIntent,
) -> EventGroupBuilder {
    let mut payout_transfer_events = EventGroup::builder();

    MPC20TransferFromMsg {
        from: *payer,
        to: payment_intent.receiver,
        amount: payment_intent.total_fees,
    }
    .as_interaction(&mut payout_transfer_events, &payment_intent.token);

    payout_transfer_events
}
