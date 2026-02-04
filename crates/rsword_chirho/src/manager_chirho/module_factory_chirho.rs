// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Module factory for creating module drivers from configuration.
//!
//! Creates the appropriate module driver (RawText, zText, etc.) based on
//! the ModDrv value in the module configuration file.

use std::path::Path;

use crate::config_chirho::ModuleConfigChirho;
use crate::error_chirho::{ErrorChirho, ResultChirho};
use crate::filters_chirho::{
    FilterChirho, FilterOptionsChirho, StripFilterChirho,
    OsisToHtmlFilterChirho, OsisToPlainFilterChirho,
    ThmlToHtmlFilterChirho, ThmlToPlainFilterChirho,
    GbfToHtmlFilterChirho, GbfToPlainFilterChirho,
};
use crate::modules_chirho::{
    SwModuleChirho,
    RawTextChirho, RawText4Chirho, ZTextChirho, ZText4Chirho,
    RawComChirho, RawCom4Chirho, ZComChirho, ZCom4Chirho,
    RawLdChirho, RawLd4Chirho, ZLdChirho, ZLd4Chirho,
    RawGenBookChirho, ZGenBookChirho,
};
use crate::MarkupChirho;

/// Output format for text rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormatChirho {
    /// Plain text output.
    #[default]
    PlainChirho,
    /// HTML output.
    HtmlChirho,
    /// Raw output (no filtering).
    RawChirho,
}

/// Module driver type from configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleDriverTypeChirho {
    /// RawText (uncompressed 6-byte index).
    RawTextChirho,
    /// RawText4 (uncompressed 8-byte index).
    RawText4Chirho,
    /// zText (compressed text).
    ZTextChirho,
    /// zText4 (compressed text 8-byte).
    ZText4Chirho,
    /// RawCom (uncompressed commentary).
    RawComChirho,
    /// RawCom4 (uncompressed commentary 8-byte).
    RawCom4Chirho,
    /// zCom (compressed commentary).
    ZComChirho,
    /// zCom4 (compressed commentary 8-byte).
    ZCom4Chirho,
    /// RawLD (uncompressed lexicon).
    RawLdChirho,
    /// RawLD4 (uncompressed lexicon 8-byte).
    RawLd4Chirho,
    /// zLD (compressed lexicon).
    ZLdChirho,
    /// zLD4 (compressed lexicon 8-byte).
    ZLd4Chirho,
    /// RawGenBook (general book).
    RawGenBookChirho,
    /// zGenBook (compressed general book).
    ZGenBookChirho,
    /// Unknown driver.
    UnknownChirho,
}

impl ModuleDriverTypeChirho {
    /// Parse driver type from string.
    pub fn from_str_chirho(s_chirho: &str) -> Self {
        match s_chirho {
            "RawText" => Self::RawTextChirho,
            "RawText4" => Self::RawText4Chirho,
            "zText" => Self::ZTextChirho,
            "zText4" => Self::ZText4Chirho,
            "RawCom" => Self::RawComChirho,
            "RawCom4" => Self::RawCom4Chirho,
            "zCom" => Self::ZComChirho,
            "zCom4" => Self::ZCom4Chirho,
            "RawLD" => Self::RawLdChirho,
            "RawLD4" => Self::RawLd4Chirho,
            "zLD" => Self::ZLdChirho,
            "zLD4" => Self::ZLd4Chirho,
            "RawGenBook" => Self::RawGenBookChirho,
            "zGenBook" => Self::ZGenBookChirho,
            _ => Self::UnknownChirho,
        }
    }

    /// Check if this is a Bible text module.
    pub fn is_bible_chirho(&self) -> bool {
        matches!(
            self,
            Self::RawTextChirho | Self::RawText4Chirho | Self::ZTextChirho | Self::ZText4Chirho
        )
    }

    /// Check if this is a commentary module.
    pub fn is_commentary_chirho(&self) -> bool {
        matches!(
            self,
            Self::RawComChirho | Self::RawCom4Chirho | Self::ZComChirho | Self::ZCom4Chirho
        )
    }

    /// Check if this is a lexicon module.
    pub fn is_lexicon_chirho(&self) -> bool {
        matches!(
            self,
            Self::RawLdChirho | Self::RawLd4Chirho | Self::ZLdChirho | Self::ZLd4Chirho
        )
    }

