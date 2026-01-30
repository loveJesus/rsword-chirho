// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! ZGenBook (compressed) module driver.
//!
//! Provides access to compressed SWORD general book modules.
//! File format:
//! - .bdt - Tree index file (tree node structure)
//! - .bdx - Block index (offset/size of compressed blocks)
//! - .bdz - Compressed data blocks

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use crate::compression_chirho::create_compressor_chirho;
use crate::error_chirho::{ErrorChirho, ResultChirho};
use crate::keys_chirho::TreeKeyChirho;
use crate::CompressionTypeChirho;

use super::raw_genbook_chirho::GenBookEntryChirho;

/// Size of a tree node entry in the index file.
const TREE_NODE_SIZE_CHIRHO: usize = 12;

/// Size of a block index entry.
const BLOCK_INDEX_SIZE_CHIRHO: usize = 12;

/// A tree node in the index structure.
#[derive(Debug, Clone, Copy)]
struct TreeNodeChirho {
    /// Block number containing this entry.
    block_num_chirho: u32,
    /// Offset within the uncompressed block.
    offset_in_block_chirho: u32,
    /// Index of the parent node (-1 for root).
    parent_chirho: i32,
}

impl TreeNodeChirho {
    /// Read a tree node from bytes (big-endian).
    fn from_bytes_chirho(data_chirho: &[u8]) -> Self {
        assert!(data_chirho.len() >= TREE_NODE_SIZE_CHIRHO);
        Self {
            block_num_chirho: u32::from_be_bytes([
                data_chirho[0],
                data_chirho[1],
                data_chirho[2],
                data_chirho[3],
            ]),
            offset_in_block_chirho: u32::from_be_bytes([
                data_chirho[4],
                data_chirho[5],
                data_chirho[6],
                data_chirho[7],
            ]),
            parent_chirho: i32::from_be_bytes([
                data_chirho[8],
                data_chirho[9],
                data_chirho[10],
                data_chirho[11],
            ]),
        }
    }

    /// Check if this is an empty/placeholder node.
    fn is_empty_chirho(&self) -> bool {
        self.block_num_chirho == 0xFFFFFFFF
    }
}

/// A block index entry.
#[derive(Debug, Clone, Copy)]
struct BlockIndexChirho {
    /// Offset to the compressed block in .bdz file.
    comp_offset_chirho: u32,
    /// Size of the compressed block.
    comp_size_chirho: u32,
    /// Size of the uncompressed block (kept for SWORD binary format compatibility).
    #[allow(dead_code)]
    uncomp_size_chirho: u32,
}

impl BlockIndexChirho {
    /// Read a block index from bytes (big-endian).
    fn from_bytes_chirho(data_chirho: &[u8]) -> Self {
        assert!(data_chirho.len() >= BLOCK_INDEX_SIZE_CHIRHO);
        Self {
            comp_offset_chirho: u32::from_be_bytes([
                data_chirho[0],
                data_chirho[1],
                data_chirho[2],
                data_chirho[3],
            ]),
            comp_size_chirho: u32::from_be_bytes([
                data_chirho[4],
                data_chirho[5],
                data_chirho[6],
                data_chirho[7],
            ]),
            uncomp_size_chirho: u32::from_be_bytes([
                data_chirho[8],
                data_chirho[9],
                data_chirho[10],
                data_chirho[11],
            ]),
        }
    }
}

/// Cached decompressed block.
struct CachedBlockChirho {
    /// Block number.
    block_num_chirho: u32,
    /// Decompressed data.
    data_chirho: Vec<u8>,
}

/// ZGenBook (compressed general book) module driver.
pub struct ZGenBookChirho {
    /// Path to the module data directory.
    path_chirho: PathBuf,
    /// Base filename for the module files.
    basename_chirho: String,
    /// Tree nodes from the index file.
    nodes_chirho: Vec<TreeNodeChirho>,
    /// Key names for each node.
    key_names_chirho: Vec<String>,
    /// Map from key path to node index.
    key_index_chirho: HashMap<String, usize>,
    /// Children for each node.
    children_chirho: Vec<Vec<usize>>,
    /// Block index entries.
    block_index_chirho: Vec<BlockIndexChirho>,
    /// Compression type.
    compression_type_chirho: CompressionTypeChirho,
    /// Block cache (single block for now).
    cached_block_chirho: Option<CachedBlockChirho>,
}

