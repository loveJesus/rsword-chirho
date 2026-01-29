// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! RawVerse storage format implementation.
//!
//! RawVerse is the simplest storage format for SWORD Bible modules.
//! It uses 6-byte index entries (4-byte offset + 2-byte size) and
//! stores text in separate OT and NT files.
//!
//! Files:
//! - `ot.vss` - Old Testament verse index (6 bytes per verse)
//! - `nt.vss` - New Testament verse index (6 bytes per verse)
//! - `ot` - Old Testament text data
//! - `nt` - New Testament text data

use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use crate::byte_order_chirho::{RawVerseIndexChirho, ReadSwordChirho, WriteSwordChirho};
use crate::error_chirho::{ErrorChirho, ResultChirho};

/// RawVerse storage format with 6-byte index entries.
pub struct RawVerseChirho {
    /// Path to the module data directory.
    path_chirho: PathBuf,
    /// OT index file.
    ot_idx_chirho: Option<File>,
    /// NT index file.
    nt_idx_chirho: Option<File>,
    /// OT text file.
    ot_text_chirho: Option<File>,
    /// NT text file.
    nt_text_chirho: Option<File>,
}

impl RawVerseChirho {
    /// Open an existing RawVerse module.
    pub fn open_chirho<P: AsRef<Path>>(path_chirho: P) -> ResultChirho<Self> {
        let path_chirho = path_chirho.as_ref().to_path_buf();

        let ot_idx_chirho = File::open(path_chirho.join("ot.vss")).ok();
        let nt_idx_chirho = File::open(path_chirho.join("nt.vss")).ok();
        let ot_text_chirho = File::open(path_chirho.join("ot")).ok();
        let nt_text_chirho = File::open(path_chirho.join("nt")).ok();

        if ot_idx_chirho.is_none() && nt_idx_chirho.is_none() {
            return Err(ErrorChirho::InvalidModulePathChirho { path_chirho });
        }

        Ok(Self {
            path_chirho,
            ot_idx_chirho,
            nt_idx_chirho,
            ot_text_chirho,
            nt_text_chirho,
        })
    }

