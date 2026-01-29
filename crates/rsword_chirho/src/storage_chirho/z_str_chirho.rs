// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! zStr compressed storage format for lexicons and dictionaries.
//!
//! zStr stores compressed blocks of key-value data. Each entry has an index
//! pointing to a block and an offset within that block.
//!
//! Files:
//! - `.idx` - Entry index (8 bytes per entry: offset + size)
//! - `.zdx` - Block index (12 bytes per block)
//! - `.zdt` - Compressed data blocks

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use crate::byte_order_chirho::ReadSwordChirho;
use crate::compression_chirho::CompressorChirho;
use crate::error_chirho::{ErrorChirho, ResultChirho};

/// zStr compressed storage format for lexicons.
pub struct ZStrChirho {
    /// Path to the module data directory.
    path_chirho: PathBuf,
    /// Entry index file.
    idx_file_chirho: Option<File>,
    /// Block index file.
    zdx_file_chirho: Option<File>,
    /// Compressed data file.
    zdt_file_chirho: Option<File>,
    /// Decompressor.
    compressor_chirho: Box<dyn CompressorChirho>,
    /// Block cache (block_num -> uncompressed data).
    block_cache_chirho: HashMap<u32, Vec<u8>>,
}

impl ZStrChirho {
    /// Open an existing zStr module.
    pub fn open_chirho<P: AsRef<Path>>(
        path_chirho: P,
        basename_chirho: &str,
        compressor_chirho: Box<dyn CompressorChirho>,
    ) -> ResultChirho<Self> {
        let path_chirho = path_chirho.as_ref().to_path_buf();

        let idx_path_chirho = path_chirho.join(format!("{}.idx", basename_chirho));
        let zdx_path_chirho = path_chirho.join(format!("{}.zdx", basename_chirho));
        let zdt_path_chirho = path_chirho.join(format!("{}.zdt", basename_chirho));

        let idx_file_chirho = File::open(&idx_path_chirho).ok();
        let zdx_file_chirho = File::open(&zdx_path_chirho).ok();
        let zdt_file_chirho = File::open(&zdt_path_chirho).ok();

        if idx_file_chirho.is_none() {
            return Err(ErrorChirho::InvalidModulePathChirho { path_chirho });
        }

        Ok(Self {
            path_chirho,
            idx_file_chirho,
            zdx_file_chirho,
            zdt_file_chirho,
            compressor_chirho,
            block_cache_chirho: HashMap::new(),
        })
    }

    /// Read block index entry.
    fn read_block_index_chirho(&mut self, block_num_chirho: u32) -> ResultChirho<(u32, u32, u32)> {
        let byte_offset_chirho = block_num_chirho as u64 * 12;

        let zdx_file_chirho = self.zdx_file_chirho.as_mut()
            .ok_or_else(|| ErrorChirho::generic_chirho("Block index not available"))?;

        zdx_file_chirho.seek(SeekFrom::Start(byte_offset_chirho))?;

        let comp_offset_chirho = zdx_file_chirho.read_u32_sword_chirho()?;
        let comp_size_chirho = zdx_file_chirho.read_u32_sword_chirho()?;
        let uncomp_size_chirho = zdx_file_chirho.read_u32_sword_chirho()?;

        Ok((comp_offset_chirho, comp_size_chirho, uncomp_size_chirho))
    }

    /// Read and decompress a block.
    fn read_block_chirho(&mut self, block_num_chirho: u32) -> ResultChirho<Vec<u8>> {
        // Check cache first
        if let Some(data_chirho) = self.block_cache_chirho.get(&block_num_chirho) {
            return Ok(data_chirho.clone());
        }

        let (comp_offset_chirho, comp_size_chirho, _uncomp_size_chirho) =
            self.read_block_index_chirho(block_num_chirho)?;

        if comp_size_chirho == 0 {
            return Ok(Vec::new());
        }

        let zdt_file_chirho = self.zdt_file_chirho.as_mut()
            .ok_or_else(|| ErrorChirho::generic_chirho("Data file not available"))?;

        zdt_file_chirho.seek(SeekFrom::Start(comp_offset_chirho as u64))?;

        let mut compressed_chirho = vec![0u8; comp_size_chirho as usize];
        zdt_file_chirho.read_exact(&mut compressed_chirho)?;

        let decompressed_chirho = self.compressor_chirho.decompress_chirho(&compressed_chirho)?;

        // Cache the block
        self.block_cache_chirho.insert(block_num_chirho, decompressed_chirho.clone());

        Ok(decompressed_chirho)
    }

