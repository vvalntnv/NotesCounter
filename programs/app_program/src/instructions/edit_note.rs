use anchor_lang::prelude::*;

use crate::state::note::NoteData;

pub fn hanlder(_ctx: Context<EditNote>, _new_content: String) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct EditNote<'info> {
    pub note: Account<'info, NoteData>,
}
