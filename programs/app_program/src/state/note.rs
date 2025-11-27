use anchor_lang::prelude::*;
use shared::Spacy;

#[account]
#[derive(Spacy)]
pub struct NoteData {
    #[size(200)]
    pub content: String,
    pub last_edited: i64,
    pub note_id: u64,
}
