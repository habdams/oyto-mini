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

declare_id!("AiD45wxFcNuh1QC1EAU5i3dxbyVGTyNnZUdiHPbh8Fvk");

#[program]
pub mod oyto_mini {
    use super::*;

    pub fn create_rule(ctx: Context<CreateRule>, event_type: EventType, reward: u64) -> Result<()> {
        instructions::create_rule::create_handler(ctx, event_type, reward)
    }

    pub fn submit_contribution(ctx: Context<SubmitContribution>, pr_size: u64) -> Result<()> {
        instructions::submit_contribution::submit_handler(ctx, pr_size)
    }
}