impl ZGenBookChirho {
    /// Open a ZGenBook module.
    pub fn open_chirho<P: AsRef<Path>>(
        path_chirho: P,
        basename_chirho: &str,
        compression_type_chirho: CompressionTypeChirho,
    ) -> ResultChirho<Self> {
        let path_chirho = path_chirho.as_ref().to_path_buf();
        let mut module_chirho = Self {
            path_chirho,
            basename_chirho: basename_chirho.to_string(),
            nodes_chirho: Vec::new(),
            key_names_chirho: Vec::new(),
            key_index_chirho: HashMap::new(),
            children_chirho: Vec::new(),
            block_index_chirho: Vec::new(),
            compression_type_chirho,
            cached_block_chirho: None,
        };
        module_chirho.load_indices_chirho()?;
        Ok(module_chirho)
    }

    /// Open with default ZIP compression.
    pub fn open_zip_chirho<P: AsRef<Path>>(path_chirho: P, basename_chirho: &str) -> ResultChirho<Self> {
        Self::open_chirho(path_chirho, basename_chirho, CompressionTypeChirho::ZipChirho)
    }

    /// Load the tree and block indices.
    fn load_indices_chirho(&mut self) -> ResultChirho<()> {
        // Load tree index (.bdt)
        self.load_tree_index_chirho()?;

        // Load block index (.bdx)
        self.load_block_index_chirho()?;

        Ok(())
    }

    /// Load the tree index from the .bdt file.
    fn load_tree_index_chirho(&mut self) -> ResultChirho<()> {
        let index_path_chirho = self.path_chirho.join(format!("{}.bdt", self.basename_chirho));

        let file_chirho = File::open(&index_path_chirho).map_err(ErrorChirho::IoChirho)?;
        let metadata_chirho = file_chirho.metadata().map_err(ErrorChirho::IoChirho)?;
        let file_size_chirho = metadata_chirho.len() as usize;

        let mut reader_chirho = BufReader::new(file_chirho);
        let mut buffer_chirho = vec![0u8; file_size_chirho];
        reader_chirho.read_exact(&mut buffer_chirho).map_err(ErrorChirho::IoChirho)?;

        // Parse variable-length records (same format as RawGenBook)
        let mut pos_chirho = 0;
        while pos_chirho + TREE_NODE_SIZE_CHIRHO <= file_size_chirho {
            let node_chirho = TreeNodeChirho::from_bytes_chirho(&buffer_chirho[pos_chirho..]);
            pos_chirho += TREE_NODE_SIZE_CHIRHO;

            // Read the null-terminated name
            let name_start_chirho = pos_chirho;
            while pos_chirho < file_size_chirho && buffer_chirho[pos_chirho] != 0 {
                pos_chirho += 1;
            }

            let name_chirho = if pos_chirho > name_start_chirho {
                String::from_utf8_lossy(&buffer_chirho[name_start_chirho..pos_chirho]).to_string()
            } else {
                String::new()
            };

            // Skip the null terminator
            if pos_chirho < file_size_chirho && buffer_chirho[pos_chirho] == 0 {
                pos_chirho += 1;
            }

            self.nodes_chirho.push(node_chirho);
            self.key_names_chirho.push(name_chirho);
            self.children_chirho.push(Vec::new());
        }

        // Build the children lists
        for (idx_chirho, node_chirho) in self.nodes_chirho.iter().enumerate() {
            if node_chirho.parent_chirho >= 0 {
                let parent_idx_chirho = node_chirho.parent_chirho as usize;
                if parent_idx_chirho < self.children_chirho.len() {
                    self.children_chirho[parent_idx_chirho].push(idx_chirho);
                }
            }
        }

        // Build the key path index
        self.build_key_index_chirho();

        Ok(())
    }

