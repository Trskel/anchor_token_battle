use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Enemy is dead already.")]
    NotEnoughHealth,
    #[msg("Account is not authorised for this operation.")]
    Unauthorised,
}
