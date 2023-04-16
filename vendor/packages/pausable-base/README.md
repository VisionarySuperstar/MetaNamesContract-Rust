# Pausable Base

Crate which allows contracts to implement an emergency stop mechanism that can be triggered by an authorized account.

## Base State Actions

## new

Creates new instance of `PausableBaseState`.

## pause

Sets pause flag to `true`. Panics if already paused.

## unpause

Sets pause flag to `false`. Panics if already unpaused.

## paused

Returns current flag.

## assert_paused

Verifies that contract is paused.

## assert_not_paused

Verifies that contract is unpaused.
