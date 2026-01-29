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
    RawTextChirho, ZTextChirho,
    RawComChirho, ZComChirho,
    RawLdChirho, ZLdChirho,
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
            ModuleDriverTypeChirho::RawTextChirho | ModuleDriverTypeChirho::RawText4Chirho => {
                let mut module_chirho = RawTextChirho::new_chirho(
                    &self.data_path_chirho,
                    self.config_chirho.clone(),
                )?;
                module_chirho.set_key_text_chirho(key_chirho)?;
                module_chirho.get_raw_entry_chirho()
            }
            ModuleDriverTypeChirho::ZTextChirho | ModuleDriverTypeChirho::ZText4Chirho => {
                let mut module_chirho = ZTextChirho::new_chirho(
                    &self.data_path_chirho,
                    self.config_chirho.clone(),
                )?;
                module_chirho.set_key_text_chirho(key_chirho)?;
                module_chirho.get_raw_entry_chirho()
            }
            ModuleDriverTypeChirho::RawComChirho | ModuleDriverTypeChirho::RawCom4Chirho => {
                let mut module_chirho = RawComChirho::new_chirho(
                    &self.data_path_chirho,
                    self.config_chirho.clone(),
                )?;
                module_chirho.set_key_text_chirho(key_chirho)?;
                module_chirho.get_raw_entry_chirho()
            }
            ModuleDriverTypeChirho::ZComChirho | ModuleDriverTypeChirho::ZCom4Chirho => {
                let mut module_chirho = ZComChirho::new_chirho(
                    &self.data_path_chirho,
                    self.config_chirho.clone(),
                )?;
                module_chirho.set_key_text_chirho(key_chirho)?;
                module_chirho.get_raw_entry_chirho()
            }
            ModuleDriverTypeChirho::RawLdChirho | ModuleDriverTypeChirho::RawLd4Chirho => {
                let mut module_chirho = RawLdChirho::new_chirho(
                    &self.data_path_chirho,
                    self.config_chirho.clone(),
                )?;
                module_chirho.get_entry_chirho(key_chirho)
            }
            ModuleDriverTypeChirho::ZLdChirho | ModuleDriverTypeChirho::ZLd4Chirho => {
                let mut module_chirho = ZLdChirho::new_chirho(
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
