# MPC1155-Base Contract

Base implementation of MPC1155 contract.

# Actions

## execute_set_uri

Set uri for the tokens.

Params:

```json
SetUriMsg {
    "new_uri": "<uri>",
}
```

## execute_mint

Mint a new token. Can only be executed by minter account.

Params:

```json
MintMsg {
    "to": "<address>",
    "token_info": {
        "token_id": 1,
        "amount": 1,
        "token_uri": "<token-uri>",
    }
}
```

## execute_batch_mint

Batch mint a new token. Can only be executed by minter account.

Params:

```json
BatchMintMsg {
    "to": "<address>",
    "token_infos": [
        {
            "token_id": 1,
            "amount": 1,
            "token_uri": "<token-uri>",
        }
    ]
}
```

## execute_transfer_from

Only with approval extension. Transfer token from owner to spender.

Params:

```json
TransferFromMsg {
    "from": "<address>",
    "to": "<address>",
    "token_info": {
        "token_id": 1,
        "amount": 1,
    },
}
```

## execute_batch_transfer_from

Only with approval extension. Batch transfer token from owner to spender.

Params:

```json
BatchTransferFromMsg {
    "from": "<address>",
    "to": "<address>",
    "token_infos": [
        {
            "token_id": 1,
            "amount": 1,
        }
    ],
}
```

## execute_burn

Destroy your token forever.

Params:

```json
BurnMsg {
    "from": "<address>",
    "token_info": {
        "token_id": 1,
        "amount": 1,
    },
}
```

## execute_batch_burn

Batch destroy your token forever.

Params:

```json
BatchBurnMsg {
    "from": "<address>",
    "token_infos": [
        {
            "token_id": 1,
            "amount": 1,
        }
    ],
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

## execute_revoke_for_all

Remove operator.

Params:

```json
RevokeForAllMsg {
    "operator": "<address>",
}
```
