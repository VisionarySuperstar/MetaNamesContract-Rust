# Ownable Base

Crate which provides a basic access control mechanism,
where there is an account (an owner) that can be granted exclusive access to specific functions.

## Base State Actions

## new

Creates new instance of `OwnableBaseState`.

## transfer_ownership

Transferes ownership to specified address. Panic if `ctx.sender` is not the actual owner.

## assert_only_owner

Verifies that `ctx.sender` is an actual owner.

## get_owner

Returns current owner address.