    /// Check if this is a compressed module.
    pub fn is_compressed_chirho(&self) -> bool {
        matches!(
            self,
            Self::ZTextChirho
                | Self::ZText4Chirho
                | Self::ZComChirho
                | Self::ZCom4Chirho
                | Self::ZLdChirho
                | Self::ZLd4Chirho
        )
    }

    /// Check if this uses 4-byte (instead of 2-byte) size fields.
    pub fn is_four_byte_chirho(&self) -> bool {
        matches!(
            self,
            Self::RawText4Chirho
                | Self::ZText4Chirho
                | Self::RawCom4Chirho
                | Self::ZCom4Chirho
                | Self::RawLd4Chirho
                | Self::ZLd4Chirho
        )
    }
}

/// Loaded module that can read text.
pub struct LoadedModuleChirho {
    /// Module name.
    pub name_chirho: String,
    /// Module configuration.
    pub config_chirho: ModuleConfigChirho,
    /// Data path.
    pub data_path_chirho: std::path::PathBuf,
    /// Driver type.
    pub driver_type_chirho: ModuleDriverTypeChirho,
}

impl LoadedModuleChirho {
    /// Read a verse or entry by key text.
    pub fn read_entry_chirho(&self, key_chirho: &str) -> ResultChirho<String> {
        match self.driver_type_chirho {
            ModuleDriverTypeChirho::RawTextChirho => {
                let mut module_chirho = RawTextChirho::new_chirho(
                    &self.data_path_chirho,
                    self.config_chirho.clone(),
                )?;
                module_chirho.set_key_text_chirho(key_chirho)?;
                module_chirho.get_raw_entry_chirho()
            }
            ModuleDriverTypeChirho::RawText4Chirho => {
                let mut module_chirho = RawText4Chirho::new_chirho(
                    &self.data_path_chirho,
                    self.config_chirho.clone(),
                )?;
                module_chirho.set_key_text_chirho(key_chirho)?;
                module_chirho.get_raw_entry_chirho()
            }
            ModuleDriverTypeChirho::ZTextChirho => {
                let mut module_chirho = ZTextChirho::new_chirho(
                    &self.data_path_chirho,
                    self.config_chirho.clone(),
                )?;
                module_chirho.set_key_text_chirho(key_chirho)?;
                module_chirho.get_raw_entry_chirho()
            }
            ModuleDriverTypeChirho::ZText4Chirho => {
                let mut module_chirho = ZText4Chirho::new_chirho(
                    &self.data_path_chirho,
                    self.config_chirho.clone(),
                )?;
                module_chirho.set_key_text_chirho(key_chirho)?;
                module_chirho.get_raw_entry_chirho()
            }
            ModuleDriverTypeChirho::RawComChirho => {
                let mut module_chirho = RawComChirho::new_chirho(
                    &self.data_path_chirho,
                    self.config_chirho.clone(),
                )?;
                module_chirho.set_key_text_chirho(key_chirho)?;
                module_chirho.get_raw_entry_chirho()
            }
            ModuleDriverTypeChirho::RawCom4Chirho => {
                let mut module_chirho = RawCom4Chirho::new_chirho(
                    &self.data_path_chirho,
                    self.config_chirho.clone(),
                )?;
                module_chirho.set_key_text_chirho(key_chirho)?;
                module_chirho.get_raw_entry_chirho()
            }
            ModuleDriverTypeChirho::ZComChirho => {
                let mut module_chirho = ZComChirho::new_chirho(
                    &self.data_path_chirho,
                    self.config_chirho.clone(),
                )?;
                module_chirho.set_key_text_chirho(key_chirho)?;
                module_chirho.get_raw_entry_chirho()
            }
            ModuleDriverTypeChirho::ZCom4Chirho => {
                let mut module_chirho = ZCom4Chirho::new_chirho(
                    &self.data_path_chirho,
                    self.config_chirho.clone(),
                )?;
                module_chirho.set_key_text_chirho(key_chirho)?;
                module_chirho.get_raw_entry_chirho()
            }
            ModuleDriverTypeChirho::RawLdChirho => {
                let mut module_chirho = RawLdChirho::new_chirho(
                    &self.data_path_chirho,
                    self.config_chirho.clone(),
                )?;
                module_chirho.get_entry_chirho(key_chirho)
            }
            ModuleDriverTypeChirho::RawLd4Chirho => {
                let mut module_chirho = RawLd4Chirho::new_chirho(
                    &self.data_path_chirho,
                    self.config_chirho.clone(),
                )?;
                module_chirho.get_entry_chirho(key_chirho)
            }
            ModuleDriverTypeChirho::ZLdChirho => {
                let mut module_chirho = ZLdChirho::new_chirho(
                    &self.data_path_chirho,
                    self.config_chirho.clone(),
                )?;
                module_chirho.get_entry_chirho(key_chirho)
            }
            ModuleDriverTypeChirho::ZLd4Chirho => {
                let mut module_chirho = ZLd4Chirho::new_chirho(
                    &self.data_path_chirho,
                    self.config_chirho.clone(),
                )?;
                module_chirho.get_entry_chirho(key_chirho)
            }
            _ => Err(ErrorChirho::invalid_config_chirho(format!(
                "Unsupported module driver: {:?}",
                self.driver_type_chirho
            ))),
        }
    }

