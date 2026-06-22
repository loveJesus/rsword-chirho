// For God so loved the world, that he gave his only begotten Son,
// that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! zStr compressed storage format for lexicons and dictionaries.
//!
//! This matches the on-disk layout produced by SWORD's `zStr` backend (used by
//! `zLD`/`zLD4`). Resolving an entry is a two-level lookup: a key index points
//! into a key-data file, whose record names a compressed block and the entry's
//! slot inside that block's internal offset table.
//!
//! Files (for basename `dict`):
//! - `dict.idx` — key index, 8 bytes/entry: `u32 datOffset` + `u32 datSize`
//! - `dict.dat` — key records: `<key>` + separator + `u32 block` + `u32 blockIndex`
//!   (the locator is the final 8 bytes of each record)
//! - `dict.zdx` — block index, 8 bytes/block: `u32 zdtOffset` + `u32 compressedSize`
//! - `dict.zdt` — concatenated zlib-compressed blocks. Each decompressed block is
//!   `u32 count` + `(u32 offset, u32 size) * count` + the entry texts.

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use crate::compression_chirho::CompressorChirho;
use crate::error_chirho::{ErrorChirho, ResultChirho};

/// Size in bytes of one `.idx` key-index record (`u32 offset` + `u32 size`).
const IDX_ENTRY_SIZE_CHIRHO: usize = 8;
/// Size in bytes of one `.zdx` block-index record (`u32 offset` + `u32 size`).
const ZDX_ENTRY_SIZE_CHIRHO: usize = 8;
/// Size in bytes of the block/blockIndex locator at the tail of a `.dat` record.
const DAT_LOCATOR_SIZE_CHIRHO: usize = 8;

/// Read a little-endian `u32` from `buf` at `pos`, or `None` if out of range.
fn read_u32_le_chirho(buf_chirho: &[u8], pos_chirho: usize) -> Option<u32> {
    buf_chirho
        .get(pos_chirho..pos_chirho + 4)
        .map(|b_chirho| u32::from_le_bytes([b_chirho[0], b_chirho[1], b_chirho[2], b_chirho[3]]))
}

/// zStr compressed storage format for lexicons.
pub struct ZStrChirho {
    /// Path to the module data directory.
    path_chirho: PathBuf,
    /// Key index (`.idx`) loaded into memory (8 bytes per entry).
    idx_chirho: Vec<u8>,
    /// Key data (`.dat`) loaded into memory.
    dat_chirho: Vec<u8>,
    /// Block index (`.zdx`) loaded into memory (8 bytes per block).
    zdx_chirho: Vec<u8>,
    /// Compressed data file (`.zdt`).
    zdt_file_chirho: Option<File>,
    /// Decompressor.
    compressor_chirho: Box<dyn CompressorChirho>,
    /// Block cache (block_num -> uncompressed data).
    block_cache_chirho: HashMap<u32, Vec<u8>>,
}

impl ZStrChirho {
    /// Open an existing zStr module. `path_chirho` is the directory holding the
    /// `<basename>.idx/.dat/.zdx/.zdt` files.
    pub fn open_chirho<P: AsRef<Path>>(
        path_chirho: P,
        basename_chirho: &str,
        compressor_chirho: Box<dyn CompressorChirho>,
    ) -> ResultChirho<Self> {
        let path_chirho = path_chirho.as_ref().to_path_buf();

        let read_opt_chirho = |ext_chirho: &str| -> Option<Vec<u8>> {
            let mut f_chirho = File::open(path_chirho.join(format!("{}.{}", basename_chirho, ext_chirho))).ok()?;
            let mut buf_chirho = Vec::new();
            f_chirho.read_to_end(&mut buf_chirho).ok()?;
            Some(buf_chirho)
        };

        // The key index and key data are required; without them no entry can be resolved.
        let idx_chirho = read_opt_chirho("idx")
            .ok_or_else(|| ErrorChirho::InvalidModulePathChirho { path_chirho: path_chirho.clone() })?;
        let dat_chirho = read_opt_chirho("dat")
            .ok_or_else(|| ErrorChirho::InvalidModulePathChirho { path_chirho: path_chirho.clone() })?;
        let zdx_chirho = read_opt_chirho("zdx").unwrap_or_default();
        let zdt_file_chirho = File::open(path_chirho.join(format!("{}.zdt", basename_chirho))).ok();

        Ok(Self {
            path_chirho,
            idx_chirho,
            dat_chirho,
            zdx_chirho,
            zdt_file_chirho,
            compressor_chirho,
            block_cache_chirho: HashMap::new(),
        })
    }

