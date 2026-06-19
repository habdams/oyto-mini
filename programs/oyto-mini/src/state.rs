use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
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