    /// Read and filter an entry.
    pub fn read_entry_filtered_chirho(
        &self,
        key_chirho: &str,
        format_chirho: OutputFormatChirho,
        options_chirho: &FilterOptionsChirho,
    ) -> ResultChirho<String> {
        let raw_chirho = self.read_entry_chirho(key_chirho)?;

        if format_chirho == OutputFormatChirho::RawChirho {
            return Ok(raw_chirho);
        }

        // Get the source markup type
        let markup_chirho = self.config_chirho.source_type_chirho()
            .map(|s_chirho| match s_chirho {
                "OSIS" => MarkupChirho::OsisChirho,
                "ThML" => MarkupChirho::ThmlChirho,
                "GBF" => MarkupChirho::GbfChirho,
                "HTML" => MarkupChirho::HtmlChirho,
                "Plain" => MarkupChirho::PlainChirho,
                _ => MarkupChirho::UnknownChirho,
            })
            .unwrap_or(MarkupChirho::UnknownChirho);

        // Create the appropriate filter
        let filter_chirho: Box<dyn FilterChirho> = match (markup_chirho, format_chirho) {
            (MarkupChirho::OsisChirho, OutputFormatChirho::HtmlChirho) => {
                Box::new(OsisToHtmlFilterChirho::with_options_chirho(options_chirho.clone()))
            }
            (MarkupChirho::OsisChirho, OutputFormatChirho::PlainChirho) => {
                Box::new(OsisToPlainFilterChirho::with_options_chirho(options_chirho.clone()))
            }
            (MarkupChirho::ThmlChirho, OutputFormatChirho::HtmlChirho) => {
                Box::new(ThmlToHtmlFilterChirho::with_options_chirho(options_chirho.clone()))
            }
            (MarkupChirho::ThmlChirho, OutputFormatChirho::PlainChirho) => {
                Box::new(ThmlToPlainFilterChirho::with_options_chirho(options_chirho.clone()))
            }
            (MarkupChirho::GbfChirho, OutputFormatChirho::HtmlChirho) => {
                Box::new(GbfToHtmlFilterChirho::with_options_chirho(options_chirho.clone()))
            }
            (MarkupChirho::GbfChirho, OutputFormatChirho::PlainChirho) => {
                Box::new(GbfToPlainFilterChirho::with_options_chirho(options_chirho.clone()))
            }
            _ => {
                // Default: strip all tags
                Box::new(StripFilterChirho)
            }
        };

        filter_chirho.process_chirho(&raw_chirho)
    }

    /// Get the markup type from config.
    pub fn markup_type_chirho(&self) -> MarkupChirho {
        self.config_chirho.source_type_chirho()
            .map(|s_chirho| match s_chirho {
                "OSIS" => MarkupChirho::OsisChirho,
                "ThML" => MarkupChirho::ThmlChirho,
                "GBF" => MarkupChirho::GbfChirho,
                "HTML" => MarkupChirho::HtmlChirho,
                "Plain" => MarkupChirho::PlainChirho,
                _ => MarkupChirho::UnknownChirho,
            })
            .unwrap_or(MarkupChirho::UnknownChirho)
    }

