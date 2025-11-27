use anchor_lang::prelude::*;
mod errors;
mod instructions;
mod state;

pub use instructions::*;

declare_id!("RU8qag4xkWaDZA4edLmxmNbCUAzWbevWqFnJVTX1kSS");

#[program]
pub mod counter_service {
    use super::*;

    pub fn create_user_stats(ctx: Context<CreateUserStats>) -> Result<()> {
        instructions::create_user_stats::handler(ctx)
    }

    pub fn increment_on_create(ctx: Context<IncrementOnCreate>) -> Result<()> {
        instructions::increment_on_create::handler(ctx)
    }
}