    /// Read an entry by index position.
    pub fn read_entry_chirho(&mut self, idx_off_chirho: u32) -> ResultChirho<(String, String)> {
        // Index entry format: 4-byte block_num + 4-byte offset_in_block
        let byte_offset_chirho = idx_off_chirho as u64 * 8;

        let idx_file_chirho = self.idx_file_chirho.as_mut()
            .ok_or_else(|| ErrorChirho::generic_chirho("Index file not available"))?;

        idx_file_chirho.seek(SeekFrom::Start(byte_offset_chirho))?;

        let block_num_chirho = idx_file_chirho.read_u32_sword_chirho()?;
        let offset_in_block_chirho = idx_file_chirho.read_u32_sword_chirho()?;

        let block_data_chirho = self.read_block_chirho(block_num_chirho)?;

        if offset_in_block_chirho as usize >= block_data_chirho.len() {
            return Ok((String::new(), String::new()));
        }

        // Parse key-value from block at offset
        let data_start_chirho = &block_data_chirho[offset_in_block_chirho as usize..];

        // Find null terminator separating key from value
        let null_pos_chirho = data_start_chirho.iter().position(|&b| b == 0);

        let (key_bytes_chirho, rest_chirho) = if let Some(pos_chirho) = null_pos_chirho {
            (&data_start_chirho[..pos_chirho], &data_start_chirho[pos_chirho + 1..])
        } else {
            return Ok((String::new(), String::new()));
        };

        // Find end of value (next null or end of relevant data)
        let value_end_chirho = rest_chirho.iter().position(|&b| b == 0)
            .unwrap_or(rest_chirho.len());
        let value_bytes_chirho = &rest_chirho[..value_end_chirho];

        let key_chirho = String::from_utf8_lossy(key_bytes_chirho).to_string();
        let value_chirho = String::from_utf8_lossy(value_bytes_chirho).to_string();

        Ok((key_chirho, value_chirho))
    }

    /// Find an entry by key using binary search.
    pub fn find_by_key_chirho(&mut self, key_chirho: &str) -> ResultChirho<Option<String>> {
        let entry_count_chirho = self.entry_count_chirho()?;
        if entry_count_chirho == 0 {
            return Ok(None);
        }

        // Binary search
        let mut low_chirho = 0u32;
        let mut high_chirho = entry_count_chirho;

        while low_chirho < high_chirho {
            let mid_chirho = (low_chirho + high_chirho) / 2;
            let (entry_key_chirho, entry_value_chirho) = self.read_entry_chirho(mid_chirho)?;

            match entry_key_chirho.as_str().cmp(key_chirho) {
                std::cmp::Ordering::Equal => return Ok(Some(entry_value_chirho)),
                std::cmp::Ordering::Less => low_chirho = mid_chirho + 1,
                std::cmp::Ordering::Greater => high_chirho = mid_chirho,
            }
        }

        Ok(None)
    }

    /// Get the number of entries.
    pub fn entry_count_chirho(&mut self) -> ResultChirho<u32> {
        let idx_file_chirho = self.idx_file_chirho.as_mut()
            .ok_or_else(|| ErrorChirho::generic_chirho("Index file not available"))?;

        let file_size_chirho = idx_file_chirho.seek(SeekFrom::End(0))?;
        Ok((file_size_chirho / 8) as u32)
    }

    /// Clear the block cache.
    pub fn clear_cache_chirho(&mut self) {
        self.block_cache_chirho.clear();
    }

    /// Get the path to the module.
    pub fn path_chirho(&self) -> &Path {
        &self.path_chirho
    }
}