    /// Read all verses for a chapter in batch (more efficient than read_entry_chirho per verse).
    ///
    /// This creates a single module driver instance and reuses it for all verses,
    /// which is significantly faster than calling read_entry_chirho for each verse.
    ///
    /// # Arguments
    /// * `book_chirho` - Book name (e.g., "Genesis", "John")
    /// * `chapter_chirho` - Chapter number (1-based)
    /// * `max_verses_chirho` - Maximum number of verses to read (safety limit)
    ///
    /// # Returns
    /// Vector of (verse_number, text) tuples
    pub fn read_chapter_batch_chirho(
        &self,
        book_chirho: &str,
        chapter_chirho: u32,
        max_verses_chirho: u32,
    ) -> ResultChirho<Vec<(u32, String)>> {
        match self.driver_type_chirho {
            ModuleDriverTypeChirho::RawTextChirho => {
                let mut module_chirho = RawTextChirho::new_chirho(
                    &self.data_path_chirho,
                    self.config_chirho.clone(),
                )?;
                Self::read_chapter_from_bible_module_chirho(
                    &mut module_chirho,
                    book_chirho,
                    chapter_chirho,
                    max_verses_chirho,
                )
            }
            ModuleDriverTypeChirho::RawText4Chirho => {
                let mut module_chirho = RawText4Chirho::new_chirho(
                    &self.data_path_chirho,
                    self.config_chirho.clone(),
                )?;
                Self::read_chapter_from_bible_module_chirho(
                    &mut module_chirho,
                    book_chirho,
                    chapter_chirho,
                    max_verses_chirho,
                )
            }
            ModuleDriverTypeChirho::ZTextChirho => {
                let mut module_chirho = ZTextChirho::new_chirho(
                    &self.data_path_chirho,
                    self.config_chirho.clone(),
                )?;
                Self::read_chapter_from_bible_module_chirho(
                    &mut module_chirho,
                    book_chirho,
                    chapter_chirho,
                    max_verses_chirho,
                )
            }
            ModuleDriverTypeChirho::ZText4Chirho => {
                let mut module_chirho = ZText4Chirho::new_chirho(
                    &self.data_path_chirho,
                    self.config_chirho.clone(),
                )?;
                Self::read_chapter_from_bible_module_chirho(
                    &mut module_chirho,
                    book_chirho,
                    chapter_chirho,
                    max_verses_chirho,
                )
            }
            _ => {
                // Fall back to single verse reading for non-Bible modules
                let mut verses_chirho = Vec::new();
                for verse_num_chirho in 1..=max_verses_chirho {
                    let ref_str_chirho = format!("{} {}:{}", book_chirho, chapter_chirho, verse_num_chirho);
                    match self.read_entry_chirho(&ref_str_chirho) {
                        Ok(text_chirho) if !text_chirho.trim().is_empty() => {
                            verses_chirho.push((verse_num_chirho, text_chirho));
                        }
                        _ => {
                            // Stop if we hit empty verses
                            if verse_num_chirho > 1 {
                                break;
                            }
                        }
                    }
                }
                Ok(verses_chirho)
            }
        }
    }

    /// Helper function to read chapter from a Bible module using SwModuleChirho trait.
    fn read_chapter_from_bible_module_chirho(
        module_chirho: &mut dyn SwModuleChirho,
        book_chirho: &str,
        chapter_chirho: u32,
        max_verses_chirho: u32,
    ) -> ResultChirho<Vec<(u32, String)>> {
        let mut verses_chirho = Vec::new();
        let mut consecutive_empty_chirho = 0;
        let mut last_text_chirho: Option<String> = None;

        for verse_num_chirho in 1..=max_verses_chirho {
            let ref_str_chirho = format!("{} {}:{}", book_chirho, chapter_chirho, verse_num_chirho);

            // Parse and set key
            let key_chirho = crate::keys_chirho::verse_key_chirho::VerseKeyChirho::from_str_chirho(&ref_str_chirho)?;
            module_chirho.set_key_chirho(&key_chirho)?;

            match module_chirho.get_raw_entry_chirho() {
                Ok(text_chirho) if !text_chirho.trim().is_empty() => {
                    // Check for duplicate content (indicates we've gone past valid verses)
                    if let Some(ref last_chirho) = last_text_chirho {
                        if text_chirho.trim() == last_chirho.trim() {
                            consecutive_empty_chirho += 1;
                            if consecutive_empty_chirho >= 2 {
                                break;
                            }
                            continue;
                        }
                    }
                    consecutive_empty_chirho = 0;
                    last_text_chirho = Some(text_chirho.clone());
                    verses_chirho.push((verse_num_chirho, text_chirho));
                }
                _ => {
                    consecutive_empty_chirho += 1;
                    if consecutive_empty_chirho >= 2 || verse_num_chirho > 1 {
                        break;
                    }
                }
            }
        }

        Ok(verses_chirho)
    }

