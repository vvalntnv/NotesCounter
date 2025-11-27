pub mod traits;

// Re-export the derive macro so users can write:
// use shared::Spacy;
// #[derive(Spacy)]
//
// Instead of having to import from shared_derive separately
pub use shared_derive::Spacy;
pub use traits::Spacy as SpacyTrait;