    /// Load the block index from the .bdx file.
    fn load_block_index_chirho(&mut self) -> ResultChirho<()> {
        let index_path_chirho = self.path_chirho.join(format!("{}.bdx", self.basename_chirho));

        let file_chirho = File::open(&index_path_chirho).map_err(ErrorChirho::IoChirho)?;
        let metadata_chirho = file_chirho.metadata().map_err(ErrorChirho::IoChirho)?;
        let file_size_chirho = metadata_chirho.len() as usize;

        let num_blocks_chirho = file_size_chirho / BLOCK_INDEX_SIZE_CHIRHO;

        let mut reader_chirho = BufReader::new(file_chirho);
        let mut buffer_chirho = vec![0u8; BLOCK_INDEX_SIZE_CHIRHO];

        for _ in 0..num_blocks_chirho {
            reader_chirho.read_exact(&mut buffer_chirho).map_err(ErrorChirho::IoChirho)?;
            let block_chirho = BlockIndexChirho::from_bytes_chirho(&buffer_chirho);
            self.block_index_chirho.push(block_chirho);
        }

        Ok(())
    }

    /// Build the key path index by traversing the tree.
    fn build_key_index_chirho(&mut self) {
        for idx_chirho in 0..self.nodes_chirho.len() {
            let path_chirho = self.build_path_chirho(idx_chirho);
            self.key_index_chirho.insert(path_chirho, idx_chirho);
        }
    }

    /// Build the full path for a node by walking up to root.
    fn build_path_chirho(&self, node_idx_chirho: usize) -> String {
        let mut components_chirho: Vec<&str> = Vec::new();
        let mut idx_chirho = node_idx_chirho;

        while idx_chirho < self.nodes_chirho.len() {
            components_chirho.push(&self.key_names_chirho[idx_chirho]);
            let parent_chirho = self.nodes_chirho[idx_chirho].parent_chirho;
            if parent_chirho < 0 {
                break;
            }
            idx_chirho = parent_chirho as usize;
        }

        components_chirho.reverse();
        format!("/{}", components_chirho.join("/"))
    }

    /// Read and decompress a block.
    fn read_block_chirho(&mut self, block_num_chirho: u32) -> ResultChirho<Vec<u8>> {
        // Check cache first
        if let Some(ref cached_chirho) = self.cached_block_chirho {
            if cached_chirho.block_num_chirho == block_num_chirho {
                return Ok(cached_chirho.data_chirho.clone());
            }
        }

        let block_idx_chirho = block_num_chirho as usize;
        if block_idx_chirho >= self.block_index_chirho.len() {
            return Err(ErrorChirho::generic_chirho(format!(
                "Block {} out of range (max: {})",
                block_num_chirho,
                self.block_index_chirho.len()
            )));
        }

        let block_info_chirho = &self.block_index_chirho[block_idx_chirho];

        // Read compressed data
        let data_path_chirho = self.path_chirho.join(format!("{}.bdz", self.basename_chirho));
        let mut file_chirho = File::open(&data_path_chirho).map_err(ErrorChirho::IoChirho)?;

        file_chirho.seek(SeekFrom::Start(block_info_chirho.comp_offset_chirho as u64))
            .map_err(ErrorChirho::IoChirho)?;

        let mut compressed_chirho = vec![0u8; block_info_chirho.comp_size_chirho as usize];
        file_chirho.read_exact(&mut compressed_chirho).map_err(ErrorChirho::IoChirho)?;

        // Decompress
        let compressor_chirho = create_compressor_chirho(self.compression_type_chirho);
        let decompressed_chirho = compressor_chirho.decompress_chirho(&compressed_chirho)?;

        // Cache the result
        self.cached_block_chirho = Some(CachedBlockChirho {
            block_num_chirho,
            data_chirho: decompressed_chirho.clone(),
        });

        Ok(decompressed_chirho)
    }

    /// Get the number of entries.
    pub fn entry_count_chirho(&self) -> usize {
        self.nodes_chirho.iter().filter(|n_chirho| !n_chirho.is_empty_chirho()).count()
    }

