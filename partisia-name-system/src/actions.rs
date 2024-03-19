use contract_version_base::state::ContractVersionBase;
use pbc_contract_common::{
    avl_tree_map::AvlTreeMap, context::ContractContext, events::EventGroup,
    sorted_vec_map::SortedVecMap,
};

use crate::{
    msg::{
        PnsCustomRecordDeleteMsg, PnsCustomRecordMintMsg, PnsCustomRecordUpdateMsg,
        PnsDomainUpdateExpirationMsg, PnsMintMsg, PnsRecordDeleteAllMsg, PnsRecordDeleteMsg,
        PnsRecordMintMsg, PnsRecordUpdateMsg,
    },
    state::{
        Domain, PartisiaNameSystemState, MAX_CUSTOM_RECORDS, MAX_DOMAIN_LEN, MAX_RECORD_DATA_LENGTH,
    },
    ContractError,
};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Inits contract state.
/// Returns [`(PartisiaNameSystemState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
pub fn execute_init(ctx: &ContractContext) -> PartisiaNameSystemState {
    PartisiaNameSystemState {
        domains: AvlTreeMap::new(),
        version: ContractVersionBase::new(CONTRACT_NAME, CONTRACT_VERSION),
    }
}

/// Mint a new token. Can only be executed by minter account.
/// Returns [`Vec<EventGroup>`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
pub fn execute_mint(
    ctx: &ContractContext,
    state: &mut PartisiaNameSystemState,
    msg: &PnsMintMsg,
) -> Vec<EventGroup> {
    assert!(!state.is_minted(&msg.domain), "{}", ContractError::Minted);

    if let Some(parent_id) = msg.parent_id.clone() {
        assert!(state.is_minted(&parent_id), "{}", ContractError::NotFound);
        assert!(
            state.is_active(&parent_id, ctx.block_production_time),
            "{}",
            ContractError::DomainExpired
        );
    }

    state.domains.insert(
        msg.domain.clone(),
        Domain {
            token_id: msg.token_id,
            records: SortedVecMap::new(),
            custom_records: SortedVecMap::new(),
            minted_at: ctx.block_production_time,
            expires_at: msg.expires_at,
            parent_id: msg.parent_id.clone(),
        },
    );

    vec![]
}

/// Mint a new record for a domain
/// Returns [`Vec<EventGroup>`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
pub fn execute_record_mint(
    ctx: &ContractContext,
    state: &mut PartisiaNameSystemState,
    msg: &PnsRecordMintMsg,
) -> Vec<EventGroup> {
    assert!(state.is_minted(&msg.domain), "{}", ContractError::NotFound);
    assert!(
        state.is_active(&msg.domain, ctx.block_production_time),
        "{}",
        ContractError::DomainExpired
    );
    assert!(
        msg.data.clone().len() < MAX_RECORD_DATA_LENGTH,
        "{}",
        ContractError::RecordDataTooLong
    );

    let mut domain = state.domains.get(&msg.domain).unwrap();
    domain.mint_record(&msg.class, &msg.data);
    state.domains.insert(msg.domain.clone(), domain);

    vec![]
}

/// Update a record for a domain
/// Returns [`Vec<EventGroup>`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
pub fn execute_record_update(
    ctx: &ContractContext,
    state: &mut PartisiaNameSystemState,
    msg: &PnsRecordUpdateMsg,
) -> Vec<EventGroup> {
    assert!(state.is_minted(&msg.domain), "{}", ContractError::NotFound);
    assert!(
        state.is_active(&msg.domain, ctx.block_production_time),
        "{}",
        ContractError::DomainExpired
    );

    let mut domain = state.domains.get(&msg.domain).unwrap();
    assert!(
        domain.is_record_minted(&msg.class),
        "{}",
        ContractError::NotFound
    );

    domain.update_record_data(&msg.class, &msg.data);
    state.domains.insert(msg.domain.clone(), domain);

    vec![]
}

/// Delete a record for a domain
/// Returns [`Vec<EventGroup>`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
pub fn execute_record_delete(
    ctx: &ContractContext,
    state: &mut PartisiaNameSystemState,
    msg: &PnsRecordDeleteMsg,
) -> Vec<EventGroup> {
    assert!(state.is_minted(&msg.domain), "{}", ContractError::NotFound);
    assert!(
        state.is_active(&msg.domain, ctx.block_production_time),
        "{}",
        ContractError::DomainExpired
    );

    let mut domain = state.domains.get(&msg.domain).unwrap();
    assert!(
        domain.is_record_minted(&msg.class),
        "{}",
        ContractError::NotFound
    );

    domain.delete_record(&msg.class);
    state.domains.insert(msg.domain.clone(), domain);

    vec![]
}

