# MPC721-Base Contract

Base implementation of MPC721 contract.

# Actions

## execute_set_base_uri

Set base uri for the tokens.

Params:

```json
SetBaseUriMsg {
    "new_base_uri": "<uri>",
}
```

## execute_mint

Mint a new token. Can only be executed by minter account.

Params:

```json
MintMsg {
    "token_id": 1,
    "to": "<address>",
    "token_uri": "<optional uri>",
}
```

## execute_transfer

Transfer token to another account.

Params:

```json
TransferMsg {
    "to": "<address>",
    "token_id": 1,
}
```

## execute_transfer_from

Only with approval extension. Transfer token from owner to spender.

Params:

```json
TransferFromMsg {
    "from": "<address>",
    "to": "<address>",
    "token_id": 1,
}
```

## execute_approve

Allows spender to transfer token from the owner account.

Params:

```json
ApproveMsg {
    "spedner": "<address>",
    "token_id": 1,
}
```

## execute_approve_for_all

Allows operator to transfer any owner tokens from his account.

Params:

```json
ApproveForAllMsg {
    "operator": "<address>",
}
```

## execute_revoke

Remove approval.

Params:

```json
RevokeMsg {
    "spedner": "<address>",
    "token_id": 1,
}
```

## execute_revoke_for_all

Remove operator.

Params:

```json
RevokeForAllMsg {
    "operator": "<address>",
}
```

## execute_burn

Destroy your token forever.

Params:

```json
BurnMsg {
    "token_id": 1,
}
```
