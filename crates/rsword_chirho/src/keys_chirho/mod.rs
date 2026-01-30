// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Key types for navigating SWORD modules.
//!
//! Keys are used to identify and navigate to specific entries in modules.
//! Different module types use different key implementations:
//! - `VerseKeyChirho` for Bibles and commentaries
//! - `TreeKeyChirho` for general books
//! - `StrKeyChirho` for lexicons/dictionaries

pub mod sw_key_chirho;
pub mod verse_key_chirho;
pub mod list_key_chirho;
pub mod tree_key_chirho;
pub mod str_key_chirho;
pub mod date_key_chirho;

pub use sw_key_chirho::SwKeyChirho;
pub use verse_key_chirho::VerseKeyChirho;
pub use list_key_chirho::ListKeyChirho;
pub use tree_key_chirho::TreeKeyChirho;
pub use str_key_chirho::StrKeyChirho;
pub use date_key_chirho::DateKeyChirho;
