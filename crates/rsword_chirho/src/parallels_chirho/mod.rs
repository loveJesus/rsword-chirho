// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Parallel passage detection for identifying related Scripture passages.
//!
//! This module provides detection of:
//! - Synoptic Gospel parallels (Matthew, Mark, Luke)
//! - Old Testament parallel histories (Chronicles/Kings, Samuel/Kings)
//! - Cross-references between related passages

mod parallel_passage_chirho;

pub use parallel_passage_chirho::{
    ParallelPassageChirho, ParallelSetChirho, ParallelTypeChirho,
    ParallelManagerChirho,
};
