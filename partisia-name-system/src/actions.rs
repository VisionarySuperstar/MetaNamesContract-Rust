use contract_version_base::state::ContractVersionBase;
use pbc_contract_common::{
    context::ContractContext, events::EventGroup, sorted_vec_map::SortedVecMap,
};

use crate::{
    msg::{
        PnsDomainUpdateExpirationMsg, PnsMintMsg, PnsRecordDeleteAllMsg, PnsRecordDeleteMsg,
        PnsRecordMintMsg, PnsRecordUpdateMsg,
    },
    state::{Domain, PartisiaNameSystemState, MAX_DOMAIN_LEN, MAX_RECORD_DATA_LENGTH},
    ContractError,
};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// ## Description
/// Inits contract state.
/// Returns [`(PartisiaNameSystemState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
pub fn execute_init(ctx: &ContractContext) -> PartisiaNameSystemState {
    PartisiaNameSystemState {
        domains: SortedVecMap::new(),
        records: SortedVecMap::new(),
        version: ContractVersionBase::new(CONTRACT_NAME, CONTRACT_VERSION),
    }
}

/// ## Description
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
            minted_at: ctx.block_production_time,
            expires_at: msg.expires_at,
            parent_id: msg.parent_id.clone(),
        },
    );

    vec![]
}

/// ## Description
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

    let domain = state.domains.get_mut(&msg.domain).unwrap();
    domain.mint_record(&msg.class, &msg.data);

    vec![]
}

/// ## Description
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

    let domain = state.domains.get_mut(&msg.domain).unwrap();
    assert!(
        domain.is_record_minted(&msg.class),
        "{}",
        ContractError::NotFound
    );

    domain.update_record_data(&msg.class, &msg.data);

    vec![]
}

/// ## Description
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

    let domain = state.domains.get_mut(&msg.domain).unwrap();
    assert!(
        domain.is_record_minted(&msg.class),
        "{}",
        ContractError::NotFound
    );

    domain.delete_record(&msg.class);

    vec![]
}

/// ## Description
/// Delete all records for a domain
/// Does not require the domain to be active
pub fn execute_record_delete_all(
    ctx: &ContractContext,
    state: &mut PartisiaNameSystemState,
    msg: &PnsRecordDeleteAllMsg,
) -> Vec<EventGroup> {
    assert!(state.is_minted(&msg.domain), "{}", ContractError::NotFound);

    let domain = state.domains.get_mut(&msg.domain).unwrap();
    domain.records = SortedVecMap::new();

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

    let domain = state.domains.get_mut(&msg.domain).unwrap();
    domain.expires_at = msg.expires_at;

    vec![]
}

/// ## Description
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

/// ## Description
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
