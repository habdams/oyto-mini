use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

pub const ANCHOR_DISCRIMINATOR: usize = 8;

pub const RULE_ACCOUNT_SPACE: usize = ANCHOR_DISCRIMINATOR + 1 + 8;

pub const CONTRIBUTION_ACCOUNT_SPACE: usize = ANCHOR_DISCRIMINATOR + 32 + 1;

pub const COMPENSATION_ACCOUNT_SPACE: usize = ANCHOR_DISCRIMINATOR + 32 + 8;
