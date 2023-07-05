use contract_version_base::state::ContractVersionBase;
use pbc_contract_common::{
    context::ContractContext, events::EventGroup, sorted_vec_map::SortedVecMap,
};

use crate::{
    msg::{PnsMintMsg, PnsRecordDeleteMsg, PnsRecordMintMsg, PnsRecordUpdateMsg},
    state::{Domain, PartisiaNameSystemState},
    ContractError,
};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const MAX_DOMAIN_LEN: usize = 32;

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
    }

    state.domains.insert(
        msg.domain.clone(),
        Domain {
            token_id: msg.token_id,
            records: SortedVecMap::new(),
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

    let domain = state.domains.get_mut(&msg.domain).unwrap();
    assert!(
        domain.is_record_minted(&msg.class),
        "{}",
        ContractError::NotFound
    );

    domain.delete_record(&msg.class);

    vec![]
}

pub fn validate_domain(domain: &[u8]) {
    assert!(
        domain.len() <= MAX_DOMAIN_LEN,
        "{}",
        ContractError::InvalidDomain
    )
}

pub fn validate_domain_with_parent(domain: &[u8], parent: &[u8]) {
    assert!(
        parent.len() < domain.len() && domain.starts_with(parent),
        "{}",
        ContractError::InvalidDomainWithParent
    )
}
