// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! RawGenBook module driver.
//!
//! Provides access to SWORD general book modules using tree-structured keys.
//!
//! ## File format
//!
//! SWORD RawGenBook modules use three files:
//! - `.idx` - Offset index (4-byte little-endian offsets into .dat for each entry)
//! - `.dat` - Tree structure data (TreeKeyIdxBuf + name + userData for each entry)
//! - `.bdt` - Content data (actual entry content)
//!
//! ### Tree node format in .dat
//!
//! Each entry consists of:
//! 1. TreeKeyIdxBuf (12 bytes, little-endian):
//!    - parent: i32 (stored as 4× actual index, -1 for root)
//!    - next_sibling: i32 (stored as 4× actual index, -1 if none)
//!    - first_child: i32 (stored as 4× actual index, -1 if none)
//! 2. Key name (null-terminated string, may have 0xFF padding for root entry)
//! 3. userData (10 bytes):
//!    - version: u16 (usually 0x0008)
//!    - content_offset: u32 (little-endian, offset into .bdt)
//!    - content_size: u32 (little-endian, size of content in .bdt)
//! 4. Next entry data follows immediately

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};

use crate::error_chirho::{ErrorChirho, ResultChirho};
use crate::keys_chirho::TreeKeyChirho;

/// Size of TreeKeyIdxBuf in bytes.
const TREE_KEY_IDX_BUF_SIZE_CHIRHO: usize = 12;

/// A tree node in the index structure.
#[derive(Debug, Clone)]
struct TreeNodeChirho {
    /// Index of the parent node (-1 for root). Stored as 4× actual index.
    parent_chirho: i32,
    /// Index of the next sibling (-1 if none). Stored as 4× actual index.
    next_sibling_chirho: i32,
    /// Index of the first child (-1 if none). Stored as 4× actual index.
    first_child_chirho: i32,
    /// Key name for this node.
    name_chirho: String,
    /// Offset to content in .bdt file.
    content_offset_chirho: u32,
    /// Size of content in .bdt file.
    content_size_chirho: u32,
}

impl TreeNodeChirho {
    /// Get the actual parent index (divide by 4).
    fn parent_idx_chirho(&self) -> Option<usize> {
        if self.parent_chirho < 0 {
            None
        } else {
            Some((self.parent_chirho / 4) as usize)
        }
    }

    /// Get the actual next sibling index (divide by 4).
    fn next_sibling_idx_chirho(&self) -> Option<usize> {
        if self.next_sibling_chirho < 0 {
            None
        } else {
            Some((self.next_sibling_chirho / 4) as usize)
        }
    }

    /// Get the actual first child index (divide by 4).
    fn first_child_idx_chirho(&self) -> Option<usize> {
        if self.first_child_chirho < 0 {
            None
        } else {
            Some((self.first_child_chirho / 4) as usize)
        }
    }

    /// Check if this node has content.
    fn has_content_chirho(&self) -> bool {
        self.content_size_chirho > 0
    }
}

/// A general book entry with its key and content.
#[derive(Debug, Clone)]
pub struct GenBookEntryChirho {
    /// The tree key path.
    pub key_chirho: String,
    /// The entry content.
    pub content_chirho: String,
}

/// RawGenBook module driver.
pub struct RawGenBookChirho {
    /// Path to the module data directory.
    path_chirho: PathBuf,
    /// Base filename for the module files.
    basename_chirho: String,
    /// Tree nodes from the index file.
    nodes_chirho: Vec<TreeNodeChirho>,
    /// Map from key path to node index.
    key_index_chirho: HashMap<String, usize>,
    /// Children for each node (built from tree structure).
    children_chirho: Vec<Vec<usize>>,
}

impl RawGenBookChirho {
    /// Create a new empty RawGenBook module.
    pub fn create_chirho<P: AsRef<Path>>(path_chirho: P, basename_chirho: &str) -> ResultChirho<Self> {
        let path_chirho = path_chirho.as_ref().to_path_buf();

        // Create empty index and data files
        let idx_path_chirho = path_chirho.join(format!("{}.idx", basename_chirho));
        let dat_path_chirho = path_chirho.join(format!("{}.dat", basename_chirho));
        let bdt_path_chirho = path_chirho.join(format!("{}.bdt", basename_chirho));

        // Ensure directory exists
        std::fs::create_dir_all(&path_chirho).map_err(ErrorChirho::IoChirho)?;

        // Create empty files
        File::create(&idx_path_chirho).map_err(ErrorChirho::IoChirho)?;
        File::create(&dat_path_chirho).map_err(ErrorChirho::IoChirho)?;
        File::create(&bdt_path_chirho).map_err(ErrorChirho::IoChirho)?;

        Ok(Self {
            path_chirho,
            basename_chirho: basename_chirho.to_string(),
            nodes_chirho: Vec::new(),
            key_index_chirho: HashMap::new(),
            children_chirho: Vec::new(),
        })
    }

