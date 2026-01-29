// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! ListKey implementation for handling multiple keys/search results.

use std::cmp::Ordering;
use std::fmt;

use crate::keys_chirho::sw_key_chirho::{KeyErrorChirho, PositionChirho, SwKeyChirho};
use crate::keys_chirho::verse_key_chirho::VerseKeyChirho;

/// A key that contains a list of other keys.
///
/// ListKey is typically used to hold search results or verse ranges.
pub struct ListKeyChirho {
    /// List of contained keys.
    keys_chirho: Vec<Box<dyn SwKeyChirho>>,
    /// Current position in the list.
    position_chirho: usize,
    /// Error status.
    error_chirho: KeyErrorChirho,
    /// Text representation.
    text_chirho: String,
}

impl ListKeyChirho {
    /// Create a new empty ListKey.
    pub fn new_chirho() -> Self {
        Self {
            keys_chirho: Vec::new(),
            position_chirho: 0,
            error_chirho: KeyErrorChirho::NoneChirho,
            text_chirho: String::new(),
        }
    }

    /// Create a ListKey from a vector of keys.
    pub fn from_keys_chirho(keys_chirho: Vec<Box<dyn SwKeyChirho>>) -> Self {
        let mut list_chirho = Self {
            keys_chirho,
            position_chirho: 0,
            error_chirho: KeyErrorChirho::NoneChirho,
            text_chirho: String::new(),
        };
        list_chirho.update_text_chirho();
        list_chirho
    }

    /// Add a key to the list.
    pub fn add_chirho(&mut self, key_chirho: Box<dyn SwKeyChirho>) {
        self.keys_chirho.push(key_chirho);
        self.update_text_chirho();
    }

    /// Add a VerseKey to the list.
    pub fn add_verse_chirho(&mut self, key_chirho: VerseKeyChirho) {
        self.add_chirho(Box::new(key_chirho));
    }

    /// Clear all keys from the list.
    pub fn clear_chirho(&mut self) {
        self.keys_chirho.clear();
        self.position_chirho = 0;
        self.text_chirho.clear();
    }

    /// Get the number of keys in the list.
    pub fn count_chirho(&self) -> usize {
        self.keys_chirho.len()
    }

    /// Check if the list is empty.
    pub fn is_empty_chirho(&self) -> bool {
        self.keys_chirho.is_empty()
    }

    /// Get the current key.
    pub fn get_element_chirho(&self) -> Option<&dyn SwKeyChirho> {
        self.keys_chirho.get(self.position_chirho).map(|k| k.as_ref())
    }

    /// Get a key at a specific index.
    pub fn get_at_chirho(&self, index_chirho: usize) -> Option<&dyn SwKeyChirho> {
        self.keys_chirho.get(index_chirho).map(|k| k.as_ref())
    }

    /// Remove a key at a specific index.
    pub fn remove_at_chirho(&mut self, index_chirho: usize) -> Option<Box<dyn SwKeyChirho>> {
        if index_chirho < self.keys_chirho.len() {
            let key_chirho = self.keys_chirho.remove(index_chirho);
            if self.position_chirho >= self.keys_chirho.len() && !self.keys_chirho.is_empty() {
                self.position_chirho = self.keys_chirho.len() - 1;
            }
            self.update_text_chirho();
            Some(key_chirho)
        } else {
            None
        }
    }

    /// Get the current position.
    pub fn get_position_index_chirho(&self) -> usize {
        self.position_chirho
    }

    /// Set the current position.
    pub fn set_position_index_chirho(&mut self, index_chirho: usize) {
        if index_chirho < self.keys_chirho.len() {
            self.position_chirho = index_chirho;
            self.error_chirho = KeyErrorChirho::NoneChirho;
        } else if self.keys_chirho.is_empty() {
            self.position_chirho = 0;
            self.error_chirho = KeyErrorChirho::OutOfBoundsChirho;
        } else {
            self.position_chirho = self.keys_chirho.len() - 1;
            self.error_chirho = KeyErrorChirho::OutOfBoundsChirho;
        }
        self.update_text_chirho();
    }

    /// Update the text representation.
    fn update_text_chirho(&mut self) {
        if let Some(key_chirho) = self.get_element_chirho() {
            self.text_chirho = key_chirho.get_text_chirho().to_string();
        } else {
            self.text_chirho.clear();
        }
    }

    /// Iterate over all keys.
    pub fn iter_chirho(&self) -> impl Iterator<Item = &dyn SwKeyChirho> {
        self.keys_chirho.iter().map(|k| k.as_ref())
    }
}

impl Clone for ListKeyChirho {
    fn clone(&self) -> Self {
        Self {
            keys_chirho: self.keys_chirho.iter().map(|k| k.clone_key_chirho()).collect(),
            position_chirho: self.position_chirho,
            error_chirho: self.error_chirho,
            text_chirho: self.text_chirho.clone(),
        }
    }
}

