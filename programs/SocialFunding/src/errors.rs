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

    #[msg("You are not authenticated.")]
    AuthenticationError,

    #[msg("Contract is not open yet.")]
    ContractPause,

    #[msg("Not is project stage.")]
    NotInProjectStage,
    #[msg("Not is project voting stage.")]
    NotInVotingStage,
    #[msg("Not is project execute stage.")]
    NotInExecuteStage,
    #[msg("Not is project donate stage.")]
    NotInDonateStage,
    #[msg("Not is funding stage.")]
    NotInDistributeStage,

    #[msg("Invalid char detected.")]
    InvalidChar,

    #[msg("Already executed, can't call more!")]
    AlreadyExecuted,

    InsufficiantError,
}
