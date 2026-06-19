use anchor_lang::prelude::*;

#[constant]
pub const SEED: &str = "anchor";

pub const ANCHOR_DISCRIMINATOR: usize = 8;

pub const RULE_ACCOUNT_SPACE: usize = ANCHOR_DISCRIMINATOR +
    1 + // EventType enum
    8; // reward
