use pbc_contract_common::context::ContractContext;

use crate::{
    msg::{ACInitMsg, ACRoleMsg, ACSetAdminRoleMsg},
    state::AccessControlState,
    ContractError,
};

/// Initializes access control extension state
pub fn execute_init<RoleEnum: Ord + Clone>(
    msg: ACInitMsg<RoleEnum>,
) -> AccessControlState<RoleEnum> {
    let mut state = AccessControlState::<RoleEnum>::new();
    state._setup_role(
        msg.admin_role.clone(),
        msg.admin_role.clone(),
        &msg.admin_addresses,
    );

    for role in msg.additional_roles.iter() {
        state._setup_role(role.clone(), msg.admin_role.clone(), &[]);
    }

    state
}

/// Grants specified tole to specified account
/// Throws error if caller is not admin of specified role
pub fn execute_grant_role<RoleEnum: Ord + Clone>(
    ctx: &ContractContext,
    state: &mut AccessControlState<RoleEnum>,
    msg: &ACRoleMsg<RoleEnum>,
) {
    let admin_role = state.get_role_admin(msg.role.clone());
    assert!(admin_role.is_some(), "{}", ContractError::MissingAdminRole);

    execute_assert_only_role(state, admin_role.unwrap(), ctx);
    state._set_role(msg.role.clone(), &msg.account);
}

/// Revokes specified tole from specified account
/// Throws error if caller is not admin of specified role
pub fn execute_revoke_role<RoleEnum: Ord + Clone>(
    ctx: &ContractContext,
    state: &mut AccessControlState<RoleEnum>,
    msg: &ACRoleMsg<RoleEnum>,
) {
    let admin_role = state.get_role_admin(msg.role.clone());
    assert!(admin_role.is_some(), "{}", ContractError::MissingAdminRole);

    execute_assert_only_role(state, admin_role.unwrap(), ctx);
    state._revoke_role(msg.role.clone(), &msg.account);
}

/// Sets new tole admin for role
/// Throws error if caller is not admin of specified role
pub fn execute_set_role_admin<RoleEnum: Ord + Clone>(
    ctx: &ContractContext,
    state: &mut AccessControlState<RoleEnum>,
    msg: &ACSetAdminRoleMsg<RoleEnum>,
) {
    let admin_role = state.get_role_admin(msg.role.clone());
    assert!(admin_role.is_some(), "{}", ContractError::MissingAdminRole);

    execute_assert_only_role(state, admin_role.unwrap(), ctx);
    state._set_role_admin(msg.role.clone(), msg.new_admin_role.clone());
}

/// Validates that only specified role member can have access
pub fn execute_assert_only_role<RoleEnum: Ord + Clone>(
    state: &AccessControlState<RoleEnum>,
    role: &RoleEnum,
    ctx: &ContractContext,
) {
    assert!(
        state.has_role(role.clone(), &ctx.sender),
        "{}",
        ContractError::MissingRole
    );
}
