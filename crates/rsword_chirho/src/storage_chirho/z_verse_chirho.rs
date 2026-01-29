// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! zVerse compressed storage format implementation.
//!
//! zVerse stores compressed blocks of verse text. Each verse has an index
//! entry pointing to a block and an offset within that block.
//!
//! Files (extension depends on compression type):
//! - zlib: `ot.czv` / `nt.czv`, `ot.czs` / `nt.czs`, `ot.czz` / `nt.czz`
//! - bzip2: `ot.bzv` / `nt.bzv`, `ot.bzs` / `nt.bzs`, `ot.bzz` / `nt.bzz`
//! - xz: `ot.xzv` / `nt.xzv`, `ot.xzs` / `nt.xzs`, `ot.xzz` / `nt.xzz`

use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use crate::byte_order_chirho::{ZBlockIndexChirho, ZVerseIndexChirho, ReadSwordChirho};
use crate::compression_chirho::CompressorChirho;
use crate::error_chirho::{ErrorChirho, ResultChirho};
use crate::{BlockTypeChirho, CompressionTypeChirho};

/// zVerse compressed storage format.
pub struct ZVerseChirho {
    /// Path to the module data directory.
    path_chirho: PathBuf,
    /// Block type (verse, chapter, or book).
    block_type_chirho: BlockTypeChirho,
    /// OT verse index.
    ot_verse_idx_chirho: Option<File>,
    /// NT verse index.
    nt_verse_idx_chirho: Option<File>,
    /// OT block index.
    ot_block_idx_chirho: Option<File>,
    /// NT block index.
    nt_block_idx_chirho: Option<File>,
    /// OT compressed data.
    ot_data_chirho: Option<File>,
    /// NT compressed data.
    nt_data_chirho: Option<File>,
    /// Decompressor.
    compressor_chirho: Box<dyn CompressorChirho>,
    /// Block cache (testament, block_num -> uncompressed data).
    block_cache_chirho: HashMap<(u8, u32), Vec<u8>>,
}

