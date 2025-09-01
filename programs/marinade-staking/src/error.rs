use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,
    #[msg("Invalid token pair")]
    InvalidTokenPair,
    #[msg("InvalidSwapDirection")]

    InvalidSwapDirection
}
