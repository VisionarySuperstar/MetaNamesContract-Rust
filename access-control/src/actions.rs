use pbc_contract_common::context::ContractContext;

use crate::{
    msg::{ACInitMsg, ACRoleMsg, ACSetAdminRoleMsg},
    state::{AccessControlState, DEFAULT_ADMIN_ROLE},
    ContractError,
};

/// Initializes access control extension state
pub fn execute_init(msg: &ACInitMsg) -> AccessControlState {
    let mut state = AccessControlState::default();
    state._setup_role(DEFAULT_ADMIN_ROLE, DEFAULT_ADMIN_ROLE, &msg.admin_addresses);

    for role in msg.additional_roles.iter() {
        state._setup_role(*role, DEFAULT_ADMIN_ROLE, &[]);
    }

    state
}

/// Grants specified tole to specified account
/// Throws error if caller is not admin of specified role
pub fn execute_grant_role(ctx: &ContractContext, state: &mut AccessControlState, msg: &ACRoleMsg) {
    let admin_role = state.get_role_admin(msg.role).unwrap_or(DEFAULT_ADMIN_ROLE);
    execute_assert_only_role(state, admin_role, ctx);

    state._set_role(msg.role, &msg.account);
}

/// Revokes specified tole from specified account
/// Throws error if caller is not admin of specified role
pub fn execute_revoke_role(ctx: &ContractContext, state: &mut AccessControlState, msg: &ACRoleMsg) {
    let admin_role = state.get_role_admin(msg.role).unwrap_or(DEFAULT_ADMIN_ROLE);
    execute_assert_only_role(state, admin_role, ctx);

    state._revoke_role(msg.role, &msg.account);
}

/// Sets new tole admin for role
/// Throws error if caller is not admin of specified role
pub fn execute_set_role_admin(
    ctx: &ContractContext,
    state: &mut AccessControlState,
    msg: &ACSetAdminRoleMsg,
) {
    let admin_role = state.get_role_admin(msg.role).unwrap_or(DEFAULT_ADMIN_ROLE);
    execute_assert_only_role(state, admin_role, ctx);

    state._set_role_admin(msg.role, msg.new_admin_role);
}

/// Validates that only specified role member can have access
pub fn execute_assert_only_role(state: &AccessControlState, role: u8, ctx: &ContractContext) {
    assert!(
        state.has_role(role, &ctx.sender),
        "{}",
        ContractError::MissingRole
    );
}
