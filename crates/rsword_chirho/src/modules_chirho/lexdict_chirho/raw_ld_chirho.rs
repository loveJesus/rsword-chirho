// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! RawLD module driver for uncompressed lexicon/dictionary modules.
//!
//! RawLD modules store lexicon entries in plain files with string keys.
//! Used for Strong's numbers, dictionaries, glossaries, etc.

use std::path::Path;

use crate::config_chirho::ModuleConfigChirho;
use crate::error_chirho::{ErrorChirho, ResultChirho};
use crate::keys_chirho::sw_key_chirho::SwKeyChirho;
use crate::keys_chirho::str_key_chirho::StrKeyChirho;
use crate::modules_chirho::sw_module_chirho::{ModuleBaseChirho, SwModuleChirho};
use crate::storage_chirho::raw_str_chirho::RawStrChirho;
use crate::{DirectionChirho, EncodingChirho, MarkupChirho};

/// RawLD lexicon module driver.
pub struct RawLdChirho {
    /// Base module functionality.
    base_chirho: ModuleBaseChirho,
    /// Storage backend.
    storage_chirho: RawStrChirho,
    /// Current string key.
    key_chirho: StrKeyChirho,
    /// Total number of entries.
    entry_count_chirho: u32,
    /// Current entry index.
    current_index_chirho: u32,
}

impl RawLdChirho {
    /// Create a new RawLD module from a path and configuration.
    pub fn new_chirho<P: AsRef<Path>>(
        path_chirho: P,
        config_chirho: ModuleConfigChirho,
    ) -> ResultChirho<Self> {
        // Determine the basename for the lexicon files
        let basename_chirho = config_chirho
            .get_chirho("DataPath")
            .and_then(|p| p.rsplit('/').next())
            .unwrap_or(&config_chirho.name_chirho);

        let mut storage_chirho = RawStrChirho::open_chirho(&path_chirho, basename_chirho, false)?;
        let entry_count_chirho = storage_chirho.entry_count_chirho()?;

        Ok(Self {
            base_chirho: ModuleBaseChirho::new_chirho(config_chirho),
            storage_chirho,
            key_chirho: StrKeyChirho::new_chirho(),
            entry_count_chirho,
            current_index_chirho: 0,
        })
    }

    /// Get the current string key.
    pub fn str_key_chirho(&self) -> &StrKeyChirho {
        &self.key_chirho
    }

    /// Get a mutable reference to the string key.
    pub fn str_key_mut_chirho(&mut self) -> &mut StrKeyChirho {
        &mut self.key_chirho
    }

    /// Read the entry at the current key.
    fn read_current_chirho(&mut self) -> ResultChirho<String> {
        if self.key_chirho.is_empty_chirho() {
            // If no key set, try to read by index
            return self.read_by_index_chirho(self.current_index_chirho);
        }

        let key_text_chirho = self.key_chirho.get_text_chirho().to_string();
        super::lookup_with_strongs_fallback_chirho(&key_text_chirho, |k_chirho| {
            self.storage_chirho.find_by_key_chirho(k_chirho)
        })?
        .ok_or_else(|| ErrorChirho::key_not_found_chirho(&key_text_chirho))
    }

    /// Read entry by index.
    fn read_by_index_chirho(&mut self, index_chirho: u32) -> ResultChirho<String> {
        let (key_chirho, value_chirho) = self.storage_chirho.read_entry_chirho(index_chirho)?;
        self.key_chirho.set_key_text_chirho(&key_chirho);
        self.current_index_chirho = index_chirho;
        Ok(value_chirho)
    }

    /// Get the total number of entries.
    pub fn entry_count_chirho(&self) -> u32 {
        self.entry_count_chirho
    }

    /// Set the key from a string.
    pub fn set_key_text_chirho(&mut self, text_chirho: &str) {
        self.key_chirho.set_key_text_chirho(text_chirho);
    }

    /// Look up an entry by key.
    pub fn get_entry_chirho(&mut self, key_chirho: &str) -> ResultChirho<String> {
        self.set_key_text_chirho(key_chirho);
        self.read_current_chirho()
    }

    /// Get all keys in the lexicon.
    pub fn get_keys_chirho(&mut self) -> ResultChirho<Vec<String>> {
        let mut keys_chirho = Vec::new();
        for i_chirho in 0..self.entry_count_chirho {
            let (key_chirho, _) = self.storage_chirho.read_entry_chirho(i_chirho)?;
            keys_chirho.push(key_chirho);
        }
        Ok(keys_chirho)
    }

    /// Search for keys matching a pattern.
    pub fn search_keys_chirho(&mut self, pattern_chirho: &str) -> ResultChirho<Vec<String>> {
        let pattern_lower_chirho = pattern_chirho.to_lowercase();
        let mut matches_chirho = Vec::new();

        for i_chirho in 0..self.entry_count_chirho {
            let (key_chirho, _) = self.storage_chirho.read_entry_chirho(i_chirho)?;
            if key_chirho.to_lowercase().contains(&pattern_lower_chirho) {
                matches_chirho.push(key_chirho);
            }
        }

        Ok(matches_chirho)
    }

    /// Write an entry to the lexicon.
    ///
    /// # Arguments
    /// * `key_chirho` - The entry key (e.g., Strong's number "G25")
    /// * `value_chirho` - The entry content/definition
    pub fn write_entry_chirho(&mut self, key_chirho: &str, value_chirho: &str) -> ResultChirho<()> {
        self.storage_chirho.write_entry_chirho(key_chirho, value_chirho)?;
        self.entry_count_chirho = self.storage_chirho.entry_count_chirho()?;
        Ok(())
    }