    /// Get a genbook wrapper for this module if it's a general book.
    /// Returns None if this is not a general book module.
    /// Supports both RawGenBook and zGenBook module types.
    pub fn as_genbook_chirho(&self) -> Option<GenBookWrapperChirho> {
        // For genbook modules, DataPath typically ends with the basename
        // e.g., "./modules/genbook/rawgenbook/josephus/josephus"
        // We need: directory = parent of data_path, basename = file_stem of data_path
        let basename_chirho = self.data_path_chirho
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(&self.name_chirho);

        // The directory is the parent of data_path_chirho
        let dir_path_chirho = self.data_path_chirho
            .parent()
            .unwrap_or(&self.data_path_chirho);

        match self.driver_type_chirho {
            ModuleDriverTypeChirho::RawGenBookChirho => {
                RawGenBookChirho::open_chirho(dir_path_chirho, basename_chirho)
                    .ok()
                    .map(GenBookWrapperChirho::RawChirho)
            }
            ModuleDriverTypeChirho::ZGenBookChirho => {
                // Get compression type from config, default to Zip
                let compression_type_chirho = match self.config_chirho.compress_type_chirho() {
                    Some("ZIP") | Some("Zip") | Some("zip") => crate::CompressionTypeChirho::ZipChirho,
                    Some("LZSS") | Some("lzss") => crate::CompressionTypeChirho::LzssChirho,
                    Some("BZIP2") | Some("bzip2") | Some("BZ2") | Some("bz2") => crate::CompressionTypeChirho::Bzip2Chirho,
                    Some("XZ") | Some("xz") | Some("LZMA") | Some("lzma") => crate::CompressionTypeChirho::XzChirho,
                    _ => crate::CompressionTypeChirho::ZipChirho, // Default to Zip
                };
                ZGenBookChirho::open_chirho(dir_path_chirho, basename_chirho, compression_type_chirho)
                    .ok()
                    .map(GenBookWrapperChirho::CompressedChirho)
            }
            _ => None,
        }
    }
}

/// Wrapper enum for both RawGenBook and ZGenBook types.
/// Provides a unified interface for accessing general book content.
pub enum GenBookWrapperChirho {
    /// Uncompressed RawGenBook module.
    RawChirho(RawGenBookChirho),
    /// Compressed zGenBook module.
    CompressedChirho(ZGenBookChirho),
}

impl GenBookWrapperChirho {
    /// Get root-level keys from the general book.
    pub fn get_root_keys_chirho(&self) -> Vec<String> {
        match self {
            Self::RawChirho(gb_chirho) => gb_chirho.get_root_keys_chirho(),
            Self::CompressedChirho(gb_chirho) => gb_chirho.get_root_keys_chirho(),
        }
    }

    /// Get children of a specific key.
    pub fn get_children_chirho(&self, key_chirho: &str) -> Vec<String> {
        match self {
            Self::RawChirho(gb_chirho) => gb_chirho.get_children_chirho(key_chirho),
            Self::CompressedChirho(gb_chirho) => gb_chirho.get_children_chirho(key_chirho),
        }
    }

    /// Read an entry by key.
    pub fn read_entry_chirho(&mut self, key_chirho: &str) -> ResultChirho<Option<String>> {
        match self {
            Self::RawChirho(gb_chirho) => gb_chirho.read_entry_chirho(key_chirho),
            Self::CompressedChirho(gb_chirho) => gb_chirho.read_entry_chirho(key_chirho),
        }
    }
}

