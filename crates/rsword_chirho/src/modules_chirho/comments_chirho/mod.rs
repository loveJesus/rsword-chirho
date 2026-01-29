// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Commentary module implementations.
//!
//! Provides RawCom, RawCom4, zCom, and zCom4 module drivers.

mod raw_com_chirho;
mod z_com_chirho;

pub use raw_com_chirho::{RawComChirho, RawComIteratorChirho};
pub use z_com_chirho::{ZComChirho, ZComIteratorChirho};

// TODO: Implement RawCom4Chirho, ZCom4Chirho
