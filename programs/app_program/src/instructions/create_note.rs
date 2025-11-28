use crate::state::note::NoteData;
use anchor_lang::prelude::*;
use counter_service::cpi as counter_cpi;
use counter_service::cpi::accounts::IncrementOnCreate;
use counter_service::program::CounterService;
use shared::traits::Spacy;

pub fn _create_note(ctx: Context<CreateNote>, note_content: String) -> Result<()> {
    let note_data = &mut ctx.accounts.note;

    let user = &ctx.accounts.signer;
    let user_stats = &ctx.accounts.counter;
    note_data.content = note_content;
    let clock = Clock::get()?;
    note_data.last_edited = clock.unix_timestamp;
    let counter_program = &ctx.accounts.counter_program.key();

    let user_key = user.key();
    let seeds = [b"userinfo", user_key.as_ref()];
    let (expected_pda, bump) = Pubkey::find_program_address(&seeds, counter_program);

    require_keys_eq!(*user_stats.key, expected_pda);

    let increment_data = IncrementOnCreate {
        user: ctx.accounts.signer.to_account_info(),
        user_stats: ctx.accounts.counter.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };

    let signer_seeds: &[&[&[u8]]] = &[&[seeds[0], seeds[1], &[bump]]];

    let cpi_context = CpiContext::new_with_signer(
        ctx.accounts.counter_program.to_account_info(),
        increment_data,
        signer_seeds,
    );

    msg!("panaira");
    counter_cpi::increment_on_create(cpi_context)?;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateNote<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = <NoteData as Spacy>::SIZE,
    )]
    pub note: Account<'info, NoteData>,
    pub system_program: Program<'info, System>,
    /// CHECK: the counter belongs to the counter_service
    #[account(mut)]
    pub counter: UncheckedAccount<'info>,
    pub counter_program: Program<'info, CounterService>,
}
