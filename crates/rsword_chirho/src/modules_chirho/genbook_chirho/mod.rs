// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! General book module implementations.
//!
//! Provides RawGenBook and ZGenBook module drivers for tree-structured content.

mod raw_genbook_chirho;
mod z_genbook_chirho;

pub use raw_genbook_chirho::{RawGenBookChirho, RawGenBookIteratorChirho, GenBookEntryChirho};
pub use z_genbook_chirho::{ZGenBookChirho, ZGenBookIteratorChirho};
