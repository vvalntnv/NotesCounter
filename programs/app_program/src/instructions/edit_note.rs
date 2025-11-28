use anchor_lang::prelude::*;
use counter_service::cpi as counter_service_cpi;
use counter_service::program::CounterService;

use crate::state::note::NoteData;

pub fn _edit_note(ctx: Context<EditNote>, new_content: String) -> Result<()> {
    let counter_program = &ctx.accounts.counter_program;
    let counter = &ctx.accounts.counter;
    let user = &ctx.accounts.signer;

    let note = &mut ctx.accounts.note;
    note.content = new_content;

    let user_key = user.key();
    let seeds = [b"userinfo", user_key.as_ref()];
    let (expected_pda, bump) = Pubkey::find_program_address(&seeds, &counter_program.key());

    require_keys_eq!(*counter.key, expected_pda);

    let signer_seeds: &[&[&[u8]]] = &[&[seeds[0], seeds[1], &[bump]]];

    let cpi_context = CpiContext::new_with_signer(
        counter_program.to_account_info(),
        counter_service_cpi::accounts::IncrementOnEdit {
            user: user.to_account_info(),
            user_stats: counter.to_account_info(),
        },
        signer_seeds,
    );

    counter_service_cpi::increment_on_edit(cpi_context)?;
    msg!("Tuka li sme banda?");

    Ok(())
}

#[derive(Accounts)]
pub struct EditNote<'info> {
    pub signer: Signer<'info>,
    pub note: Account<'info, NoteData>,

    /// CHECK: the counter belongs to the counter_service
    #[account(mut)]
    pub counter: UncheckedAccount<'info>,
    pub counter_program: Program<'info, CounterService>,
}
