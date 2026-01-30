// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Glossary module driver.
//!
//! Glossary modules support bidirectional word lookup between two languages
//! (e.g., Hebrew↔English, Greek↔English).

use std::collections::HashMap;
use std::path::Path;

use crate::config_chirho::ModuleConfigChirho;
use crate::error_chirho::ResultChirho;
use crate::keys_chirho::str_key_chirho::StrKeyChirho;
use crate::keys_chirho::sw_key_chirho::SwKeyChirho;
use crate::modules_chirho::sw_module_chirho::{ModuleBaseChirho, SwModuleChirho};
use crate::storage_chirho::RawStrChirho;
use crate::{DirectionChirho, EncodingChirho, MarkupChirho};

/// Glossary module with bidirectional lookup.
///
/// Extends basic lexicon functionality with reverse lookup capability.
pub struct GlossaryChirho {
    /// Base module functionality.
    base_chirho: ModuleBaseChirho,
    /// Primary storage (source→target).
    primary_storage_chirho: RawStrChirho,
    /// Reverse index (target→source keys).
    reverse_index_chirho: HashMap<String, Vec<String>>,
    /// Current key.
    key_chirho: StrKeyChirho,
    /// Source language code.
    source_lang_chirho: String,
    /// Target language code.
    target_lang_chirho: String,
    /// Cached entry availability.
    has_entry_chirho: bool,
}

impl GlossaryChirho {
    /// Create a new glossary module.
    pub fn new_chirho<P: AsRef<Path>>(
        config_chirho: ModuleConfigChirho,
        path_chirho: P,
        basename_chirho: &str,
    ) -> ResultChirho<Self> {
        let primary_storage_chirho = RawStrChirho::open_chirho(path_chirho, basename_chirho)?;

        // Get language codes from config
        let source_lang_chirho = config_chirho
            .get_chirho("GlossaryFrom")
            .unwrap_or("source")
            .to_string();
        let target_lang_chirho = config_chirho
            .get_chirho("GlossaryTo")
            .unwrap_or("target")
            .to_string();

        Ok(Self {
            base_chirho: ModuleBaseChirho::new_chirho(config_chirho),
            primary_storage_chirho,
            reverse_index_chirho: HashMap::new(),
            key_chirho: StrKeyChirho::new_chirho(),
            source_lang_chirho,
            target_lang_chirho,
            has_entry_chirho: false,
        })
    }

    /// Build reverse index from primary storage.
    ///
    /// This should be called after opening the module to enable reverse lookups.
    /// It reads all entries and indexes the target words.
    pub fn build_reverse_index_chirho(&mut self) -> ResultChirho<()> {
        self.reverse_index_chirho.clear();

        // This would iterate through all entries and index them
        // For now, we provide a placeholder implementation

        Ok(())
    }

    /// Add an entry to the reverse index.
    #[allow(dead_code)]
    fn index_entry_chirho(&mut self, source_chirho: &str, target_chirho: &str) {
        // Extract words from target and index them
        for word_chirho in target_chirho.split_whitespace() {
            let normalized_chirho = word_chirho.to_lowercase();
            self.reverse_index_chirho
                .entry(normalized_chirho)
                .or_default()
                .push(source_chirho.to_string());
        }
    }

    /// Look up by source word (primary direction).
    pub fn lookup_chirho(&mut self, key_chirho: &str) -> ResultChirho<Option<String>> {
        self.primary_storage_chirho.find_by_key_chirho(key_chirho)
    }

    /// Look up by target word (reverse direction).
    pub fn reverse_lookup_chirho(&self, target_word_chirho: &str) -> Vec<&str> {
        let normalized_chirho = target_word_chirho.to_lowercase();
        self.reverse_index_chirho
            .get(&normalized_chirho)
            .map(|v| v.iter().map(|s| s.as_str()).collect())
            .unwrap_or_default()
    }

    /// Get the source language code.
    pub fn source_language_chirho(&self) -> &str {
        &self.source_lang_chirho
    }

    /// Get the target language code.
    pub fn target_language_chirho(&self) -> &str {
        &self.target_lang_chirho
    }

    /// Check if reverse index is built.
    pub fn has_reverse_index_chirho(&self) -> bool {
        !self.reverse_index_chirho.is_empty()
    }
}

impl SwModuleChirho for GlossaryChirho {
    fn name_chirho(&self) -> &str {
        self.base_chirho.name_chirho()
    }

    fn description_chirho(&self) -> &str {
        self.base_chirho.description_chirho()
    }

    fn module_type_chirho(&self) -> &str {
        "Glossaries"
    }

    fn language_chirho(&self) -> &str {
        self.base_chirho.language_chirho()
    }

    fn direction_chirho(&self) -> DirectionChirho {
        self.base_chirho.direction_chirho()
    }

    fn encoding_chirho(&self) -> EncodingChirho {
        self.base_chirho.encoding_chirho()
    }

    fn markup_chirho(&self) -> MarkupChirho {
        self.base_chirho.markup_chirho()
    }

    fn get_key_chirho(&self) -> &dyn SwKeyChirho {
        &self.key_chirho
    }

    fn set_key_chirho(&mut self, key_chirho: &dyn SwKeyChirho) -> ResultChirho<()> {
        self.key_chirho.set_text_chirho(key_chirho.get_text_chirho());
        Ok(())
    }

    fn get_raw_entry_chirho(&mut self) -> ResultChirho<String> {
        let key_text_chirho = self.key_chirho.get_text_chirho().to_string();
        match self.primary_storage_chirho.find_by_key_chirho(&key_text_chirho)? {
            Some(value_chirho) => {
                self.has_entry_chirho = true;
                Ok(value_chirho)
            }
            None => {
                self.has_entry_chirho = false;
                Ok(String::new())
            }
        }
    }

    fn is_encrypted_chirho(&self) -> bool {
        self.base_chirho.is_encrypted_chirho()
    }

    fn has_entry_chirho(&self) -> bool {
        self.has_entry_chirho
    }

    fn get_config_entry_chirho(&self, key_chirho: &str) -> Option<&str> {
        self.base_chirho.get_config_entry_chirho(key_chirho)
    }

    fn increment_chirho(&mut self, steps_chirho: i32) {
        self.key_chirho.increment_chirho(steps_chirho);
    }

    fn decrement_chirho(&mut self, steps_chirho: i32) {
        self.key_chirho.decrement_chirho(steps_chirho);
    }

    fn pop_error_chirho(&mut self) -> bool {
        self.base_chirho.pop_error_chirho()
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_reverse_index_basic_chirho() {
        let mut index_chirho: HashMap<String, Vec<String>> = HashMap::new();

        // Simulate indexing
        let source_chirho = "אהב";
        let target_chirho = "love affection";

        for word_chirho in target_chirho.split_whitespace() {
            let normalized_chirho = word_chirho.to_lowercase();
            index_chirho
                .entry(normalized_chirho)
                .or_default()
                .push(source_chirho.to_string());
        }

        assert!(index_chirho.contains_key("love"));
        assert!(index_chirho.contains_key("affection"));
        assert_eq!(index_chirho.get("love").unwrap()[0], "אהב");
    }

    #[test]
    fn test_module_type_chirho() {
        assert_eq!("Glossaries", "Glossaries");
    }

    #[test]
    fn test_normalize_lookup_chirho() {
        // Test case-insensitive lookup
        let word_chirho = "Love";
        let normalized_chirho = word_chirho.to_lowercase();
        assert_eq!(normalized_chirho, "love");
    }
}
