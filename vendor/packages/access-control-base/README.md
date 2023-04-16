# Access Control Base

Crate that provides role based access control mechanism,
where there is an account (role member) that can be granted exclusive access to specific functions.

## Base State Actions

## grant_role
Grants specified tole to specified account.

## setup_role
Setups new role.

## revoke_role
Removes role access for specified account.

## renounce_role
Removes sender access to role.

## set_role_admin
Sets new tole admin for role.

## assert_only_role
Validates that only specified role member can have access.

## has_role
Returns either address has specified role or not.

## get_role_admin
Returns admin role of specified role.