use thiserror::Error;

/// ## Description
/// This enum describes mpc1-multisig contract errors
#[derive(Error, Debug)]
pub enum ContractError {
    #[error("Members list is empty")]
    MembersListIsEmpty,

    #[error("Required weight is zero")]
    RequiredWeightIsZero,

    #[error("Unreachable weight")]
    UnreachableWeight,

    #[error("Duplicated member")]
    DuplicatedMember,

    #[error("Invalid voting phase period")]
    InvalidVotingPhase,

    #[error("Invalid voting power(weight)")]
    InvalidVotingPower,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Proposal not found")]
    ProposalNotFound,

    #[error("Proposal is not in the voting phase")]
    ProposalIsNotInTheVotingPhase,

    #[error("Proposal voting phase has expired")]
    Expired,

    #[error("Member has already voted")]
    AlreadyVoted,

    #[error("Proposal is not accepted yet or rejected")]
    ProposalIsNotAcceptedOrRejected,

    #[error("Cannot close executed or rejected proposal")]
    WrongCloseStatus,

    #[error("Proposal not expired yet")]
    ProposalNotExpired,

    #[error("Empty execute calls list")]
    EmptyExecuteCallsList,
}
