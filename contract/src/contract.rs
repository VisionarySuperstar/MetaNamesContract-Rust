use std::vec;

use crate::{msg::InitMsg, state::ContractState};

use contract_version_base::state::ContractVersionBase;
use pbc_contract_common::{address::Address, context::ContractContext, events::EventGroup};

use mpc721_base::{actions as nft_actions, msg as nft_msg};

use partisia_name_system::{actions as pns_actions, msg as pns_msg, state::RecordClass};

use crate::ContractError;

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[init]
pub fn initialize(ctx: ContractContext, msg: InitMsg) -> (ContractState, Vec<EventGroup>) {
    let pns = pns_actions::execute_init(&ctx);
    let nft = nft_actions::execute_init(
        &ctx,
        &nft_msg::NFTInitMsg {
            name: msg.name,
            symbol: msg.symbol,
            uri_template: msg.uri_template,
        },
    );
    let state = ContractState {
        pns,
        nft,
        version: ContractVersionBase::new(CONTRACT_NAME, CONTRACT_VERSION),
    };

    (state, vec![])
}

#[action(shortname = 0x03)]
pub fn transfer_from(
    ctx: ContractContext,
    state: ContractState,
    from: Address,
    to: Address,
    token_id: u128,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = nft_actions::execute_transfer_from(
        &ctx,
        &mut state.nft,
        &nft_msg::NFTTransferFromMsg { from, to, token_id },
    );

    (state, events)
}

#[action(shortname = 0x05)]
pub fn approve(
    ctx: ContractContext,
    state: ContractState,
    approved: Option<Address>,
    token_id: u128,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = nft_actions::execute_approve(
        &ctx,
        &mut state.nft,
        &nft_msg::NFTApproveMsg { approved, token_id },
    );

    (state, events)
}

#[action(shortname = 0x07)]
pub fn set_approval_for_all(
    ctx: ContractContext,
    state: ContractState,
    operator: Address,
    approved: bool,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = nft_actions::execute_set_approval_for_all(
        &ctx,
        &mut state.nft,
        &nft_msg::NFTApproveForAllMsg { operator, approved },
    );

    (state, events)
}

#[action(shortname = 0x09)]
pub fn mint(
    ctx: ContractContext,
    state: ContractState,
    domain: Vec<u8>,
    to: Address,
    token_uri: Option<String>,
    parent_id: Option<Vec<u8>>,
) -> (ContractState, Vec<EventGroup>) {
    assert!(!state.pns.is_minted(&domain), "{}", ContractError::Minted);

    // TODO: Manage parentship

    let mut state = state;
    let token_id = state.nft.get_next_token_id();
    let nft_events = nft_actions::execute_mint(
        &ctx,
        &mut state.nft,
        &nft_msg::NFTMintMsg {
            to,
            token_id,
            token_uri: token_uri.clone(),
        },
    );

    let pns_events = pns_actions::execute_mint(
        &ctx,
        &mut state.pns,
        &pns_msg::PnsMintMsg {
            domain,
            to,
            token_uri,
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

#[action(shortname = 0x21)]
pub fn mint_record(
    ctx: ContractContext,
    state: ContractState,
    domain: Vec<u8>,
    class: RecordClass,
    data: String,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = pns_actions::execute_record_mint(
        &ctx,
        &mut state.pns,
        &pns_msg::PnsRecordMintMsg {
            domain,
            class,
            data,
        },
    );

    (state, events)
}

#[action(shortname = 0x22)]
pub fn update_record(
    ctx: ContractContext,
    state: ContractState,
    domain: Vec<u8>,
    class: RecordClass,
    data: String,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = pns_actions::execute_record_update(
        &ctx,
        &mut state.pns,
        &pns_msg::PnsRecordUpdateMsg {
            domain,
            class,
            data,
        },
    );

    (state, events)
}

#[action(shortname = 0x23)]
pub fn delete_record(
    ctx: ContractContext,
    state: ContractState,
    domain: Vec<u8>,
    class: RecordClass,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = pns_actions::execute_record_delete(
        &ctx,
        &mut state.pns,
        &pns_msg::PnsRecordDeleteMsg { domain, class },
    );

    (state, events)
}
