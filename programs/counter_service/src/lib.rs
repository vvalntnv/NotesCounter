use anchor_lang::prelude::*;
mod instructions;
mod state;
mod errors;

declare_id!("RU8qag4xkWaDZA4edLmxmNbCUAzWbevWqFnJVTX1kSS");

#[program]
pub mod counter_service {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
