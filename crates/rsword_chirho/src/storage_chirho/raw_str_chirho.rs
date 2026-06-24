// For God so loved the world, that he gave his only begotten Son,
// that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! RawStr storage format for (uncompressed) lexicons, dictionaries and devotionals.
//!
//! Matches SWORD's `RawLD`/`RawLD4` on-disk layout:
//! - `.idx` — index, one record per entry: `u32 dataOffset` + size. The size is
//!   `u16` for `RawLD` (6-byte records) and `u32` for `RawLD4` (8-byte records).
//! - `.dat` — at `[dataOffset .. dataOffset+dataSize]`, a `<key><sep><value>`
//!   record. Real modules separate key and value with a newline (`\r\n`/`\n`);
//!   modules written by this crate use `\n`. A legacy NUL separator is also
//!   accepted on read.

use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use crate::byte_order_chirho::{ReadSwordChirho, WriteSwordChirho};
use crate::error_chirho::{ErrorChirho, ResultChirho};

/// `.idx` record size for `RawLD` (`u32` offset + `u16` size).
const IDX_SIZE_2BYTE_CHIRHO: u64 = 6;
/// `.idx` record size for `RawLD4` (`u32` offset + `u32` size).
const IDX_SIZE_4BYTE_CHIRHO: u64 = 8;

/// Split a `.dat` record into `(key, value)` on the first newline or NUL.
fn split_record_chirho(buf_chirho: &[u8]) -> (String, String) {
    match buf_chirho.iter().position(|&b_chirho| b_chirho == b'\n' || b_chirho == 0) {
        Some(pos_chirho) => {
            let key_chirho = String::from_utf8_lossy(&buf_chirho[..pos_chirho])
                .trim_end_matches(['\r', '\0'])
                .to_string();
            let value_chirho = String::from_utf8_lossy(&buf_chirho[pos_chirho + 1..]).to_string();
            (key_chirho, value_chirho)
        }
        None => (
            String::from_utf8_lossy(buf_chirho).trim_end_matches(['\r', '\0', ' ']).to_string(),
            String::new(),
        ),
    }
}

/// RawStr storage format for lexicons.
pub struct RawStrChirho {
    /// Path to the module data directory.
    path_chirho: PathBuf,
    /// Index file.
    idx_file_chirho: Option<File>,
    /// Data file.
    dat_file_chirho: Option<File>,
    /// Whether index records use 4-byte sizes (`RawLD4`) instead of 2-byte (`RawLD`).
    four_byte_chirho: bool,
}

impl RawStrChirho {
    /// Size of one `.idx` record for this module.
    fn idx_entry_size_chirho(&self) -> u64 {
        if self.four_byte_chirho { IDX_SIZE_4BYTE_CHIRHO } else { IDX_SIZE_2BYTE_CHIRHO }
    }

