use anchor_lang::prelude::*;
use shared::traits::Spacy;

use crate::state::user_counter::UserCounter;

pub fn handler(ctx: Context<CreateUserStats>) -> Result<()> {
    ctx.accounts.user_data.created_count = 0;
    ctx.accounts.user_data.edited_count = 0;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateUserStats<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = <UserCounter as Spacy>::SIZE,
        seeds = [b"useraccount", user.key().as_ref()],
        bump
    )]
    pub user_data: Account<'info, UserCounter>,
    pub system_program: Program<'info, System>,
}
