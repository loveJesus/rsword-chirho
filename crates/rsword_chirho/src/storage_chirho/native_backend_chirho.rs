// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Native filesystem storage backend.
//!
//! This module provides the `NativeBackendChirho` implementation of `StorageBackendChirho`
//! that uses the standard library's filesystem operations. This is the default backend
//! for native platforms (non-WASM).

#[cfg(not(target_arch = "wasm32"))]
use std::fs::{self, File, OpenOptions};
#[cfg(not(target_arch = "wasm32"))]
use std::io::{Read, Seek, SeekFrom, Write};
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

#[cfg(not(target_arch = "wasm32"))]
use crate::error_chirho::{ErrorChirho, ResultChirho};
#[cfg(not(target_arch = "wasm32"))]
use super::backend_chirho::StorageBackendChirho;

/// Native filesystem storage backend.
///
/// Uses std::fs for all file operations. This is efficient for local
/// module access and supports all SWORD module operations including
/// reading, writing, and memory-mapped access when available.
///
/// # Example
///
/// ```rust,ignore
/// use rsword_chirho::storage_chirho::{StorageBackendChirho, NativeBackendChirho};
///
/// let backend_chirho = NativeBackendChirho::new_chirho();
///
/// // Read module data
/// let data_chirho = backend_chirho.read_file_chirho("/path/to/module/nt")?;
///
/// // Read specific range (efficient for verse lookup)
/// let verse_data_chirho = backend_chirho.read_range_chirho("/path/to/module/nt", 1024, 256)?;
/// ```
#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone, Default)]
pub struct NativeBackendChirho;

