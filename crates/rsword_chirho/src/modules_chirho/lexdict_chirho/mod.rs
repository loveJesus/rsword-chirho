// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Lexicon and dictionary module implementations.
//!
//! Provides RawLD, RawLD4, and zLD module drivers.

mod raw_ld_chirho;
mod z_ld_chirho;

pub use raw_ld_chirho::{RawLdChirho, RawLdIteratorChirho};
pub use z_ld_chirho::{ZLdChirho, ZLdIteratorChirho};

// TODO: Implement RawLd4Chirho