/// Load a module from a base path and configuration.
pub fn load_module_chirho(
    base_path_chirho: &Path,
    config_chirho: &ModuleConfigChirho,
) -> ResultChirho<LoadedModuleChirho> {
    let driver_str_chirho = config_chirho.module_driver_chirho()
        .ok_or_else(|| ErrorChirho::invalid_config_chirho("Missing ModDrv"))?;

    let driver_type_chirho = ModuleDriverTypeChirho::from_str_chirho(driver_str_chirho);

    if driver_type_chirho == ModuleDriverTypeChirho::UnknownChirho {
        return Err(ErrorChirho::invalid_config_chirho(format!(
            "Unknown module driver: {}",
            driver_str_chirho
        )));
    }

    let data_path_chirho = config_chirho.data_path_chirho()
        .ok_or_else(|| ErrorChirho::invalid_config_chirho("Missing DataPath"))?;

    // Strip leading "./" from data path if present
    let clean_data_path_chirho = data_path_chirho.strip_prefix("./").unwrap_or(data_path_chirho);

    let full_path_chirho = base_path_chirho.join(clean_data_path_chirho);

    if !full_path_chirho.exists() {
        return Err(ErrorChirho::InvalidModulePathChirho {
            path_chirho: full_path_chirho,
        });
    }

    Ok(LoadedModuleChirho {
        name_chirho: config_chirho.name_chirho.clone(),
        config_chirho: config_chirho.clone(),
        data_path_chirho: full_path_chirho,
        driver_type_chirho,
    })
}

/// Module creation options.
#[derive(Debug, Clone)]
pub struct CreateModuleOptionsChirho {
    /// Module driver type.
    pub driver_type_chirho: ModuleDriverTypeChirho,
    /// Module name.
    pub name_chirho: String,
    /// Module description.
    pub description_chirho: Option<String>,
    /// Module language (ISO code).
    pub language_chirho: Option<String>,
    /// Versification system (for Bible modules).
    pub versification_chirho: Option<String>,
    /// Source markup type.
    pub source_type_chirho: Option<String>,
}

impl CreateModuleOptionsChirho {
    /// Create options for a new module.
    pub fn new_chirho(name_chirho: &str, driver_type_chirho: ModuleDriverTypeChirho) -> Self {
        Self {
            driver_type_chirho,
            name_chirho: name_chirho.to_string(),
            description_chirho: None,
            language_chirho: None,
            versification_chirho: None,
            source_type_chirho: None,
        }
    }

    /// Set the module description.
    pub fn with_description_chirho(mut self, desc_chirho: &str) -> Self {
        self.description_chirho = Some(desc_chirho.to_string());
        self
    }

    /// Set the module language.
    pub fn with_language_chirho(mut self, lang_chirho: &str) -> Self {
        self.language_chirho = Some(lang_chirho.to_string());
        self
    }

    /// Set the versification system.
    pub fn with_versification_chirho(mut self, v11n_chirho: &str) -> Self {
        self.versification_chirho = Some(v11n_chirho.to_string());
        self
    }

    /// Set the source type.
    pub fn with_source_type_chirho(mut self, source_chirho: &str) -> Self {
        self.source_type_chirho = Some(source_chirho.to_string());
        self
    }
}

