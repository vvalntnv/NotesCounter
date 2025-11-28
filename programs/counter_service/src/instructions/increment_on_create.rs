use crate::state::user_counter::*;
use anchor_lang::prelude::*;
use shared::traits::Spacy;

pub fn _increment_on_create(ctx: Context<IncrementOnCreate>) -> Result<()> {
    let user_stats = &mut ctx.accounts.user_stats;
    user_stats.created_count += 1;

    Ok(())
}

#[derive(Accounts)]
pub struct IncrementOnCreate<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    // This I do not like, because sometimes the user_stats won't
    // have to be created, but WHATEVER
    #[account(
        init_if_needed,
        payer = user,
        space = <UserCounter as Spacy>::SIZE,
        seeds = [b"userinfo", user.key().as_ref()],
        bump
    )]
    pub user_stats: Account<'info, UserCounter>,
    pub system_program: Program<'info, System>,
}
