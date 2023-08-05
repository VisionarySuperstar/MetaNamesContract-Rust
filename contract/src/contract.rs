use std::vec;

use crate::{
    actions::{action_build_mint_callback, action_mint},
    msg::{InitMsg, MintMsg},
    state::ContractState,
};

use contract_version_base::state::ContractVersionBase;
use pbc_contract_common::{
    address::Address,
    context::{CallbackContext, ContractContext},
    events::EventGroup,
};

use nft::{actions as nft_actions, msg as nft_msg};

use access_control::{actions as ac_actions, msg as ac_msg, state::DEFAULT_ADMIN_ROLE};
use partisia_name_system::{actions as pns_actions, msg as pns_msg, state::RecordClass};
use utils::events::assert_callback_success;

use crate::ContractError;

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const ADMIN_ROLE: u8 = DEFAULT_ADMIN_ROLE;

#[init]
pub fn initialize(ctx: ContractContext, msg: InitMsg) -> (ContractState, Vec<EventGroup>) {
    assert!(
        msg.payable_mint_info.token.is_some(),
        "{}",
        ContractError::PayableTokenNotSet
    );
    assert!(
        msg.payable_mint_info.receiver.is_some(),
        "{}",
        ContractError::PayableReceiverNotSet
    );

    let pns = pns_actions::execute_init(&ctx);
    let nft = nft_actions::execute_init(
        &ctx,
        &nft_msg::NFTInitMsg {
            name: msg.name,
            symbol: msg.symbol,
            uri_template: msg.uri_template,
        },
    );
    let access_control = ac_actions::execute_init(&ac_msg::ACInitMsg {
        admin_addresses: msg.admin_addresses,
    });

    let state = ContractState {
        access_control,
        nft,
        payable_mint_info: msg.payable_mint_info,
        pns,
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
    // Basic validations
    assert!(!state.pns.is_minted(&domain), "{}", ContractError::Minted);

    pns_actions::validate_domain(&domain);

    let mut events = vec![];
    let mut mut_state = state;

    let is_admin = mut_state.access_control.has_role(ADMIN_ROLE, &ctx.sender);
    if parent_id.is_some() || is_admin {
        let (new_state, mint_events) =
            action_mint(ctx, mut_state, domain, to, token_uri, parent_id);

        mut_state = new_state;

        events.extend(mint_events);
    } else {
        let payout_transfer_events = action_build_mint_callback(
            ctx,
            mut_state.payable_mint_info,
            &MintMsg {
                domain,
                to,
                token_uri,
                parent_id,
            },
            0x30,
        );

        events.extend(payout_transfer_events);
    }

    (mut_state, events)
}

#[callback(shortname = 0x30)]
pub fn on_mint_callback(
    ctx: ContractContext,
    callback_ctx: CallbackContext,
    state: ContractState,
    msg: MintMsg,
) -> (ContractState, Vec<EventGroup>) {
    assert_callback_success(&callback_ctx);

    action_mint(ctx, state, msg.domain, msg.to, msg.token_uri, msg.parent_id)
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

#[action(shortname = 0x24)]
pub fn update_admin_address(
    ctx: ContractContext,
    mut state: ContractState,
    admin: Address,
    active: bool,
) -> (ContractState, Vec<EventGroup>) {
    if active {
        ac_actions::execute_grant_role(
            &ctx,
            &mut state.access_control,
            &ac_msg::ACRoleMsg {
                role: ADMIN_ROLE,
                account: admin,
            },
        );
    } else {
        ac_actions::execute_revoke_role(
            &ctx,
            &mut state.access_control,
            &ac_msg::ACRoleMsg {
                role: ADMIN_ROLE,
                account: admin,
            },
        );
    }

    (state, vec![])
}