    /// Open an existing RawStr module. `four_byte_chirho` selects `RawLD4`
    /// (8-byte index records) over `RawLD` (6-byte).
    pub fn open_chirho<P: AsRef<Path>>(
        path_chirho: P,
        basename_chirho: &str,
        four_byte_chirho: bool,
    ) -> ResultChirho<Self> {
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
            four_byte_chirho,
        })
    }

    /// Create a new RawStr module.
    pub fn create_chirho<P: AsRef<Path>>(
        path_chirho: P,
        basename_chirho: &str,
        four_byte_chirho: bool,
    ) -> ResultChirho<Self> {
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
            four_byte_chirho,
        })
    }

    /// Read an entry by index position, returning `(key, value)`.
    pub fn read_entry_chirho(&mut self, idx_off_chirho: u32) -> ResultChirho<(String, String)> {
        let four_byte_chirho = self.four_byte_chirho;
        let byte_offset_chirho = idx_off_chirho as u64 * self.idx_entry_size_chirho();

        let idx_file_chirho = self.idx_file_chirho.as_mut()
            .ok_or_else(|| ErrorChirho::generic_chirho("Index file not available"))?;

        idx_file_chirho.seek(SeekFrom::Start(byte_offset_chirho))?;

        let data_offset_chirho = idx_file_chirho.read_u32_sword_chirho()?;
        let data_size_chirho = if four_byte_chirho {
            idx_file_chirho.read_u32_sword_chirho()?
        } else {
            idx_file_chirho.read_u16_sword_chirho()? as u32
        };

        if data_size_chirho == 0 {
            return Ok((String::new(), String::new()));
        }

        let dat_file_chirho = self.dat_file_chirho.as_mut()
            .ok_or_else(|| ErrorChirho::generic_chirho("Data file not available"))?;

        dat_file_chirho.seek(SeekFrom::Start(data_offset_chirho as u64))?;

        let mut buf_chirho = vec![0u8; data_size_chirho as usize];
        dat_file_chirho.read_exact(&mut buf_chirho)?;

        Ok(split_record_chirho(&buf_chirho))
    }

    /// Find an entry by key (binary search over the sorted key index).
    pub fn find_by_key_chirho(&mut self, key_chirho: &str) -> ResultChirho<Option<String>> {
        let entry_count_chirho = self.entry_count_chirho()? as u64;
        if entry_count_chirho == 0 {
            return Ok(None);
        }

        let mut low_chirho = 0u64;
        let mut high_chirho = entry_count_chirho;

        while low_chirho < high_chirho {
            let mid_chirho = low_chirho + (high_chirho - low_chirho) / 2;
            let (entry_key_chirho, entry_value_chirho) = self.read_entry_chirho(mid_chirho as u32)?;

            match entry_key_chirho.as_str().cmp(key_chirho) {
                std::cmp::Ordering::Equal => return Ok(Some(entry_value_chirho)),
                std::cmp::Ordering::Less => low_chirho = mid_chirho + 1,
                std::cmp::Ordering::Greater => high_chirho = mid_chirho,
            }
        }

        Ok(None)
    }

    /// Write an entry (`<key>\n<value>` in `.dat`, with a matching index record).
    pub fn write_entry_chirho(&mut self, key_chirho: &str, value_chirho: &str) -> ResultChirho<()> {
        let four_byte_chirho = self.four_byte_chirho;
        let key_bytes_chirho = key_chirho.as_bytes();
        let value_bytes_chirho = value_chirho.as_bytes();
        let data_size_chirho = key_bytes_chirho.len() + 1 + value_bytes_chirho.len();

        if !four_byte_chirho && data_size_chirho > u16::MAX as usize {
            return Err(ErrorChirho::generic_chirho(
                "Entry too large for RawLD (2-byte size); use RawLD4",
            ));
        }

        let dat_file_chirho = self.dat_file_chirho.as_mut()
            .ok_or_else(|| ErrorChirho::generic_chirho("Data file not available"))?;
        let data_offset_chirho = dat_file_chirho.seek(SeekFrom::End(0))? as u32;
        dat_file_chirho.write_all(key_bytes_chirho)?;
        dat_file_chirho.write_all(b"\n")?;
        dat_file_chirho.write_all(value_bytes_chirho)?;

        let idx_file_chirho = self.idx_file_chirho.as_mut()
            .ok_or_else(|| ErrorChirho::generic_chirho("Index file not available"))?;
        idx_file_chirho.seek(SeekFrom::End(0))?;
        idx_file_chirho.write_u32_sword_chirho(data_offset_chirho)?;
        if four_byte_chirho {
            idx_file_chirho.write_u32_sword_chirho(data_size_chirho as u32)?;
        } else {
            idx_file_chirho.write_u16_sword_chirho(data_size_chirho as u16)?;
        }

        Ok(())
    }

    /// Get the path to the module.
    pub fn path_chirho(&self) -> &Path {
        &self.path_chirho
    }

    /// Get the number of entries.
    pub fn entry_count_chirho(&mut self) -> ResultChirho<u32> {
        let entry_size_chirho = self.idx_entry_size_chirho();
        let idx_file_chirho = self.idx_file_chirho.as_mut()
            .ok_or_else(|| ErrorChirho::generic_chirho("Index file not available"))?;

        let file_size_chirho = idx_file_chirho.seek(SeekFrom::End(0))?;
        Ok((file_size_chirho / entry_size_chirho) as u32)
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

        // RawLD (2-byte sizes). Keys are written in sorted order for binary search.
        let mut storage_chirho = RawStrChirho::create_chirho(&mod_path_chirho, "dict", false).unwrap();
        storage_chirho.write_entry_chirho("G0026", "agape - love").unwrap();
        storage_chirho.write_entry_chirho("G2316", "theos - God").unwrap();

        assert_eq!(storage_chirho.entry_count_chirho().unwrap(), 2);
        assert_eq!(
            storage_chirho.find_by_key_chirho("G2316").unwrap(),
            Some("theos - God".to_string())
        );
        assert_eq!(
            storage_chirho.find_by_key_chirho("G0026").unwrap(),
            Some("agape - love".to_string())
        );
        assert_eq!(storage_chirho.find_by_key_chirho("G9999").unwrap(), None);
    }

    #[test]
    fn test_reads_real_format_newline_separator_chirho() {
        // Real RawLD records separate key and value with a CR/LF, not a NUL.
        // Hand-build a 6-byte-index module and confirm we parse it like SWORD does.
        let temp_dir_chirho = TempDir::new().unwrap();
        let dir_chirho = temp_dir_chirho.path();

        let rec0_chirho = b"01.01\r\n<i>Morning</i> reading".to_vec();
        let rec1_chirho = b"01.02\r\n<i>Evening</i> reading".to_vec();
        let mut dat_chirho = Vec::new();
        let mut idx_chirho = Vec::new();
        for rec_chirho in [&rec0_chirho, &rec1_chirho] {
            let off_chirho = dat_chirho.len() as u32;
            idx_chirho.extend_from_slice(&off_chirho.to_le_bytes());
            idx_chirho.extend_from_slice(&(rec_chirho.len() as u16).to_le_bytes());
            dat_chirho.extend_from_slice(rec_chirho);
        }
        std::fs::write(dir_chirho.join("dict.dat"), &dat_chirho).unwrap();
        std::fs::write(dir_chirho.join("dict.idx"), &idx_chirho).unwrap();

        let mut storage_chirho = RawStrChirho::open_chirho(dir_chirho, "dict", false).unwrap();
        assert_eq!(storage_chirho.entry_count_chirho().unwrap(), 2);
        let (key_chirho, value_chirho) = storage_chirho.read_entry_chirho(0).unwrap();
        assert_eq!(key_chirho, "01.01");
        assert!(value_chirho.contains("Morning"));
        assert!(storage_chirho.find_by_key_chirho("01.02").unwrap().unwrap().contains("Evening"));
    }
}
