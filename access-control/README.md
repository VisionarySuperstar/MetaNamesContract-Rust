# Access Control

Crate that provides role based access control mechanism,
where there is an account (role member) that can be granted exclusive access to specific functions.

## Command Actions
All command actions that changes the state.
Prefer command action instead of state actions

### execute_init
Setup state

### execute_grant_role
Grants role to address

### execute_revoke_role
Revoke role to address

### execute_set_role_admin
Change admin role of a role

### execute_assert_only_role
Check if address has given role

## State Actions
Please do not use methods starting with `_`

### has_role
Returns either address has specified role or not.

### get_role_admin
Returns admin role of specified role.
