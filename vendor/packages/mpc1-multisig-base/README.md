# MPC1-MULTISIG-Base Contract

Base implementation of MPC1-MULTISIG contract.

# Actions

## execute_create_proposal

Creates a new proposal.

Pararms:

```json
CreateProposalMsg {
    "title": "Title",
    "description": "Decsription",
    "voting_phase_period": 86400,
    "calls": [
        "contract": "<contract_address>",
        "method_name": "transfer",
        "base64_encoded_payload": "<base64_encoded_msg>"
    ]
}
```

## execute_vote

Performes a yes or no vote for specified proposal.

Pararms:

```json
ProposalVoteMsg {
    "proposal_id": 1,
    "vote": 1,
}
```

## execute_execute_proposal

Executes proposal if accepted.

Pararms:

```json
ProposalExecuteMsg {
    "proposal_id": 1,
}
```

## execute_close_proposal

Closes proposal if expired or threshold was not reached.

Pararms:

```json
ProposalCloseMsg {
    "proposal_id": 1,
}
```
