// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! TreeKey implementation for general book navigation.

use std::cmp::Ordering;
use std::fmt;

use crate::keys_chirho::sw_key_chirho::{KeyErrorChirho, PositionChirho, SwKeyChirho};

/// A key for navigating tree-structured content (general books).
#[derive(Clone)]
pub struct TreeKeyChirho {
    /// Path components.
    path_chirho: Vec<String>,
    /// Current index.
    index_chirho: i64,
    /// Error status.
    error_chirho: KeyErrorChirho,
}

impl TreeKeyChirho {
    /// Create a new TreeKey.
    pub fn new_chirho() -> Self {
        Self {
            path_chirho: Vec::new(),
            index_chirho: 0,
            error_chirho: KeyErrorChirho::NoneChirho,
        }
    }

    /// Create a TreeKey from a path string (e.g., "/Chapter1/Section2").
    pub fn from_path_chirho(path_chirho: &str) -> Self {
        let mut key_chirho = Self::new_chirho();
        key_chirho.set_path_chirho(path_chirho);
        key_chirho
    }

    /// Set the path from a string.
    pub fn set_path_chirho(&mut self, path_chirho: &str) {
        self.path_chirho = path_chirho
            .split('/')
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect();
        self.error_chirho = KeyErrorChirho::NoneChirho;
    }

    /// Get the path as a string.
    pub fn get_path_chirho(&self) -> String {
        if self.path_chirho.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", self.path_chirho.join("/"))
        }
    }

    /// Get the current depth.
    pub fn get_depth_chirho(&self) -> usize {
        self.path_chirho.len()
    }

    /// Get the name at a specific depth.
    pub fn get_name_at_chirho(&self, depth_chirho: usize) -> Option<&str> {
        self.path_chirho.get(depth_chirho).map(|s| s.as_str())
    }

    /// Get the current leaf name.
    pub fn get_local_name_chirho(&self) -> &str {
        self.path_chirho.last().map(|s| s.as_str()).unwrap_or("")
    }

    /// Go to parent node.
    pub fn parent_chirho(&mut self) -> bool {
        if self.path_chirho.pop().is_some() {
            true
        } else {
            self.error_chirho = KeyErrorChirho::OutOfBoundsChirho;
            false
        }
    }

    /// Append a child to the path.
    pub fn append_child_chirho(&mut self, name_chirho: &str) {
        self.path_chirho.push(name_chirho.to_string());
    }

    /// Check if this is a root key.
    pub fn is_root_chirho(&self) -> bool {
        self.path_chirho.is_empty()
    }
}

impl Default for TreeKeyChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

impl fmt::Display for TreeKeyChirho {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_path_chirho())
    }
}

impl fmt::Debug for TreeKeyChirho {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TreeKeyChirho")
            .field("path_chirho", &self.path_chirho)
            .field("index_chirho", &self.index_chirho)
            .finish()
    }
}

impl SwKeyChirho for TreeKeyChirho {
    fn get_text_chirho(&self) -> &str {
        self.get_local_name_chirho()
    }

    fn set_text_chirho(&mut self, text_chirho: &str) {
        self.set_path_chirho(text_chirho);
    }

    fn get_osis_ref_chirho(&self) -> String {
        self.get_path_chirho()
    }

    fn pop_error_chirho(&mut self) -> KeyErrorChirho {
        let error_chirho = self.error_chirho;
        self.error_chirho = KeyErrorChirho::NoneChirho;
        error_chirho
    }

    fn has_error_chirho(&self) -> bool {
        self.error_chirho.is_error_chirho()
    }

    fn clone_key_chirho(&self) -> Box<dyn SwKeyChirho> {
        Box::new(self.clone())
    }

    fn get_index_chirho(&self) -> i64 {
        self.index_chirho
    }

    fn set_index_chirho(&mut self, index_chirho: i64) {
        self.index_chirho = index_chirho;
    }

    fn increment_chirho(&mut self, _steps_chirho: i32) {
        // Tree navigation requires module-specific implementation
        self.error_chirho = KeyErrorChirho::OutOfBoundsChirho;
    }

    fn decrement_chirho(&mut self, _steps_chirho: i32) {
        // Tree navigation requires module-specific implementation
        self.error_chirho = KeyErrorChirho::OutOfBoundsChirho;
    }

    fn set_position_chirho(&mut self, position_chirho: PositionChirho) {
        match position_chirho {
            PositionChirho::TopChirho => {
                self.path_chirho.clear();
                self.index_chirho = 0;
            }
            PositionChirho::BottomChirho => {
                // Would need module data to determine the actual bottom
                self.error_chirho = KeyErrorChirho::OutOfBoundsChirho;
            }
        }
    }

    fn compare_chirho(&self, other_chirho: &dyn SwKeyChirho) -> Ordering {
        self.get_path_chirho().cmp(&other_chirho.get_text_chirho().to_string())
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_new_tree_key_chirho() {
        let key_chirho = TreeKeyChirho::new_chirho();
        assert!(key_chirho.is_root_chirho());
        assert_eq!(key_chirho.get_path_chirho(), "/");
    }

    #[test]
    fn test_from_path_chirho() {
        let key_chirho = TreeKeyChirho::from_path_chirho("/Chapter1/Section2");
        assert_eq!(key_chirho.get_depth_chirho(), 2);
        assert_eq!(key_chirho.get_name_at_chirho(0), Some("Chapter1"));
        assert_eq!(key_chirho.get_name_at_chirho(1), Some("Section2"));
        assert_eq!(key_chirho.get_local_name_chirho(), "Section2");
    }

    #[test]
    fn test_parent_chirho() {
        let mut key_chirho = TreeKeyChirho::from_path_chirho("/Chapter1/Section2");
        assert!(key_chirho.parent_chirho());
        assert_eq!(key_chirho.get_path_chirho(), "/Chapter1");
        assert!(key_chirho.parent_chirho());
        assert_eq!(key_chirho.get_path_chirho(), "/");
        assert!(!key_chirho.parent_chirho());
        assert!(key_chirho.has_error_chirho());
    }

    #[test]
    fn test_append_child_chirho() {
        let mut key_chirho = TreeKeyChirho::new_chirho();
        key_chirho.append_child_chirho("Chapter1");
        key_chirho.append_child_chirho("Section2");
        assert_eq!(key_chirho.get_path_chirho(), "/Chapter1/Section2");
    }
}
