use std::collections::BTreeMap;

use pbc_contract_common::{
    address::{Address, AddressType},
    context::ContractContext,
    events::EventGroup,
};

use crate::{
    actions::{
        execute_close_proposal, execute_create_proposal, execute_execute_proposal, execute_init,
        execute_vote,
    },
    msg::{
        CreateProposalMsg, InitMsg, MultisigMember, ProposalCloseMsg, ProposalExecuteCallMsg,
        ProposalExecuteMsg, ProposalVoteMsg,
    },
    state::{
        Ballot, MPC1MultisigContractState, Proposal, ProposalExecuteCall, SubmittedVotes,
        EXECUTED_STATUS, REJECTED_STATUS, VOTING_PHASE_STATUS, YES_VOTE,
    },
};

fn mock_address(le: u8) -> Address {
    Address {
        address_type: AddressType::Account,
        identifier: [
            le, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8,
        ],
    }
}

fn mock_contract_context(sender: u8) -> ContractContext {
    ContractContext {
        contract_address: mock_address(1u8),
        sender: mock_address(sender),
        block_time: 100,
        block_production_time: 100,
        current_transaction: [
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        ],
        original_transaction: [
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
            0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        ],
    }
}

fn mock_transfer_base64_payload() -> String {
    "yu3VvwIACQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABk".to_string()
}

fn mock_transfer_payload_with_name_bytes() -> Vec<u8> {
    vec![
        202, 237, 213, 191, 2, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 100,
    ]
}

#[test]
fn proper_execute_init() {
    let msg = InitMsg {
        members: vec![
            MultisigMember {
                address: mock_address(1),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(2),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(3),
                weight: 1,
            },
        ],
        threshold_weight: 2,
        voting_phase_period: 86400,
    };

    let (state, events) = execute_init(&mock_contract_context(1), &msg);
    assert_eq!(events.len(), 0);
    assert_eq!(
        state,
        MPC1MultisigContractState {
            members: BTreeMap::from([
                (mock_address(1), 1),
                (mock_address(2), 1),
                (mock_address(3), 1),
            ]),
            threshold_weight: 2,
            total_weight: 3,
            voting_phase_period: 86400,
            proposals_count: 0,
            proposals: BTreeMap::new(),
        }
    )
}

#[test]
#[should_panic(expected = "Members list is empty")]
fn empty_members_list_on_init() {
    let msg = InitMsg {
        members: vec![],
        threshold_weight: 2,
        voting_phase_period: 86400,
    };

    let (_, _) = execute_init(&mock_contract_context(1), &msg);
}

#[test]
#[should_panic(expected = "Required weight is zero")]
fn zero_threshold_weight_on_init() {
    let msg = InitMsg {
        members: vec![
            MultisigMember {
                address: mock_address(1),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(2),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(3),
                weight: 1,
            },
        ],
        threshold_weight: 0,
        voting_phase_period: 86400,
    };

    let (_, _) = execute_init(&mock_contract_context(1), &msg);
}

#[test]
#[should_panic(expected = "Unreachable weight")]
fn unreachable_weight_on_init() {
    let msg = InitMsg {
        members: vec![
            MultisigMember {
                address: mock_address(1),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(2),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(3),
                weight: 1,
            },
        ],
        threshold_weight: 5,
        voting_phase_period: 86400,
    };

    let (_, _) = execute_init(&mock_contract_context(1), &msg);
}

#[test]
#[should_panic(expected = "Duplicated member")]
fn duplicated_member_on_init() {
    let msg = InitMsg {
        members: vec![
            MultisigMember {
                address: mock_address(1),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(2),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(2),
                weight: 1,
            },
        ],
        threshold_weight: 2,
        voting_phase_period: 86400,
    };

    let (_, _) = execute_init(&mock_contract_context(1), &msg);
}

#[test]
#[should_panic(expected = "Invalid voting power(weight)")]
fn zero_member_weight_on_init() {
    let msg = InitMsg {
        members: vec![
            MultisigMember {
                address: mock_address(1),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(2),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(3),
                weight: 0,
            },
        ],
        threshold_weight: 2,
        voting_phase_period: 86400,
    };

    let (_, _) = execute_init(&mock_contract_context(1), &msg);
}

