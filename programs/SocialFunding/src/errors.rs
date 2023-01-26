use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Your community does not have enough members.")]
    InsufficientNumber,

    #[msg("You are not in this community.")]
    NotMember,

    #[msg(
        "
    Your project is not published because it did not receive enough votes. "
    )]
    NotPublish,

    AuthenticationError,

    ContractPause,

    NotInProjectStage,
    NotInVotingStage,
    NotInExecuteStage,
    NotInDonateStage,
    NotInDistributeStage,

    InvalidChar,

    AlreadyExecuted,
}