/// Delete all records for a domain
/// Does not require the domain to be active
pub fn execute_record_delete_all(
    ctx: &ContractContext,
    state: &mut PartisiaNameSystemState,
    msg: &PnsRecordDeleteAllMsg,
) -> Vec<EventGroup> {
    assert!(state.is_minted(&msg.domain), "{}", ContractError::NotFound);

    let mut domain = state.domains.get(&msg.domain).unwrap();
    domain.records = SortedVecMap::new();
    domain.custom_records = SortedVecMap::new();
    state.domains.insert(msg.domain.clone(), domain);

    vec![]
}

/// Mint a new custom record for a domain
/// Returns [`Vec<EventGroup>`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
pub fn execute_custom_record_mint(
    ctx: &ContractContext,
    state: &mut PartisiaNameSystemState,
    msg: &PnsCustomRecordMintMsg,
) -> Vec<EventGroup> {
    assert!(state.is_minted(&msg.domain), "{}", ContractError::NotFound);
    assert!(
        state.is_active(&msg.domain, ctx.block_production_time),
        "{}",
        ContractError::DomainExpired
    );
    assert!(
        msg.data.len() < MAX_RECORD_DATA_LENGTH,
        "{}",
        ContractError::RecordDataTooLong
    );

    let mut domain = state.domains.get(&msg.domain).unwrap();
    assert!(
        domain.custom_records.len() < MAX_CUSTOM_RECORDS,
        "{}",
        ContractError::MaxCustomRecords
    );

    domain.mint_custom_record(msg.key.as_str(), msg.data.as_slice());
    state.domains.insert(msg.domain.clone(), domain);

    vec![]
}

/// Update a custom record for a domain
/// Returns [`Vec<EventGroup>`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
pub fn execute_custom_record_update(
    ctx: &ContractContext,
    state: &mut PartisiaNameSystemState,
    msg: &PnsCustomRecordUpdateMsg,
) -> Vec<EventGroup> {
    assert!(state.is_minted(&msg.domain), "{}", ContractError::NotFound);
    assert!(
        state.is_active(&msg.domain, ctx.block_production_time),
        "{}",
        ContractError::DomainExpired
    );
    assert!(
        msg.data.len() < MAX_RECORD_DATA_LENGTH,
        "{}",
        ContractError::RecordDataTooLong
    );

    let mut domain = state.domains.get(&msg.domain).unwrap();
    assert!(
        domain.is_custom_record_minted(&msg.key),
        "{}",
        ContractError::NotFound
    );

    domain.update_custom_record_data(msg.key.as_str(), msg.data.as_slice());
    state.domains.insert(msg.domain.clone(), domain);

    vec![]
}

/// Delete a custom record for a domain
/// Returns [`Vec<EventGroup>`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
pub fn execute_custom_record_delete(
    ctx: &ContractContext,
    state: &mut PartisiaNameSystemState,
    msg: &PnsCustomRecordDeleteMsg,
) -> Vec<EventGroup> {
    assert!(state.is_minted(&msg.domain), "{}", ContractError::NotFound);
    assert!(
        state.is_active(&msg.domain, ctx.block_production_time),
        "{}",
        ContractError::DomainExpired
    );

    let mut domain = state.domains.get(&msg.domain).unwrap();
    assert!(
        domain.is_custom_record_minted(&msg.key),
        "{}",
        ContractError::NotFound
    );

    domain.delete_custom_record(&msg.key);
    state.domains.insert(msg.domain.clone(), domain);

    vec![]
}

///## Description
/// Update the expiration date for a domain
pub fn execute_update_expiration(
    ctx: &ContractContext,
    state: &mut PartisiaNameSystemState,
    msg: &PnsDomainUpdateExpirationMsg,
) -> Vec<EventGroup> {
    assert!(state.is_minted(&msg.domain), "{}", ContractError::NotFound);

    let mut domain = state.domains.get(&msg.domain).unwrap();
    domain.expires_at = msg.expires_at;
    state.domains.insert(msg.domain.clone(), domain);

    vec![]
}

/// Validate the domain name
/// Returns [`()`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
pub fn validate_domain(domain: &str) {
    assert!(
        domain.len() <= MAX_DOMAIN_LEN,
        "{}",
        ContractError::InvalidDomain
    )
}

/// Validate the domain name with parent
/// Returns [`()`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
pub fn validate_domain_with_parent(domain: &str, parent: &str) {
    assert!(
        parent.len() < domain.len() && domain.starts_with(parent),
        "{}",
        ContractError::InvalidDomainWithParent
    )
}
