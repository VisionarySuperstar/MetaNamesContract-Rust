# MPC20-Base

Base implementation of MPC20 contract.

# Actions

## execute_mint

Mint specified amount of tokens to provided address.
Only works when minter option is enabled

Pararms:

```json
MintMsg {
    "recipient": "<address>",
    "amount": 123
}
```

## execute_transfer

Moves amount tokens from the msg sender account to specified `to` account.

Params:

```json
TransferMsg {
    "to": "<address>",
    "amount": 123
}
```

## execute_transfer_from

Only with approval extension. Transfers amount tokens from owner -> recipient if sender has sufficient pre-approval.

Params:

```json
TransferFromMsg {
    "owner": "<address>",
    "to": "<address>",
    "amount": 123
}
```

## execute_burn

Burn is a method to destroy your tokens forever.

Params:

```json
BurnMsg {
    "amount": 123
}
```

## execute_burn_from

Only with approval extension. Destroys your tokens forever.

Params:

```json
BurnFromMsg {
    "owner": "<address>",
    "amount": 123
}
```

## execute_approve

Sets amount as the allowance of spender over the caller's tokens.

Params:

```json
ApproveMsg {
    "spender": "<address>",
    "amount": 123
}
```

## execute_increase_allowance

Allows spender to access an additional amount tokens from the owner's account.

Params:

```json
IncreaseAllowanceMsg {
    "spender": "<address>",
    "amount": 123
}
```

## execute_decrease_allowance

Lowers the spender's access of tokens from the owner's account by amount.

Params:

```json
DecreaseAllowanceMsg {
    "spender": "<address>",
    "amount": 123
}
```