    /// Open a RawGenBook module.
    pub fn open_chirho<P: AsRef<Path>>(path_chirho: P, basename_chirho: &str) -> ResultChirho<Self> {
        let path_chirho = path_chirho.as_ref().to_path_buf();
        let mut module_chirho = Self {
            path_chirho,
            basename_chirho: basename_chirho.to_string(),
            nodes_chirho: Vec::new(),
            key_index_chirho: HashMap::new(),
            children_chirho: Vec::new(),
        };
        module_chirho.load_index_chirho()?;
        Ok(module_chirho)
    }

    /// Load the tree index from .idx and .dat files.
    fn load_index_chirho(&mut self) -> ResultChirho<()> {
        let idx_path_chirho = self.path_chirho.join(format!("{}.idx", self.basename_chirho));
        let dat_path_chirho = self.path_chirho.join(format!("{}.dat", self.basename_chirho));

        // Read idx file (4-byte offsets into dat)
        let idx_file_chirho = File::open(&idx_path_chirho).map_err(ErrorChirho::IoChirho)?;
        let idx_metadata_chirho = idx_file_chirho.metadata().map_err(ErrorChirho::IoChirho)?;
        let idx_size_chirho = idx_metadata_chirho.len() as usize;
        let num_entries_chirho = idx_size_chirho / 4;

        let mut idx_reader_chirho = BufReader::new(idx_file_chirho);
        let mut idx_data_chirho = vec![0u8; idx_size_chirho];
        idx_reader_chirho.read_exact(&mut idx_data_chirho).map_err(ErrorChirho::IoChirho)?;

        // Parse offsets
        let offsets_chirho: Vec<u32> = (0..num_entries_chirho)
            .map(|i_chirho| {
                u32::from_le_bytes([
                    idx_data_chirho[i_chirho * 4],
                    idx_data_chirho[i_chirho * 4 + 1],
                    idx_data_chirho[i_chirho * 4 + 2],
                    idx_data_chirho[i_chirho * 4 + 3],
                ])
            })
            .collect();

        // Read dat file (tree structure + names + userData)
        let dat_file_chirho = File::open(&dat_path_chirho).map_err(ErrorChirho::IoChirho)?;
        let dat_metadata_chirho = dat_file_chirho.metadata().map_err(ErrorChirho::IoChirho)?;
        let dat_size_chirho = dat_metadata_chirho.len() as usize;

        let mut dat_reader_chirho = BufReader::new(dat_file_chirho);
        let mut dat_data_chirho = vec![0u8; dat_size_chirho];
        dat_reader_chirho.read_exact(&mut dat_data_chirho).map_err(ErrorChirho::IoChirho)?;

        // Parse each entry
        for entry_idx_chirho in 0..num_entries_chirho {
            let start_chirho = offsets_chirho[entry_idx_chirho] as usize;
            let end_chirho = if entry_idx_chirho + 1 < num_entries_chirho {
                offsets_chirho[entry_idx_chirho + 1] as usize
            } else {
                dat_size_chirho
            };

            if start_chirho + TREE_KEY_IDX_BUF_SIZE_CHIRHO > dat_size_chirho {
                break;
            }

            let record_chirho = &dat_data_chirho[start_chirho..end_chirho];
            if let Some(node_chirho) = self.parse_node_chirho(record_chirho) {
                self.nodes_chirho.push(node_chirho);
            }
        }

        // Build children lists from tree structure
        self.children_chirho = vec![Vec::new(); self.nodes_chirho.len()];
        for (idx_chirho, node_chirho) in self.nodes_chirho.iter().enumerate() {
            if let Some(parent_idx_chirho) = node_chirho.parent_idx_chirho() {
                if parent_idx_chirho < self.children_chirho.len() {
                    self.children_chirho[parent_idx_chirho].push(idx_chirho);
                }
            }
        }

        // Build key path index
        self.build_key_index_chirho();

        Ok(())
    }