    /// Read an entry by tree key path.
    pub fn read_entry_chirho(&mut self, key_chirho: &str) -> ResultChirho<Option<String>> {
        let normalized_chirho = self.normalize_key_chirho(key_chirho);

        if let Some(&idx_chirho) = self.key_index_chirho.get(&normalized_chirho) {
            // Copy the node values before calling read_block_chirho to avoid borrow conflict
            let (block_num_chirho, offset_in_block_chirho, is_empty_chirho) = {
                let node_chirho = &self.nodes_chirho[idx_chirho];
                (node_chirho.block_num_chirho, node_chirho.offset_in_block_chirho, node_chirho.is_empty_chirho())
            };

            if is_empty_chirho {
                return Ok(None);
            }

            // Read and decompress the block
            let block_data_chirho = self.read_block_chirho(block_num_chirho)?;

            // Extract the entry from the block
            // Format: entries are stored as length-prefixed strings
            let offset_chirho = offset_in_block_chirho as usize;
            if offset_chirho >= block_data_chirho.len() {
                return Ok(None);
            }

            // Read length (4 bytes big-endian)
            if offset_chirho + 4 > block_data_chirho.len() {
                return Ok(None);
            }

            let entry_size_chirho = u32::from_be_bytes([
                block_data_chirho[offset_chirho],
                block_data_chirho[offset_chirho + 1],
                block_data_chirho[offset_chirho + 2],
                block_data_chirho[offset_chirho + 3],
            ]) as usize;

            let data_start_chirho = offset_chirho + 4;
            let data_end_chirho = data_start_chirho + entry_size_chirho;

            if data_end_chirho > block_data_chirho.len() {
                return Ok(None);
            }

            let content_chirho = String::from_utf8_lossy(
                &block_data_chirho[data_start_chirho..data_end_chirho]
            ).to_string();

            Ok(Some(content_chirho))
        } else {
            Ok(None)
        }
    }

    /// Read an entry using a TreeKey.
    pub fn read_by_key_chirho(&mut self, key_chirho: &TreeKeyChirho) -> ResultChirho<Option<String>> {
        self.read_entry_chirho(&key_chirho.get_path_chirho())
    }

    /// Normalize a key path (ensure leading /).
    fn normalize_key_chirho(&self, key_chirho: &str) -> String {
        if key_chirho.starts_with('/') {
            key_chirho.to_string()
        } else {
            format!("/{}", key_chirho)
        }
    }

    /// Get all keys at a specific depth.
    pub fn get_keys_at_depth_chirho(&self, depth_chirho: usize) -> Vec<String> {
        self.key_index_chirho
            .keys()
            .filter(|k_chirho| k_chirho.split('/').filter(|s_chirho| !s_chirho.is_empty()).count() == depth_chirho)
            .cloned()
            .collect()
    }

    /// Get children of a key.
    pub fn get_children_chirho(&self, key_chirho: &str) -> Vec<String> {
        let normalized_chirho = self.normalize_key_chirho(key_chirho);

        if let Some(&idx_chirho) = self.key_index_chirho.get(&normalized_chirho) {
            self.children_chirho[idx_chirho]
                .iter()
                .map(|&child_idx_chirho| self.build_path_chirho(child_idx_chirho))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get all root-level keys.
    pub fn get_root_keys_chirho(&self) -> Vec<String> {
        self.nodes_chirho
            .iter()
            .enumerate()
            .filter(|(_, n_chirho)| n_chirho.parent_chirho < 0)
            .map(|(idx_chirho, _)| self.build_path_chirho(idx_chirho))
            .collect()
    }

    /// Check if a key exists.
    pub fn has_key_chirho(&self, key_chirho: &str) -> bool {
        let normalized_chirho = self.normalize_key_chirho(key_chirho);
        self.key_index_chirho.contains_key(&normalized_chirho)
    }

    /// Create an iterator over all entries.
    pub fn iter_chirho(&mut self) -> ZGenBookIteratorChirho<'_> {
        ZGenBookIteratorChirho {
            module_chirho: self,
            index_chirho: 0,
        }
    }

    /// Get the compression type.
    pub fn compression_type_chirho(&self) -> CompressionTypeChirho {
        self.compression_type_chirho
    }

    /// Get the number of blocks.
    pub fn block_count_chirho(&self) -> usize {
        self.block_index_chirho.len()
    }
}

/// Iterator over ZGenBook entries.
pub struct ZGenBookIteratorChirho<'a> {
    module_chirho: &'a mut ZGenBookChirho,
    index_chirho: usize,
}