#[test]
fn proper_create_proposal() {
    let msg = InitMsg {
        members: vec![
            MultisigMember {
                address: mock_address(1),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(2),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(3),
                weight: 1,
            },
        ],
        threshold_weight: 2,
        voting_phase_period: 86400,
    };

    let (mut state, events) = execute_init(&mock_contract_context(1), &msg);

    let create_proposal_msg = CreateProposalMsg {
        title: "Proposal #1".to_string(),
        description: "Description".to_string(),
        voting_phase_period: None,
        calls: vec![ProposalExecuteCallMsg {
            contract: mock_address(20),
            base64_encoded_payload: mock_transfer_base64_payload(),
        }],
    };
    let events =
        execute_create_proposal(&mock_contract_context(1), &mut state, &create_proposal_msg);
    assert_eq!(
        state,
        MPC1MultisigContractState {
            members: BTreeMap::from([
                (mock_address(1), 1),
                (mock_address(2), 1),
                (mock_address(3), 1),
            ]),
            threshold_weight: 2,
            total_weight: 3,
            voting_phase_period: 86400,
            proposals_count: 1,
            proposals: BTreeMap::from([(
                1,
                Proposal {
                    title: "Proposal #1".to_string(),
                    description: "Description".to_string(),
                    expires_at: 86500,
                    execute_calls: vec![ProposalExecuteCall {
                        contract: mock_address(20),
                        payload: mock_transfer_payload_with_name_bytes(),
                    }],
                    status: VOTING_PHASE_STATUS,
                    threshold_weight: 2,
                    total_weight: 3,
                    votes: SubmittedVotes { yes: 1, no: 0 },
                    ballots: vec![Ballot {
                        member: mock_address(1),
                        vote: YES_VOTE,
                        weight: 1,
                    }],
                }
            )]),
        }
    );
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn unauthorized_member_on_create_proposal() {
    let msg = InitMsg {
        members: vec![
            MultisigMember {
                address: mock_address(1),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(2),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(3),
                weight: 1,
            },
        ],
        threshold_weight: 2,
        voting_phase_period: 86400,
    };

    let (mut state, events) = execute_init(&mock_contract_context(1), &msg);

    let create_proposal_msg = CreateProposalMsg {
        title: "Proposal #1".to_string(),
        description: "Description".to_string(),
        voting_phase_period: None,
        calls: vec![ProposalExecuteCallMsg {
            contract: mock_address(20),
            base64_encoded_payload: mock_transfer_base64_payload(),
        }],
    };
    let _ = execute_create_proposal(&mock_contract_context(4), &mut state, &create_proposal_msg);
}

#[test]
#[should_panic(expected = "Invalid voting phase period")]
fn invalid_voting_phase_on_create_proposal() {
    let msg = InitMsg {
        members: vec![
            MultisigMember {
                address: mock_address(1),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(2),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(3),
                weight: 1,
            },
        ],
        threshold_weight: 2,
        voting_phase_period: 86400,
    };

    let (mut state, events) = execute_init(&mock_contract_context(1), &msg);

    let create_proposal_msg = CreateProposalMsg {
        title: "Proposal #1".to_string(),
        description: "Description".to_string(),
        voting_phase_period: Some(100_000),
        calls: vec![ProposalExecuteCallMsg {
            contract: mock_address(20),
            base64_encoded_payload: mock_transfer_base64_payload(),
        }],
    };
    let _ = execute_create_proposal(&mock_contract_context(1), &mut state, &create_proposal_msg);
}

#[test]
#[should_panic(expected = "Empty execute calls list")]
fn empty_execute_calls_list_on_create_proposal() {
    let msg = InitMsg {
        members: vec![
            MultisigMember {
                address: mock_address(1),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(2),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(3),
                weight: 1,
            },
        ],
        threshold_weight: 2,
        voting_phase_period: 86400,
    };

    let (mut state, events) = execute_init(&mock_contract_context(1), &msg);

    let create_proposal_msg = CreateProposalMsg {
        title: "Proposal #1".to_string(),
        description: "Description".to_string(),
        voting_phase_period: None,
        calls: vec![],
    };
    let _ = execute_create_proposal(&mock_contract_context(1), &mut state, &create_proposal_msg);
}

#[test]
fn proper_vote() {
    let msg = InitMsg {
        members: vec![
            MultisigMember {
                address: mock_address(1),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(2),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(3),
                weight: 1,
            },
        ],
        threshold_weight: 3,
        voting_phase_period: 86400,
    };

    let (mut state, events) = execute_init(&mock_contract_context(1), &msg);

    let create_proposal_msg = CreateProposalMsg {
        title: "Proposal #1".to_string(),
        description: "Description".to_string(),
        voting_phase_period: None,
        calls: vec![ProposalExecuteCallMsg {
            contract: mock_address(20),
            base64_encoded_payload: mock_transfer_base64_payload(),
        }],
    };
    let events =
        execute_create_proposal(&mock_contract_context(1), &mut state, &create_proposal_msg);

    let vote_msg = ProposalVoteMsg {
        proposal_id: 1,
        vote: YES_VOTE,
    };
    let events = execute_vote(&mock_contract_context(2), &mut state, &vote_msg);
    assert_eq!(
        state,
        MPC1MultisigContractState {
            members: BTreeMap::from([
                (mock_address(1), 1),
                (mock_address(2), 1),
                (mock_address(3), 1),
            ]),
            threshold_weight: 3,
            total_weight: 3,
            voting_phase_period: 86400,
            proposals_count: 1,
            proposals: BTreeMap::from([(
                1,
                Proposal {
                    title: "Proposal #1".to_string(),
                    description: "Description".to_string(),
                    expires_at: 86500,
                    execute_calls: vec![ProposalExecuteCall {
                        contract: mock_address(20),
                        payload: mock_transfer_payload_with_name_bytes(),
                    }],
                    status: VOTING_PHASE_STATUS,
                    threshold_weight: 3,
                    total_weight: 3,
                    votes: SubmittedVotes { yes: 2, no: 0 },
                    ballots: vec![
                        Ballot {
                            member: mock_address(1),
                            vote: YES_VOTE,
                            weight: 1,
                        },
                        Ballot {
                            member: mock_address(2),
                            vote: YES_VOTE,
                            weight: 1,
                        }
                    ],
                }
            )]),
        }
    );
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn unauthorized_member_on_vote() {
    let msg = InitMsg {
        members: vec![
            MultisigMember {
                address: mock_address(1),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(2),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(3),
                weight: 1,
            },
        ],
        threshold_weight: 3,
        voting_phase_period: 86400,
    };

    let (mut state, events) = execute_init(&mock_contract_context(1), &msg);

    let create_proposal_msg = CreateProposalMsg {
        title: "Proposal #1".to_string(),
        description: "Description".to_string(),
        voting_phase_period: None,
        calls: vec![ProposalExecuteCallMsg {
            contract: mock_address(20),
            base64_encoded_payload: mock_transfer_base64_payload(),
        }],
    };
    let events =
        execute_create_proposal(&mock_contract_context(1), &mut state, &create_proposal_msg);

    let vote_msg = ProposalVoteMsg {
        proposal_id: 1,
        vote: YES_VOTE,
    };
    let events = execute_vote(&mock_contract_context(4), &mut state, &vote_msg);
}

#[test]
#[should_panic(expected = "Proposal not found")]
fn proposal_not_found_on_vote() {
    let msg = InitMsg {
        members: vec![
            MultisigMember {
                address: mock_address(1),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(2),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(3),
                weight: 1,
            },
        ],
        threshold_weight: 3,
        voting_phase_period: 86400,
    };

    let (mut state, events) = execute_init(&mock_contract_context(1), &msg);

    let create_proposal_msg = CreateProposalMsg {
        title: "Proposal #1".to_string(),
        description: "Description".to_string(),
        voting_phase_period: None,
        calls: vec![ProposalExecuteCallMsg {
            contract: mock_address(20),
            base64_encoded_payload: mock_transfer_base64_payload(),
        }],
    };
    let events =
        execute_create_proposal(&mock_contract_context(1), &mut state, &create_proposal_msg);

    let vote_msg = ProposalVoteMsg {
        proposal_id: 2,
        vote: YES_VOTE,
    };
    let events = execute_vote(&mock_contract_context(2), &mut state, &vote_msg);
}

#[test]
#[should_panic(expected = "Proposal is not in the voting phase")]
fn invalid_voting_phase_on_vote() {
    let msg = InitMsg {
        members: vec![
            MultisigMember {
                address: mock_address(1),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(2),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(3),
                weight: 1,
            },
        ],
        threshold_weight: 3,
        voting_phase_period: 86400,
    };

    let (mut state, events) = execute_init(&mock_contract_context(1), &msg);

    let create_proposal_msg = CreateProposalMsg {
        title: "Proposal #1".to_string(),
        description: "Description".to_string(),
        voting_phase_period: None,
        calls: vec![ProposalExecuteCallMsg {
            contract: mock_address(20),
            base64_encoded_payload: mock_transfer_base64_payload(),
        }],
    };
    let events =
        execute_create_proposal(&mock_contract_context(1), &mut state, &create_proposal_msg);
    state.proposals.get_mut(&1).unwrap().status = EXECUTED_STATUS;

    let vote_msg = ProposalVoteMsg {
        proposal_id: 1,
        vote: YES_VOTE,
    };
    let events = execute_vote(&mock_contract_context(2), &mut state, &vote_msg);
}

#[test]
#[should_panic(expected = "Proposal voting phase has expired")]
fn proposal_expired_on_vote() {
    let msg = InitMsg {
        members: vec![
            MultisigMember {
                address: mock_address(1),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(2),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(3),
                weight: 1,
            },
        ],
        threshold_weight: 3,
        voting_phase_period: 86400,
    };

    let (mut state, events) = execute_init(&mock_contract_context(1), &msg);

    let create_proposal_msg = CreateProposalMsg {
        title: "Proposal #1".to_string(),
        description: "Description".to_string(),
        voting_phase_period: None,
        calls: vec![ProposalExecuteCallMsg {
            contract: mock_address(20),
            base64_encoded_payload: mock_transfer_base64_payload(),
        }],
    };
    let events =
        execute_create_proposal(&mock_contract_context(1), &mut state, &create_proposal_msg);

    let vote_msg = ProposalVoteMsg {
        proposal_id: 1,
        vote: YES_VOTE,
    };

    let mut context = mock_contract_context(2);
    context.block_production_time = 86501;
    let events = execute_vote(&context, &mut state, &vote_msg);
}

#[test]
#[should_panic(expected = "Member has already voted")]
fn member_already_voted_on_vote() {
    let msg = InitMsg {
        members: vec![
            MultisigMember {
                address: mock_address(1),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(2),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(3),
                weight: 1,
            },
        ],
        threshold_weight: 3,
        voting_phase_period: 86400,
    };

    let (mut state, events) = execute_init(&mock_contract_context(1), &msg);

    let create_proposal_msg = CreateProposalMsg {
        title: "Proposal #1".to_string(),
        description: "Description".to_string(),
        voting_phase_period: None,
        calls: vec![ProposalExecuteCallMsg {
            contract: mock_address(20),
            base64_encoded_payload: mock_transfer_base64_payload(),
        }],
    };
    let events =
        execute_create_proposal(&mock_contract_context(1), &mut state, &create_proposal_msg);

    let vote_msg = ProposalVoteMsg {
        proposal_id: 1,
        vote: YES_VOTE,
    };
    let events = execute_vote(&mock_contract_context(1), &mut state, &vote_msg);
}

#[allow(deprecated)]
#[test]
fn proper_execute_proposal() {
    let msg = InitMsg {
        members: vec![
            MultisigMember {
                address: mock_address(1),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(2),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(3),
                weight: 1,
            },
        ],
        threshold_weight: 2,
        voting_phase_period: 86400,
    };

    let (mut state, events) = execute_init(&mock_contract_context(1), &msg);

    let create_proposal_msg = CreateProposalMsg {
        title: "Proposal #1".to_string(),
        description: "Description".to_string(),
        voting_phase_period: None,
        calls: vec![ProposalExecuteCallMsg {
            contract: mock_address(20),
            base64_encoded_payload: mock_transfer_base64_payload(),
        }],
    };
    let events =
        execute_create_proposal(&mock_contract_context(1), &mut state, &create_proposal_msg);

    let vote_msg = ProposalVoteMsg {
        proposal_id: 1,
        vote: YES_VOTE,
    };
    let events = execute_vote(&mock_contract_context(2), &mut state, &vote_msg);

    let execute_proposal_msg = ProposalExecuteMsg { proposal_id: 1 };
    let events =
        execute_execute_proposal(&mock_contract_context(1), &mut state, &execute_proposal_msg);
    assert_eq!(
        state,
        MPC1MultisigContractState {
            members: BTreeMap::from([
                (mock_address(1), 1),
                (mock_address(2), 1),
                (mock_address(3), 1),
            ]),
            threshold_weight: 2,
            total_weight: 3,
            voting_phase_period: 86400,
            proposals_count: 1,
            proposals: BTreeMap::from([(
                1,
                Proposal {
                    title: "Proposal #1".to_string(),
                    description: "Description".to_string(),
                    expires_at: 86500,
                    execute_calls: vec![ProposalExecuteCall {
                        contract: mock_address(20),
                        payload: mock_transfer_payload_with_name_bytes(),
                    }],
                    status: EXECUTED_STATUS,
                    threshold_weight: 2,
                    total_weight: 3,
                    votes: SubmittedVotes { yes: 2, no: 0 },
                    ballots: vec![
                        Ballot {
                            member: mock_address(1),
                            vote: YES_VOTE,
                            weight: 1,
                        },
                        Ballot {
                            member: mock_address(2),
                            vote: YES_VOTE,
                            weight: 1,
                        }
                    ],
                }
            )]),
        }
    );
    assert_eq!(events.len(), 1);

    let mut transfer_event = EventGroup::new();
    transfer_event.send_from_contract(
        &mock_address(20),
        mock_transfer_payload_with_name_bytes(),
        None,
    );
    assert_eq!(events[0], transfer_event);
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn unauthorized_member_on_execute_proposal() {
    let msg = InitMsg {
        members: vec![
            MultisigMember {
                address: mock_address(1),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(2),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(3),
                weight: 1,
            },
        ],
        threshold_weight: 2,
        voting_phase_period: 86400,
    };

    let (mut state, events) = execute_init(&mock_contract_context(1), &msg);

    let create_proposal_msg = CreateProposalMsg {
        title: "Proposal #1".to_string(),
        description: "Description".to_string(),
        voting_phase_period: None,
        calls: vec![ProposalExecuteCallMsg {
            contract: mock_address(20),
            base64_encoded_payload: mock_transfer_base64_payload(),
        }],
    };
    let events =
        execute_create_proposal(&mock_contract_context(1), &mut state, &create_proposal_msg);

    let vote_msg = ProposalVoteMsg {
        proposal_id: 1,
        vote: YES_VOTE,
    };
    let events = execute_vote(&mock_contract_context(2), &mut state, &vote_msg);

    let execute_proposal_msg = ProposalExecuteMsg { proposal_id: 1 };
    let events =
        execute_execute_proposal(&mock_contract_context(5), &mut state, &execute_proposal_msg);
}

#[test]
#[should_panic(expected = "Proposal not found")]
fn proposal_not_found_on_execute_proposal() {
    let msg = InitMsg {
        members: vec![
            MultisigMember {
                address: mock_address(1),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(2),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(3),
                weight: 1,
            },
        ],
        threshold_weight: 2,
        voting_phase_period: 86400,
    };

    let (mut state, events) = execute_init(&mock_contract_context(1), &msg);

    let create_proposal_msg = CreateProposalMsg {
        title: "Proposal #1".to_string(),
        description: "Description".to_string(),
        voting_phase_period: None,
        calls: vec![ProposalExecuteCallMsg {
            contract: mock_address(20),
            base64_encoded_payload: mock_transfer_base64_payload(),
        }],
    };
    let events =
        execute_create_proposal(&mock_contract_context(1), &mut state, &create_proposal_msg);

    let vote_msg = ProposalVoteMsg {
        proposal_id: 1,
        vote: YES_VOTE,
    };
    let events = execute_vote(&mock_contract_context(2), &mut state, &vote_msg);

    let execute_proposal_msg = ProposalExecuteMsg { proposal_id: 2 };
    let events =
        execute_execute_proposal(&mock_contract_context(1), &mut state, &execute_proposal_msg);
}

#[test]
#[should_panic(expected = "Proposal is not accepted yet or rejected")]
fn proposal_is_not_accepted_on_execute_proposal() {
    let msg = InitMsg {
        members: vec![
            MultisigMember {
                address: mock_address(1),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(2),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(3),
                weight: 1,
            },
        ],
        threshold_weight: 2,
        voting_phase_period: 86400,
    };

    let (mut state, events) = execute_init(&mock_contract_context(1), &msg);

    let create_proposal_msg = CreateProposalMsg {
        title: "Proposal #1".to_string(),
        description: "Description".to_string(),
        voting_phase_period: None,
        calls: vec![ProposalExecuteCallMsg {
            contract: mock_address(20),
            base64_encoded_payload: mock_transfer_base64_payload(),
        }],
    };
    let events =
        execute_create_proposal(&mock_contract_context(1), &mut state, &create_proposal_msg);

    let execute_proposal_msg = ProposalExecuteMsg { proposal_id: 1 };
    let events =
        execute_execute_proposal(&mock_contract_context(1), &mut state, &execute_proposal_msg);
}

#[test]
fn proper_close_proposal() {
    let msg = InitMsg {
        members: vec![
            MultisigMember {
                address: mock_address(1),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(2),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(3),
                weight: 1,
            },
        ],
        threshold_weight: 2,
        voting_phase_period: 86400,
    };

    let (mut state, events) = execute_init(&mock_contract_context(1), &msg);

    let create_proposal_msg = CreateProposalMsg {
        title: "Proposal #1".to_string(),
        description: "Description".to_string(),
        voting_phase_period: None,
        calls: vec![ProposalExecuteCallMsg {
            contract: mock_address(20),
            base64_encoded_payload: mock_transfer_base64_payload(),
        }],
    };
    let events =
        execute_create_proposal(&mock_contract_context(1), &mut state, &create_proposal_msg);

    let proposal_close_msg = ProposalCloseMsg { proposal_id: 1 };
    let mut context = mock_contract_context(1);
    context.block_production_time = 86501;
    let events = execute_close_proposal(&context, &mut state, &proposal_close_msg);
    assert_eq!(
        state,
        MPC1MultisigContractState {
            members: BTreeMap::from([
                (mock_address(1), 1),
                (mock_address(2), 1),
                (mock_address(3), 1),
            ]),
            threshold_weight: 2,
            total_weight: 3,
            voting_phase_period: 86400,
            proposals_count: 1,
            proposals: BTreeMap::from([(
                1,
                Proposal {
                    title: "Proposal #1".to_string(),
                    description: "Description".to_string(),
                    expires_at: 86500,
                    execute_calls: vec![ProposalExecuteCall {
                        contract: mock_address(20),
                        payload: mock_transfer_payload_with_name_bytes(),
                    }],
                    status: REJECTED_STATUS,
                    threshold_weight: 2,
                    total_weight: 3,
                    votes: SubmittedVotes { yes: 1, no: 0 },
                    ballots: vec![Ballot {
                        member: mock_address(1),
                        vote: YES_VOTE,
                        weight: 1,
                    }],
                }
            )]),
        }
    );
}

#[test]
#[should_panic(expected = "Unauthorized")]
fn unauthorized_member_on_close_proposal() {
    let msg = InitMsg {
        members: vec![
            MultisigMember {
                address: mock_address(1),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(2),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(3),
                weight: 1,
            },
        ],
        threshold_weight: 2,
        voting_phase_period: 86400,
    };

    let (mut state, events) = execute_init(&mock_contract_context(1), &msg);

    let create_proposal_msg = CreateProposalMsg {
        title: "Proposal #1".to_string(),
        description: "Description".to_string(),
        voting_phase_period: None,
        calls: vec![ProposalExecuteCallMsg {
            contract: mock_address(20),
            base64_encoded_payload: mock_transfer_base64_payload(),
        }],
    };
    let events =
        execute_create_proposal(&mock_contract_context(1), &mut state, &create_proposal_msg);

    let proposal_close_msg = ProposalCloseMsg { proposal_id: 1 };
    let mut context = mock_contract_context(5);
    context.block_production_time = 86501;
    let events = execute_close_proposal(&context, &mut state, &proposal_close_msg);
}

#[test]
#[should_panic(expected = "Proposal not found")]
fn proposal_not_found_on_close_proposal() {
    let msg = InitMsg {
        members: vec![
            MultisigMember {
                address: mock_address(1),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(2),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(3),
                weight: 1,
            },
        ],
        threshold_weight: 2,
        voting_phase_period: 86400,
    };

    let (mut state, events) = execute_init(&mock_contract_context(1), &msg);

    let create_proposal_msg = CreateProposalMsg {
        title: "Proposal #1".to_string(),
        description: "Description".to_string(),
        voting_phase_period: None,
        calls: vec![ProposalExecuteCallMsg {
            contract: mock_address(20),
            base64_encoded_payload: mock_transfer_base64_payload(),
        }],
    };
    let events =
        execute_create_proposal(&mock_contract_context(1), &mut state, &create_proposal_msg);

    let proposal_close_msg = ProposalCloseMsg { proposal_id: 2 };
    let mut context = mock_contract_context(1);
    context.block_production_time = 86501;
    let events = execute_close_proposal(&context, &mut state, &proposal_close_msg);
}

#[test]
#[should_panic(expected = "Cannot close executed or rejected proposal")]
fn wrong_close_status_on_close_proposal() {
    let msg = InitMsg {
        members: vec![
            MultisigMember {
                address: mock_address(1),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(2),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(3),
                weight: 1,
            },
        ],
        threshold_weight: 2,
        voting_phase_period: 86400,
    };

    let (mut state, events) = execute_init(&mock_contract_context(1), &msg);

    let create_proposal_msg = CreateProposalMsg {
        title: "Proposal #1".to_string(),
        description: "Description".to_string(),
        voting_phase_period: None,
        calls: vec![ProposalExecuteCallMsg {
            contract: mock_address(20),
            base64_encoded_payload: mock_transfer_base64_payload(),
        }],
    };
    let events =
        execute_create_proposal(&mock_contract_context(1), &mut state, &create_proposal_msg);

    let vote_msg = ProposalVoteMsg {
        proposal_id: 1,
        vote: YES_VOTE,
    };
    let events = execute_vote(&mock_contract_context(2), &mut state, &vote_msg);

    let proposal_close_msg = ProposalCloseMsg { proposal_id: 1 };
    let mut context = mock_contract_context(1);
    context.block_production_time = 86501;
    let events = execute_close_proposal(&context, &mut state, &proposal_close_msg);
}

#[test]
#[should_panic(expected = "Proposal not expired yet")]
fn proposal_not_expired_on_close_proposal() {
    let msg = InitMsg {
        members: vec![
            MultisigMember {
                address: mock_address(1),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(2),
                weight: 1,
            },
            MultisigMember {
                address: mock_address(3),
                weight: 1,
            },
        ],
        threshold_weight: 2,
        voting_phase_period: 86400,
    };

    let (mut state, events) = execute_init(&mock_contract_context(1), &msg);

    let create_proposal_msg = CreateProposalMsg {
        title: "Proposal #1".to_string(),
        description: "Description".to_string(),
        voting_phase_period: None,
        calls: vec![ProposalExecuteCallMsg {
            contract: mock_address(20),
            base64_encoded_payload: mock_transfer_base64_payload(),
        }],
    };
    let events =
        execute_create_proposal(&mock_contract_context(1), &mut state, &create_proposal_msg);

    let proposal_close_msg = ProposalCloseMsg { proposal_id: 1 };
    let mut context = mock_contract_context(1);
    context.block_production_time = 86499;
    let events = execute_close_proposal(&context, &mut state, &proposal_close_msg);
}
