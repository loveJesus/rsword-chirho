// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! RawText4 module driver for uncompressed Bible modules with 8-byte indices.
//!
//! RawText4 modules store Bible text in plain files with 8-byte index entries
//! (4-byte offset + 4-byte size), allowing for larger text entries than RawText.

use std::path::Path;
use std::sync::Arc;

use crate::config_chirho::ModuleConfigChirho;
use crate::error_chirho::{ErrorChirho, ResultChirho};
use crate::keys_chirho::sw_key_chirho::SwKeyChirho;
use crate::keys_chirho::verse_key_chirho::VerseKeyChirho;
use crate::modules_chirho::sw_module_chirho::{ModuleBaseChirho, SwModuleChirho};
use crate::storage_chirho::raw_verse4_chirho::RawVerse4Chirho;
use crate::versification_chirho::{kjv_chirho, TestamentChirho, VersificationChirho};
use crate::{DirectionChirho, EncodingChirho, MarkupChirho};

/// RawText4 Bible module driver with 8-byte index entries.
pub struct RawText4Chirho {
    /// Base module functionality.
    base_chirho: ModuleBaseChirho,
    /// Storage backend.
    storage_chirho: RawVerse4Chirho,
    /// Current verse key.
    key_chirho: VerseKeyChirho,
    /// Versification system.
    v11n_chirho: Arc<VersificationChirho>,
}

impl RawText4Chirho {
    /// Create a new RawText4 module from a path and configuration.
    pub fn new_chirho<P: AsRef<Path>>(
        path_chirho: P,
        config_chirho: ModuleConfigChirho,
    ) -> ResultChirho<Self> {
        let storage_chirho = RawVerse4Chirho::open_chirho(path_chirho)?;

        // Get versification from config or default to KJV
        let v11n_name_chirho = config_chirho.versification_chirho();
        let v11n_chirho = if v11n_name_chirho == "KJV" {
            Arc::new(kjv_chirho().clone())
        } else {
            // TODO: Support other versification systems
            Arc::new(kjv_chirho().clone())
        };

        let key_chirho = VerseKeyChirho::with_versification_chirho(v11n_chirho.clone());

        Ok(Self {
            base_chirho: ModuleBaseChirho::new_chirho(config_chirho),
            storage_chirho,
            key_chirho,
            v11n_chirho,
        })
    }

    /// Get the current verse key.
    pub fn verse_key_chirho(&self) -> &VerseKeyChirho {
        &self.key_chirho
    }

    /// Get a mutable reference to the verse key.
    pub fn verse_key_mut_chirho(&mut self) -> &mut VerseKeyChirho {
        &mut self.key_chirho
    }

    /// Read the text at the current position.
    fn read_current_chirho(&mut self) -> ResultChirho<String> {
        let testament_chirho = self.key_chirho.get_testament_chirho();
        let index_chirho = self.calculate_verse_index_chirho();

        self.storage_chirho.read_verse_chirho(testament_chirho, index_chirho)
    }

    /// Calculate the verse index for the current key position.
    fn calculate_verse_index_chirho(&self) -> u32 {
        let testament_enum_chirho = if self.key_chirho.get_testament_chirho() == 1 {
            TestamentChirho::OldChirho
        } else {
            TestamentChirho::NewChirho
        };

        self.v11n_chirho
            .calculate_index_chirho(
                testament_enum_chirho,
                self.key_chirho.get_book_chirho() as usize,
                self.key_chirho.get_chapter_chirho() as u8,
                self.key_chirho.get_verse_chirho() as u8,
            )
            .unwrap_or(0)
    }

    /// Get the versification system.
    pub fn versification_chirho(&self) -> &VersificationChirho {
        &self.v11n_chirho
    }

    /// Set the key from a verse reference string.
    pub fn set_key_text_chirho(&mut self, text_chirho: &str) -> ResultChirho<()> {
        self.key_chirho.parse_chirho(text_chirho)?;
        Ok(())
    }

    /// Get text at a specific verse reference.
    pub fn get_verse_chirho(&mut self, reference_chirho: &str) -> ResultChirho<String> {
        self.set_key_text_chirho(reference_chirho)?;
        self.read_current_chirho()
    }

    /// Write text at the current key position.
    ///
    /// # Arguments
    /// * `text_chirho` - The text content to write
    pub fn write_verse_chirho(&mut self, text_chirho: &str) -> ResultChirho<()> {
        let testament_chirho = self.key_chirho.get_testament_chirho();
        let index_chirho = self.calculate_verse_index_chirho();
        self.storage_chirho.write_verse_chirho(testament_chirho, index_chirho, text_chirho)
    }

    /// Write text at a specific verse reference.
    ///
    /// # Arguments
    /// * `reference_chirho` - The verse reference (e.g., "Gen 1:1")
    /// * `text_chirho` - The text content to write
    pub fn write_verse_at_chirho(&mut self, reference_chirho: &str, text_chirho: &str) -> ResultChirho<()> {
        self.set_key_text_chirho(reference_chirho)?;
        self.write_verse_chirho(text_chirho)
    }

