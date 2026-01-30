// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Commentary module implementations.
//!
//! Provides RawCom, RawCom4, zCom, zCom4, and HREFCom module drivers.

mod raw_com_chirho;
mod raw_com4_chirho;
mod z_com_chirho;
mod z_com4_chirho;
mod href_com_chirho;

pub use raw_com_chirho::{RawComChirho, RawComIteratorChirho};
pub use raw_com4_chirho::{RawCom4Chirho, RawCom4IteratorChirho};
pub use z_com_chirho::{ZComChirho, ZComIteratorChirho};
pub use z_com4_chirho::{ZCom4Chirho, ZCom4IteratorChirho};
pub use href_com_chirho::HrefComChirho;
