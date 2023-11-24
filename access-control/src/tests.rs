use utils::tests::{mock_address, mock_contract_context};

use crate::{
    actions::{execute_assert_only_role, execute_grant_role, execute_revoke_role},
    msg::ACRoleMsg,
    state::AccessControlState,
};

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Debug)]
enum RoleEnum {
    Admin {},
    Read {},
}

#[test]
fn proper_access_control() {
    let alice = mock_address(1u8);
    let bob = mock_address(2u8);
    let jack = mock_address(3u8);

    let mut access_control = AccessControlState::<RoleEnum>::new();

    assert!(!access_control.has_role(RoleEnum::Admin {}, &alice));
    assert_eq!(access_control.get_role_admin(RoleEnum::Admin {}), None);

    access_control._setup_role(RoleEnum::Admin {}, RoleEnum::Admin {}, &[alice]);
    assert!(!access_control.has_role(RoleEnum::Read {}, &alice));
    assert!(access_control.has_role(RoleEnum::Admin {}, &alice));

    assert!(!access_control.has_role(RoleEnum::Read {}, &bob));
    access_control._setup_role(RoleEnum::Read {}, RoleEnum::Admin {}, &[bob]);
    assert!(access_control.has_role(RoleEnum::Read {}, &bob));

    assert!(!access_control.has_role(RoleEnum::Read {}, &jack));
    execute_grant_role(
        &mock_contract_context(1u8),
        &mut access_control,
        &ACRoleMsg {
            role: RoleEnum::Read {},
            account: jack,
        },
    );
    assert!(access_control.has_role(RoleEnum::Read {}, &jack));

    assert_eq!(
        access_control.get_role_admin(RoleEnum::Read {}),
        Some(&RoleEnum::Admin {})
    );

    execute_assert_only_role(
        &access_control,
        &RoleEnum::Read {},
        &mock_contract_context(3u8),
    );

    assert!(access_control.has_role(RoleEnum::Read {}, &jack));
    execute_revoke_role(
        &mock_contract_context(1u8),
        &mut access_control,
        &ACRoleMsg {
            role: RoleEnum::Read {},
            account: jack,
        },
    );
    assert!(!access_control.has_role(RoleEnum::Read {}, &jack));

    access_control._setup_role(RoleEnum::Admin {}, RoleEnum::Admin {}, &[bob]);

    assert_eq!(
        access_control.get_role_admin(RoleEnum::Admin {}),
        Some(&RoleEnum::Admin {})
    );

    access_control._renounce_role(RoleEnum::Admin {}, &mock_contract_context(1u8));
    assert!(!access_control.has_role(RoleEnum::Admin {}, &alice));
}

#[test]
#[should_panic(expected = "AccessControl-base: Specified address is missing role")]
fn test_role_mismatch() {
    let jack = mock_address(3u8);

    let mut access_control = AccessControlState::<RoleEnum>::new();

    access_control._setup_role(RoleEnum::Admin {}, RoleEnum::Admin {}, &[jack]);

    execute_assert_only_role(
        &access_control,
        &RoleEnum::Read {},
        &mock_contract_context(3u8),
    );
}