    /// Link the current verse to another verse (make it reference the same text).
    ///
    /// Note: Both verses must be in the same testament for linking to work.
    ///
    /// # Arguments
    /// * `source_reference_chirho` - The verse reference to link to
    pub fn link_verse_chirho(&mut self, source_reference_chirho: &str) -> ResultChirho<()> {
        let dest_testament_chirho = self.key_chirho.get_testament_chirho();
        let dest_index_chirho = self.calculate_verse_index_chirho();

        // Parse the source reference
        let mut source_key_chirho = VerseKeyChirho::with_versification_chirho(self.v11n_chirho.clone());
        source_key_chirho.parse_chirho(source_reference_chirho)?;

        let source_testament_chirho = source_key_chirho.get_testament_chirho();
        if source_testament_chirho != dest_testament_chirho {
            return Err(ErrorChirho::generic_chirho(
                "Cannot link verses across testaments".to_string()
            ));
        }

        let source_testament_enum_chirho = if source_testament_chirho == 1 {
            TestamentChirho::OldChirho
        } else {
            TestamentChirho::NewChirho
        };

        let source_index_chirho = self.v11n_chirho
            .calculate_index_chirho(
                source_testament_enum_chirho,
                source_key_chirho.get_book_chirho() as usize,
                source_key_chirho.get_chapter_chirho() as u8,
                source_key_chirho.get_verse_chirho() as u8,
            )
            .unwrap_or(0);

        self.storage_chirho.link_verse_chirho(
            dest_testament_chirho,
            dest_index_chirho,
            source_index_chirho,
        )
    }
}

impl SwModuleChirho for RawText4Chirho {
    fn name_chirho(&self) -> &str {
        self.base_chirho.name_chirho()
    }

    fn description_chirho(&self) -> &str {
        self.base_chirho.description_chirho()
    }

    fn module_type_chirho(&self) -> &str {
        "Biblical Texts"
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
        if self.key_chirho.has_error_chirho() {
            return Err(ErrorChirho::invalid_verse_reference_chirho(
                key_chirho.get_text_chirho(),
            ));
        }
        Ok(())
    }

    fn get_raw_entry_chirho(&mut self) -> ResultChirho<String> {
        self.read_current_chirho()
    }

    fn is_encrypted_chirho(&self) -> bool {
        self.base_chirho.is_encrypted_chirho()
    }

    fn get_config_entry_chirho(&self, key_chirho: &str) -> Option<&str> {
        self.base_chirho.get_config_entry_chirho(key_chirho)
    }

    fn increment_chirho(&mut self, steps_chirho: i32) {
        self.key_chirho.increment_chirho(steps_chirho);
        if self.key_chirho.has_error_chirho() {
            self.base_chirho.set_error_chirho(true);
        }
    }

    fn decrement_chirho(&mut self, steps_chirho: i32) {
        self.key_chirho.decrement_chirho(steps_chirho);
        if self.key_chirho.has_error_chirho() {
            self.base_chirho.set_error_chirho(true);
        }
    }

    fn pop_error_chirho(&mut self) -> bool {
        let key_error_chirho = self.key_chirho.has_error_chirho();
        let _ = self.key_chirho.pop_error_chirho();
        key_error_chirho || self.base_chirho.pop_error_chirho()
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use tempfile::TempDir;

    fn create_test_module_chirho() -> (TempDir, RawText4Chirho) {
        let temp_dir_chirho = TempDir::new().unwrap();
        let mod_path_chirho = temp_dir_chirho.path().join("testmod4");

        // Create storage and write some test data
        let mut storage_chirho = RawVerse4Chirho::create_chirho(&mod_path_chirho).unwrap();

        // Write Genesis 1:1 at index 4 (testament intro + book intro + chapter intro + verse)
        storage_chirho
            .write_verse_chirho(1, 4, "In the beginning God created the heaven and the earth.")
            .unwrap();

        // Write Genesis 1:2 at index 5
        storage_chirho
            .write_verse_chirho(1, 5, "And the earth was without form, and void.")
            .unwrap();

        // Create module config
        let mut config_chirho = ModuleConfigChirho::new_chirho("TestBible4".to_string());
        config_chirho.set_chirho("Description", "Test Bible Module 4-byte");
        config_chirho.set_chirho("Lang", "en");

        let module_chirho = RawText4Chirho::new_chirho(&mod_path_chirho, config_chirho).unwrap();

        (temp_dir_chirho, module_chirho)
    }

    #[test]
    fn test_module_creation_chirho() {
        let (_temp_chirho, module_chirho) = create_test_module_chirho();
        assert_eq!(module_chirho.name_chirho(), "TestBible4");
        assert_eq!(module_chirho.module_type_chirho(), "Biblical Texts");
    }

    #[test]
    fn test_read_verse_chirho() {
        let (_temp_chirho, mut module_chirho) = create_test_module_chirho();

        let text_chirho = module_chirho.get_verse_chirho("Genesis 1:1").unwrap();
        assert!(text_chirho.contains("In the beginning"));
    }
}
