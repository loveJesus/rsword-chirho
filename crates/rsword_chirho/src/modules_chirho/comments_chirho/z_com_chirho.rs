// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! zCom module driver for compressed commentary modules.
//!
//! zCom modules store commentary text in compressed blocks with verse, block,
//! and data index files. Supports zlib, bzip2, and xz compression.

use std::path::Path;
use std::sync::Arc;

use crate::compression_chirho::{create_compressor_chirho, parse_compression_type_chirho};
use crate::config_chirho::ModuleConfigChirho;
use crate::error_chirho::{ErrorChirho, ResultChirho};
use crate::keys_chirho::sw_key_chirho::SwKeyChirho;
use crate::keys_chirho::verse_key_chirho::VerseKeyChirho;
use crate::modules_chirho::sw_module_chirho::{ModuleBaseChirho, SwModuleChirho};
use crate::storage_chirho::z_verse_chirho::ZVerseChirho;
use crate::versification_chirho::{kjv_chirho, TestamentChirho, VersificationChirho};
use crate::{BlockTypeChirho, DirectionChirho, EncodingChirho, MarkupChirho};

/// zCom compressed commentary module driver.
pub struct ZComChirho {
    /// Base module functionality.
    base_chirho: ModuleBaseChirho,
    /// Storage backend.
    storage_chirho: ZVerseChirho,
    /// Current verse key.
    key_chirho: VerseKeyChirho,
    /// Versification system.
    v11n_chirho: Arc<VersificationChirho>,
}

impl ZComChirho {
    /// Create a new zCom module from a path and configuration.
    pub fn new_chirho<P: AsRef<Path>>(
        path_chirho: P,
        config_chirho: ModuleConfigChirho,
    ) -> ResultChirho<Self> {
        // Get compression type from config
        let comp_type_str_chirho = config_chirho
            .get_chirho("CompressType")
            .unwrap_or("ZIP");
        let comp_type_chirho = parse_compression_type_chirho(comp_type_str_chirho);
        let compressor_chirho = create_compressor_chirho(comp_type_chirho);

        let mut storage_chirho = ZVerseChirho::open_with_comp_type_chirho(
            &path_chirho,
            compressor_chirho,
            comp_type_chirho,
        )?;

        // Get block type from config
        let block_type_chirho = match config_chirho.get_chirho("BlockType") {
            Some("BOOK") => BlockTypeChirho::BookBlocksChirho,
            Some("CHAPTER") => BlockTypeChirho::ChapterBlocksChirho,
            Some("VERSE") => BlockTypeChirho::VerseBlocksChirho,
            _ => BlockTypeChirho::ChapterBlocksChirho, // Default
        };
        storage_chirho.set_block_type_chirho(block_type_chirho);

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

    /// Read the commentary at the current position.
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

    /// Get commentary at a specific verse reference.
    pub fn get_commentary_chirho(&mut self, reference_chirho: &str) -> ResultChirho<String> {
        self.set_key_text_chirho(reference_chirho)?;
        self.read_current_chirho()
    }

    /// Find commentary entries that exist near the current key.
    ///
    /// Commentaries often have entries only for certain verses. This method
    /// helps find the next verse that has commentary content.
    pub fn find_next_entry_chirho(&mut self) -> bool {
        let max_search_chirho = 200; // Limit search to avoid infinite loops

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

    /// Clear the block cache to free memory.
    pub fn clear_cache_chirho(&mut self) {
        self.storage_chirho.clear_cache_chirho();
    }

    /// Iterate through commentary entries from current position.
    pub fn iter_chirho(&mut self) -> ZComIteratorChirho<'_> {
        ZComIteratorChirho {
            module_chirho: self,
            finished_chirho: false,
        }
    }
}

impl SwModuleChirho for ZComChirho {
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
        // Commentary may not have entry at every verse
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

/// Iterator over zCom module entries.
pub struct ZComIteratorChirho<'a> {
    module_chirho: &'a mut ZComChirho,
    finished_chirho: bool,
}

impl<'a> Iterator for ZComIteratorChirho<'a> {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished_chirho {
            return None;
        }

        // Find next non-empty entry
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

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::TempDir;

    use crate::byte_order_chirho::WriteSwordChirho;
    use crate::compression_chirho::{CompressorChirho, ZipCompressorChirho};