    /// Parse a single node from record data.
    fn parse_node_chirho(&self, record_chirho: &[u8]) -> Option<TreeNodeChirho> {
        if record_chirho.len() < TREE_KEY_IDX_BUF_SIZE_CHIRHO {
            return None;
        }

        // Parse TreeKeyIdxBuf (12 bytes, little-endian)
        let parent_chirho = i32::from_le_bytes([
            record_chirho[0],
            record_chirho[1],
            record_chirho[2],
            record_chirho[3],
        ]);
        let next_sibling_chirho = i32::from_le_bytes([
            record_chirho[4],
            record_chirho[5],
            record_chirho[6],
            record_chirho[7],
        ]);
        let first_child_chirho = i32::from_le_bytes([
            record_chirho[8],
            record_chirho[9],
            record_chirho[10],
            record_chirho[11],
        ]);

        // Find name start (skip any 0xFF or 0x00 padding, common in root entry)
        let mut name_start_chirho = TREE_KEY_IDX_BUF_SIZE_CHIRHO;
        while name_start_chirho < record_chirho.len()
            && (record_chirho[name_start_chirho] == 0xFF
                || record_chirho[name_start_chirho] == 0x00)
        {
            name_start_chirho += 1;
        }

        // Find name end (null terminator)
        let name_end_chirho = record_chirho[name_start_chirho..]
            .iter()
            .position(|&b_chirho| b_chirho == 0)
            .map(|pos_chirho| name_start_chirho + pos_chirho)
            .unwrap_or(record_chirho.len());

        let name_chirho = String::from_utf8_lossy(
            &record_chirho[name_start_chirho..name_end_chirho]
        ).to_string();

        // Parse userData after name+null
        // Format: 2 bytes version + 4 bytes offset + 4 bytes size
        let mut content_offset_chirho = 0u32;
        let mut content_size_chirho = 0u32;

        let user_data_start_chirho = name_end_chirho + 1;
        if user_data_start_chirho + 10 <= record_chirho.len() {
            // Skip 2-byte version field
            content_offset_chirho = u32::from_le_bytes([
                record_chirho[user_data_start_chirho + 2],
                record_chirho[user_data_start_chirho + 3],
                record_chirho[user_data_start_chirho + 4],
                record_chirho[user_data_start_chirho + 5],
            ]);
            content_size_chirho = u32::from_le_bytes([
                record_chirho[user_data_start_chirho + 6],
                record_chirho[user_data_start_chirho + 7],
                record_chirho[user_data_start_chirho + 8],
                record_chirho[user_data_start_chirho + 9],
            ]);
        }

        Some(TreeNodeChirho {
            parent_chirho,
            next_sibling_chirho,
            first_child_chirho,
            name_chirho,
            content_offset_chirho,
            content_size_chirho,
        })
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
        let mut idx_chirho = Some(node_idx_chirho);

        while let Some(i_chirho) = idx_chirho {
            if i_chirho >= self.nodes_chirho.len() {
                break;
            }
            let node_chirho = &self.nodes_chirho[i_chirho];
            if !node_chirho.name_chirho.is_empty() {
                components_chirho.push(&node_chirho.name_chirho);
            }
            idx_chirho = node_chirho.parent_idx_chirho();
        }

        components_chirho.reverse();
        format!("/{}", components_chirho.join("/"))
    }

    /// Get the number of entries.
    pub fn entry_count_chirho(&self) -> usize {
        self.nodes_chirho.iter().filter(|n_chirho| n_chirho.has_content_chirho()).count()
    }

    /// Read content for a node index.
    fn read_content_chirho(&self, idx_chirho: usize) -> ResultChirho<Option<String>> {
        if idx_chirho >= self.nodes_chirho.len() {
            return Ok(None);
        }

        let node_chirho = &self.nodes_chirho[idx_chirho];
        if !node_chirho.has_content_chirho() {
            return Ok(None);
        }

        // Content is in .bdt file
        let bdt_path_chirho = self.path_chirho.join(format!("{}.bdt", self.basename_chirho));
        let mut file_chirho = File::open(&bdt_path_chirho).map_err(ErrorChirho::IoChirho)?;

        file_chirho.seek(SeekFrom::Start(node_chirho.content_offset_chirho as u64))
            .map_err(ErrorChirho::IoChirho)?;

        let mut buffer_chirho = vec![0u8; node_chirho.content_size_chirho as usize];
        file_chirho.read_exact(&mut buffer_chirho).map_err(ErrorChirho::IoChirho)?;

        let content_chirho = String::from_utf8_lossy(&buffer_chirho).to_string();
        Ok(Some(content_chirho))
    }

    /// Read an entry by tree key path.
    pub fn read_entry_chirho(&self, key_chirho: &str) -> ResultChirho<Option<String>> {
        let normalized_chirho = self.normalize_key_chirho(key_chirho);

        if let Some(&idx_chirho) = self.key_index_chirho.get(&normalized_chirho) {
            self.read_content_chirho(idx_chirho)
        } else {
            Ok(None)
        }
    }

