use anchor_lang::prelude::*;

use crate::constants::*;
use crate::events::*;
use crate::state::*;

pub fn submit_handler(ctx: Context<SubmitContribution>) -> Result<()> {
    let contribution = &mut ctx.accounts.contribution;

    let compensation = &mut ctx.accounts.compensation;

    let rule = &ctx.accounts.rule;

    contribution.contributor = ctx.accounts.contributor.key();

    contribution.event_type = rule.event_type.clone();

    compensation.contributor = ctx.accounts.contributor.key();

    compensation.amount = rule.reward;

    emit!(CompensationCreated {
        contributor: ctx.accounts.contributor.key(),
        amount: rule.reward,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct SubmitContribution<'info> {
    #[account(mut)]
    pub contributor: Signer<'info>,

    pub rule: Account<'info, RuleAccount>,

    #[account(
        init,
        payer = contributor,
        space = CONTRIBUTION_ACCOUNT_SPACE
    )]
    pub contribution: Account<'info, ContributionAccount>,

    #[account(
        init,
        payer = contributor,
        space = COMPENSATION_ACCOUNT_SPACE
    )]
    pub compensation: Account<'info, CompensationAccount>,

    pub system_program: Program<'info, System>,
}
