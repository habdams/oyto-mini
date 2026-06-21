use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum EventType {
    MergedPr,
    Review,
    IssueClosed,
}

#[account]
pub struct RuleAccount {
    pub event_type: EventType,
    pub reward: u64,
}

#[account]
pub struct ContributionAccount {
    pub contributor: Pubkey,
    pub event_type: EventType,
}

#[account]
pub struct CompensationAccount {
    pub contributor: Pubkey,
    pub amount: u64,
}