    fn create_test_module_chirho() -> (TempDir, ZComChirho) {
        let temp_dir_chirho = TempDir::new().unwrap();
        let mod_path_chirho = temp_dir_chirho.path().join("testzcom");
        fs::create_dir_all(&mod_path_chirho).unwrap();

        // Create compressed test data
        let compressor_chirho = ZipCompressorChirho::new_chirho();

        // Block 0: Commentary for Genesis 1:1
        let verse1_chirho = "Commentary on In the beginning God created the heaven and the earth.";
        let block0_text_chirho = format!("{}\0", verse1_chirho);
        let compressed0_chirho = compressor_chirho
            .compress_chirho(block0_text_chirho.as_bytes())
            .unwrap();

        // Write OT data file
        let mut data_file_chirho = File::create(mod_path_chirho.join("ot.czz")).unwrap();
        data_file_chirho.write_all(&compressed0_chirho).unwrap();

        // Write OT block index (12 bytes per block)
        let mut block_idx_chirho = File::create(mod_path_chirho.join("ot.czs")).unwrap();
        block_idx_chirho.write_u32_sword_chirho(0).unwrap();
        block_idx_chirho.write_u32_sword_chirho(compressed0_chirho.len() as u32).unwrap();
        block_idx_chirho.write_u32_sword_chirho(block0_text_chirho.len() as u32).unwrap();

        // Write OT verse index (10 bytes per verse)
        // Genesis 1:1 is at index 4 (0=testament intro, 1=milestone, 2=book intro, 3=chapter intro)
        let mut verse_idx_chirho = File::create(mod_path_chirho.join("ot.czv")).unwrap();

        // Write empty entries for indices 0, 1, 2, 3 (intros)
        for _ in 0..4 {
            verse_idx_chirho.write_u32_sword_chirho(0).unwrap();
            verse_idx_chirho.write_u32_sword_chirho(0).unwrap();
            verse_idx_chirho.write_u16_sword_chirho(0).unwrap();
        }

        // Index 4: Genesis 1:1
        verse_idx_chirho.write_u32_sword_chirho(0).unwrap();
        verse_idx_chirho.write_u32_sword_chirho(0).unwrap();
        verse_idx_chirho.write_u16_sword_chirho(verse1_chirho.len() as u16).unwrap();

        // Create NT files (empty but needed)
        File::create(mod_path_chirho.join("nt.czv")).unwrap();
        File::create(mod_path_chirho.join("nt.czs")).unwrap();
        File::create(mod_path_chirho.join("nt.czz")).unwrap();

        // Create module config
        let mut config_chirho = ModuleConfigChirho::new_chirho("TestZCom".to_string());
        config_chirho.set_chirho("Description", "Test Compressed Commentary");
        config_chirho.set_chirho("Lang", "en");
        config_chirho.set_chirho("CompressType", "ZIP");
        config_chirho.set_chirho("BlockType", "CHAPTER");

        let module_chirho = ZComChirho::new_chirho(&mod_path_chirho, config_chirho).unwrap();

        (temp_dir_chirho, module_chirho)
    }

    #[test]
    fn test_module_creation_chirho() {
        let (_temp_chirho, module_chirho) = create_test_module_chirho();
        assert_eq!(module_chirho.name_chirho(), "TestZCom");
        assert_eq!(module_chirho.module_type_chirho(), "Commentaries");
    }

    #[test]
    fn test_key_navigation_chirho() {
        let (_temp_chirho, mut module_chirho) = create_test_module_chirho();

        assert_eq!(module_chirho.verse_key_chirho().get_book_name_chirho(), "Genesis");
        assert_eq!(module_chirho.verse_key_chirho().get_chapter_chirho(), 1);
        assert_eq!(module_chirho.verse_key_chirho().get_verse_chirho(), 1);

        module_chirho.increment_chirho(1);
        assert_eq!(module_chirho.verse_key_chirho().get_verse_chirho(), 2);
    }

    #[test]
    fn test_read_compressed_commentary_chirho() {
        let (_temp_chirho, mut module_chirho) = create_test_module_chirho();

        let text_chirho = module_chirho.get_commentary_chirho("Genesis 1:1").unwrap();
        assert!(text_chirho.contains("Commentary"));
    }
}
