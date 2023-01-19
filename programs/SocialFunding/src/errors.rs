use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Your community does not have enough members.")]
    InsufficientNumber,
}
