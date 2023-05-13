# Partisia Name System

Crate which allows contracts to implement a domain name system.

It uses a fork of MPC 721 to permit hierarchy.

## Actions
TODO: Update

- `transfer(to: Address, token_id: u128)`
- `transfer_from(from: Address, to: Address, token_id: u128)`
- `approve(spender: Address, token_id: u128)`
- `set_base_uri(new_base_uri: String)`
- `mint(token_id: u128, to: Address, token_uri: Option\<String\>)`

## State Fields
TODO: Update

Minimal State struct in Json format:

```json
{
    "mpc721": {
        "owner": "<address>" | null,
        "name": "nft_name",
        "symbol": "nft_symbol",
        "base_uri": "nft_base_uri" | null,
        "minter": "<address>",
        "supply": 1,
        "tokens": [
            {
                "key": 1, // token_id
                "value": {
                    "owner": "<token_owner_address>",
                    "approvals": [
                        "<approved_address_1>",
                        "<approved_address_2>",
                    ],
                    "token_uri": "token_uri" | null,
}
}
```