impl Default for ListKeyChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

impl fmt::Display for ListKeyChirho {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text_chirho)
    }
}

impl fmt::Debug for ListKeyChirho {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ListKeyChirho")
            .field("count_chirho", &self.keys_chirho.len())
            .field("position_chirho", &self.position_chirho)
            .field("text_chirho", &self.text_chirho)
            .finish()
    }
}

impl SwKeyChirho for ListKeyChirho {
    fn get_text_chirho(&self) -> &str {
        &self.text_chirho
    }

    fn set_text_chirho(&mut self, text_chirho: &str) {
        // For ListKey, setting text doesn't make sense directly
        // but we can try to parse it as a verse reference
        if let Ok(key_chirho) = VerseKeyChirho::from_str_chirho(text_chirho) {
            self.clear_chirho();
            self.add_verse_chirho(key_chirho);
        }
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
        self.position_chirho as i64
    }

    fn set_index_chirho(&mut self, index_chirho: i64) {
        self.set_position_index_chirho(index_chirho as usize);
    }

    fn increment_chirho(&mut self, steps_chirho: i32) {
        let new_pos_chirho = self.position_chirho as i64 + steps_chirho as i64;
        if new_pos_chirho >= 0 && (new_pos_chirho as usize) < self.keys_chirho.len() {
            self.position_chirho = new_pos_chirho as usize;
            self.error_chirho = KeyErrorChirho::NoneChirho;
        } else {
            self.error_chirho = KeyErrorChirho::OutOfBoundsChirho;
        }
        self.update_text_chirho();
    }

    fn decrement_chirho(&mut self, steps_chirho: i32) {
        self.increment_chirho(-steps_chirho);
    }

    fn set_position_chirho(&mut self, position_chirho: PositionChirho) {
        match position_chirho {
            PositionChirho::TopChirho => {
                self.position_chirho = 0;
                self.error_chirho = KeyErrorChirho::NoneChirho;
            }
            PositionChirho::BottomChirho => {
                if self.keys_chirho.is_empty() {
                    self.position_chirho = 0;
                    self.error_chirho = KeyErrorChirho::OutOfBoundsChirho;
                } else {
                    self.position_chirho = self.keys_chirho.len() - 1;
                    self.error_chirho = KeyErrorChirho::NoneChirho;
                }
            }
        }
        self.update_text_chirho();
    }

    fn compare_chirho(&self, other_chirho: &dyn SwKeyChirho) -> Ordering {
        self.get_index_chirho().cmp(&other_chirho.get_index_chirho())
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_empty_list_chirho() {
        let list_chirho = ListKeyChirho::new_chirho();
        assert!(list_chirho.is_empty_chirho());
        assert_eq!(list_chirho.count_chirho(), 0);
    }

    #[test]
    fn test_add_keys_chirho() {
        let mut list_chirho = ListKeyChirho::new_chirho();

        let key1_chirho = VerseKeyChirho::from_str_chirho("John 3:16").unwrap();
        let key2_chirho = VerseKeyChirho::from_str_chirho("Genesis 1:1").unwrap();

        list_chirho.add_verse_chirho(key1_chirho);
        list_chirho.add_verse_chirho(key2_chirho);

        assert_eq!(list_chirho.count_chirho(), 2);
        assert!(!list_chirho.is_empty_chirho());
    }

    #[test]
    fn test_navigate_list_chirho() {
        let mut list_chirho = ListKeyChirho::new_chirho();

        list_chirho.add_verse_chirho(VerseKeyChirho::from_str_chirho("Gen 1:1").unwrap());
        list_chirho.add_verse_chirho(VerseKeyChirho::from_str_chirho("Gen 1:2").unwrap());
        list_chirho.add_verse_chirho(VerseKeyChirho::from_str_chirho("Gen 1:3").unwrap());

        assert_eq!(list_chirho.get_position_index_chirho(), 0);

        list_chirho.increment_chirho(1);
        assert_eq!(list_chirho.get_position_index_chirho(), 1);
        assert!(!list_chirho.has_error_chirho());

        list_chirho.increment_chirho(1);
        assert_eq!(list_chirho.get_position_index_chirho(), 2);

        list_chirho.increment_chirho(1);
        assert!(list_chirho.has_error_chirho());
    }

    #[test]
    fn test_set_position_chirho() {
        let mut list_chirho = ListKeyChirho::new_chirho();

        list_chirho.add_verse_chirho(VerseKeyChirho::from_str_chirho("Gen 1:1").unwrap());
        list_chirho.add_verse_chirho(VerseKeyChirho::from_str_chirho("Rev 22:21").unwrap());

        list_chirho.set_position_chirho(PositionChirho::BottomChirho);
        assert_eq!(list_chirho.get_position_index_chirho(), 1);

        list_chirho.set_position_chirho(PositionChirho::TopChirho);
        assert_eq!(list_chirho.get_position_index_chirho(), 0);
    }
}