    /// Number of key-index entries.
    pub fn entry_count_chirho(&mut self) -> ResultChirho<u32> {
        Ok((self.idx_chirho.len() / IDX_ENTRY_SIZE_CHIRHO) as u32)
    }

    /// Read the `(datOffset, datSize)` locator for key-index entry `idx_off_chirho`.
    fn read_idx_chirho(&self, idx_off_chirho: u32) -> ResultChirho<(u32, u32)> {
        let base_chirho = idx_off_chirho as usize * IDX_ENTRY_SIZE_CHIRHO;
        let off_chirho = read_u32_le_chirho(&self.idx_chirho, base_chirho)
            .ok_or_else(|| ErrorChirho::generic_chirho("zStr: index entry out of range"))?;
        let size_chirho = read_u32_le_chirho(&self.idx_chirho, base_chirho + 4)
            .ok_or_else(|| ErrorChirho::generic_chirho("zStr: index entry out of range"))?;
        Ok((off_chirho, size_chirho))
    }

    /// Parse the `.dat` record for entry `idx_off_chirho` into
    /// `(key, block_number, index_in_block)`.
    fn read_dat_record_chirho(&self, idx_off_chirho: u32) -> ResultChirho<(String, u32, u32)> {
        let (dat_off_chirho, dat_size_chirho) = self.read_idx_chirho(idx_off_chirho)?;
        let start_chirho = dat_off_chirho as usize;
        let end_chirho = start_chirho + dat_size_chirho as usize;
        let record_chirho = self.dat_chirho.get(start_chirho..end_chirho)
            .ok_or_else(|| ErrorChirho::generic_chirho("zStr: .dat record out of range"))?;

        if record_chirho.len() < DAT_LOCATOR_SIZE_CHIRHO {
            return Ok((String::new(), 0, 0));
        }

        let split_chirho = record_chirho.len() - DAT_LOCATOR_SIZE_CHIRHO;
        let key_bytes_chirho = &record_chirho[..split_chirho];
        let block_chirho = read_u32_le_chirho(record_chirho, split_chirho).unwrap_or(0);
        let index_in_block_chirho = read_u32_le_chirho(record_chirho, split_chirho + 4).unwrap_or(0);

        // The key is followed by a separator (CR/LF/NUL) before the locator; trim it.
        let key_chirho = String::from_utf8_lossy(key_bytes_chirho)
            .trim_end_matches(['\r', '\n', '\0', ' '])
            .to_string();

        Ok((key_chirho, block_chirho, index_in_block_chirho))
    }

    /// Read the `(zdtOffset, compressedSize)` locator for a compressed block.
    fn read_block_index_chirho(&self, block_num_chirho: u32) -> ResultChirho<(u32, u32)> {
        let base_chirho = block_num_chirho as usize * ZDX_ENTRY_SIZE_CHIRHO;
        let off_chirho = read_u32_le_chirho(&self.zdx_chirho, base_chirho)
            .ok_or_else(|| ErrorChirho::generic_chirho("zStr: block index out of range"))?;
        let size_chirho = read_u32_le_chirho(&self.zdx_chirho, base_chirho + 4)
            .ok_or_else(|| ErrorChirho::generic_chirho("zStr: block index out of range"))?;
        Ok((off_chirho, size_chirho))
    }

