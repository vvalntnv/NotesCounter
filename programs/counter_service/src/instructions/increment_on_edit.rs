use crate::state::user_counter::*;
use anchor_lang::prelude::*;

pub fn _increment_on_edit(ctx: Context<IncrementOnEdit>) -> Result<()> {
    let user_stats = &mut ctx.accounts.user_stats;
    user_stats.edited_count += 1;

    msg!("tuka dali ima neshto za mene {}", user_stats.edited_count);
    msg!("eto pda: {}", user_stats.key().to_string());

    Ok(())
}

#[derive(Accounts)]
pub struct IncrementOnEdit<'info> {
    pub user: Signer<'info>,

    // Since we're editing an existing note, we can be sure
    // that the user_stats account is already initialized
    #[account(
        seeds = [b"userinfo", user.key().as_ref()],
        bump
    )]
    pub user_stats: Account<'info, UserCounter>,
}
