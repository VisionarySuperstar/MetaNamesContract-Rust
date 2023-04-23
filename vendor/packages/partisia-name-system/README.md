# Partisia Name System

Crate which allows contracts to implement a domain name system.

It tries to follow the MPC721 standard.

# Actions

## execute_init

Init the PNS

Params:

```json
InitMsg {
    "owner": "<optional address>",
    "name": "<string>",
    "symbol": "<string>",
    "base": "<optional string>",
    "minter": "<address>",
}
```

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
    "token_id": "name.meta",
    "to": "<address>",
    "token_uri": "<optional uri>",
}
```

## execute_multi_mint

Mint a series of new tokens. Can only be executed by minter account.

Params:

```json
MultiMintMsg {
    "mints": ["<MintMsg>"]
}
```

## execute_record_mint

Mint a new record for a given token id. Can only be executed by owner account of token.

Params:

```json
RecordMintMsg {
    "token_id": "name.meta",
    "class": "<RecordClass>",
    "data": "<string>",
}
```


## execute_record_update

Update an existing record data given a token id and record class. Can only be executed by owner account of token.

Params:

```json
RecordUpdateMsg {
    "token_id": "name.meta",
    "class": "<RecordClass>",
    "data": "<string>",
}
```


## execute_record_delete

Delete an existing record given a token id and record class. Can only be executed by owner account of token.

Params:

```json
RecordDeleteMsg {
    "token_id": "name.meta",
    "class": "<RecordClass>",
}
```

## execute_transfer

Transfer token to another account.

Params:

```json
TransferMsg {
    "to": "<address>",
    "token_id": "name.meta",
}
```

## execute_transfer_from

Only with approval extension. Transfer token from owner to spender.

Params:

```json
TransferFromMsg {
    "from": "<address>",
    "to": "<address>",
    "token_id": "name.meta",
}
```

## execute_approve

Allows spender to transfer token from the owner account.

Params:

```json
ApproveMsg {
    "spedner": "<address>",
    "token_id": "name.meta",
}
```

## execute_ownership_check

Check the ownership of a token given an address.

Params:

```json
CheckOwnerMsg {
    "owner": "<address>",
    "token_id": "name.meta",
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
    "spender": "<address>",
    "token_id": "name.meta",
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
    "token_id": "name.meta",
}
```
