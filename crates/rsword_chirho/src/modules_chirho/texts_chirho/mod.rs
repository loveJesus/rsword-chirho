// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Bible text module implementations.
//!
//! Provides RawText, RawText4, zText, and zText4 module drivers.

mod raw_text_chirho;
mod raw_text4_chirho;
mod z_text_chirho;
mod z_text4_chirho;

pub use raw_text_chirho::{RawTextChirho, RawTextIteratorChirho};
pub use raw_text4_chirho::RawText4Chirho;
pub use z_text_chirho::{ZTextChirho, ZTextIteratorChirho};
pub use z_text4_chirho::{ZText4Chirho, ZText4IteratorChirho};
