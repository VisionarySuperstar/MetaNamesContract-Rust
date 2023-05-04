use std::collections::BTreeMap;

use pbc_contract_common::{address::Address, context::ContractContext, events::EventGroup};

use crate::{
    msg::{CreateProposalMsg, InitMsg, ProposalCloseMsg, ProposalExecuteMsg, ProposalVoteMsg},
    state::{
        Ballot, MPC1MultisigContractState, Proposal, ProposalExecuteCall, SubmittedVotes,
        ACCEPTED_STATUS, EXECUTED_STATUS, REJECTED_STATUS, VOTING_PHASE_STATUS, YES_VOTE,
    },
    ContractError,
};

/// ## Description
/// Inits contract state.
/// Returns [`(MPC1MultisigContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **_ctx** is an object of type [`ContractContext`]
///
/// * **msg** is an object of type [`InitMsg`]
pub fn execute_init(
    _ctx: &ContractContext,
    msg: &InitMsg,
) -> (MPC1MultisigContractState, Vec<EventGroup>) {
    assert!(
        !msg.members.is_empty(),
        "{}",
        ContractError::MembersListIsEmpty
    );
    assert!(
        msg.threshold_weight != 0,
        "{}",
        ContractError::RequiredWeightIsZero
    );

    let total_weight = msg.members.iter().map(|m| m.weight).sum();
    assert!(
        msg.threshold_weight <= total_weight,
        "{}",
        ContractError::UnreachableWeight
    );

    let mut members: BTreeMap<Address, u64> = BTreeMap::new();
    for member in msg.members.iter() {
        assert!(
            !members.contains_key(&member.address),
            "{}",
            ContractError::DuplicatedMember
        );
        assert!(member.weight > 0, "{}", ContractError::InvalidVotingPower);

        members.insert(member.address, member.weight);
    }

    let state = MPC1MultisigContractState {
        members,
        threshold_weight: msg.threshold_weight,
        total_weight,
        voting_phase_period: msg.voting_phase_period,
        proposals_count: 0,
        proposals: BTreeMap::new(),
    };

    (state, vec![])
}

/// ## Description
/// Creates a new proposal.
/// Returns [`(MPC1MultisigContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC1MultisigContractState`]
///
/// * **msg** is an object of type [`CreateProposalMsg`]
pub fn execute_create_proposal(
    ctx: &ContractContext,
    state: &mut MPC1MultisigContractState,
    msg: &CreateProposalMsg,
) -> Vec<EventGroup> {
    assert!(
        state.members.contains_key(&ctx.sender),
        "{}",
        ContractError::Unauthorized
    );
    let member_power = *state.members.get(&ctx.sender).unwrap();

    let max_voting_phase = ctx.block_production_time as u64 + state.voting_phase_period;
    let voting_phase_end = if let Some(period) = msg.voting_phase_period {
        let voting_phase = ctx.block_production_time as u64 + period;
        assert!(
            voting_phase <= max_voting_phase,
            "{}",
            ContractError::InvalidVotingPhase
        );
        voting_phase
    } else {
        max_voting_phase
    };

    assert!(
        !msg.calls.is_empty(),
        "{}",
        ContractError::EmptyExecuteCallsList
    );

    let execute_calls: Vec<ProposalExecuteCall> = msg
        .calls
        .iter()
        .map(|call| ProposalExecuteCall {
            contract: call.contract,
            payload: base64::decode(&call.base64_encoded_payload).unwrap(),
        })
        .collect();

    state.save_proposal(&Proposal {
        title: msg.title.clone(),
        description: msg.description.clone(),
        expires_at: voting_phase_end,
        execute_calls,
        status: VOTING_PHASE_STATUS,
        threshold_weight: state.threshold_weight,
        total_weight: state.total_weight,
        votes: SubmittedVotes::yes(member_power),
        ballots: vec![Ballot {
            member: ctx.sender,
            vote: YES_VOTE,
            weight: member_power,
        }],
    });

    vec![]
}

