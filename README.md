# Meta Names Smart Contracts
This repository holds the Meta Names smart contracts and is organized into several packages:
- `contract`: contains the main smart contract
- `contract-proxy`: Implements a proxy contract for simplified deployment
- `contract-voting`: Implements a voting contract for proposals
- `contract-version-base`: implemnets versioning each package
- `access-control`: implements access control
- `nft`: implements MPC721 standard
- `partisia-name-system`: implements the DNS for partisia
- `rpc-msg-derive`: implements remote procedure call for msg
- `utils`: various utilities

All packages are rigorously tested using unit and integration tests via Cucumber.

## Deployments
Testnet: `020f77781340c36be023184bc9d69f29c928304d27`

Mainnet: `02bb03946a0b6d1feaa96f78cfc8ef3f5a4ceee727`

## SDK
Use the [Meta Names SDK](https://github.com/MetaNames/sdk) to interact with the contract.

## Security
If you find a security issue, email us at `metanames@proton.me`.

## Contributing
Contributions are welcome! Feel free to fork the project, and create a pull request.
