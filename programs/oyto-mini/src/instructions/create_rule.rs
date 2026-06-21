use anchor_lang::prelude::*;

use crate::constants::*;
use crate::error::*;
use crate::events::*;
use crate::state::*;

pub fn create_handler(ctx: Context<CreateRule>, event_type: EventType, reward: u64) -> Result<()> {
    require!(reward > 0, PayrollError::InvalidReward);

    let rule = &mut ctx.accounts.rule;

    rule.event_type = event_type;
    rule.reward = reward;

    emit!(RuleCreated { reward });

    Ok(())
}

#[derive(Accounts)]
pub struct CreateRule<'info> {
    #[account(
        init,
        payer = authority,
        space = RULE_ACCOUNT_SPACE
    )]
    pub rule: Account<'info, RuleAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}