/// ## Description
/// Performs a yes or no vote for specified proposal.
/// Returns [`(MPC1MultisigContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC1MultisigContractState`]
///
/// * **msg** is an object of type [`ProposalVoteMsg`]
pub fn execute_vote(
    ctx: &ContractContext,
    state: &mut MPC1MultisigContractState,
    msg: &ProposalVoteMsg,
) -> Vec<EventGroup> {
    assert!(
        state.members.contains_key(&ctx.sender),
        "{}",
        ContractError::Unauthorized
    );
    let member_power = *state.members.get(&ctx.sender).unwrap();

    assert!(
        state.proposals.contains_key(&msg.proposal_id),
        "{}",
        ContractError::ProposalNotFound
    );

    let proposal = state.proposals.get_mut(&msg.proposal_id).unwrap();
    assert!(
        proposal.status == VOTING_PHASE_STATUS,
        "{}",
        ContractError::ProposalIsNotInTheVotingPhase
    );
    assert!(
        proposal.not_expired(ctx.block_production_time as u64),
        "{}",
        ContractError::Expired
    );
    assert!(
        proposal.not_voted(&ctx.sender),
        "{}",
        ContractError::AlreadyVoted
    );

    proposal.register_vote(&ctx.sender, msg.vote, member_power);
    proposal.update_status(ctx.block_production_time as u64);

    vec![]
}

/// ## Description
/// Executes proposal if accepted.
/// Returns [`(MPC1MultisigContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC1MultisigContractState`]
///
/// * **msg** is an object of type [`ProposalExecuteMsg`]
#[allow(deprecated)]
pub fn execute_execute_proposal(
    ctx: &ContractContext,
    state: &mut MPC1MultisigContractState,
    msg: &ProposalExecuteMsg,
) -> Vec<EventGroup> {
    assert!(
        state.members.contains_key(&ctx.sender),
        "{}",
        ContractError::Unauthorized
    );
    assert!(
        state.proposals.contains_key(&msg.proposal_id),
        "{}",
        ContractError::ProposalNotFound
    );

    let proposal = state.proposals.get_mut(&msg.proposal_id).unwrap();

    assert!(
        proposal.status == ACCEPTED_STATUS,
        "{}",
        ContractError::ProposalIsNotAcceptedOrRejected
    );
    proposal.mark_executed();

    let mut event_group = EventGroup::new();
    for call in proposal.execute_calls.iter() {
        event_group.send_from_contract(&call.contract, call.payload.clone(), None);
    }

    vec![event_group]
}

/// ## Description
/// Closes proposal if expired or threshold was not reached.
/// Returns [`(MPC1MultisigContractState, Vec<EventGroup>)`] if operation was successful,
/// otherwise panics with error message defined in [`ContractError`]
/// ## Params
/// * **ctx** is an object of type [`ContractContext`]
///
/// * **state** is an object of type [`MPC1MultisigContractState`]
///
/// * **msg** is an object of type [`ProposalCloseMsg`]
pub fn execute_close_proposal(
    ctx: &ContractContext,
    state: &mut MPC1MultisigContractState,
    msg: &ProposalCloseMsg,
) -> Vec<EventGroup> {
    assert!(
        state.members.contains_key(&ctx.sender),
        "{}",
        ContractError::Unauthorized
    );
    assert!(
        state.proposals.contains_key(&msg.proposal_id),
        "{}",
        ContractError::ProposalNotFound
    );

    let proposal = state.proposals.get_mut(&msg.proposal_id).unwrap();

    assert!(
        ![ACCEPTED_STATUS, REJECTED_STATUS, EXECUTED_STATUS,]
            .iter()
            .any(|s| *s == proposal.status),
        "{}",
        ContractError::WrongCloseStatus,
    );
    assert!(
        !proposal.not_expired(ctx.block_production_time as u64),
        "{}",
        ContractError::ProposalNotExpired
    );

    proposal.mark_rejected();

    vec![]
}
