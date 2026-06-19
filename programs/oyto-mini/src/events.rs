use anchor_lang::prelude::*;

#[event]
pub struct RuleCreated {
    pub reward: u64,
}
