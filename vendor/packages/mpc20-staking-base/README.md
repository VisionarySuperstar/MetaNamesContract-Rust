# MPC20-Staking-Base Contract

Base implementation of MPC20-STAKING contract.

# Actions

## execute_stake

Stake specified amount of tokens to earn rewards.

Pararms:

```json
StakeMsg {
    amount: 10,
}
```

## execute_unstake

Withdraw staked tokens.

Pararms:

```json
UnstakeMsg {
    amount: 11,
}
```

## execute_claim

Claim earned rewards.

Pararms:

```json
ClaimMsg {
    amount: 10 | null
}
```

## execute_compound

Compound earned rewards(e.g. stake them).
Only works when deposit token is reward token.

Pararms:

```json
CompoundMsg {
    amount: 10 | null
}
```

## [MPC20 Base actions](https://github.com/partisiablockchainapplications/CoreContracts/blob/master/packages/mpc20-base/README.md)