/// Create a new SWORD module at the specified path.
///
/// This creates the necessary data files and configuration for a new module.
/// The returned LoadedModuleChirho can be used to write entries to the module.
///
/// # Arguments
/// * `base_path_chirho` - Base path for the module data files
/// * `options_chirho` - Module creation options
///
/// # Returns
/// A LoadedModuleChirho that can be used to write entries
pub fn create_module_chirho(
    base_path_chirho: &Path,
    options_chirho: &CreateModuleOptionsChirho,
) -> ResultChirho<LoadedModuleChirho> {
    use crate::storage_chirho::{RawVerseChirho, RawVerse4Chirho, RawStrChirho};
    use crate::modules_chirho::RawGenBookChirho;

    // Ensure base directory exists
    std::fs::create_dir_all(base_path_chirho).map_err(ErrorChirho::IoChirho)?;

    // Create the storage files based on driver type
    match options_chirho.driver_type_chirho {
        ModuleDriverTypeChirho::RawTextChirho | ModuleDriverTypeChirho::RawComChirho => {
            RawVerseChirho::create_chirho(base_path_chirho)?;
        }
        ModuleDriverTypeChirho::RawText4Chirho | ModuleDriverTypeChirho::RawCom4Chirho => {
            RawVerse4Chirho::create_chirho(base_path_chirho)?;
        }
        ModuleDriverTypeChirho::RawLdChirho | ModuleDriverTypeChirho::RawLd4Chirho => {
            let basename_chirho = base_path_chirho.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("module");
            RawStrChirho::create_chirho(base_path_chirho, basename_chirho)?;
        }
        ModuleDriverTypeChirho::RawGenBookChirho => {
            let basename_chirho = base_path_chirho.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("module");
            RawGenBookChirho::create_chirho(base_path_chirho, basename_chirho)?;
        }
        ModuleDriverTypeChirho::ZTextChirho
        | ModuleDriverTypeChirho::ZText4Chirho
        | ModuleDriverTypeChirho::ZComChirho
        | ModuleDriverTypeChirho::ZCom4Chirho
        | ModuleDriverTypeChirho::ZLdChirho
        | ModuleDriverTypeChirho::ZLd4Chirho
        | ModuleDriverTypeChirho::ZGenBookChirho => {
            return Err(ErrorChirho::generic_chirho(
                "Creating compressed modules directly is not supported. Create a raw module and use mod2zmod to compress.".to_string()
            ));
        }
        ModuleDriverTypeChirho::UnknownChirho => {
            return Err(ErrorChirho::invalid_config_chirho("Unknown driver type"));
        }
    }

    // Build module configuration
    let mut config_chirho = ModuleConfigChirho::new_chirho(options_chirho.name_chirho.clone());

    let driver_str_chirho = match options_chirho.driver_type_chirho {
        ModuleDriverTypeChirho::RawTextChirho => "RawText",
        ModuleDriverTypeChirho::RawText4Chirho => "RawText4",
        ModuleDriverTypeChirho::RawComChirho => "RawCom",
        ModuleDriverTypeChirho::RawCom4Chirho => "RawCom4",
        ModuleDriverTypeChirho::RawLdChirho => "RawLD",
        ModuleDriverTypeChirho::RawLd4Chirho => "RawLD4",
        ModuleDriverTypeChirho::RawGenBookChirho => "RawGenBook",
        _ => "RawText",
    };

    config_chirho.set_chirho("ModDrv", driver_str_chirho);
    config_chirho.set_chirho("DataPath", base_path_chirho.to_str().unwrap_or("."));

    if let Some(ref desc_chirho) = options_chirho.description_chirho {
        config_chirho.set_chirho("Description", desc_chirho);
    }

    if let Some(ref lang_chirho) = options_chirho.language_chirho {
        config_chirho.set_chirho("Lang", lang_chirho);
    }

    if let Some(ref v11n_chirho) = options_chirho.versification_chirho {
        config_chirho.set_chirho("Versification", v11n_chirho);
    }

    if let Some(ref source_chirho) = options_chirho.source_type_chirho {
        config_chirho.set_chirho("SourceType", source_chirho);
    }

    Ok(LoadedModuleChirho {
        name_chirho: options_chirho.name_chirho.clone(),
        config_chirho,
        data_path_chirho: base_path_chirho.to_path_buf(),
        driver_type_chirho: options_chirho.driver_type_chirho,
    })
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_driver_type_parsing_chirho() {
        assert_eq!(
            ModuleDriverTypeChirho::from_str_chirho("RawText"),
            ModuleDriverTypeChirho::RawTextChirho
        );
        assert_eq!(
            ModuleDriverTypeChirho::from_str_chirho("zText"),
            ModuleDriverTypeChirho::ZTextChirho
        );
        assert_eq!(
            ModuleDriverTypeChirho::from_str_chirho("RawCom"),
            ModuleDriverTypeChirho::RawComChirho
        );
        assert_eq!(
            ModuleDriverTypeChirho::from_str_chirho("RawLD"),
            ModuleDriverTypeChirho::RawLdChirho
        );
        assert_eq!(
            ModuleDriverTypeChirho::from_str_chirho("Unknown"),
            ModuleDriverTypeChirho::UnknownChirho
        );
    }

    #[test]
    fn test_driver_type_categories_chirho() {
        assert!(ModuleDriverTypeChirho::RawTextChirho.is_bible_chirho());
        assert!(ModuleDriverTypeChirho::ZTextChirho.is_bible_chirho());
        assert!(!ModuleDriverTypeChirho::RawComChirho.is_bible_chirho());

        assert!(ModuleDriverTypeChirho::RawComChirho.is_commentary_chirho());
        assert!(ModuleDriverTypeChirho::ZComChirho.is_commentary_chirho());

        assert!(ModuleDriverTypeChirho::RawLdChirho.is_lexicon_chirho());
        assert!(ModuleDriverTypeChirho::ZLdChirho.is_lexicon_chirho());

        assert!(ModuleDriverTypeChirho::ZTextChirho.is_compressed_chirho());
        assert!(!ModuleDriverTypeChirho::RawTextChirho.is_compressed_chirho());
    }
}
