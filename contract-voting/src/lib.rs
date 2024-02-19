#![doc = include_str!("../README.md")]
#![allow(unused_variables)]

#[macro_use]
extern crate pbc_contract_codegen;
extern crate contract_version_base;
extern crate pbc_contract_common;

use contract_version_base::state::ContractVersionBase;
use pbc_contract_common::address::Address;
use pbc_contract_common::avl_tree_map::AvlTreeMap;
use pbc_contract_common::context::ContractContext;
use pbc_contract_common::sorted_vec_map::SortedVecSet;

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The state of the vote, which is persisted on-chain.
#[state]
pub struct VoteState {
    /// Owner of the voting contract.
    pub owner: Address,
    /// Identification of the proposal being voted for.
    pub proposal_id: u64,
    /// The list of eligible voters.
    pub voters: SortedVecSet<Address>,
    /// The deadline of the vote in UTC millis
    /// (milliseconds after 1970-01-01 00:00:00 UTC)
    pub deadline_utc_millis: i64,
    /// The votes cast by the voters.
    /// true is for the proposal, false is against.
    pub votes: AvlTreeMap<Address, bool>,
    /// The result of the vote.
    /// None until the votes has been counted,
    /// Some(true) if the proposal passed,
    /// Some(false) if the proposal failed.
    pub result: Option<bool>,
    pub version: ContractVersionBase,
}

/// Initialize a new vote for a proposal
///
/// # Arguments
///
/// * `_ctx` - the contract context containing information about the sender and the blockchain.
/// * `proposal_id` - the id of the proposal.
/// * `voters` - the list of eligible voters.
/// * `deadline_utc_millis` - deadline of the vote in UTC millis.
///
/// # Returns
///
/// The initial state of the vote.
///
#[init]
pub fn initialize(
    ctx: ContractContext,
    proposal_id: u64,
    voters: Vec<Address>,
    deadline_utc_millis: i64,
) -> VoteState {
    assert_ne!(voters.len(), 0, "Voters are required");
    let unique_voters: SortedVecSet<Address> = voters.iter().cloned().collect();
    assert_eq!(
        voters.len(),
        unique_voters.len(),
        "All voters must be unique"
    );
    VoteState {
        owner: ctx.sender,
        proposal_id,
        voters: unique_voters,
        deadline_utc_millis,
        votes: AvlTreeMap::new(),
        result: None,
        version: ContractVersionBase::new(CONTRACT_NAME, CONTRACT_VERSION),
    }
}

/// Cast a vote for the proposal.
/// The vote is cast by the sender of the action.
/// Voters can cast and update their vote until the deadline.
#[action(shortname = 0x01)]
pub fn vote(ctx: ContractContext, mut state: VoteState, vote: bool) -> VoteState {
    assert!(
        state.result.is_none() && ctx.block_production_time < state.deadline_utc_millis,
        "The deadline has passed"
    );
    assert!(state.voters.contains(&ctx.sender), "Not an eligible voter");
    state.votes.insert(ctx.sender, vote);
    state
}

/// Count the votes and publish the result.
/// Counting will fail if the deadline has not passed.
#[action(shortname = 0x02)]
pub fn count(ctx: ContractContext, mut state: VoteState) -> VoteState {
    assert_eq!(state.result, None, "The votes have already been counted");
    assert!(
        ctx.block_production_time >= state.deadline_utc_millis,
        "The deadline has not yet passed"
    );
    let voters_approving = state.votes.iter().filter(|(_, v)| *v).count();
    let vote_passed = voters_approving > state.voters.len() / 2;
    state.result = Some(vote_passed);
    state
}

/// Add voters to the list of eligible voters.
/// Voters can be added until the deadline.
#[action(shortname = 0x03)]
pub fn add_voters(ctx: ContractContext, mut state: VoteState, voters: Vec<Address>) -> VoteState {
    assert_eq!(ctx.sender, state.owner, "Only the owner can add voters");
    assert_eq!(state.result, None, "The votes have already been counted");

    let mut unique_voters: SortedVecSet<Address> = voters.iter().cloned().collect();

    state.voters.append(&mut unique_voters);
    state
}

/// Remove voters from the list of eligible voters.
/// Voters can be removed until the deadline.
#[action(shortname = 0x04)]
pub fn remove_voters(
    ctx: ContractContext,
    mut state: VoteState,
    voters: Vec<Address>,
) -> VoteState {
    assert_eq!(ctx.sender, state.owner, "Only the owner can add voters");
    assert_eq!(state.result, None, "The votes have already been counted");

    let voters_to_remove: SortedVecSet<Address> = voters.iter().cloned().collect();

    state.voters.retain(|v| !voters_to_remove.contains(v));
    for key in voters_to_remove.iter() {
        state.votes.remove(key);
    }
    state
}
