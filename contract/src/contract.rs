use crate::{msg::InitMsg, state::ContractState};

use contract_version_base::state::ContractVersionBase;
use pbc_contract_common::{address::Address, context::ContractContext, events::EventGroup};

use nft::{actions as nft_actions, msg as nft_msg};

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

#[action(shortname = 0x06)]
pub fn approve_domain(
    ctx: ContractContext,
    state: ContractState,
    approved: Option<Address>,
    domain: String,
) -> (ContractState, Vec<EventGroup>) {
    assert!(
        state.pns.is_minted(&domain),
        "{}",
        ContractError::DomainNotMinted
    );

    let token_id = state.pns.get_token_id(&domain).unwrap();

    approve(ctx, state, approved, token_id)
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

#[action(shortname = 0x21)]
pub fn mint_record(
    ctx: ContractContext,
    state: ContractState,
    domain: String,
    class: RecordClass,
    data: Vec<u8>,
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
    domain: String,
    class: RecordClass,
    data: Vec<u8>,
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
    domain: String,
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