impl<'a> Iterator for ZGenBookIteratorChirho<'a> {
    type Item = ResultChirho<GenBookEntryChirho>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index_chirho < self.module_chirho.nodes_chirho.len() {
            let idx_chirho = self.index_chirho;
            self.index_chirho += 1;

            // Copy the is_empty check to avoid borrow conflict
            let is_empty_chirho = self.module_chirho.nodes_chirho[idx_chirho].is_empty_chirho();
            if is_empty_chirho {
                continue;
            }

            let key_chirho = self.module_chirho.build_path_chirho(idx_chirho);

            match self.module_chirho.read_entry_chirho(&key_chirho) {
                Ok(Some(content_chirho)) => {
                    return Some(Ok(GenBookEntryChirho {
                        key_chirho,
                        content_chirho,
                    }));
                }
                Ok(None) => continue,
                Err(e_chirho) => return Some(Err(e_chirho)),
            }
        }
        None
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_tree_node_from_bytes_chirho() {
        let data_chirho: [u8; 12] = [
            0x00, 0x00, 0x00, 0x05, // block_num = 5
            0x00, 0x00, 0x01, 0x00, // offset = 256
            0xFF, 0xFF, 0xFF, 0xFF, // parent = -1 (root)
        ];

        let node_chirho = TreeNodeChirho::from_bytes_chirho(&data_chirho);
        assert_eq!(node_chirho.block_num_chirho, 5);
        assert_eq!(node_chirho.offset_in_block_chirho, 256);
        assert_eq!(node_chirho.parent_chirho, -1);
    }

    #[test]
    fn test_tree_node_empty_chirho() {
        let node_chirho = TreeNodeChirho {
            block_num_chirho: 0xFFFFFFFF,
            offset_in_block_chirho: 0,
            parent_chirho: -1,
        };
        assert!(node_chirho.is_empty_chirho());

        let node2_chirho = TreeNodeChirho {
            block_num_chirho: 1,
            offset_in_block_chirho: 100,
            parent_chirho: 0,
        };
        assert!(!node2_chirho.is_empty_chirho());
    }

    #[test]
    fn test_block_index_from_bytes_chirho() {
        let data_chirho: [u8; 12] = [
            0x00, 0x00, 0x10, 0x00, // comp_offset = 4096
            0x00, 0x00, 0x08, 0x00, // comp_size = 2048
            0x00, 0x00, 0x20, 0x00, // uncomp_size = 8192
        ];

        let block_chirho = BlockIndexChirho::from_bytes_chirho(&data_chirho);
        assert_eq!(block_chirho.comp_offset_chirho, 4096);
        assert_eq!(block_chirho.comp_size_chirho, 2048);
        assert_eq!(block_chirho.uncomp_size_chirho, 8192);
    }

    #[test]
    fn test_normalize_key_chirho() {
        // Create a minimal module for testing
        let module_chirho = ZGenBookChirho {
            path_chirho: PathBuf::from("/test"),
            basename_chirho: "test".to_string(),
            nodes_chirho: Vec::new(),
            key_names_chirho: Vec::new(),
            key_index_chirho: HashMap::new(),
            children_chirho: Vec::new(),
            block_index_chirho: Vec::new(),
            compression_type_chirho: CompressionTypeChirho::ZipChirho,
            cached_block_chirho: None,
        };

        assert_eq!(module_chirho.normalize_key_chirho("test/path"), "/test/path");
        assert_eq!(module_chirho.normalize_key_chirho("/test/path"), "/test/path");
    }
}
