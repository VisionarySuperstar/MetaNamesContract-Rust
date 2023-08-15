use crate::{
    actions::{
        action_build_mint_callback, action_build_renew_callback, action_mint,
        action_renew_subscription,
    },
    msg::{InitMsg, MintMsg, RenewDomainMsg},
    state::{ContractConfig, ContractState, ContractStats, UserRole},
};

use contract_version_base::state::ContractVersionBase;
use pbc_contract_common::{
    address::Address,
    context::{CallbackContext, ContractContext},
    events::EventGroup,
};

use nft::{actions as nft_actions, msg as nft_msg};

use access_control::{actions as ac_actions, msg as ac_msg};
use partisia_name_system::{actions as pns_actions, msg as pns_msg, state::RecordClass};
use utils::events::assert_callback_success;

use crate::ContractError;

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[init]
pub fn initialize(ctx: ContractContext, msg: InitMsg) -> (ContractState, Vec<EventGroup>) {
    assert!(
        msg.config.payable_mint_info.token.is_some(),
        "{}",
        ContractError::PayableTokenNotSet
    );
    assert!(
        msg.config.payable_mint_info.receiver.is_some(),
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
        config: msg.config,
        nft,
        pns,
        stats: ContractStats::default(),
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
    assert_contract_enabled(&state);

    let mut state = state;
    let mut nft_events = nft_actions::execute_transfer_from(
        &ctx,
        &mut state.nft,
        &nft_msg::NFTTransferFromMsg { from, to, token_id },
    );

    let (name, _) = state.pns.get_domain_by_token_id(token_id).unwrap();
    let msg = &pns_msg::PnsRecordDeleteAllMsg {
        domain: name.clone(),
    };
    let pns_events = pns_actions::execute_record_delete_all(&ctx, &mut state.pns, msg);

    nft_events.extend(pns_events);

    (state, nft_events)
}

#[action(shortname = 0x04)]
pub fn transfer_domain(
    ctx: ContractContext,
    state: ContractState,
    from: Address,
    to: Address,
    domain: String,
) -> (ContractState, Vec<EventGroup>) {
    assert_contract_enabled(&state);

    let token_id = state.pns.get_token_id(&domain);
    assert!(token_id.is_some(), "{}", ContractError::DomainNotMinted);

    transfer_from(ctx, state, from, to, token_id.unwrap())
}

#[action(shortname = 0x05)]
pub fn approve(
    ctx: ContractContext,
    state: ContractState,
    approved: Option<Address>,
    token_id: u128,
) -> (ContractState, Vec<EventGroup>) {
    assert_contract_enabled(&state);

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
    assert_contract_enabled(&state);

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
    assert_contract_enabled(&state);

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
    subscription_years: Option<u32>,
) -> (ContractState, Vec<EventGroup>) {
    assert_contract_enabled(&state);

    // Basic validations
    assert!(!state.pns.is_minted(&domain), "{}", ContractError::Minted);

    pns_actions::validate_domain(&domain);

    let mut events = vec![];
    let mut mut_state = state;

    let is_admin = mut_state
        .access_control
        .has_role(UserRole::Admin {} as u8, &ctx.sender);
    if parent_id.is_some() || is_admin {
        let (new_state, mint_events) =
            action_mint(ctx, mut_state, domain, to, token_uri, parent_id, None);

        mut_state = new_state;

        events.extend(mint_events);
    } else {
        if mut_state.config.whitelist_enabled {
            let is_whitelisted = mut_state
                .access_control
                .has_role(UserRole::Whitelist {} as u8, &ctx.sender);
            assert!(is_whitelisted, "{}", ContractError::UserNotWhitelisted);
        }

        if mut_state.config.mint_count_limit_enabled && !is_admin {
            let mint_count = mut_state.stats.mint_count.get(&ctx.sender);
            assert!(
                mint_count.is_none() || mint_count <= Some(&mut_state.config.mint_count_limit),
                "{}",
                ContractError::MintCountLimitReached
            );
        }

        let subscription_years = subscription_years.unwrap_or(1);
        let payout_transfer_events = action_build_mint_callback(
            ctx,
            mut_state.config.payable_mint_info,
            &MintMsg {
                domain,
                to,
                token_uri,
                parent_id,
                subscription_years: Some(subscription_years),
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
    assert_contract_enabled(&state);

    assert_callback_success(&callback_ctx);

    action_mint(
        ctx,
        state,
        msg.domain,
        msg.to,
        msg.token_uri,
        msg.parent_id,
        msg.subscription_years,
    )
}

#[action(shortname = 0x21)]
pub fn mint_record(
    ctx: ContractContext,
    state: ContractState,
    domain: String,
    class: RecordClass,
    data: Vec<u8>,
) -> (ContractState, Vec<EventGroup>) {
    assert_contract_enabled(&state);

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
    assert_contract_enabled(&state);

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
    assert_contract_enabled(&state);

    let mut state = state;
    let events = pns_actions::execute_record_delete(
        &ctx,
        &mut state.pns,
        &pns_msg::PnsRecordDeleteMsg { domain, class },
    );

    (state, events)
}

#[action(shortname = 0x24)]
pub fn update_user_role(
    ctx: ContractContext,
    mut state: ContractState,
    role: UserRole,
    address: Address,
    active: bool,
) -> (ContractState, Vec<EventGroup>) {
    if active {
        ac_actions::execute_grant_role(
            &ctx,
            &mut state.access_control,
            &ac_msg::ACRoleMsg {
                role: role as u8,
                account: address,
            },
        );
    } else {
        ac_actions::execute_revoke_role(
            &ctx,
            &mut state.access_control,
            &ac_msg::ACRoleMsg {
                role: role as u8,
                account: address,
            },
        );
    }

    (state, vec![])
}

#[action(shortname = 0x25)]
pub fn update_config(
    ctx: ContractContext,
    mut state: ContractState,
    config: ContractConfig,
) -> (ContractState, Vec<EventGroup>) {
    let is_admin = state
        .access_control
        .has_role(UserRole::Admin {} as u8, &ctx.sender);
    assert!(is_admin, "{}", ContractError::Unauthorized);

    state.config = config;

    (state, vec![])
}

fn assert_contract_enabled(state: &ContractState) {
    assert!(
        state.config.contract_enabled,
        "{}",
        ContractError::ContractDisabled
    );
}

#[action(shortname = 0x26)]
pub fn renew_subscription(
    ctx: ContractContext,
    mut state: ContractState,
    domain: String,
    payer: Address,
    subscription_years: u32,
) -> (ContractState, Vec<EventGroup>) {
    assert_contract_enabled(&state);
    assert!(
        subscription_years > 0,
        "{}",
        ContractError::InvalidSubscriptionYears
    );

    let is_admin = state
        .access_control
        .has_role(UserRole::Admin {} as u8, &ctx.sender);

    let events;
    if is_admin {
        let (new_state, renew_events) =
            action_renew_subscription(ctx, state, domain, subscription_years);

        state = new_state;
        events = renew_events;
    } else {
        events = action_build_renew_callback(
            ctx,
            state.config.payable_mint_info,
            &RenewDomainMsg {
                domain,
                payer,
                subscription_years,
            },
            0x31,
        );
    };

    (state, events)
}

#[callback(shortname = 0x31)]
pub fn on_renew_subscription_callback(
    ctx: ContractContext,
    callback_ctx: CallbackContext,
    state: ContractState,
    msg: RenewDomainMsg,
) -> (ContractState, Vec<EventGroup>) {
    assert_contract_enabled(&state);

    assert_callback_success(&callback_ctx);

    action_renew_subscription(ctx, state, msg.domain, msg.subscription_years)
}
