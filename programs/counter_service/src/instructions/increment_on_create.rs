use crate::state::user_counter::*;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{CreateAccount};
use shared::SpacyTrait;

pub fn handler(ctx: Context<IncrementOnCreate>) -> Result<()> {
    let user_stats_ua = &ctx.accounts.user_stats;
    let user_status_info = user_stats_ua.to_account_info();
    let user = &ctx.accounts.user;
    let user_key = user.key();
    let seeds = &[b"userstats", user_key.as_ref()];

    let (pda, bump) = Pubkey::find_program_address(seeds, ctx.program_id);

    let is_uninitialized =
        user_stats_ua.owner == &system_program::ID && user_stats_ua.data_is_empty();

    if is_uninitialized {
        let space = UserCounter::SIZE;
        let cost = Rent::get()?.minimum_balance(space);

        let cpi_accounts = CreateAccount {
            from: user.to_account_info(),
            to: user_stats_ua.to_account_info(),
        };

        let signer_seeds: &[&[&[u8]]] = &[seeds];
        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            cpi_accounts,
            &signer_seeds,
        );

        anchor_lang::system_program::create_account(cpi_ctx, cost, space as u64, &ctx.program_id)?;
    }

    let user_account_info = ctx.accounts.user_stats.to_account_info();
    let mut user_stats: Account<UserCounter> = Account::try_from(&user_account_info)?;

    Ok(())
}

#[derive(Accounts)]
pub struct IncrementOnCreate<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    // This I do not like, because sometimes the user_stats won't
    // have to be created, but WHATEVER
    #[account(mut)]
    pub user_stats: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}
