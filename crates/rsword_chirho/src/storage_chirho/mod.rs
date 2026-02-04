// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Storage backends for SWORD modules.
//!
//! This module provides implementations for reading and writing the various
//! binary storage formats used by SWORD modules.
//!
//! ## Storage Backend Abstraction
//!
//! The `backend_chirho` submodule provides a platform-agnostic storage abstraction:
//!
//! - `StorageBackendChirho` - Trait for all storage operations
//! - `NativeBackendChirho` - Native filesystem implementation (std::fs)
//! - `WasmBackendChirho` - WASM implementation (IndexedDB/virtual FS)
//!
//! Use `get_default_backend_chirho()` to get the appropriate backend for
//! the current platform.

// Storage backend abstraction (platform-agnostic)
pub mod backend_chirho;

#[cfg(not(target_arch = "wasm32"))]
pub mod native_backend_chirho;

#[cfg(target_arch = "wasm32")]
pub mod wasm_backend_chirho;

// Re-export backend types
pub use backend_chirho::StorageBackendChirho;
pub use backend_chirho::get_default_backend_chirho;

#[cfg(not(target_arch = "wasm32"))]
pub use native_backend_chirho::NativeBackendChirho;

#[cfg(target_arch = "wasm32")]
pub use wasm_backend_chirho::WasmBackendChirho;

// Binary format implementations
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