#[cfg(not(target_arch = "wasm32"))]
impl NativeBackendChirho {
    /// Create a new native storage backend.
    pub fn new_chirho() -> Self {
        Self
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl StorageBackendChirho for NativeBackendChirho {
    fn read_file_chirho(&self, path_chirho: &str) -> ResultChirho<Vec<u8>> {
        fs::read(path_chirho).map_err(ErrorChirho::IoChirho)
    }

    fn read_range_chirho(
        &self,
        path_chirho: &str,
        offset_chirho: u64,
        len_chirho: usize,
    ) -> ResultChirho<Vec<u8>> {
        let mut file_chirho = File::open(path_chirho).map_err(ErrorChirho::IoChirho)?;
        file_chirho
            .seek(SeekFrom::Start(offset_chirho))
            .map_err(ErrorChirho::IoChirho)?;

        let mut buffer_chirho = vec![0u8; len_chirho];
        file_chirho
            .read_exact(&mut buffer_chirho)
            .map_err(ErrorChirho::IoChirho)?;

        Ok(buffer_chirho)
    }

    fn write_file_chirho(&self, path_chirho: &str, data_chirho: &[u8]) -> ResultChirho<()> {
        // Create parent directories if needed
        if let Some(parent_chirho) = Path::new(path_chirho).parent() {
            fs::create_dir_all(parent_chirho).map_err(ErrorChirho::IoChirho)?;
        }

        fs::write(path_chirho, data_chirho).map_err(ErrorChirho::IoChirho)
    }

    fn file_exists_chirho(&self, path_chirho: &str) -> bool {
        Path::new(path_chirho).exists()
    }

    fn list_dir_chirho(&self, path_chirho: &str) -> ResultChirho<Vec<String>> {
        let entries_chirho = fs::read_dir(path_chirho).map_err(ErrorChirho::IoChirho)?;

        let mut names_chirho = Vec::new();
        for entry_chirho in entries_chirho {
            let entry_chirho = entry_chirho.map_err(ErrorChirho::IoChirho)?;
            if let Some(name_chirho) = entry_chirho.file_name().to_str() {
                names_chirho.push(name_chirho.to_string());
            }
        }

        Ok(names_chirho)
    }

    fn create_dir_chirho(&self, path_chirho: &str) -> ResultChirho<()> {
        fs::create_dir_all(path_chirho).map_err(ErrorChirho::IoChirho)
    }

    fn file_size_chirho(&self, path_chirho: &str) -> ResultChirho<u64> {
        let metadata_chirho = fs::metadata(path_chirho).map_err(ErrorChirho::IoChirho)?;
        Ok(metadata_chirho.len())
    }

    fn append_file_chirho(&self, path_chirho: &str, data_chirho: &[u8]) -> ResultChirho<()> {
        let mut file_chirho = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path_chirho)
            .map_err(ErrorChirho::IoChirho)?;

        file_chirho.write_all(data_chirho).map_err(ErrorChirho::IoChirho)
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests_chirho {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_read_write_file_chirho() {
        let temp_dir_chirho = TempDir::new().unwrap();
        let file_path_chirho = temp_dir_chirho.path().join("test.txt");
        let path_str_chirho = file_path_chirho.to_str().unwrap();

        let backend_chirho = NativeBackendChirho::new_chirho();

        // Write
        backend_chirho
            .write_file_chirho(path_str_chirho, b"Hello, World!")
            .unwrap();

        // Read
        let data_chirho = backend_chirho.read_file_chirho(path_str_chirho).unwrap();
        assert_eq!(&data_chirho, b"Hello, World!");
    }

    #[test]
    fn test_read_range_chirho() {
        let temp_dir_chirho = TempDir::new().unwrap();
        let file_path_chirho = temp_dir_chirho.path().join("test.txt");
        let path_str_chirho = file_path_chirho.to_str().unwrap();

        let backend_chirho = NativeBackendChirho::new_chirho();
        backend_chirho
            .write_file_chirho(path_str_chirho, b"Hello, World!")
            .unwrap();

        // Read range "World"
        let data_chirho = backend_chirho
            .read_range_chirho(path_str_chirho, 7, 5)
            .unwrap();
        assert_eq!(&data_chirho, b"World");
    }

    #[test]
    fn test_file_exists_chirho() {
        let temp_dir_chirho = TempDir::new().unwrap();
        let file_path_chirho = temp_dir_chirho.path().join("test.txt");
        let path_str_chirho = file_path_chirho.to_str().unwrap();

        let backend_chirho = NativeBackendChirho::new_chirho();

        assert!(!backend_chirho.file_exists_chirho(path_str_chirho));

        backend_chirho
            .write_file_chirho(path_str_chirho, b"test")
            .unwrap();

        assert!(backend_chirho.file_exists_chirho(path_str_chirho));
    }

    #[test]
    fn test_list_dir_chirho() {
        let temp_dir_chirho = TempDir::new().unwrap();
        let dir_path_chirho = temp_dir_chirho.path().to_str().unwrap();

        let backend_chirho = NativeBackendChirho::new_chirho();

        // Create some files
        backend_chirho
            .write_file_chirho(
                temp_dir_chirho.path().join("a.txt").to_str().unwrap(),
                b"a",
            )
            .unwrap();
        backend_chirho
            .write_file_chirho(
                temp_dir_chirho.path().join("b.txt").to_str().unwrap(),
                b"b",
            )
            .unwrap();

        let files_chirho = backend_chirho.list_dir_chirho(dir_path_chirho).unwrap();
        assert!(files_chirho.contains(&"a.txt".to_string()));
        assert!(files_chirho.contains(&"b.txt".to_string()));
    }

    #[test]
    fn test_append_file_chirho() {
        let temp_dir_chirho = TempDir::new().unwrap();
        let file_path_chirho = temp_dir_chirho.path().join("test.txt");
        let path_str_chirho = file_path_chirho.to_str().unwrap();

        let backend_chirho = NativeBackendChirho::new_chirho();

        backend_chirho
            .write_file_chirho(path_str_chirho, b"Hello")
            .unwrap();
        backend_chirho
            .append_file_chirho(path_str_chirho, b", World!")
            .unwrap();

        let data_chirho = backend_chirho.read_file_chirho(path_str_chirho).unwrap();
        assert_eq!(&data_chirho, b"Hello, World!");
    }
}
