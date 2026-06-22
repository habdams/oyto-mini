use anchor_lang::prelude::*;

#[error_code]
pub enum PayrollError {
    #[msg("Reward must be greater than zero")]
    InvalidReward,

    #[msg("Contribution too small")]
    ContributionTooSmall,
}
