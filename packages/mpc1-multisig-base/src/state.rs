use std::collections::BTreeMap;

use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::Address;
use read_write_rpc_derive::ReadWriteRPC;
use read_write_state_derive::ReadWriteState;

/// ## Description
/// This structure describes main mpc1-multisig contract state.
#[derive(ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct MPC1MultisigContractState {
    /// multisig members
    pub members: BTreeMap<Address, u64>,
    /// required threshold
    pub threshold_weight: u64,
    /// total multisig weight
    pub total_weight: u64,
    /// voting period in UTC timestamp
    pub voting_phase_period: u64,
    /// proposals counter
    pub proposals_count: u64,
    /// proposal information by id
    pub proposals: BTreeMap<u64, Proposal>,
}

impl MPC1MultisigContractState {
    /// ## Description
    /// Stores newly created proposal
    /// ## Params
    /// * **proposal** is an object of type [`Proposal`]
    pub fn save_proposal(&mut self, proposal: &Proposal) {
        self.proposals_count += 1;
        self.proposals
            .insert(self.proposals_count, proposal.clone());
    }
}

/// ## Description
/// This structure describes proposal information
#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct Proposal {
    /// proposal title
    pub title: String,
    /// proposal description
    pub description: String,
    /// proposal expiration date
    pub expires_at: u64,
    /// messages to execute
    pub execute_calls: Vec<ProposalExecuteCall>,
    /// current proposal status
    pub status: ProposalStatus,
    /// required threshold
    pub threshold_weight: u64,
    /// total multisig weight
    pub total_weight: u64,
    /// information about submitted yes/no votes
    pub votes: SubmittedVotes,
    /// information about multisig members decisions
    pub ballots: Vec<Ballot>,
}

impl Proposal {
    /// ## Description
    /// Registeres new vote
    /// ## Params
    /// * **member** is an object of type [`Address`]
    ///
    /// * **vote** is an object of type [`Vote`]
    ///
    /// * **weight** is a field of type [`u64`]
    pub fn register_vote(&mut self, member: &Address, vote: Vote, weight: u64) {
        match vote {
            YES_VOTE => self.votes.yes += weight,
            NO_VOTE => self.votes.no += weight,
            _ => panic!("Unknown Vote type"),
        }

        self.ballots.push(Ballot {
            member: *member,
            vote,
            weight,
        });
    }

    /// ## Description
    /// Updates proposal status
    /// ## Params
    /// * **block_time** is a field of type [`u64`]
    pub fn update_status(&mut self, block_time: u64) {
        self.status = self.current_status(block_time)
    }

    /// ## Description
    /// Marks proposal as executed
    pub fn mark_executed(&mut self) {
        self.status = EXECUTED_STATUS
    }

    /// ## Description
    /// Marks proposal as rejected
    pub fn mark_rejected(&mut self) {
        self.status = REJECTED_STATUS
    }

    /// ## Description
    /// Checks that member is not voted yet
    /// ## Params
    /// * **member** is an object of type [`Address`]
    pub fn not_voted(&self, member: &Address) -> bool {
        !self.ballots.iter().any(|b| b.member == *member)
    }

    /// ## Description
    /// Checks that proposal is not expired yet
    /// ## Params
    /// * **block_time** is a field of type [`u64`]
    pub fn not_expired(&self, block_time: u64) -> bool {
        block_time < self.expires_at
    }

    /// ## Description
    /// Returns current proposal status
    /// ## Params
    /// * **block_time** is a field of type [`u64`]
    pub fn current_status(&self, block_time: u64) -> ProposalStatus {
        let mut status = self.status;
        if status == VOTING_PHASE_STATUS {
            if self.is_passed() {
                status = ACCEPTED_STATUS;
            }

            if self.is_rejected() || !self.not_expired(block_time) {
                status = REJECTED_STATUS;
            }
        }

        status
    }

    /// ## Description
    /// Checks passed proposal or not
    pub fn is_passed(&self) -> bool {
        self.votes.yes >= self.threshold_weight
    }

    /// ## Description
    /// Checks rejected proposal or not
    pub fn is_rejected(&self) -> bool {
        self.votes.no > (self.total_weight - self.threshold_weight)
    }
}

/// ## Description
/// This structure describes proposal execute call
#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct ProposalExecuteCall {
    /// contract to call
    pub contract: Address,
    /// msg payload
    pub payload: Vec<u8>,
}

/// ## Description
/// Defines a type for proposal status
pub type ProposalStatus = u8;
pub const VOTING_PHASE_STATUS: ProposalStatus = 1;
pub const ACCEPTED_STATUS: ProposalStatus = 2;
pub const REJECTED_STATUS: ProposalStatus = 3;
pub const EXECUTED_STATUS: ProposalStatus = 4;

/// ## Description
/// Defines a type for proposal vote
pub type Vote = u8;
pub const YES_VOTE: Vote = 1;
pub const NO_VOTE: Vote = 2;

/// ## Description
/// This structure describes submitted votes
#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct SubmittedVotes {
    /// amount of yes votes
    pub yes: u64,
    /// amount of no votes
    pub no: u64,
}

impl SubmittedVotes {
    /// ## Description
    /// Creates struct with submitted yes vote
    /// ## Params
    /// * **weight** is a field of type [`u64`]
    pub fn yes(weight: u64) -> Self {
        Self { yes: weight, no: 0 }
    }
}

/// ## Description
/// This structure describes ballot information
#[derive(ReadWriteRPC, ReadWriteState, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct Ballot {
    /// voted address
    pub member: Address,
    /// vote
    pub vote: Vote,
    /// multisig member weight
    pub weight: u64,
}
