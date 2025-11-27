use anchor_lang::prelude::*;
use shared::{Spacy};

#[account]
#[derive(Spacy)]
pub struct UserCounter {
    pub created_count: u64,
    pub edited_count: u64,
}