    /// Create a new RawVerse module.
    pub fn create_chirho<P: AsRef<Path>>(path_chirho: P) -> ResultChirho<Self> {
        let path_chirho = path_chirho.as_ref().to_path_buf();
        std::fs::create_dir_all(&path_chirho)?;

        let ot_idx_chirho = Some(OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path_chirho.join("ot.vss"))?);
        let nt_idx_chirho = Some(OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path_chirho.join("nt.vss"))?);
        let ot_text_chirho = Some(OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path_chirho.join("ot"))?);
        let nt_text_chirho = Some(OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(path_chirho.join("nt"))?);

        Ok(Self {
            path_chirho,
            ot_idx_chirho,
            nt_idx_chirho,
            ot_text_chirho,
            nt_text_chirho,
        })
    }

    /// Find the offset and size for a verse.
    ///
    /// # Arguments
    /// * `testament_chirho` - Testament (1=OT, 2=NT)
    /// * `idx_off_chirho` - Index offset (verse number within testament)
    pub fn find_offset_chirho(
        &mut self,
        testament_chirho: u8,
        idx_off_chirho: u32,
    ) -> ResultChirho<RawVerseIndexChirho> {
        let byte_offset_chirho = idx_off_chirho as u64 * RawVerseIndexChirho::SIZE_CHIRHO as u64;

        let idx_file_chirho = match testament_chirho {
            1 => self.ot_idx_chirho.as_mut(),
            2 => self.nt_idx_chirho.as_mut(),
            _ => return Err(ErrorChirho::generic_chirho("Invalid testament")),
        };

        let idx_file_chirho = idx_file_chirho
            .ok_or_else(|| ErrorChirho::generic_chirho("Index file not available"))?;

        idx_file_chirho.seek(SeekFrom::Start(byte_offset_chirho))?;

        let offset_chirho = idx_file_chirho.read_u32_sword_chirho()?;
        let size_chirho = idx_file_chirho.read_u16_sword_chirho()?;

        Ok(RawVerseIndexChirho {
            offset_chirho,
            size_chirho,
        })
    }

    /// Read verse text at a given location.
    ///
    /// # Arguments
    /// * `testament_chirho` - Testament (1=OT, 2=NT)
    /// * `offset_chirho` - Offset in the text file
    /// * `size_chirho` - Size of the text
    pub fn read_text_chirho(
        &mut self,
        testament_chirho: u8,
        offset_chirho: u32,
        size_chirho: u16,
    ) -> ResultChirho<String> {
        if size_chirho == 0 {
            return Ok(String::new());
        }

        let text_file_chirho = match testament_chirho {
            1 => self.ot_text_chirho.as_mut(),
            2 => self.nt_text_chirho.as_mut(),
            _ => return Err(ErrorChirho::generic_chirho("Invalid testament")),
        };

        let text_file_chirho = text_file_chirho
            .ok_or_else(|| ErrorChirho::generic_chirho("Text file not available"))?;

        text_file_chirho.seek(SeekFrom::Start(offset_chirho as u64))?;

        let mut buf_chirho = vec![0u8; size_chirho as usize];
        text_file_chirho.read_exact(&mut buf_chirho)?;

        // Try UTF-8 first, fall back to Latin-1
        match String::from_utf8(buf_chirho.clone()) {
            Ok(s) => Ok(s),
            Err(_) => {
                // Convert from Latin-1
                Ok(buf_chirho.iter().map(|&b| b as char).collect())
            }
        }
    }

    /// Read a verse by its index offset.
    pub fn read_verse_chirho(
        &mut self,
        testament_chirho: u8,
        idx_off_chirho: u32,
    ) -> ResultChirho<String> {
        let index_chirho = self.find_offset_chirho(testament_chirho, idx_off_chirho)?;
        self.read_text_chirho(testament_chirho, index_chirho.offset_chirho, index_chirho.size_chirho)
    }

    /// Write verse text.
    ///
    /// # Arguments
    /// * `testament_chirho` - Testament (1=OT, 2=NT)
    /// * `idx_off_chirho` - Index offset (verse number)
    /// * `text_chirho` - Text to write
    pub fn write_verse_chirho(
        &mut self,
        testament_chirho: u8,
        idx_off_chirho: u32,
        text_chirho: &str,
    ) -> ResultChirho<()> {
        let byte_offset_chirho = idx_off_chirho as u64 * RawVerseIndexChirho::SIZE_CHIRHO as u64;

        let (idx_file_chirho, text_file_chirho) = match testament_chirho {
            1 => (self.ot_idx_chirho.as_mut(), self.ot_text_chirho.as_mut()),
            2 => (self.nt_idx_chirho.as_mut(), self.nt_text_chirho.as_mut()),
            _ => return Err(ErrorChirho::generic_chirho("Invalid testament")),
        };

        let idx_file_chirho = idx_file_chirho
            .ok_or_else(|| ErrorChirho::generic_chirho("Index file not available"))?;
        let text_file_chirho = text_file_chirho
            .ok_or_else(|| ErrorChirho::generic_chirho("Text file not available"))?;

        // Get current text file position (where we'll write)
        let text_offset_chirho = text_file_chirho.seek(SeekFrom::End(0))? as u32;

        // Write the text
        let text_bytes_chirho = text_chirho.as_bytes();
        let size_chirho = text_bytes_chirho.len() as u16;

        if size_chirho > 0 {
            text_file_chirho.write_all(text_bytes_chirho)?;
            text_file_chirho.write_all(b"\n")?; // Add newline for readability
        }

        // Write the index entry
        idx_file_chirho.seek(SeekFrom::Start(byte_offset_chirho))?;
        idx_file_chirho.write_u32_sword_chirho(if size_chirho > 0 { text_offset_chirho } else { 0 })?;
        idx_file_chirho.write_u16_sword_chirho(size_chirho)?;

        Ok(())
    }

    /// Link one verse to another (make it point to the same text).
    pub fn link_verse_chirho(
        &mut self,
        testament_chirho: u8,
        dest_idx_chirho: u32,
        src_idx_chirho: u32,
    ) -> ResultChirho<()> {
        let src_entry_chirho = self.find_offset_chirho(testament_chirho, src_idx_chirho)?;

        let byte_offset_chirho = dest_idx_chirho as u64 * RawVerseIndexChirho::SIZE_CHIRHO as u64;

        let idx_file_chirho = match testament_chirho {
            1 => self.ot_idx_chirho.as_mut(),
            2 => self.nt_idx_chirho.as_mut(),
            _ => return Err(ErrorChirho::generic_chirho("Invalid testament")),
        };

        let idx_file_chirho = idx_file_chirho
            .ok_or_else(|| ErrorChirho::generic_chirho("Index file not available"))?;

        idx_file_chirho.seek(SeekFrom::Start(byte_offset_chirho))?;
        idx_file_chirho.write_u32_sword_chirho(src_entry_chirho.offset_chirho)?;
        idx_file_chirho.write_u16_sword_chirho(src_entry_chirho.size_chirho)?;

        Ok(())
    }

    /// Get the path to the module.
    pub fn path_chirho(&self) -> &Path {
        &self.path_chirho
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

        let mut storage_chirho = RawVerseChirho::create_chirho(&mod_path_chirho).unwrap();

        // Write a verse
        storage_chirho.write_verse_chirho(1, 0, "In the beginning").unwrap();

        // Read it back
        let text_chirho = storage_chirho.read_verse_chirho(1, 0).unwrap();
        assert_eq!(text_chirho, "In the beginning");
    }

    #[test]
    fn test_empty_verse_chirho() {
        let temp_dir_chirho = TempDir::new().unwrap();
        let mod_path_chirho = temp_dir_chirho.path().join("testmod");

        let mut storage_chirho = RawVerseChirho::create_chirho(&mod_path_chirho).unwrap();

        // Write an empty verse
        storage_chirho.write_verse_chirho(1, 0, "").unwrap();

        // Read it back
        let text_chirho = storage_chirho.read_verse_chirho(1, 0).unwrap();
        assert!(text_chirho.is_empty());
    }

    #[test]
    fn test_link_verse_chirho() {
        let temp_dir_chirho = TempDir::new().unwrap();
        let mod_path_chirho = temp_dir_chirho.path().join("testmod");

        let mut storage_chirho = RawVerseChirho::create_chirho(&mod_path_chirho).unwrap();

        // Write source verse
        storage_chirho.write_verse_chirho(1, 5, "Original text").unwrap();

        // Link another verse to it
        storage_chirho.link_verse_chirho(1, 10, 5).unwrap();

        // Both should return the same text
        let text1_chirho = storage_chirho.read_verse_chirho(1, 5).unwrap();
        let text2_chirho = storage_chirho.read_verse_chirho(1, 10).unwrap();
        assert_eq!(text1_chirho, text2_chirho);
    }
}