    /// Iterate through all entries.
    pub fn iter_chirho(&mut self) -> RawLdIteratorChirho<'_> {
        RawLdIteratorChirho {
            module_chirho: self,
            current_chirho: 0,
        }
    }
}

impl SwModuleChirho for RawLdChirho {
    fn name_chirho(&self) -> &str {
        self.base_chirho.name_chirho()
    }

    fn description_chirho(&self) -> &str {
        self.base_chirho.description_chirho()
    }

    fn module_type_chirho(&self) -> &str {
        "Lexicons / Dictionaries"
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
        self.key_chirho.set_key_text_chirho(key_chirho.get_text_chirho());
        Ok(())
    }

    fn get_raw_entry_chirho(&mut self) -> ResultChirho<String> {
        self.read_current_chirho()
    }

    fn has_entry_chirho(&self) -> bool {
        !self.key_chirho.is_empty_chirho() || self.current_index_chirho < self.entry_count_chirho
    }

    fn is_encrypted_chirho(&self) -> bool {
        self.base_chirho.is_encrypted_chirho()
    }

    fn get_config_entry_chirho(&self, key_chirho: &str) -> Option<&str> {
        self.base_chirho.get_config_entry_chirho(key_chirho)
    }

    fn increment_chirho(&mut self, steps_chirho: i32) {
        let new_index_chirho = self.current_index_chirho as i64 + steps_chirho as i64;
        if new_index_chirho >= 0 && (new_index_chirho as u32) < self.entry_count_chirho {
            self.current_index_chirho = new_index_chirho as u32;
            if let Ok((key_chirho, _)) = self.storage_chirho.read_entry_chirho(self.current_index_chirho) {
                self.key_chirho.set_key_text_chirho(&key_chirho);
            }
        } else {
            self.base_chirho.set_error_chirho(true);
        }
    }

    fn decrement_chirho(&mut self, steps_chirho: i32) {
        self.increment_chirho(-steps_chirho);
    }

    fn pop_error_chirho(&mut self) -> bool {
        self.base_chirho.pop_error_chirho()
    }
}

/// Iterator over RawLD module entries.
pub struct RawLdIteratorChirho<'a> {
    module_chirho: &'a mut RawLdChirho,
    current_chirho: u32,
}

impl<'a> Iterator for RawLdIteratorChirho<'a> {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_chirho >= self.module_chirho.entry_count_chirho {
            return None;
        }

        let result_chirho = self.module_chirho
            .storage_chirho
            .read_entry_chirho(self.current_chirho)
            .ok();

        self.current_chirho += 1;
        result_chirho
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use tempfile::TempDir;

    fn create_test_module_chirho() -> (TempDir, RawLdChirho) {
        let temp_dir_chirho = TempDir::new().unwrap();
        let mod_path_chirho = temp_dir_chirho.path().join("testlex");

        // Create storage and write some test data
        let mut storage_chirho = RawStrChirho::create_chirho(&mod_path_chirho, "testlex", false).unwrap();

        // Write some lexicon entries
        storage_chirho.write_entry_chirho("G2316", "theos (theh'-os) - God").unwrap();
        storage_chirho.write_entry_chirho("G26", "agape (ag-ah'-pay) - love").unwrap();
        storage_chirho.write_entry_chirho("H430", "elohim (el-o-heem') - God").unwrap();

        // Create module config
        let mut config_chirho = ModuleConfigChirho::new_chirho("TestLex".to_string());
        config_chirho.set_chirho("Description", "Test Lexicon Module");
        config_chirho.set_chirho("Lang", "en");
        config_chirho.set_chirho("DataPath", "./modules/lexdict/testlex/testlex");

        let module_chirho = RawLdChirho::new_chirho(&mod_path_chirho, config_chirho).unwrap();

        (temp_dir_chirho, module_chirho)
    }

    #[test]
    fn test_module_creation_chirho() {
        let (_temp_chirho, module_chirho) = create_test_module_chirho();
        assert_eq!(module_chirho.name_chirho(), "TestLex");
        assert_eq!(module_chirho.module_type_chirho(), "Lexicons / Dictionaries");
        assert_eq!(module_chirho.entry_count_chirho(), 3);
    }

    #[test]
    fn test_lookup_entry_chirho() {
        let (_temp_chirho, mut module_chirho) = create_test_module_chirho();

        let entry_chirho = module_chirho.get_entry_chirho("G2316").unwrap();
        assert!(entry_chirho.contains("theos"));
        assert!(entry_chirho.contains("God"));
    }

    #[test]
    fn test_iterate_entries_chirho() {
        let (_temp_chirho, mut module_chirho) = create_test_module_chirho();

        let entries_chirho: Vec<_> = module_chirho.iter_chirho().collect();
        assert_eq!(entries_chirho.len(), 3);
    }

    #[test]
    fn test_search_keys_chirho() {
        let (_temp_chirho, mut module_chirho) = create_test_module_chirho();

        let greek_keys_chirho = module_chirho.search_keys_chirho("G").unwrap();
        assert_eq!(greek_keys_chirho.len(), 2);

        let hebrew_keys_chirho = module_chirho.search_keys_chirho("H").unwrap();
        assert_eq!(hebrew_keys_chirho.len(), 1);
    }
}
