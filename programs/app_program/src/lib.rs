use anchor_lang::prelude::*;

pub mod cpi;
pub mod errors;
pub mod instructions;
pub mod state;

pub use instructions::*;

declare_id!("8b3Co1nvrEdyeA5tKCoLNd5kgxedGRxY333SvdqviyA7");

#[program]
pub mod app_program {
    use super::*;

    pub fn create_note(ctx: Context<CreateNote>, content: String) -> Result<()> {
        instructions::create_note::handler(ctx, content)
    }
}
