use anchor_lang::prelude::*;

#[event]
pub struct RuleCreated {
    pub reward: u64,
}

#[event]
pub struct CompensationCreated {
    pub contributor: Pubkey,
    pub amount: u64,
}
