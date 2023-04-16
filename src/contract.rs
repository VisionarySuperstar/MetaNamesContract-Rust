use crate::state::ContractState;

use contract_version_base::state::ContractVersionBase;
use pbc_contract_common::{address::Address, context::ContractContext, events::EventGroup};

use partisia_name_system::{
    actions::{
        execute_approve, execute_approve_for_all, execute_burn, execute_init, execute_mint,
        execute_multi_mint, execute_ownership_check, execute_record_delete, execute_record_mint,
        execute_record_update, execute_revoke, execute_revoke_for_all, execute_set_base_uri,
        execute_transfer, execute_transfer_from, execute_update_minter,
    },
    msg::{
        ApproveForAllMsg, ApproveMsg, BurnMsg, CheckOwnerMsg, InitMsg, MintMsg, MultiMintMsg,
        RecordDeleteMsg, RecordMintMsg, RecordUpdateMsg, RevokeForAllMsg, RevokeMsg, SetBaseUriMsg,
        TransferFromMsg, TransferMsg, UpdateMinterMsg,
    },
};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[init]
pub fn initialize(ctx: ContractContext, msg: InitMsg) -> (ContractState, Vec<EventGroup>) {
    let (pns, events) = execute_init(&ctx, &msg);
    let state = ContractState {
        pns,
        version: ContractVersionBase::new(CONTRACT_NAME, CONTRACT_VERSION),
    };

    (state, events)
}

#[action(shortname = 0x01)]
pub fn transfer(
    ctx: ContractContext,
    state: ContractState,
    to: Address,
    token_id: String,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_transfer(&ctx, &mut state.pns, &TransferMsg { to, token_id });

    (state, events)
}

#[action(shortname = 0x03)]
pub fn transfer_from(
    ctx: ContractContext,
    state: ContractState,
    from: Address,
    to: Address,
    token_id: String,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_transfer_from(
        &ctx,
        &mut state.pns,
        &TransferFromMsg { from, to, token_id },
    );

    (state, events)
}

#[action(shortname = 0x05)]
pub fn approve(
    ctx: ContractContext,
    state: ContractState,
    spender: Address,
    token_id: String,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_approve(&ctx, &mut state.pns, &ApproveMsg { spender, token_id });

    (state, events)
}

#[action(shortname = 0x07)]
pub fn set_base_uri(
    ctx: ContractContext,
    state: ContractState,
    new_base_uri: String,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_set_base_uri(&ctx, &mut state.pns, &SetBaseUriMsg { new_base_uri });

    (state, events)
}

#[action(shortname = 0x09)]
pub fn mint(
    ctx: ContractContext,
    state: ContractState,
    token_id: String,
    to: Address,
    parent: Option<String>,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_mint(
        &ctx,
        &mut state.pns,
        &MintMsg {
            token_id,
            to,
            parent,
        },
    );

    (state, events)
}

#[action(shortname = 0x11)]
pub fn approve_for_all(
    ctx: ContractContext,
    state: ContractState,
    operator: Address,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_approve_for_all(&ctx, &mut state.pns, &ApproveForAllMsg { operator });

    (state, events)
}

#[action(shortname = 0x13)]
pub fn revoke(
    ctx: ContractContext,
    state: ContractState,
    spender: Address,
    token_id: String,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_revoke(&ctx, &mut state.pns, &RevokeMsg { spender, token_id });

    (state, events)
}

#[action(shortname = 0x15)]
pub fn revoke_for_all(
    ctx: ContractContext,
    state: ContractState,
    operator: Address,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_revoke_for_all(&ctx, &mut state.pns, &RevokeForAllMsg { operator });

    (state, events)
}

#[action(shortname = 0x17)]
pub fn burn(
    ctx: ContractContext,
    state: ContractState,
    token_id: String,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_burn(&ctx, &mut state.pns, &BurnMsg { token_id });

    (state, events)
}

#[action(shortname = 0x18)]
pub fn check_ownership(
    ctx: ContractContext,
    state: ContractState,
    owner: Address,
    token_id: String,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_ownership_check(&ctx, &mut state.pns, &CheckOwnerMsg { owner, token_id });
    (state, events)
}
#[action(shortname = 0x19)]
pub fn update_minter(
    ctx: ContractContext,
    state: ContractState,
    new_minter: Address,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_update_minter(&ctx, &mut state.pns, UpdateMinterMsg { new_minter });
    (state, events)
}
#[action(shortname = 0x20)]
pub fn multi_mint(
    ctx: ContractContext,
    state: ContractState,
    mints: Vec<MintMsg>,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;

    execute_multi_mint(&ctx, &mut state.pns, &MultiMintMsg { mints });
    (state, vec![])
}

#[action(shortname = 0x21)]
pub fn mint_record(
    ctx: ContractContext,
    state: ContractState,
    token_id: String,
    class: u8,
    data: String,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_record_mint(
        &ctx,
        &mut state.pns,
        &RecordMintMsg {
            token_id,
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
    token_id: String,
    class: u8,
    data: String,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_record_update(
        &ctx,
        &mut state.pns,
        &RecordUpdateMsg {
            token_id,
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
    token_id: String,
    class: u8,
) -> (ContractState, Vec<EventGroup>) {
    let mut state = state;
    let events = execute_record_delete(&ctx, &mut state.pns, &RecordDeleteMsg { token_id, class });

    (state, events)
}
