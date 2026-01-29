// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! RawStr storage format for lexicons and dictionaries.
//!
//! RawStr stores key-value pairs where keys are null-terminated strings.
//! Used for Strong's numbers, dictionaries, etc.

use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use crate::byte_order_chirho::{ReadSwordChirho, WriteSwordChirho};
use crate::error_chirho::{ErrorChirho, ResultChirho};

/// RawStr storage format for lexicons.
pub struct RawStrChirho {
    /// Path to the module data directory.
    path_chirho: PathBuf,
    /// Index file.
    idx_file_chirho: Option<File>,
    /// Data file.
    dat_file_chirho: Option<File>,
}

impl RawStrChirho {
    /// Open an existing RawStr module.
    pub fn open_chirho<P: AsRef<Path>>(path_chirho: P, basename_chirho: &str) -> ResultChirho<Self> {
        let path_chirho = path_chirho.as_ref().to_path_buf();

        let idx_path_chirho = path_chirho.join(format!("{}.idx", basename_chirho));
        let dat_path_chirho = path_chirho.join(format!("{}.dat", basename_chirho));

        let idx_file_chirho = File::open(&idx_path_chirho).ok();
        let dat_file_chirho = File::open(&dat_path_chirho).ok();

        if idx_file_chirho.is_none() {
            return Err(ErrorChirho::InvalidModulePathChirho { path_chirho });
        }

        Ok(Self {
            path_chirho,
            idx_file_chirho,
            dat_file_chirho,
        })
    }

    /// Create a new RawStr module.
    pub fn create_chirho<P: AsRef<Path>>(path_chirho: P, basename_chirho: &str) -> ResultChirho<Self> {
        let path_chirho = path_chirho.as_ref().to_path_buf();
        std::fs::create_dir_all(&path_chirho)?;

        let idx_path_chirho = path_chirho.join(format!("{}.idx", basename_chirho));
        let dat_path_chirho = path_chirho.join(format!("{}.dat", basename_chirho));

        let idx_file_chirho = Some(OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&idx_path_chirho)?);
        let dat_file_chirho = Some(OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&dat_path_chirho)?);

        Ok(Self {
            path_chirho,
            idx_file_chirho,
            dat_file_chirho,
        })
    }

    /// Read an entry by index position.
    pub fn read_entry_chirho(&mut self, idx_off_chirho: u32) -> ResultChirho<(String, String)> {
        // Index entry format: 4-byte offset + 4-byte size
        let byte_offset_chirho = idx_off_chirho as u64 * 8;

        let idx_file_chirho = self.idx_file_chirho.as_mut()
            .ok_or_else(|| ErrorChirho::generic_chirho("Index file not available"))?;

        idx_file_chirho.seek(SeekFrom::Start(byte_offset_chirho))?;

        let data_offset_chirho = idx_file_chirho.read_u32_sword_chirho()?;
        let data_size_chirho = idx_file_chirho.read_u32_sword_chirho()?;

        if data_size_chirho == 0 {
            return Ok((String::new(), String::new()));
        }

        let dat_file_chirho = self.dat_file_chirho.as_mut()
            .ok_or_else(|| ErrorChirho::generic_chirho("Data file not available"))?;

        dat_file_chirho.seek(SeekFrom::Start(data_offset_chirho as u64))?;

        let mut buf_chirho = vec![0u8; data_size_chirho as usize];
        dat_file_chirho.read_exact(&mut buf_chirho)?;

        // Find null terminator separating key from value
        let null_pos_chirho = buf_chirho.iter().position(|&b| b == 0);

        let (key_bytes_chirho, value_bytes_chirho) = if let Some(pos_chirho) = null_pos_chirho {
            (&buf_chirho[..pos_chirho], &buf_chirho[pos_chirho + 1..])
        } else {
            (&buf_chirho[..], &[][..])
        };

        let key_chirho = String::from_utf8_lossy(key_bytes_chirho).to_string();
        let value_chirho = String::from_utf8_lossy(value_bytes_chirho).to_string();

        Ok((key_chirho, value_chirho))
    }

    /// Find an entry by key.
    pub fn find_by_key_chirho(&mut self, key_chirho: &str) -> ResultChirho<Option<String>> {
        let idx_file_chirho = self.idx_file_chirho.as_mut()
            .ok_or_else(|| ErrorChirho::generic_chirho("Index file not available"))?;

        // Get file size
        let file_size_chirho = idx_file_chirho.seek(SeekFrom::End(0))?;
        let entry_count_chirho = file_size_chirho / 8;

        // Binary search
        let mut low_chirho = 0u64;
        let mut high_chirho = entry_count_chirho;

        while low_chirho < high_chirho {
            let mid_chirho = (low_chirho + high_chirho) / 2;
            let (entry_key_chirho, entry_value_chirho) = self.read_entry_chirho(mid_chirho as u32)?;

            match entry_key_chirho.as_str().cmp(key_chirho) {
                std::cmp::Ordering::Equal => return Ok(Some(entry_value_chirho)),
                std::cmp::Ordering::Less => low_chirho = mid_chirho + 1,
                std::cmp::Ordering::Greater => high_chirho = mid_chirho,
            }
        }

        Ok(None)
    }

    /// Write an entry.
    pub fn write_entry_chirho(&mut self, key_chirho: &str, value_chirho: &str) -> ResultChirho<()> {
        let idx_file_chirho = self.idx_file_chirho.as_mut()
            .ok_or_else(|| ErrorChirho::generic_chirho("Index file not available"))?;
        let dat_file_chirho = self.dat_file_chirho.as_mut()
            .ok_or_else(|| ErrorChirho::generic_chirho("Data file not available"))?;

        // Get current data file position
        let data_offset_chirho = dat_file_chirho.seek(SeekFrom::End(0))? as u32;

        // Write key + null + value
        let key_bytes_chirho = key_chirho.as_bytes();
        let value_bytes_chirho = value_chirho.as_bytes();
        let data_size_chirho = key_bytes_chirho.len() + 1 + value_bytes_chirho.len();

        dat_file_chirho.write_all(key_bytes_chirho)?;
        dat_file_chirho.write_all(&[0])?;
        dat_file_chirho.write_all(value_bytes_chirho)?;

        // Write index entry
        idx_file_chirho.seek(SeekFrom::End(0))?;
        idx_file_chirho.write_u32_sword_chirho(data_offset_chirho)?;
        idx_file_chirho.write_u32_sword_chirho(data_size_chirho as u32)?;

        Ok(())
    }

    /// Get the path to the module.
    pub fn path_chirho(&self) -> &Path {
        &self.path_chirho
    }

    /// Get the number of entries.
    pub fn entry_count_chirho(&mut self) -> ResultChirho<u32> {
        let idx_file_chirho = self.idx_file_chirho.as_mut()
            .ok_or_else(|| ErrorChirho::generic_chirho("Index file not available"))?;

        let file_size_chirho = idx_file_chirho.seek(SeekFrom::End(0))?;
        Ok((file_size_chirho / 8) as u32)
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_create_and_write_chirho() {
        let temp_dir_chirho = TempDir::new().unwrap();
        let mod_path_chirho = temp_dir_chirho.path().join("testmod");

        let mut storage_chirho = RawStrChirho::create_chirho(&mod_path_chirho, "dict").unwrap();

        storage_chirho.write_entry_chirho("G2316", "theos - God").unwrap();
        storage_chirho.write_entry_chirho("G26", "agape - love").unwrap();

        let value_chirho = storage_chirho.find_by_key_chirho("G2316").unwrap();
        assert_eq!(value_chirho, Some("theos - God".to_string()));
    }
}
