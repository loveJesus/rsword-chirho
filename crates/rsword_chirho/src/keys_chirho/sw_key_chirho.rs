// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Base key trait for SWORD module navigation.

use std::cmp::Ordering;
use std::fmt;

/// Position constants for key navigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PositionChirho {
    /// Position at the top (first entry).
    TopChirho,
    /// Position at the bottom (last entry).
    BottomChirho,
}

/// Base trait for all SWORD module keys.
///
/// Keys are used to navigate within modules and identify specific entries.
/// Different module types implement specialized key types (VerseKey, TreeKey, etc.)
/// but all share this common interface.
pub trait SwKeyChirho: fmt::Display + fmt::Debug {
    /// Get the text representation of this key.
    fn get_text_chirho(&self) -> &str;

    /// Set this key from a text string.
    fn set_text_chirho(&mut self, text_chirho: &str);

    /// Get a short text representation.
    fn get_short_text_chirho(&self) -> String {
        self.get_text_chirho().to_string()
    }

    /// Get the OSIS reference format.
    fn get_osis_ref_chirho(&self) -> String {
        self.get_text_chirho().to_string()
    }

    /// Get the error status and clear it.
    fn pop_error_chirho(&mut self) -> KeyErrorChirho;

    /// Check if there was an error.
    fn has_error_chirho(&self) -> bool;

    /// Clone this key into a boxed trait object.
    fn clone_key_chirho(&self) -> Box<dyn SwKeyChirho>;

    /// Get the raw index position.
    fn get_index_chirho(&self) -> i64;

    /// Set the raw index position.
    fn set_index_chirho(&mut self, index_chirho: i64);

    /// Check if this key can be traversed (incremented/decremented).
    fn is_traversable_chirho(&self) -> bool {
        true
    }

    /// Increment the key position.
    fn increment_chirho(&mut self, steps_chirho: i32);

    /// Decrement the key position.
    fn decrement_chirho(&mut self, steps_chirho: i32);

    /// Set to a specific position.
    fn set_position_chirho(&mut self, position_chirho: PositionChirho);

    /// Compare with another key.
    fn compare_chirho(&self, other_chirho: &dyn SwKeyChirho) -> Ordering;

    /// Check equality with another key.
    fn equals_chirho(&self, other_chirho: &dyn SwKeyChirho) -> bool {
        self.compare_chirho(other_chirho) == Ordering::Equal
    }
}

/// Key error status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KeyErrorChirho {
    /// No error.
    #[default]
    NoneChirho,
    /// Key is out of bounds.
    OutOfBoundsChirho,
    /// Key parsing failed.
    ParseErrorChirho,
    /// Key not found.
    NotFoundChirho,
}

impl KeyErrorChirho {
    /// Check if this represents an error.
    pub fn is_error_chirho(&self) -> bool {
        !matches!(self, Self::NoneChirho)
    }
}

/// A simple string-based key implementation.
#[derive(Debug, Clone, Default)]
pub struct SimpleKeyChirho {
    /// The key text.
    text_chirho: String,
    /// Current error status.
    error_chirho: KeyErrorChirho,
    /// Current index.
    index_chirho: i64,
}

impl SimpleKeyChirho {
    /// Create a new simple key.
    pub fn new_chirho(text_chirho: impl Into<String>) -> Self {
        Self {
            text_chirho: text_chirho.into(),
            error_chirho: KeyErrorChirho::NoneChirho,
            index_chirho: 0,
        }
    }
}

impl fmt::Display for SimpleKeyChirho {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text_chirho)
    }
}

impl SwKeyChirho for SimpleKeyChirho {
    fn get_text_chirho(&self) -> &str {
        &self.text_chirho
    }

    fn set_text_chirho(&mut self, text_chirho: &str) {
        self.text_chirho = text_chirho.to_string();
        self.error_chirho = KeyErrorChirho::NoneChirho;
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
        // Simple key doesn't support traversal
        self.error_chirho = KeyErrorChirho::OutOfBoundsChirho;
    }

    fn decrement_chirho(&mut self, _steps_chirho: i32) {
        // Simple key doesn't support traversal
        self.error_chirho = KeyErrorChirho::OutOfBoundsChirho;
    }

    fn set_position_chirho(&mut self, _position_chirho: PositionChirho) {
        // Simple key doesn't support positioning
    }

    fn is_traversable_chirho(&self) -> bool {
        false
    }

    fn compare_chirho(&self, other_chirho: &dyn SwKeyChirho) -> Ordering {
        self.text_chirho.cmp(&other_chirho.get_text_chirho().to_string())
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_simple_key_chirho() {
        let mut key_chirho = SimpleKeyChirho::new_chirho("test");
        assert_eq!(key_chirho.get_text_chirho(), "test");
        assert!(!key_chirho.has_error_chirho());

        key_chirho.set_text_chirho("new text");
        assert_eq!(key_chirho.get_text_chirho(), "new text");
    }

    #[test]
    fn test_key_error_chirho() {
        let mut key_chirho = SimpleKeyChirho::new_chirho("test");
        key_chirho.increment_chirho(1);
        assert!(key_chirho.has_error_chirho());

        let error_chirho = key_chirho.pop_error_chirho();
        assert_eq!(error_chirho, KeyErrorChirho::OutOfBoundsChirho);
        assert!(!key_chirho.has_error_chirho());
    }

    #[test]
    fn test_key_comparison_chirho() {
        let key1_chirho = SimpleKeyChirho::new_chirho("aaa");
        let key2_chirho = SimpleKeyChirho::new_chirho("bbb");

        assert_eq!(key1_chirho.compare_chirho(&key2_chirho), Ordering::Less);
        assert!(key1_chirho.equals_chirho(&SimpleKeyChirho::new_chirho("aaa")));
    }
}
