// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! StrKey implementation for lexicon/dictionary navigation.

use std::cmp::Ordering;
use std::fmt;

use crate::keys_chirho::sw_key_chirho::{KeyErrorChirho, PositionChirho, SwKeyChirho};

/// A key for navigating string-keyed content (lexicons/dictionaries).
#[derive(Clone)]
pub struct StrKeyChirho {
    /// The key text.
    text_chirho: String,
    /// Current index in the module.
    index_chirho: i64,
    /// Error status.
    error_chirho: KeyErrorChirho,
}

impl StrKeyChirho {
    /// Create a new StrKey.
    pub fn new_chirho() -> Self {
        Self {
            text_chirho: String::new(),
            index_chirho: 0,
            error_chirho: KeyErrorChirho::NoneChirho,
        }
    }

    /// Create a StrKey with initial text.
    pub fn with_text_chirho(text_chirho: &str) -> Self {
        Self {
            text_chirho: text_chirho.to_string(),
            index_chirho: 0,
            error_chirho: KeyErrorChirho::NoneChirho,
        }
    }

    /// Get the key text.
    pub fn text_chirho(&self) -> &str {
        &self.text_chirho
    }

    /// Set the key text.
    pub fn set_key_text_chirho(&mut self, text_chirho: &str) {
        self.text_chirho = text_chirho.to_string();
        self.error_chirho = KeyErrorChirho::NoneChirho;
    }

    /// Check if the key is empty.
    pub fn is_empty_chirho(&self) -> bool {
        self.text_chirho.is_empty()
    }
}

impl Default for StrKeyChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

impl fmt::Display for StrKeyChirho {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text_chirho)
    }
}

impl fmt::Debug for StrKeyChirho {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StrKeyChirho")
            .field("text_chirho", &self.text_chirho)
            .field("index_chirho", &self.index_chirho)
            .finish()
    }
}

impl SwKeyChirho for StrKeyChirho {
    fn get_text_chirho(&self) -> &str {
        &self.text_chirho
    }

    fn set_text_chirho(&mut self, text_chirho: &str) {
        self.set_key_text_chirho(text_chirho);
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

    fn is_traversable_chirho(&self) -> bool {
        true
    }

    fn increment_chirho(&mut self, _steps_chirho: i32) {
        // StrKey increment requires module-specific implementation
        // to find the next/previous entry
        self.error_chirho = KeyErrorChirho::OutOfBoundsChirho;
    }

    fn decrement_chirho(&mut self, _steps_chirho: i32) {
        self.error_chirho = KeyErrorChirho::OutOfBoundsChirho;
    }

    fn set_position_chirho(&mut self, position_chirho: PositionChirho) {
        match position_chirho {
            PositionChirho::TopChirho => {
                self.text_chirho.clear();
                self.index_chirho = 0;
            }
            PositionChirho::BottomChirho => {
                // Would need module data to determine the actual last entry
                self.error_chirho = KeyErrorChirho::OutOfBoundsChirho;
            }
        }
    }

    fn compare_chirho(&self, other_chirho: &dyn SwKeyChirho) -> Ordering {
        self.text_chirho.cmp(&other_chirho.get_text_chirho().to_string())
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_new_str_key_chirho() {
        let key_chirho = StrKeyChirho::new_chirho();
        assert!(key_chirho.is_empty_chirho());
    }

    #[test]
    fn test_with_text_chirho() {
        let key_chirho = StrKeyChirho::with_text_chirho("G2316");
        assert_eq!(key_chirho.text_chirho(), "G2316");
        assert!(!key_chirho.is_empty_chirho());
    }

    #[test]
    fn test_set_key_chirho() {
        let mut key_chirho = StrKeyChirho::new_chirho();
        key_chirho.set_key_text_chirho("H430");
        assert_eq!(key_chirho.text_chirho(), "H430");
    }

    #[test]
    fn test_comparison_chirho() {
        let key1_chirho = StrKeyChirho::with_text_chirho("alpha");
        let key2_chirho = StrKeyChirho::with_text_chirho("beta");

        assert_eq!(key1_chirho.compare_chirho(&key2_chirho), Ordering::Less);
    }
}
