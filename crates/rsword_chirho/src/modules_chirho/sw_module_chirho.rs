// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Base module trait and common functionality.
//!
//! The [`SwModuleChirho`] trait defines the interface for all SWORD modules.
//! Implementations include Bible texts, commentaries, lexicons, and general books.
//!
//! ## Module Interface
//!
//! All modules implement these core operations:
//!
//! - **Navigation**: Set position via key, increment/decrement
//! - **Reading**: Get raw text, rendered text, or stripped text
//! - **Metadata**: Name, description, language, encoding, markup type
//! - **Encryption**: Check encryption status, set cipher key
//!
//! ## Usage Pattern
//!
//! ```rust,ignore
//! use rsword_chirho::{SwMgrChirho, SwModuleChirho, VerseKeyChirho};
//!
//! let mgr_chirho = SwMgrChirho::with_system_paths_chirho().unwrap();
//!
//! if let Some(mut module_chirho) = mgr_chirho.get_module_driver_chirho("KJV").unwrap() {
//!     // Get module metadata
//!     println!("Name: {}", module_chirho.name_chirho());
//!     println!("Description: {}", module_chirho.description_chirho());
//!     println!("Language: {}", module_chirho.language_chirho());
//!
//!     // Navigate and read
//!     let key_chirho = VerseKeyChirho::from_str_chirho("John 1:1").unwrap();
//!     module_chirho.set_key_chirho(&key_chirho).unwrap();
//!
//!     // Raw text (with markup)
//!     let raw_chirho = module_chirho.get_raw_entry_chirho().unwrap();
//!
//!     // Rendered text (filters applied)
//!     let rendered_chirho = module_chirho.render_text_chirho().unwrap();
//!
//!     // Plain text (for search/indexing)
//!     let plain_chirho = module_chirho.strip_text_chirho().unwrap();
//! }
//! ```

use crate::config_chirho::ModuleConfigChirho;
use crate::error_chirho::ResultChirho;
use crate::keys_chirho::SwKeyChirho;
use crate::{DirectionChirho, EncodingChirho, MarkupChirho};

/// Base trait for all SWORD modules.
pub trait SwModuleChirho {
    /// Get the module name.
    fn name_chirho(&self) -> &str;

    /// Get the module description.
    fn description_chirho(&self) -> &str;

    /// Get the module type.
    fn module_type_chirho(&self) -> &str;

    /// Get the module language.
    fn language_chirho(&self) -> &str;

    /// Get the text direction.
    fn direction_chirho(&self) -> DirectionChirho;

    /// Get the text encoding.
    fn encoding_chirho(&self) -> EncodingChirho;

    /// Get the source markup.
    fn markup_chirho(&self) -> MarkupChirho;

    /// Get the current key.
    fn get_key_chirho(&self) -> &dyn SwKeyChirho;

    /// Set the current key.
    fn set_key_chirho(&mut self, key_chirho: &dyn SwKeyChirho) -> ResultChirho<()>;

    /// Get the raw entry text (without filters).
    fn get_raw_entry_chirho(&mut self) -> ResultChirho<String>;

    /// Get the rendered entry text (with filters applied).
    fn render_text_chirho(&mut self) -> ResultChirho<String> {
        // Default implementation just returns raw text
        self.get_raw_entry_chirho()
    }

    /// Get text with markup stripped (for search).
    fn strip_text_chirho(&mut self) -> ResultChirho<String> {
        // Default implementation just returns raw text
        self.get_raw_entry_chirho()
    }

    /// Check if the module is writable.
    fn is_writable_chirho(&self) -> bool {
        false
    }

    /// Check if the module is encrypted.
    fn is_encrypted_chirho(&self) -> bool;

    /// Set the cipher key for encrypted modules.
    fn set_cipher_key_chirho(&mut self, _key_chirho: &str) -> ResultChirho<()> {
        Ok(())
    }

    /// Check if module has an entry at current key.
    fn has_entry_chirho(&self) -> bool {
        true
    }

    /// Get a config value.
    fn get_config_entry_chirho(&self, key_chirho: &str) -> Option<&str>;

    /// Increment position.
    fn increment_chirho(&mut self, steps_chirho: i32);

    /// Decrement position.
    fn decrement_chirho(&mut self, steps_chirho: i32);

    /// Pop and return error status.
    fn pop_error_chirho(&mut self) -> bool;
}

/// Module wrapper with common functionality.
pub struct ModuleBaseChirho {
    /// Module configuration.
    config_chirho: ModuleConfigChirho,
    /// Current key text (reserved for future use).
    #[allow(dead_code)]
    key_text_chirho: String,
    /// Error flag.
    error_chirho: bool,
}

impl ModuleBaseChirho {
    /// Create a new module base.
    pub fn new_chirho(config_chirho: ModuleConfigChirho) -> Self {
        Self {
            config_chirho,
            key_text_chirho: String::new(),
            error_chirho: false,
        }
    }

    /// Get the config.
    pub fn config_chirho(&self) -> &ModuleConfigChirho {
        &self.config_chirho
    }

    /// Get the module name.
    pub fn name_chirho(&self) -> &str {
        &self.config_chirho.name_chirho
    }

    /// Get the module description.
    pub fn description_chirho(&self) -> &str {
        self.config_chirho.description_chirho().unwrap_or("")
    }

    /// Get the module language.
    pub fn language_chirho(&self) -> &str {
        self.config_chirho.language_chirho().unwrap_or("en")
    }

    /// Get the text direction.
    pub fn direction_chirho(&self) -> DirectionChirho {
        match self.config_chirho.direction_chirho() {
            "RtoL" => DirectionChirho::RightToLeftChirho,
            "BiDi" => DirectionChirho::BidirectionalChirho,
            _ => DirectionChirho::LeftToRightChirho,
        }
    }

    /// Get the encoding.
    pub fn encoding_chirho(&self) -> EncodingChirho {
        match self.config_chirho.encoding_chirho() {
            "Latin-1" | "ISO-8859-1" => EncodingChirho::Latin1Chirho,
            "UTF-16" => EncodingChirho::Utf16Chirho,
            "SCSU" => EncodingChirho::ScsuChirho,
            _ => EncodingChirho::Utf8Chirho,
        }
    }

    /// Get the markup.
    pub fn markup_chirho(&self) -> MarkupChirho {
        match self.config_chirho.source_type_chirho() {
            Some("OSIS") => MarkupChirho::OsisChirho,
            Some("ThML") => MarkupChirho::ThmlChirho,
            Some("GBF") => MarkupChirho::GbfChirho,
            Some("TEI") => MarkupChirho::TeiChirho,
            Some("HTML") => MarkupChirho::HtmlChirho,
            Some("Plain") => MarkupChirho::PlainChirho,
            _ => MarkupChirho::UnknownChirho,
        }
    }

    /// Check if encrypted.
    pub fn is_encrypted_chirho(&self) -> bool {
        self.config_chirho.is_encrypted_chirho()
    }

    /// Get a config entry.
    pub fn get_config_entry_chirho(&self, key_chirho: &str) -> Option<&str> {
        self.config_chirho.get_chirho(key_chirho)
    }

    /// Set error state.
    pub fn set_error_chirho(&mut self, error_chirho: bool) {
        self.error_chirho = error_chirho;
    }

    /// Pop error state.
    pub fn pop_error_chirho(&mut self) -> bool {
        let e = self.error_chirho;
        self.error_chirho = false;
        e
    }
}
