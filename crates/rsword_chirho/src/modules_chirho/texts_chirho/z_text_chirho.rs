// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! zText module driver for compressed Bible modules.
//!
//! zText modules store Bible text in compressed blocks with verse, block,
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

/// zText compressed Bible module driver.
pub struct ZTextChirho {
    /// Base module functionality.
    base_chirho: ModuleBaseChirho,
    /// Storage backend.
    storage_chirho: ZVerseChirho,
    /// Current verse key.
    key_chirho: VerseKeyChirho,
    /// Versification system.
    v11n_chirho: Arc<VersificationChirho>,
}

impl ZTextChirho {
    /// Create a new zText module from a path and configuration.
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

    /// Read the text at the current position.
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

    /// Get text at a specific verse reference.
    pub fn get_verse_chirho(&mut self, reference_chirho: &str) -> ResultChirho<String> {
        self.set_key_text_chirho(reference_chirho)?;
        self.read_current_chirho()
    }

    /// Clear the block cache to free memory.
    pub fn clear_cache_chirho(&mut self) {
        self.storage_chirho.clear_cache_chirho();
    }

    /// Iterate through all verses from current position to end.
    pub fn iter_chirho(&mut self) -> ZTextIteratorChirho<'_> {
        ZTextIteratorChirho {
            module_chirho: self,
            finished_chirho: false,
        }
    }
}

impl SwModuleChirho for ZTextChirho {
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

/// Iterator over zText module verses.
pub struct ZTextIteratorChirho<'a> {
    module_chirho: &'a mut ZTextChirho,
    finished_chirho: bool,
}

impl<'a> Iterator for ZTextIteratorChirho<'a> {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished_chirho {
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

    fn create_test_module_chirho() -> (TempDir, ZTextChirho) {
        let temp_dir_chirho = TempDir::new().unwrap();
        let mod_path_chirho = temp_dir_chirho.path().join("testzmod");
        fs::create_dir_all(&mod_path_chirho).unwrap();

        // Create compressed test data
        let compressor_chirho = ZipCompressorChirho::new_chirho();

        // Block 0: Genesis 1:1-2 (verses concatenated with null separators)
        let verse1_chirho = "In the beginning God created the heaven and the earth.";
        let verse2_chirho = "And the earth was without form, and void.";
        let block0_text_chirho = format!("{}\0{}\0", verse1_chirho, verse2_chirho);
        let compressed0_chirho = compressor_chirho
            .compress_chirho(block0_text_chirho.as_bytes())
            .unwrap();

        // Write OT data file
        let mut data_file_chirho = File::create(mod_path_chirho.join("ot.czz")).unwrap();
        data_file_chirho.write_all(&compressed0_chirho).unwrap();

        // Write OT block index (12 bytes per block)
        let mut block_idx_chirho = File::create(mod_path_chirho.join("ot.czs")).unwrap();
        // Block 0: offset=0, comp_size, uncomp_size
        block_idx_chirho.write_u32_sword_chirho(0).unwrap(); // comp_offset
        block_idx_chirho.write_u32_sword_chirho(compressed0_chirho.len() as u32).unwrap();
        block_idx_chirho.write_u32_sword_chirho(block0_text_chirho.len() as u32).unwrap();

        // Write OT verse index (10 bytes per verse)
        // According to versification, Genesis 1:1 is at index 4, Genesis 1:2 at index 5
        // (index 0 = testament intro, 1 = milestone, 2 = book intro, 3 = chapter intro)
        let mut verse_idx_chirho = File::create(mod_path_chirho.join("ot.czv")).unwrap();

        // Write empty entries for indices 0, 1, 2, 3 (intros)
        for _ in 0..4 {
            verse_idx_chirho.write_u32_sword_chirho(0).unwrap(); // block_num
            verse_idx_chirho.write_u32_sword_chirho(0).unwrap(); // offset_in_block
            verse_idx_chirho.write_u16_sword_chirho(0).unwrap(); // size
        }

        // Index 4: Genesis 1:1 (block 0, offset 0)
        verse_idx_chirho.write_u32_sword_chirho(0).unwrap(); // block 0
        verse_idx_chirho.write_u32_sword_chirho(0).unwrap(); // offset 0
        verse_idx_chirho.write_u16_sword_chirho(verse1_chirho.len() as u16).unwrap();

        // Index 5: Genesis 1:2 (block 0, offset after first verse + null)
        verse_idx_chirho.write_u32_sword_chirho(0).unwrap(); // block 0
        verse_idx_chirho.write_u32_sword_chirho((verse1_chirho.len() + 1) as u32).unwrap();
        verse_idx_chirho.write_u16_sword_chirho(verse2_chirho.len() as u16).unwrap();

        // Create NT files (empty but needed)
        File::create(mod_path_chirho.join("nt.czv")).unwrap();
        File::create(mod_path_chirho.join("nt.czs")).unwrap();
        File::create(mod_path_chirho.join("nt.czz")).unwrap();

        // Create module config
        let mut config_chirho = ModuleConfigChirho::new_chirho("TestZBible".to_string());
        config_chirho.set_chirho("Description", "Test Compressed Bible Module");
        config_chirho.set_chirho("Lang", "en");
        config_chirho.set_chirho("CompressType", "ZIP");
        config_chirho.set_chirho("BlockType", "CHAPTER");

        let module_chirho = ZTextChirho::new_chirho(&mod_path_chirho, config_chirho).unwrap();

        (temp_dir_chirho, module_chirho)
    }

    #[test]
    fn test_module_creation_chirho() {
        let (_temp_chirho, module_chirho) = create_test_module_chirho();
        assert_eq!(module_chirho.name_chirho(), "TestZBible");
        assert_eq!(module_chirho.module_type_chirho(), "Biblical Texts");
    }

    #[test]
    fn test_key_navigation_chirho() {
        let (_temp_chirho, mut module_chirho) = create_test_module_chirho();

        // Start at Genesis 1:1
        assert_eq!(module_chirho.verse_key_chirho().get_book_name_chirho(), "Genesis");
        assert_eq!(module_chirho.verse_key_chirho().get_chapter_chirho(), 1);
        assert_eq!(module_chirho.verse_key_chirho().get_verse_chirho(), 1);

        // Move to next verse
        module_chirho.increment_chirho(1);
        assert_eq!(module_chirho.verse_key_chirho().get_verse_chirho(), 2);
    }

    #[test]
    fn test_set_key_chirho() {
        let (_temp_chirho, mut module_chirho) = create_test_module_chirho();

        module_chirho.set_key_text_chirho("John 3:16").unwrap();
        assert_eq!(module_chirho.verse_key_chirho().get_book_name_chirho(), "John");
        assert_eq!(module_chirho.verse_key_chirho().get_chapter_chirho(), 3);
        assert_eq!(module_chirho.verse_key_chirho().get_verse_chirho(), 16);
    }

    #[test]
    fn test_read_compressed_verse_chirho() {
        let (_temp_chirho, mut module_chirho) = create_test_module_chirho();

        // Read Genesis 1:1
        let text_chirho = module_chirho.get_verse_chirho("Genesis 1:1").unwrap();
        assert!(text_chirho.contains("In the beginning"));

        // Read Genesis 1:2
        let text2_chirho = module_chirho.get_verse_chirho("Genesis 1:2").unwrap();
        assert!(text2_chirho.contains("without form"));
    }
}
