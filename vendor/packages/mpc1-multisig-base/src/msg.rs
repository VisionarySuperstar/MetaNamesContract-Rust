use create_type_spec_derive::CreateTypeSpec;
use pbc_contract_common::address::{Address, Shortname};
use read_write_rpc_derive::ReadWriteRPC;

use crate::state::Vote;
use rpc_msg_derive::IntoShortnameRPCEvent;
use utils::events::IntoShortnameRPCEvent;

/// ## Description
/// This structure describes fields for mpc1-multisig initialize msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct MultisigMember {
    /// multisig member address
    pub address: Address,
    /// member weight
    pub weight: u64,
}

/// ## Description
/// This structure describes fields for mpc1-multisig initialize msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct InitMsg {
    /// multisig members
    pub members: Vec<MultisigMember>,
    /// required threshold
    pub threshold_weight: u64,
    /// voting phase period in UTC timestamp
    pub voting_phase_period: u64,
}

/// ## Description
/// This structure describes fields for mpc1-multisig create proposal msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x01)]
pub struct CreateProposalMsg {
    /// proposal title
    pub title: String,
    /// proposal description
    pub description: String,
    /// optional voting period
    pub voting_phase_period: Option<u64>,
    /// proposal calls to execute
    pub calls: Vec<ProposalExecuteCallMsg>,
}

/// ## Description
/// This structure describes fields for mpc1-multisig proposal execute call msg
#[derive(ReadWriteRPC, CreateTypeSpec, Clone, PartialEq, Eq, Debug)]
pub struct ProposalExecuteCallMsg {
    /// contract to call
    pub contract: Address,
    /// base64 encoded msg payload
    pub base64_encoded_payload: String,
}

/// ## Description
/// This structure describes fields for mpc1-multisig proposal vote msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x03)]
pub struct ProposalVoteMsg {
    /// proposal id
    pub proposal_id: u64,
    /// vote type
    pub vote: Vote,
}

/// ## Description
/// This structure describes fields for mpc1-multisig proposal execute msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x05)]
pub struct ProposalExecuteMsg {
    /// proposal id to execute
    pub proposal_id: u64,
}

/// ## Description
/// This structure describes fields for mpc1-multisig proposal close msg
#[derive(ReadWriteRPC, CreateTypeSpec, IntoShortnameRPCEvent, Clone, PartialEq, Eq, Debug)]
#[rpc_msg(action = 0x07)]
pub struct ProposalCloseMsg {
    /// proposal id to close
    pub proposal_id: u64,
}
