// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Storage backends for SWORD modules.
//!
//! This module provides implementations for reading and writing the various
//! binary storage formats used by SWORD modules.

pub mod raw_verse_chirho;
pub mod raw_verse4_chirho;
pub mod z_verse_chirho;
pub mod z_verse4_chirho;
pub mod raw_str_chirho;
pub mod z_str_chirho;

pub use raw_verse_chirho::RawVerseChirho;
pub use raw_verse4_chirho::RawVerse4Chirho;
pub use z_verse_chirho::ZVerseChirho;
pub use z_verse4_chirho::ZVerse4Chirho;
pub use raw_str_chirho::RawStrChirho;
pub use z_str_chirho::ZStrChirho;