impl ZVerseChirho {
    /// Get file extension prefix for a compression type.
    fn extension_prefix_chirho(comp_type_chirho: CompressionTypeChirho) -> &'static str {
        match comp_type_chirho {
            CompressionTypeChirho::ZipChirho => "c",
            CompressionTypeChirho::Bzip2Chirho => "b",
            CompressionTypeChirho::XzChirho => "x",
            CompressionTypeChirho::LzssChirho => "c", // LZSS uses same extension as zlib
            CompressionTypeChirho::NoneChirho => "c", // Default to zlib extension
        }
    }

    /// Open an existing zVerse module.
    pub fn open_chirho<P: AsRef<Path>>(
        path_chirho: P,
        compressor_chirho: Box<dyn CompressorChirho>,
    ) -> ResultChirho<Self> {
        Self::open_with_comp_type_chirho(path_chirho, compressor_chirho, CompressionTypeChirho::ZipChirho)
    }

    /// Open an existing zVerse module with specified compression type.
    pub fn open_with_comp_type_chirho<P: AsRef<Path>>(
        path_chirho: P,
        compressor_chirho: Box<dyn CompressorChirho>,
        comp_type_chirho: CompressionTypeChirho,
    ) -> ResultChirho<Self> {
        let path_chirho = path_chirho.as_ref().to_path_buf();
        let ext_prefix_chirho = Self::extension_prefix_chirho(comp_type_chirho);

        // Try the specified extension first, then fall back to trying all extensions
        let extensions_chirho = [
            ext_prefix_chirho,
            "c", // zlib
            "b", // bzip2
            "x", // xz
        ];

        let mut ot_verse_idx_chirho = None;
        let mut nt_verse_idx_chirho = None;
        let mut ot_block_idx_chirho = None;
        let mut nt_block_idx_chirho = None;
        let mut ot_data_chirho = None;
        let mut nt_data_chirho = None;

        for ext_chirho in &extensions_chirho {
            if ot_verse_idx_chirho.is_none() {
                ot_verse_idx_chirho = File::open(path_chirho.join(format!("ot.{}zv", ext_chirho))).ok();
            }
            if nt_verse_idx_chirho.is_none() {
                nt_verse_idx_chirho = File::open(path_chirho.join(format!("nt.{}zv", ext_chirho))).ok();
            }
            if ot_block_idx_chirho.is_none() {
                ot_block_idx_chirho = File::open(path_chirho.join(format!("ot.{}zs", ext_chirho))).ok();
            }
            if nt_block_idx_chirho.is_none() {
                nt_block_idx_chirho = File::open(path_chirho.join(format!("nt.{}zs", ext_chirho))).ok();
            }
            if ot_data_chirho.is_none() {
                ot_data_chirho = File::open(path_chirho.join(format!("ot.{}zz", ext_chirho))).ok();
            }
            if nt_data_chirho.is_none() {
                nt_data_chirho = File::open(path_chirho.join(format!("nt.{}zz", ext_chirho))).ok();
            }

            // Break early if we found all files
            if (ot_verse_idx_chirho.is_some() || nt_verse_idx_chirho.is_some())
                && (ot_block_idx_chirho.is_some() || nt_block_idx_chirho.is_some())
                && (ot_data_chirho.is_some() || nt_data_chirho.is_some())
            {
                break;
            }
        }

        if ot_verse_idx_chirho.is_none() && nt_verse_idx_chirho.is_none() {
            return Err(ErrorChirho::InvalidModulePathChirho { path_chirho });
        }

        // Note: File extensions (.bzv/.czv/.xzv) don't reliably indicate compression type.
        // The actual compression is determined by CompressType in the module config.
        // We just use the provided compressor based on the config.

        Ok(Self {
            path_chirho,
            block_type_chirho: BlockTypeChirho::ChapterBlocksChirho,
            ot_verse_idx_chirho,
            nt_verse_idx_chirho,
            ot_block_idx_chirho,
            nt_block_idx_chirho,
            ot_data_chirho,
            nt_data_chirho,
            compressor_chirho,
            block_cache_chirho: HashMap::new(),
        })
    }

    /// Set the block type.
    pub fn set_block_type_chirho(&mut self, block_type_chirho: BlockTypeChirho) {
        self.block_type_chirho = block_type_chirho;
    }

    /// Find the verse index entry.
    pub fn find_verse_index_chirho(
        &mut self,
        testament_chirho: u8,
        idx_off_chirho: u32,
    ) -> ResultChirho<ZVerseIndexChirho> {
        let byte_offset_chirho = idx_off_chirho as u64 * ZVerseIndexChirho::SIZE_CHIRHO as u64;

        let idx_file_chirho = match testament_chirho {
            1 => self.ot_verse_idx_chirho.as_mut(),
            2 => self.nt_verse_idx_chirho.as_mut(),
            _ => return Err(ErrorChirho::generic_chirho("Invalid testament")),
        };

        let idx_file_chirho = idx_file_chirho
            .ok_or_else(|| ErrorChirho::generic_chirho("Verse index not available"))?;

        idx_file_chirho.seek(SeekFrom::Start(byte_offset_chirho))?;

        let block_num_chirho = idx_file_chirho.read_u32_sword_chirho()?;
        let offset_in_block_chirho = idx_file_chirho.read_u32_sword_chirho()?;
        let size_chirho = idx_file_chirho.read_u16_sword_chirho()?;

        Ok(ZVerseIndexChirho {
            block_num_chirho,
            offset_in_block_chirho,
            size_chirho,
        })
    }

    /// Find the block index entry.
    pub fn find_block_index_chirho(
        &mut self,
        testament_chirho: u8,
        block_num_chirho: u32,
    ) -> ResultChirho<ZBlockIndexChirho> {
        let byte_offset_chirho = block_num_chirho as u64 * ZBlockIndexChirho::SIZE_CHIRHO as u64;

        let idx_file_chirho = match testament_chirho {
            1 => self.ot_block_idx_chirho.as_mut(),
            2 => self.nt_block_idx_chirho.as_mut(),
            _ => return Err(ErrorChirho::generic_chirho("Invalid testament")),
        };

        let idx_file_chirho = idx_file_chirho
            .ok_or_else(|| ErrorChirho::generic_chirho("Block index not available"))?;

        idx_file_chirho.seek(SeekFrom::Start(byte_offset_chirho))?;

        let comp_offset_chirho = idx_file_chirho.read_u32_sword_chirho()?;
        let comp_size_chirho = idx_file_chirho.read_u32_sword_chirho()?;
        let uncomp_size_chirho = idx_file_chirho.read_u32_sword_chirho()?;

        Ok(ZBlockIndexChirho {
            comp_offset_chirho,
            comp_size_chirho,
            uncomp_size_chirho,
        })
    }

    /// Read and decompress a block.
    pub fn read_block_chirho(
        &mut self,
        testament_chirho: u8,
        block_num_chirho: u32,
    ) -> ResultChirho<Vec<u8>> {
        // Check cache first
        if let Some(data_chirho) = self.block_cache_chirho.get(&(testament_chirho, block_num_chirho)) {
            return Ok(data_chirho.clone());
        }

        let block_idx_chirho = self.find_block_index_chirho(testament_chirho, block_num_chirho)?;

        if block_idx_chirho.is_empty_chirho() {
            return Ok(Vec::new());
        }

        let data_file_chirho = match testament_chirho {
            1 => self.ot_data_chirho.as_mut(),
            2 => self.nt_data_chirho.as_mut(),
            _ => return Err(ErrorChirho::generic_chirho("Invalid testament")),
        };

        let data_file_chirho = data_file_chirho
            .ok_or_else(|| ErrorChirho::generic_chirho("Data file not available"))?;

        data_file_chirho.seek(SeekFrom::Start(block_idx_chirho.comp_offset_chirho as u64))?;

        let mut compressed_chirho = vec![0u8; block_idx_chirho.comp_size_chirho as usize];
        data_file_chirho.read_exact(&mut compressed_chirho)?;

        let decompressed_chirho = self.compressor_chirho.decompress_chirho(&compressed_chirho)?;

        // Cache the block
        self.block_cache_chirho.insert(
            (testament_chirho, block_num_chirho),
            decompressed_chirho.clone(),
        );

        Ok(decompressed_chirho)
    }

    /// Read verse text.
    pub fn read_verse_chirho(
        &mut self,
        testament_chirho: u8,
        idx_off_chirho: u32,
    ) -> ResultChirho<String> {
        let verse_idx_chirho = self.find_verse_index_chirho(testament_chirho, idx_off_chirho)?;

        if verse_idx_chirho.is_empty_chirho() {
            return Ok(String::new());
        }

        let block_data_chirho = self.read_block_chirho(testament_chirho, verse_idx_chirho.block_num_chirho)?;

        let start_chirho = verse_idx_chirho.offset_in_block_chirho as usize;
        let end_chirho = start_chirho + verse_idx_chirho.size_chirho as usize;

        if end_chirho > block_data_chirho.len() {
            return Err(ErrorChirho::generic_chirho("Verse extends beyond block"));
        }

        let text_bytes_chirho = &block_data_chirho[start_chirho..end_chirho];

        match String::from_utf8(text_bytes_chirho.to_vec()) {
            Ok(s) => Ok(s),
            Err(_) => Ok(text_bytes_chirho.iter().map(|&b| b as char).collect()),
        }
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
