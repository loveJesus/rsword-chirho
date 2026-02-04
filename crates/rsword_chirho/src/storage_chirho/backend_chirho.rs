// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Storage backend trait for platform-agnostic I/O.
//!
//! This module provides an abstraction layer for file system operations,
//! allowing rsword_chirho to work on both native platforms (using std::fs)
//! and WebAssembly (using IndexedDB or virtual filesystems).
//!
//! ## Native Usage
//!
//! On native platforms, use `NativeBackendChirho`:
//!
//! ```rust,ignore
//! use rsword_chirho::storage_chirho::{StorageBackendChirho, NativeBackendChirho};
//!
//! let backend_chirho = NativeBackendChirho::new_chirho();
//! let data_chirho = backend_chirho.read_file_chirho("/path/to/file")?;
//! ```
//!
//! ## WASM Usage
//!
//! On WASM, use `WasmBackendChirho` (when the `wasm` feature is enabled):
//!
//! ```rust,ignore
//! use rsword_chirho::storage_chirho::{StorageBackendChirho, WasmBackendChirho};
//!
//! let backend_chirho = WasmBackendChirho::new_chirho();
//! // Files are read from IndexedDB or pre-loaded module data
//! let data_chirho = backend_chirho.read_file_chirho("modules/kjv/nt")?;
//! ```

use crate::error_chirho::ResultChirho;

/// Abstract storage backend for platform-agnostic I/O.
///
/// Implementations of this trait provide file system operations that work
/// on different platforms:
///
/// - `NativeBackendChirho`: Uses std::fs for native platforms
/// - `WasmBackendChirho`: Uses IndexedDB or virtual filesystem for WASM
pub trait StorageBackendChirho: Send + Sync {
    /// Read an entire file into memory.
    ///
    /// # Arguments
    /// * `path_chirho` - Path to the file (platform-specific format)
    ///
    /// # Returns
    /// The file contents as a byte vector
    fn read_file_chirho(&self, path_chirho: &str) -> ResultChirho<Vec<u8>>;

    /// Read a range of bytes from a file.
    ///
    /// This is more efficient than reading the entire file when only
    /// a portion is needed (e.g., reading verse data at a specific offset).
    ///
    /// # Arguments
    /// * `path_chirho` - Path to the file
    /// * `offset_chirho` - Starting byte offset
    /// * `len_chirho` - Number of bytes to read
    ///
    /// # Returns
    /// The requested bytes
    fn read_range_chirho(
        &self,
        path_chirho: &str,
        offset_chirho: u64,
        len_chirho: usize,
    ) -> ResultChirho<Vec<u8>>;

    /// Write data to a file (creates or overwrites).
    ///
    /// # Arguments
    /// * `path_chirho` - Path to the file
    /// * `data_chirho` - Data to write
    fn write_file_chirho(&self, path_chirho: &str, data_chirho: &[u8]) -> ResultChirho<()>;

    /// Check if a file exists.
    ///
    /// # Arguments
    /// * `path_chirho` - Path to check
    ///
    /// # Returns
    /// `true` if the file exists, `false` otherwise
    fn file_exists_chirho(&self, path_chirho: &str) -> bool;

    /// List files in a directory.
    ///
    /// # Arguments
    /// * `path_chirho` - Directory path
    ///
    /// # Returns
    /// Vector of file names (not full paths) in the directory
    fn list_dir_chirho(&self, path_chirho: &str) -> ResultChirho<Vec<String>>;

    /// Create a directory (and parent directories if needed).
    ///
    /// # Arguments
    /// * `path_chirho` - Directory path to create
    fn create_dir_chirho(&self, path_chirho: &str) -> ResultChirho<()>;

    /// Get the file size in bytes.
    ///
    /// # Arguments
    /// * `path_chirho` - Path to the file
    ///
    /// # Returns
    /// The file size in bytes
    fn file_size_chirho(&self, path_chirho: &str) -> ResultChirho<u64>;

    /// Append data to a file.
    ///
    /// # Arguments
    /// * `path_chirho` - Path to the file
    /// * `data_chirho` - Data to append
    fn append_file_chirho(&self, path_chirho: &str, data_chirho: &[u8]) -> ResultChirho<()>;
}

/// Get the default storage backend for the current platform.
///
/// Returns `NativeBackendChirho` on native platforms and
/// `WasmBackendChirho` on WASM (when compiled with the `wasm` feature).
#[cfg(not(target_arch = "wasm32"))]
pub fn get_default_backend_chirho() -> Box<dyn StorageBackendChirho> {
    Box::new(super::native_backend_chirho::NativeBackendChirho::new_chirho())
}

/// Get the default storage backend for WASM.
#[cfg(target_arch = "wasm32")]
pub fn get_default_backend_chirho() -> Box<dyn StorageBackendChirho> {
    Box::new(super::wasm_backend_chirho::WasmBackendChirho::new_chirho())
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_backend_trait_object_safety_chirho() {
        // Verify the trait is object-safe
        fn _takes_backend_chirho(_b_chirho: &dyn StorageBackendChirho) {}
    }
}
