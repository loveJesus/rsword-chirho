// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! WASM storage backend for browser environments.
//!
//! This module provides the `WasmBackendChirho` implementation of `StorageBackendChirho`
//! for WebAssembly targets. It uses IndexedDB for persistent storage and supports
//! pre-loaded module data.
//!
//! ## Architecture
//!
//! In WASM, there's no native filesystem access. Instead, module data can be:
//!
//! 1. **Pre-loaded**: Module files are fetched via HTTP and stored in memory or IndexedDB
//! 2. **Virtual filesystem**: A HashMap-based virtual FS for runtime module data
//! 3. **IndexedDB**: Persistent browser storage for downloaded modules
//!
//! ## Usage
//!
//! ```rust,ignore
//! use rsword_chirho::storage_chirho::{StorageBackendChirho, WasmBackendChirho};
//!
//! // Create backend with pre-loaded module data
//! let backend_chirho = WasmBackendChirho::new_chirho();
//!
//! // Pre-load module data (typically fetched via HTTP)
//! backend_chirho.preload_file_chirho("modules/kjv/nt", &module_data);
//!
//! // Now read operations work as expected
//! let data_chirho = backend_chirho.read_file_chirho("modules/kjv/nt")?;
//! ```

#[cfg(target_arch = "wasm32")]
use std::collections::HashMap;
#[cfg(target_arch = "wasm32")]
use std::sync::{Arc, RwLock};

#[cfg(target_arch = "wasm32")]
use crate::error_chirho::{ErrorChirho, ResultChirho};
#[cfg(target_arch = "wasm32")]
use super::backend_chirho::StorageBackendChirho;

/// WASM storage backend using virtual filesystem and IndexedDB.
///
/// This backend is used when compiling for `wasm32-unknown-unknown` target.
/// It provides a virtual filesystem backed by in-memory storage and optionally
/// IndexedDB for persistence.
///
/// ## Features
///
/// - **Pre-loading**: Load module files from HTTP responses before use
/// - **In-memory cache**: Fast access to recently used module data
/// - **Virtual directories**: Simulated directory structure for module discovery
///
/// ## Limitations
///
/// - No true filesystem access (browser security model)
/// - Module data must be pre-loaded or fetched asynchronously
/// - Write operations only persist to memory (IndexedDB integration planned)
#[cfg(target_arch = "wasm32")]
pub struct WasmBackendChirho {
    /// Virtual filesystem: path -> file contents
    virtual_fs_chirho: Arc<RwLock<HashMap<String, Vec<u8>>>>,
    /// Virtual directory listing: directory -> [file names]
    virtual_dirs_chirho: Arc<RwLock<HashMap<String, Vec<String>>>>,
}

