use crate::state::note::NoteData;
use anchor_lang::prelude::*;
use shared::traits::Spacy;

#[derive(Accounts)]
pub struct CreateNote<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = <NoteData as Spacy>::SIZE,
        seeds = [b"notedata", signer.key().as_ref()],
        bump
    )]
    pub note: Account<'info, NoteData>,
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateNote>, note_content: String) {
    let note_data = &mut ctx.accounts.note;
    note_data.content = note_content;
}