    /// Read and decompress a block (cached).
    fn read_block_chirho(&mut self, block_num_chirho: u32) -> ResultChirho<Vec<u8>> {
        if let Some(data_chirho) = self.block_cache_chirho.get(&block_num_chirho) {
            return Ok(data_chirho.clone());
        }

        let (comp_offset_chirho, comp_size_chirho) = self.read_block_index_chirho(block_num_chirho)?;
        if comp_size_chirho == 0 {
            return Ok(Vec::new());
        }

        let zdt_file_chirho = self.zdt_file_chirho.as_mut()
            .ok_or_else(|| ErrorChirho::generic_chirho("zStr: data file not available"))?;
        zdt_file_chirho.seek(SeekFrom::Start(comp_offset_chirho as u64))?;
        let mut compressed_chirho = vec![0u8; comp_size_chirho as usize];
        zdt_file_chirho.read_exact(&mut compressed_chirho)?;

        let decompressed_chirho = self.compressor_chirho.decompress_chirho(&compressed_chirho)?;
        self.block_cache_chirho.insert(block_num_chirho, decompressed_chirho.clone());
        Ok(decompressed_chirho)
    }

    /// Extract entry `index_in_block_chirho` from a decompressed block using the
    /// block's leading `(offset, size)` table.
    fn entry_from_block_chirho(block_chirho: &[u8], index_in_block_chirho: u32) -> String {
        let count_chirho = match read_u32_le_chirho(block_chirho, 0) {
            Some(c_chirho) => c_chirho,
            None => return String::new(),
        };
        if index_in_block_chirho >= count_chirho {
            return String::new();
        }
        let table_pos_chirho = 4 + index_in_block_chirho as usize * 8;
        let (off_chirho, size_chirho) = match (
            read_u32_le_chirho(block_chirho, table_pos_chirho),
            read_u32_le_chirho(block_chirho, table_pos_chirho + 4),
        ) {
            (Some(o_chirho), Some(s_chirho)) => (o_chirho as usize, s_chirho as usize),
            _ => return String::new(),
        };
        let slice_chirho = block_chirho
            .get(off_chirho..off_chirho.saturating_add(size_chirho))
            .unwrap_or(&[]);
        String::from_utf8_lossy(slice_chirho)
            .trim_end_matches('\0')
            .to_string()
    }

    /// Read an entry by key-index position, returning `(key, value)`.
    pub fn read_entry_chirho(&mut self, idx_off_chirho: u32) -> ResultChirho<(String, String)> {
        let (key_chirho, block_num_chirho, index_in_block_chirho) =
            self.read_dat_record_chirho(idx_off_chirho)?;
        if key_chirho.is_empty() {
            return Ok((String::new(), String::new()));
        }
        let block_data_chirho = self.read_block_chirho(block_num_chirho)?;
        let value_chirho = Self::entry_from_block_chirho(&block_data_chirho, index_in_block_chirho);
        Ok((key_chirho, value_chirho))
    }

    /// Find an entry by key using binary search over the (sorted) key index.
    pub fn find_by_key_chirho(&mut self, key_chirho: &str) -> ResultChirho<Option<String>> {
        let entry_count_chirho = self.entry_count_chirho()?;
        if entry_count_chirho == 0 {
            return Ok(None);
        }

        let mut low_chirho = 0u32;
        let mut high_chirho = entry_count_chirho;

        while low_chirho < high_chirho {
            let mid_chirho = low_chirho + (high_chirho - low_chirho) / 2;
            let (entry_key_chirho, entry_value_chirho) = self.read_entry_chirho(mid_chirho)?;

            match entry_key_chirho.as_str().cmp(key_chirho) {
                std::cmp::Ordering::Equal => return Ok(Some(entry_value_chirho)),
                std::cmp::Ordering::Less => low_chirho = mid_chirho + 1,
                std::cmp::Ordering::Greater => high_chirho = mid_chirho,
            }
        }

        Ok(None)
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
