use utils::tests::{mock_address, mock_contract_context};

use crate::{
    actions::{execute_assert_only_role, execute_grant_role, execute_revoke_role},
    msg::ACRoleMsg,
    state::{AccessControlState, DEFAULT_ADMIN_ROLE},
};

const ROLE_A: u8 = 0x02;
const ROLE_B: u8 = 0x03;

#[test]
fn proper_access_control() {
    let alice = mock_address(1u8);
    let bob = mock_address(2u8);
    let jack = mock_address(3u8);

    let mut access_control = AccessControlState::default();

    assert!(!access_control.has_role(ROLE_A, &alice));
    assert_eq!(access_control.get_role_admin(ROLE_A), None);

    access_control._setup_role(DEFAULT_ADMIN_ROLE, &vec![alice]);
    assert!(!access_control.has_role(ROLE_A, &alice));
    assert!(access_control.has_role(DEFAULT_ADMIN_ROLE, &alice));

    assert!(!access_control.has_role(ROLE_B, &bob));
    access_control._setup_role(ROLE_B, &vec![bob]);
    assert!(access_control.has_role(ROLE_B, &bob));

    assert!(!access_control.has_role(ROLE_B, &jack));
    execute_grant_role(
        &mock_contract_context(1u8),
        &mut access_control,
        &ACRoleMsg {
            role: ROLE_B,
            account: jack,
        },
    );
    assert!(access_control.has_role(ROLE_B, &jack));

    assert_eq!(access_control.get_role_admin(ROLE_B), Some(0x00));

    execute_assert_only_role(&access_control, ROLE_B, &mock_contract_context(3u8));

    assert!(access_control.has_role(ROLE_B, &jack));
    execute_revoke_role(
        &mock_contract_context(1u8),
        &mut access_control,
        &ACRoleMsg {
            role: ROLE_B,
            account: jack,
        },
    );
    assert!(!access_control.has_role(ROLE_B, &jack));

    access_control._setup_role(ROLE_A, &vec![bob]);
    access_control._set_role_admin(ROLE_A, ROLE_B);

    assert_eq!(access_control.get_role_admin(ROLE_A), Some(0x03));

    access_control._renounce_role(DEFAULT_ADMIN_ROLE, &mock_contract_context(1u8));
    assert!(!access_control.has_role(DEFAULT_ADMIN_ROLE, &alice));
}

#[test]
#[should_panic(expected = "AccessControl-base: Specified address is missing role")]
fn test_role_mismatch() {
    let jack = mock_address(3u8);

    let mut access_control = AccessControlState::default();

    access_control._setup_role(DEFAULT_ADMIN_ROLE, &vec![jack]);

    execute_assert_only_role(&access_control, ROLE_A, &mock_contract_context(3u8));
}
