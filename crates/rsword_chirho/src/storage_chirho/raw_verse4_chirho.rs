// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! RawVerse4 storage format implementation.
//!
//! RawVerse4 is similar to RawVerse but uses 8-byte index entries
//! (4-byte offset + 4-byte size) to support larger text entries.

use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use crate::byte_order_chirho::{RawVerse4IndexChirho, ReadSwordChirho, WriteSwordChirho};
use crate::error_chirho::{ErrorChirho, ResultChirho};

/// RawVerse4 storage format with 8-byte index entries.
pub struct RawVerse4Chirho {
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

impl RawVerse4Chirho {
    /// Open an existing RawVerse4 module.
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

    /// Create a new RawVerse4 module.
    pub fn create_chirho<P: AsRef<Path>>(path_chirho: P) -> ResultChirho<Self> {
        let path_chirho = path_chirho.as_ref().to_path_buf();
        std::fs::create_dir_all(&path_chirho)?;

        let ot_idx_chirho = Some(File::create(path_chirho.join("ot.vss"))?);
        let nt_idx_chirho = Some(File::create(path_chirho.join("nt.vss"))?);
        let ot_text_chirho = Some(File::create(path_chirho.join("ot"))?);
        let nt_text_chirho = Some(File::create(path_chirho.join("nt"))?);

        Ok(Self {
            path_chirho,
            ot_idx_chirho,
            nt_idx_chirho,
            ot_text_chirho,
            nt_text_chirho,
        })
    }

    /// Find the offset and size for a verse.
    pub fn find_offset_chirho(
        &mut self,
        testament_chirho: u8,
        idx_off_chirho: u32,
    ) -> ResultChirho<RawVerse4IndexChirho> {
        let byte_offset_chirho = idx_off_chirho as u64 * RawVerse4IndexChirho::SIZE_CHIRHO as u64;

        let idx_file_chirho = match testament_chirho {
            1 => self.ot_idx_chirho.as_mut(),
            2 => self.nt_idx_chirho.as_mut(),
            _ => return Err(ErrorChirho::generic_chirho("Invalid testament")),
        };

        let idx_file_chirho = idx_file_chirho
            .ok_or_else(|| ErrorChirho::generic_chirho("Index file not available"))?;

        idx_file_chirho.seek(SeekFrom::Start(byte_offset_chirho))?;

        let offset_chirho = idx_file_chirho.read_u32_sword_chirho()?;
        let size_chirho = idx_file_chirho.read_u32_sword_chirho()?;

        Ok(RawVerse4IndexChirho {
            offset_chirho,
            size_chirho,
        })
    }

    /// Read verse text at a given location.
    pub fn read_text_chirho(
        &mut self,
        testament_chirho: u8,
        offset_chirho: u32,
        size_chirho: u32,
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

        match String::from_utf8(buf_chirho.clone()) {
            Ok(s) => Ok(s),
            Err(_) => Ok(buf_chirho.iter().map(|&b| b as char).collect()),
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
    pub fn write_verse_chirho(
        &mut self,
        testament_chirho: u8,
        idx_off_chirho: u32,
        text_chirho: &str,
    ) -> ResultChirho<()> {
        let byte_offset_chirho = idx_off_chirho as u64 * RawVerse4IndexChirho::SIZE_CHIRHO as u64;

        let (idx_file_chirho, text_file_chirho) = match testament_chirho {
            1 => (self.ot_idx_chirho.as_mut(), self.ot_text_chirho.as_mut()),
            2 => (self.nt_idx_chirho.as_mut(), self.nt_text_chirho.as_mut()),
            _ => return Err(ErrorChirho::generic_chirho("Invalid testament")),
        };

        let idx_file_chirho = idx_file_chirho
            .ok_or_else(|| ErrorChirho::generic_chirho("Index file not available"))?;
        let text_file_chirho = text_file_chirho
            .ok_or_else(|| ErrorChirho::generic_chirho("Text file not available"))?;

        let text_offset_chirho = text_file_chirho.seek(SeekFrom::End(0))? as u32;
        let text_bytes_chirho = text_chirho.as_bytes();
        let size_chirho = text_bytes_chirho.len() as u32;

        if size_chirho > 0 {
            text_file_chirho.write_all(text_bytes_chirho)?;
            text_file_chirho.write_all(b"\n")?;
        }

        idx_file_chirho.seek(SeekFrom::Start(byte_offset_chirho))?;
        idx_file_chirho.write_u32_sword_chirho(if size_chirho > 0 { text_offset_chirho } else { 0 })?;
        idx_file_chirho.write_u32_sword_chirho(size_chirho)?;

        Ok(())
    }

    /// Get the path to the module.
    pub fn path_chirho(&self) -> &Path {
        &self.path_chirho
    }
}
