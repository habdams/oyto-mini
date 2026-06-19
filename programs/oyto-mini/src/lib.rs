pub mod constants;
pub mod error;
pub mod events;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use events::*;
pub use instructions::*;
pub use state::*;

declare_id!("Ack3dg9utXRbtMY9LDsNWuZRT7VUoKUwKrsgoyjSXirM");

#[program]
pub mod oyto_mini {
    use super::*;

    pub fn create_rule(ctx: Context<CreateRule>, event_type: EventType, reward: u64) -> Result<()> {
        instructions::create_rule::handler(ctx, event_type, reward)
    }

    pub fn submit_contribution(ctx: Context<SubmitContribution>) -> Result<()> {
        instructions::submit_contribution::handler(ctx)
    }
}
