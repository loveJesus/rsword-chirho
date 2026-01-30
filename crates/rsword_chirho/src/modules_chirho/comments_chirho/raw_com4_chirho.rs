// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! RawCom4 module driver for uncompressed commentary modules with 8-byte indices.
//!
//! RawCom4 modules store commentary text in plain files with 8-byte index entries
//! (4-byte offset + 4-byte size), allowing for larger commentary entries than RawCom.

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

/// RawCom4 commentary module driver with 8-byte index entries.
pub struct RawCom4Chirho {
    /// Base module functionality.
    base_chirho: ModuleBaseChirho,
    /// Storage backend.
    storage_chirho: RawVerse4Chirho,
    /// Current verse key.
    key_chirho: VerseKeyChirho,
    /// Versification system.
    v11n_chirho: Arc<VersificationChirho>,
}

impl RawCom4Chirho {
    /// Create a new RawCom4 module from a path and configuration.
    pub fn new_chirho<P: AsRef<Path>>(
        path_chirho: P,
        config_chirho: ModuleConfigChirho,
    ) -> ResultChirho<Self> {
        let storage_chirho = RawVerse4Chirho::open_chirho(path_chirho)?;

        // Get versification from config or default to KJV
        let v11n_name_chirho = config_chirho.versification_chirho();
        let v11n_chirho = Arc::new(
            crate::versification_chirho::get_versification_chirho(v11n_name_chirho)
                .unwrap_or_else(|| kjv_chirho())
                .clone()
        );

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

    /// Read the commentary at the current position.
    fn read_current_chirho(&mut self) -> ResultChirho<String> {
        let testament_chirho = self.key_chirho.get_testament_chirho();
        let index_chirho = self.calculate_verse_index_chirho()?;

        self.storage_chirho.read_verse_chirho(testament_chirho, index_chirho)
    }

    /// Calculate the verse index for the current key position.
    fn calculate_verse_index_chirho(&self) -> ResultChirho<u32> {
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
            .ok_or_else(|| ErrorChirho::verse_out_of_bounds_chirho(
                self.key_chirho.get_text_chirho(),
            ))
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

    /// Get commentary at a specific verse reference.
    pub fn get_commentary_chirho(&mut self, reference_chirho: &str) -> ResultChirho<String> {
        self.set_key_text_chirho(reference_chirho)?;
        self.read_current_chirho()
    }

    /// Find commentary entries that exist near the current key.
    pub fn find_next_entry_chirho(&mut self) -> bool {
        let max_search_chirho = 200;

        for _ in 0..max_search_chirho {
            if let Ok(text_chirho) = self.read_current_chirho() {
                if !text_chirho.is_empty() {
                    return true;
                }
            }

            self.key_chirho.increment_chirho(1);
            if self.key_chirho.has_error_chirho() {
                return false;
            }
        }

        false
    }

    /// Iterate through commentary entries from current position.
    pub fn iter_chirho(&mut self) -> RawCom4IteratorChirho<'_> {
        RawCom4IteratorChirho {
            module_chirho: self,
            finished_chirho: false,
        }
    }
}

impl SwModuleChirho for RawCom4Chirho {
    fn name_chirho(&self) -> &str {
        self.base_chirho.name_chirho()
    }

    fn description_chirho(&self) -> &str {
        self.base_chirho.description_chirho()
    }

    fn module_type_chirho(&self) -> &str {
        "Commentaries"
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

    fn has_entry_chirho(&self) -> bool {
        true
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

/// Iterator over RawCom4 module entries.
pub struct RawCom4IteratorChirho<'a> {
    module_chirho: &'a mut RawCom4Chirho,
    finished_chirho: bool,
}

impl<'a> Iterator for RawCom4IteratorChirho<'a> {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished_chirho {
            return None;
        }

        if !self.module_chirho.find_next_entry_chirho() {
            self.finished_chirho = true;
            return None;
        }

        let key_text_chirho = self.module_chirho.key_chirho.get_text_chirho().to_string();
        let text_chirho = self.module_chirho.read_current_chirho().ok()?;

        self.module_chirho.increment_chirho(1);
        if self.module_chirho.key_chirho.has_error_chirho() {
            self.finished_chirho = true;
            let _ = self.module_chirho.key_chirho.pop_error_chirho();
        }

        Some((key_text_chirho, text_chirho))
    }
}
