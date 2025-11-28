use anchor_lang::prelude::*;
mod errors;
mod instructions;
mod state;

pub use instructions::*;

declare_id!("RU8qag4xkWaDZA4edLmxmNbCUAzWbevWqFnJVTX1kSS");

#[program]
pub mod counter_service {
    use super::*;

    pub fn increment_on_create(ctx: Context<IncrementOnCreate>) -> Result<()> {
        instructions::increment_on_create::_increment_on_create(ctx)
    }

    pub fn increment_on_edit(ctx: Context<IncrementOnEdit>) -> Result<()> {
        instructions::increment_on_edit::_increment_on_edit(ctx)
    }
}