    /// Read an entry using a TreeKey.
    pub fn read_by_key_chirho(&self, key_chirho: &TreeKeyChirho) -> ResultChirho<Option<String>> {
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
            // Use tree structure to get children (first_child + next_sibling chain)
            let mut children_chirho = Vec::new();

            if let Some(first_child_idx_chirho) = self.nodes_chirho[idx_chirho].first_child_idx_chirho() {
                let mut child_idx_chirho = Some(first_child_idx_chirho);
                while let Some(ci_chirho) = child_idx_chirho {
                    if ci_chirho >= self.nodes_chirho.len() {
                        break;
                    }
                    children_chirho.push(self.build_path_chirho(ci_chirho));
                    child_idx_chirho = self.nodes_chirho[ci_chirho].next_sibling_idx_chirho();
                }
            }

            children_chirho
        } else {
            Vec::new()
        }
    }

    /// Get all root-level keys (entries with no parent).
    pub fn get_root_keys_chirho(&self) -> Vec<String> {
        self.nodes_chirho
            .iter()
            .enumerate()
            .filter(|(_, n_chirho)| n_chirho.parent_idx_chirho().is_none())
            .map(|(idx_chirho, _)| self.build_path_chirho(idx_chirho))
            .collect()
    }

    /// Get children keys for display (just the name, not full path).
    pub fn get_children_names_chirho(&self, key_chirho: &str) -> Vec<(String, String)> {
        let normalized_chirho = self.normalize_key_chirho(key_chirho);

        if let Some(&idx_chirho) = self.key_index_chirho.get(&normalized_chirho) {
            let mut children_chirho = Vec::new();

            if let Some(first_child_idx_chirho) = self.nodes_chirho[idx_chirho].first_child_idx_chirho() {
                let mut child_idx_chirho = Some(first_child_idx_chirho);
                while let Some(ci_chirho) = child_idx_chirho {
                    if ci_chirho >= self.nodes_chirho.len() {
                        break;
                    }
                    let node_chirho = &self.nodes_chirho[ci_chirho];
                    let path_chirho = self.build_path_chirho(ci_chirho);
                    children_chirho.push((path_chirho, node_chirho.name_chirho.clone()));
                    child_idx_chirho = node_chirho.next_sibling_idx_chirho();
                }
            }

            children_chirho
        } else {
            Vec::new()
        }
    }

    /// Check if a key exists.
    pub fn has_key_chirho(&self, key_chirho: &str) -> bool {
        let normalized_chirho = self.normalize_key_chirho(key_chirho);
        self.key_index_chirho.contains_key(&normalized_chirho)
    }

    /// Check if an entry has children.
    pub fn has_children_chirho(&self, key_chirho: &str) -> bool {
        let normalized_chirho = self.normalize_key_chirho(key_chirho);
        if let Some(&idx_chirho) = self.key_index_chirho.get(&normalized_chirho) {
            self.nodes_chirho[idx_chirho].first_child_idx_chirho().is_some()
        } else {
            false
        }
    }

    /// Create an iterator over all entries.
    pub fn iter_chirho(&self) -> RawGenBookIteratorChirho<'_> {
        RawGenBookIteratorChirho {
            module_chirho: self,
            index_chirho: 0,
        }
    }
}

/// Iterator over RawGenBook entries.
pub struct RawGenBookIteratorChirho<'a> {
    module_chirho: &'a RawGenBookChirho,
    index_chirho: usize,
}

impl<'a> Iterator for RawGenBookIteratorChirho<'a> {
    type Item = ResultChirho<GenBookEntryChirho>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index_chirho < self.module_chirho.nodes_chirho.len() {
            let idx_chirho = self.index_chirho;
            self.index_chirho += 1;

            let node_chirho = &self.module_chirho.nodes_chirho[idx_chirho];
            if !node_chirho.has_content_chirho() {
                continue;
            }

            let key_chirho = self.module_chirho.build_path_chirho(idx_chirho);

            match self.module_chirho.read_content_chirho(idx_chirho) {
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
    fn test_tree_node_indices_chirho() {
        let node_chirho = TreeNodeChirho {
            parent_chirho: 4, // Actual index = 1
            next_sibling_chirho: 8, // Actual index = 2
            first_child_chirho: -1, // No children
            name_chirho: "Test".to_string(),
            content_offset_chirho: 100,
            content_size_chirho: 50,
        };

        assert_eq!(node_chirho.parent_idx_chirho(), Some(1));
        assert_eq!(node_chirho.next_sibling_idx_chirho(), Some(2));
        assert_eq!(node_chirho.first_child_idx_chirho(), None);
        assert!(node_chirho.has_content_chirho());
    }

    #[test]
    fn test_tree_node_root_chirho() {
        let node_chirho = TreeNodeChirho {
            parent_chirho: -1, // Root
            next_sibling_chirho: -1,
            first_child_chirho: 4, // First child at index 1
            name_chirho: "Root".to_string(),
            content_offset_chirho: 0,
            content_size_chirho: 0,
        };

        assert_eq!(node_chirho.parent_idx_chirho(), None);
        assert_eq!(node_chirho.next_sibling_idx_chirho(), None);
        assert_eq!(node_chirho.first_child_idx_chirho(), Some(1));
        assert!(!node_chirho.has_content_chirho());
    }
}