#[cfg(target_arch = "wasm32")]
impl WasmBackendChirho {
    /// Create a new WASM storage backend.
    pub fn new_chirho() -> Self {
        Self {
            virtual_fs_chirho: Arc::new(RwLock::new(HashMap::new())),
            virtual_dirs_chirho: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Pre-load a file into the virtual filesystem.
    ///
    /// This should be called after fetching module data via HTTP.
    ///
    /// # Arguments
    /// * `path_chirho` - Virtual path for the file
    /// * `data_chirho` - File contents
    pub fn preload_file_chirho(&self, path_chirho: &str, data_chirho: &[u8]) {
        // Add to virtual FS
        if let Ok(mut fs_chirho) = self.virtual_fs_chirho.write() {
            fs_chirho.insert(path_chirho.to_string(), data_chirho.to_vec());
        }

        // Update directory listing
        if let Some(parent_chirho) = Self::parent_path_chirho(path_chirho) {
            if let Some(name_chirho) = Self::file_name_chirho(path_chirho) {
                if let Ok(mut dirs_chirho) = self.virtual_dirs_chirho.write() {
                    dirs_chirho
                        .entry(parent_chirho.to_string())
                        .or_default()
                        .push(name_chirho.to_string());
                }
            }
        }
    }

    /// Clear all pre-loaded files.
    pub fn clear_chirho(&self) {
        if let Ok(mut fs_chirho) = self.virtual_fs_chirho.write() {
            fs_chirho.clear();
        }
        if let Ok(mut dirs_chirho) = self.virtual_dirs_chirho.write() {
            dirs_chirho.clear();
        }
    }

    /// Get parent directory path.
    fn parent_path_chirho(path_chirho: &str) -> Option<&str> {
        path_chirho.rfind('/').map(|idx_chirho| &path_chirho[..idx_chirho])
    }

    /// Get file name from path.
    fn file_name_chirho(path_chirho: &str) -> Option<&str> {
        path_chirho.rfind('/').map(|idx_chirho| &path_chirho[idx_chirho + 1..])
    }
}

#[cfg(target_arch = "wasm32")]
impl StorageBackendChirho for WasmBackendChirho {
    fn read_file_chirho(&self, path_chirho: &str) -> ResultChirho<Vec<u8>> {
        let fs_chirho = self.virtual_fs_chirho.read().map_err(|_| {
            ErrorChirho::generic_chirho("Failed to acquire read lock".to_string())
        })?;

        fs_chirho
            .get(path_chirho)
            .cloned()
            .ok_or_else(|| ErrorChirho::generic_chirho(format!("File not found: {}", path_chirho)))
    }

    fn read_range_chirho(
        &self,
        path_chirho: &str,
        offset_chirho: u64,
        len_chirho: usize,
    ) -> ResultChirho<Vec<u8>> {
        let data_chirho = self.read_file_chirho(path_chirho)?;

        let start_chirho = offset_chirho as usize;
        let end_chirho = (start_chirho + len_chirho).min(data_chirho.len());

        if start_chirho >= data_chirho.len() {
            return Err(ErrorChirho::generic_chirho(format!(
                "Offset {} beyond file size {}",
                offset_chirho,
                data_chirho.len()
            )));
        }

        Ok(data_chirho[start_chirho..end_chirho].to_vec())
    }

    fn write_file_chirho(&self, path_chirho: &str, data_chirho: &[u8]) -> ResultChirho<()> {
        self.preload_file_chirho(path_chirho, data_chirho);
        Ok(())
    }

    fn file_exists_chirho(&self, path_chirho: &str) -> bool {
        if let Ok(fs_chirho) = self.virtual_fs_chirho.read() {
            fs_chirho.contains_key(path_chirho)
        } else {
            false
        }
    }

    fn list_dir_chirho(&self, path_chirho: &str) -> ResultChirho<Vec<String>> {
        let dirs_chirho = self.virtual_dirs_chirho.read().map_err(|_| {
            ErrorChirho::generic_chirho("Failed to acquire read lock".to_string())
        })?;

        Ok(dirs_chirho.get(path_chirho).cloned().unwrap_or_default())
    }

    fn create_dir_chirho(&self, path_chirho: &str) -> ResultChirho<()> {
        let mut dirs_chirho = self.virtual_dirs_chirho.write().map_err(|_| {
            ErrorChirho::generic_chirho("Failed to acquire write lock".to_string())
        })?;

        dirs_chirho.entry(path_chirho.to_string()).or_default();
        Ok(())
    }

    fn file_size_chirho(&self, path_chirho: &str) -> ResultChirho<u64> {
        let fs_chirho = self.virtual_fs_chirho.read().map_err(|_| {
            ErrorChirho::generic_chirho("Failed to acquire read lock".to_string())
        })?;

        fs_chirho
            .get(path_chirho)
            .map(|d_chirho| d_chirho.len() as u64)
            .ok_or_else(|| ErrorChirho::generic_chirho(format!("File not found: {}", path_chirho)))
    }

    fn append_file_chirho(&self, path_chirho: &str, data_chirho: &[u8]) -> ResultChirho<()> {
        let mut fs_chirho = self.virtual_fs_chirho.write().map_err(|_| {
            ErrorChirho::generic_chirho("Failed to acquire write lock".to_string())
        })?;

        let entry_chirho = fs_chirho.entry(path_chirho.to_string()).or_default();
        entry_chirho.extend_from_slice(data_chirho);
        Ok(())
    }
}

// Ensure WasmBackendChirho is Send + Sync for the trait bound
#[cfg(target_arch = "wasm32")]
unsafe impl Send for WasmBackendChirho {}
#[cfg(target_arch = "wasm32")]
unsafe impl Sync for WasmBackendChirho {}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_preload_and_read_chirho() {
        let backend_chirho = WasmBackendChirho::new_chirho();

        backend_chirho.preload_file_chirho("modules/kjv/nt", b"test data");

        let data_chirho = backend_chirho.read_file_chirho("modules/kjv/nt").unwrap();
        assert_eq!(&data_chirho, b"test data");
    }

    #[test]
    fn test_read_range_wasm_chirho() {
        let backend_chirho = WasmBackendChirho::new_chirho();

        backend_chirho.preload_file_chirho("test.txt", b"Hello, World!");

        let data_chirho = backend_chirho.read_range_chirho("test.txt", 7, 5).unwrap();
        assert_eq!(&data_chirho, b"World");
    }

    #[test]
    fn test_file_not_found_chirho() {
        let backend_chirho = WasmBackendChirho::new_chirho();

        let result_chirho = backend_chirho.read_file_chirho("nonexistent");
        assert!(result_chirho.is_err());
    }

    #[test]
    fn test_directory_listing_chirho() {
        let backend_chirho = WasmBackendChirho::new_chirho();

        backend_chirho.preload_file_chirho("modules/kjv/nt", b"nt");
        backend_chirho.preload_file_chirho("modules/kjv/ot", b"ot");

        let files_chirho = backend_chirho.list_dir_chirho("modules/kjv").unwrap();
        assert!(files_chirho.contains(&"nt".to_string()));
        assert!(files_chirho.contains(&"ot".to_string()));
    }
}
