// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! RawGenBook module driver.
//!
//! Provides access to SWORD general book modules using tree-structured keys.
//! File format:
//! - .bdt - Tree index file (tree node structure)
//! - .bds - Data file (entry content)

use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use crate::error_chirho::{ErrorChirho, ResultChirho};
use crate::keys_chirho::TreeKeyChirho;

/// Size of a tree node entry in the index file.
const TREE_NODE_SIZE_CHIRHO: usize = 12;

/// A tree node in the index structure.
#[derive(Debug, Clone, Copy)]
struct TreeNodeChirho {
    /// Offset to the entry data in .bds file.
    offset_chirho: u32,
    /// Size of the entry data.
    size_chirho: u32,
    /// Index of the parent node (-1 for root).
    parent_chirho: i32,
}

impl TreeNodeChirho {
    /// Read a tree node from bytes (big-endian).
    fn from_bytes_chirho(data_chirho: &[u8]) -> Self {
        assert!(data_chirho.len() >= TREE_NODE_SIZE_CHIRHO);
        Self {
            offset_chirho: u32::from_be_bytes([
                data_chirho[0],
                data_chirho[1],
                data_chirho[2],
                data_chirho[3],
            ]),
            size_chirho: u32::from_be_bytes([
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
        self.offset_chirho == 0 && self.size_chirho == 0
    }

    /// Convert to bytes (big-endian).
    fn to_bytes_chirho(self) -> [u8; TREE_NODE_SIZE_CHIRHO] {
        let mut bytes_chirho = [0u8; TREE_NODE_SIZE_CHIRHO];
        bytes_chirho[0..4].copy_from_slice(&self.offset_chirho.to_be_bytes());
        bytes_chirho[4..8].copy_from_slice(&self.size_chirho.to_be_bytes());
        bytes_chirho[8..12].copy_from_slice(&self.parent_chirho.to_be_bytes());
        bytes_chirho
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
    /// Key names for each node.
    key_names_chirho: Vec<String>,
    /// Map from key path to node index.
    key_index_chirho: HashMap<String, usize>,
    /// Children for each node.
    children_chirho: Vec<Vec<usize>>,
}

impl RawGenBookChirho {
    /// Create a new empty RawGenBook module.
    pub fn create_chirho<P: AsRef<Path>>(path_chirho: P, basename_chirho: &str) -> ResultChirho<Self> {
        let path_chirho = path_chirho.as_ref().to_path_buf();

        // Create empty index and data files
        let index_path_chirho = path_chirho.join(format!("{}.bdt", basename_chirho));
        let data_path_chirho = path_chirho.join(format!("{}.bds", basename_chirho));

        // Ensure directory exists
        std::fs::create_dir_all(&path_chirho).map_err(ErrorChirho::IoChirho)?;

        // Create empty files
        File::create(&index_path_chirho).map_err(ErrorChirho::IoChirho)?;
        File::create(&data_path_chirho).map_err(ErrorChirho::IoChirho)?;

        Ok(Self {
            path_chirho,
            basename_chirho: basename_chirho.to_string(),
            nodes_chirho: Vec::new(),
            key_names_chirho: Vec::new(),
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
            key_names_chirho: Vec::new(),
            key_index_chirho: HashMap::new(),
            children_chirho: Vec::new(),
        };
        module_chirho.load_index_chirho()?;
        Ok(module_chirho)
    }

    /// Load the tree index from the .bdt file.
    fn load_index_chirho(&mut self) -> ResultChirho<()> {
        let index_path_chirho = self.path_chirho.join(format!("{}.bdt", self.basename_chirho));

        let file_chirho = File::open(&index_path_chirho).map_err(ErrorChirho::IoChirho)?;

        let metadata_chirho = file_chirho.metadata().map_err(ErrorChirho::IoChirho)?;

        let file_size_chirho = metadata_chirho.len() as usize;

        // RawGenBook uses a variable-length record format:
        // Each record: 4 bytes (offset) + 4 bytes (size) + 4 bytes (parent) + name (null-terminated)
        // We need to read the entire file and parse it

        let mut reader_chirho = BufReader::new(file_chirho);
        let mut buffer_chirho = vec![0u8; file_size_chirho];
        reader_chirho.read_exact(&mut buffer_chirho).map_err(ErrorChirho::IoChirho)?;

        // Parse variable-length records
        let mut pos_chirho = 0;
        while pos_chirho + TREE_NODE_SIZE_CHIRHO < file_size_chirho {
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

    /// Get the number of entries.
    pub fn entry_count_chirho(&self) -> usize {
        self.nodes_chirho.iter().filter(|n_chirho| !n_chirho.is_empty_chirho()).count()
    }

    /// Read an entry by tree key path.
    pub fn read_entry_chirho(&self, key_chirho: &str) -> ResultChirho<Option<String>> {
        let normalized_chirho = self.normalize_key_chirho(key_chirho);

        if let Some(&idx_chirho) = self.key_index_chirho.get(&normalized_chirho) {
            let node_chirho = &self.nodes_chirho[idx_chirho];
            if node_chirho.is_empty_chirho() || node_chirho.size_chirho == 0 {
                return Ok(None);
            }

            let data_path_chirho = self.path_chirho.join(format!("{}.bds", self.basename_chirho));
            let mut file_chirho = File::open(&data_path_chirho).map_err(ErrorChirho::IoChirho)?;

            file_chirho.seek(SeekFrom::Start(node_chirho.offset_chirho as u64)).map_err(ErrorChirho::IoChirho)?;

            let mut buffer_chirho = vec![0u8; node_chirho.size_chirho as usize];
            file_chirho.read_exact(&mut buffer_chirho).map_err(ErrorChirho::IoChirho)?;

            let content_chirho = String::from_utf8_lossy(&buffer_chirho).to_string();
            Ok(Some(content_chirho))
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

    /// Write an entry to the general book.
    ///
    /// # Arguments
    /// * `key_chirho` - The tree key path (e.g., "/Book/Chapter/Section")
    /// * `content_chirho` - The entry content
    /// * `parent_key_chirho` - Optional parent key (defaults to root if None)
    pub fn write_entry_chirho(
        &mut self,
        key_chirho: &str,
        content_chirho: &str,
        parent_key_chirho: Option<&str>,
    ) -> ResultChirho<()> {
        let normalized_chirho = self.normalize_key_chirho(key_chirho);

        // Find parent index
        let parent_idx_chirho = if let Some(pk_chirho) = parent_key_chirho {
            let normalized_parent_chirho = self.normalize_key_chirho(pk_chirho);
            self.key_index_chirho.get(&normalized_parent_chirho).copied()
                .map(|i| i as i32)
                .unwrap_or(-1)
        } else {
            -1 // Root level
        };

        // Write content to data file
        let data_path_chirho = self.path_chirho.join(format!("{}.bds", self.basename_chirho));
        let mut data_file_chirho = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&data_path_chirho)
            .map_err(ErrorChirho::IoChirho)?;

        let offset_chirho = data_file_chirho.seek(SeekFrom::End(0)).map_err(ErrorChirho::IoChirho)? as u32;
        let content_bytes_chirho = content_chirho.as_bytes();
        let size_chirho = content_bytes_chirho.len() as u32;

        data_file_chirho.write_all(content_bytes_chirho).map_err(ErrorChirho::IoChirho)?;

        // Create new node
        let node_chirho = TreeNodeChirho {
            offset_chirho,
            size_chirho,
            parent_chirho: parent_idx_chirho,
        };

        // Extract the key name (last component)
        let key_name_chirho = normalized_chirho.rsplit('/').next().unwrap_or(&normalized_chirho).to_string();

        // Append to index file
        let index_path_chirho = self.path_chirho.join(format!("{}.bdt", self.basename_chirho));
        let mut index_file_chirho = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&index_path_chirho)
            .map_err(ErrorChirho::IoChirho)?;

        index_file_chirho.write_all(&node_chirho.to_bytes_chirho()).map_err(ErrorChirho::IoChirho)?;
        index_file_chirho.write_all(key_name_chirho.as_bytes()).map_err(ErrorChirho::IoChirho)?;
        index_file_chirho.write_all(&[0u8]).map_err(ErrorChirho::IoChirho)?; // Null terminator

        // Update in-memory structures
        let new_idx_chirho = self.nodes_chirho.len();
        self.nodes_chirho.push(node_chirho);
        self.key_names_chirho.push(key_name_chirho);
        self.children_chirho.push(Vec::new());
        self.key_index_chirho.insert(normalized_chirho, new_idx_chirho);

        // Update parent's children list
        if parent_idx_chirho >= 0 {
            let parent_idx_usize_chirho = parent_idx_chirho as usize;
            if parent_idx_usize_chirho < self.children_chirho.len() {
                self.children_chirho[parent_idx_usize_chirho].push(new_idx_chirho);
            }
        }

        Ok(())
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
            if node_chirho.is_empty_chirho() || node_chirho.size_chirho == 0 {
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
            0x00, 0x00, 0x01, 0x00, // offset = 256
            0x00, 0x00, 0x00, 0x64, // size = 100
            0xFF, 0xFF, 0xFF, 0xFF, // parent = -1 (root)
        ];

        let node_chirho = TreeNodeChirho::from_bytes_chirho(&data_chirho);
        assert_eq!(node_chirho.offset_chirho, 256);
        assert_eq!(node_chirho.size_chirho, 100);
        assert_eq!(node_chirho.parent_chirho, -1);
    }

    #[test]
    fn test_tree_node_empty_chirho() {
        let node_chirho = TreeNodeChirho {
            offset_chirho: 0,
            size_chirho: 0,
            parent_chirho: -1,
        };
        assert!(node_chirho.is_empty_chirho());

        let node2_chirho = TreeNodeChirho {
            offset_chirho: 100,
            size_chirho: 50,
            parent_chirho: 0,
        };
        assert!(!node2_chirho.is_empty_chirho());
    }
}
