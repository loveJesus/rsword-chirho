// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Error types for the rsword_chirho library.
//!
//! All operations that can fail return [`ResultChirho<T>`], which is an alias for
//! `Result<T, ErrorChirho>`.
//!
//! ## Error Handling
//!
//! ```rust
//! use rsword_chirho::{VerseKeyChirho, ErrorChirho, ResultChirho};
//!
//! fn lookup_verse_chirho(reference_chirho: &str) -> ResultChirho<String> {
//!     let key_chirho = VerseKeyChirho::from_str_chirho(reference_chirho)?;
//!     Ok(key_chirho.to_string())
//! }
//!
//! // Handle specific errors
//! match lookup_verse_chirho("Invalid 99:99") {
//!     Ok(text_chirho) => println!("{}", text_chirho),
//!     Err(ErrorChirho::InvalidVerseReferenceChirho { reference_chirho }) => {
//!         eprintln!("Bad reference: {}", reference_chirho);
//!     }
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! ```
//!
//! ## Common Errors
//!
//! | Error | Description |
//! |-------|-------------|
//! | [`ErrorChirho::ModuleNotFoundChirho`] | Module not installed or path invalid |
//! | [`ErrorChirho::InvalidVerseReferenceChirho`] | Cannot parse verse reference |
//! | [`ErrorChirho::InvalidBookNameChirho`] | Book name not recognized |
//! | [`ErrorChirho::VerseOutOfBoundsChirho`] | Verse doesn't exist in versification |
//! | [`ErrorChirho::CipherKeyRequiredChirho`] | Encrypted module needs unlock key |
//! | [`ErrorChirho::NetworkChirho`] | Download or connection failed |
//!
//! ## Creating Errors
//!
//! Use convenience constructors for common error types:
//!
//! ```rust
//! use rsword_chirho::ErrorChirho;
//!
//! // Create specific errors
//! let err_chirho = ErrorChirho::module_not_found_chirho("KJV");
//! let err_chirho = ErrorChirho::invalid_verse_reference_chirho("BadRef");
//! let err_chirho = ErrorChirho::generic_chirho("Something went wrong");
//! ```

use std::io;
use std::path::PathBuf;
use thiserror::Error;

/// Result type alias for rsword_chirho operations.
pub type ResultChirho<T> = Result<T, ErrorChirho>;

/// Main error type for rsword_chirho.
#[derive(Error, Debug)]
pub enum ErrorChirho {
    /// I/O error
    #[error("I/O error: {0}")]
    IoChirho(#[from] io::Error),

    /// Module not found
    #[error("Module not found: {module_name_chirho}")]
    ModuleNotFoundChirho {
        /// Name of the module that was not found
        module_name_chirho: String,
    },

    /// Invalid module path
    #[error("Invalid module path: {path_chirho}")]
    InvalidModulePathChirho {
        /// The invalid path
        path_chirho: PathBuf,
    },

    /// Invalid verse reference
    #[error("Invalid verse reference: {reference_chirho}")]
    InvalidVerseReferenceChirho {
        /// The invalid reference string
        reference_chirho: String,
    },

    /// Invalid book name
    #[error("Invalid book name: {book_name_chirho}")]
    InvalidBookNameChirho {
        /// The invalid book name
        book_name_chirho: String,
    },

    /// Verse out of bounds
    #[error("Verse out of bounds: {book_chirho} {chapter_chirho}:{verse_chirho}")]
    VerseOutOfBoundsChirho {
        /// Book name
        book_chirho: String,
        /// Chapter number
        chapter_chirho: u32,
        /// Verse number
        verse_chirho: u32,
    },

    /// Chapter out of bounds
    #[error("Chapter out of bounds: {book_chirho} {chapter_chirho} (max: {max_chapter_chirho})")]
    ChapterOutOfBoundsChirho {
        /// Book name
        book_chirho: String,
        /// Requested chapter
        chapter_chirho: u32,
        /// Maximum valid chapter
        max_chapter_chirho: u32,
    },

    /// Invalid module configuration
    #[error("Invalid module configuration: {message_chirho}")]
    InvalidConfigChirho {
        /// Description of the configuration error
        message_chirho: String,
    },

    /// Missing configuration key
    #[error("Missing configuration key: {key_chirho}")]
    MissingConfigKeyChirho {
        /// The missing key
        key_chirho: String,
    },

    /// Unsupported module type
    #[error("Unsupported module type: {module_type_chirho}")]
    UnsupportedModuleTypeChirho {
        /// The unsupported module type
        module_type_chirho: String,
    },

    /// Unsupported compression type
    #[error("Unsupported compression: {compression_type_chirho}")]
    UnsupportedCompressionChirho {
        /// The unsupported compression type
        compression_type_chirho: String,
    },

    /// Decompression error
    #[error("Decompression error: {message_chirho}")]
    DecompressionChirho {
        /// Description of the decompression error
        message_chirho: String,
    },

    /// Compression error
    #[error("Compression error: {message_chirho}")]
    CompressionChirho {
        /// Description of the compression error
        message_chirho: String,
    },

    /// Invalid index format
    #[error("Invalid index format: expected {expected_chirho} bytes, got {actual_chirho}")]
    InvalidIndexFormatChirho {
        /// Expected number of bytes
        expected_chirho: usize,
        /// Actual number of bytes
        actual_chirho: usize,
    },

    /// Cipher key required but not provided
    #[error("Cipher key required for module: {module_name_chirho}")]
    CipherKeyRequiredChirho {
        /// Module that requires a cipher key
        module_name_chirho: String,
    },

    /// Invalid cipher key
    #[error("Invalid cipher key for module: {module_name_chirho}")]
    InvalidCipherKeyChirho {
        /// Module with invalid cipher key
        module_name_chirho: String,
    },

    /// Network error
    #[error("Network error: {message_chirho}")]
    NetworkChirho {
        /// Description of the network error
        message_chirho: String,
    },

    /// Download error
    #[error("Download error: {url_chirho}: {message_chirho}")]
    DownloadChirho {
        /// URL that failed to download
        url_chirho: String,
        /// Error message
        message_chirho: String,
    },

    /// Installation error
    #[error("Installation error: {message_chirho}")]
    InstallationChirho {
        /// Description of the installation error
        message_chirho: String,
    },

    /// XML parsing error
    #[error("XML parsing error: {message_chirho}")]
    XmlParseChirho {
        /// Description of the XML error
        message_chirho: String,
    },

    /// General parsing error
    #[error("Parse error: {message_chirho}")]
    ParseChirho {
        /// Description of the parse error
        message_chirho: String,
    },

    /// Encoding error
    #[error("Encoding error: {message_chirho}")]
    EncodingChirho {
        /// Description of the encoding error
        message_chirho: String,
    },

    /// Search error
    #[error("Search error: {message_chirho}")]
    SearchChirho {
        /// Description of the search error
        message_chirho: String,
    },

    /// Invalid versification system
    #[error("Invalid versification system: {v11n_chirho}")]
    InvalidVersificationChirho {
        /// The invalid versification name
        v11n_chirho: String,
    },

    /// Module is read-only
    #[error("Module is read-only: {module_name_chirho}")]
    ReadOnlyModuleChirho {
        /// Module that is read-only
        module_name_chirho: String,
    },

    /// Entry not found
    #[error("Entry not found: {key_chirho}")]
    EntryNotFoundChirho {
        /// Key that was not found
        key_chirho: String,
    },

    /// UTF-8 decode error
    #[error("UTF-8 decode error: {0}")]
    Utf8Chirho(#[from] std::string::FromUtf8Error),

    /// UTF-8 str error
    #[error("UTF-8 str error: {0}")]
    Utf8StrChirho(#[from] std::str::Utf8Error),

    /// Integer parse error
    #[error("Integer parse error: {0}")]
    ParseIntChirho(#[from] std::num::ParseIntError),

    /// Generic error with message
    #[error("{0}")]
    GenericChirho(String),
}

impl ErrorChirho {
    /// Create a new generic error with a message.
    pub fn generic_chirho<S: Into<String>>(message_chirho: S) -> Self {
        Self::GenericChirho(message_chirho.into())
    }

    /// Create a module not found error.
    pub fn module_not_found_chirho<S: Into<String>>(module_name_chirho: S) -> Self {
        Self::ModuleNotFoundChirho {
            module_name_chirho: module_name_chirho.into(),
        }
    }

    /// Create an invalid verse reference error.
    pub fn invalid_verse_reference_chirho<S: Into<String>>(reference_chirho: S) -> Self {
        Self::InvalidVerseReferenceChirho {
            reference_chirho: reference_chirho.into(),
        }
    }

    /// Create an invalid config error.
    pub fn invalid_config_chirho<S: Into<String>>(message_chirho: S) -> Self {
        Self::InvalidConfigChirho {
            message_chirho: message_chirho.into(),
        }
    }

    /// Create a missing config key error.
    pub fn missing_config_key_chirho<S: Into<String>>(key_chirho: S) -> Self {
        Self::MissingConfigKeyChirho {
            key_chirho: key_chirho.into(),
        }
    }

    /// Create an XML parse error.
    pub fn xml_parse_chirho<S: Into<String>>(message_chirho: S) -> Self {
        Self::XmlParseChirho {
            message_chirho: message_chirho.into(),
        }
    }

    /// Create a network error.
    pub fn network_chirho<S: Into<String>>(message_chirho: S) -> Self {
        Self::NetworkChirho {
            message_chirho: message_chirho.into(),
        }
    }

    /// Create a search error.
    pub fn search_chirho<S: Into<String>>(message_chirho: S) -> Self {
        Self::SearchChirho {
            message_chirho: message_chirho.into(),
        }
    }

    /// Create a verse out of bounds error from a reference string.
    ///
    /// This is a convenience method for when you have a reference string
    /// like "Genesis 1:100" rather than parsed book/chapter/verse values.
    pub fn verse_out_of_bounds_chirho<S: AsRef<str>>(reference_chirho: S) -> Self {
        // Try to parse the reference, or use defaults if parsing fails
        let ref_str_chirho = reference_chirho.as_ref();
        Self::VerseOutOfBoundsChirho {
            book_chirho: ref_str_chirho.to_string(),
            chapter_chirho: 0,
            verse_chirho: 0,
        }
    }

    /// Create a verse out of bounds error with explicit book/chapter/verse.
    pub fn verse_out_of_bounds_explicit_chirho<S: Into<String>>(
        book_chirho: S,
        chapter_chirho: u32,
        verse_chirho: u32,
    ) -> Self {
        Self::VerseOutOfBoundsChirho {
            book_chirho: book_chirho.into(),
            chapter_chirho,
            verse_chirho,
        }
    }

    /// Create a parse error.
    pub fn parse_chirho<S: Into<String>>(message_chirho: S) -> Self {
        Self::ParseChirho {
            message_chirho: message_chirho.into(),
        }
    }

    /// Create a key not found error.
    pub fn key_not_found_chirho<S: Into<String>>(key_chirho: S) -> Self {
        Self::EntryNotFoundChirho {
            key_chirho: key_chirho.into(),
        }
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_error_display_chirho() {
        let error_chirho = ErrorChirho::module_not_found_chirho("KJV");
        assert_eq!(error_chirho.to_string(), "Module not found: KJV");
    }

    #[test]
    fn test_generic_error_chirho() {
        let error_chirho = ErrorChirho::generic_chirho("Something went wrong");
        assert_eq!(error_chirho.to_string(), "Something went wrong");
    }

    #[test]
    fn test_verse_out_of_bounds_chirho() {
        let error_chirho = ErrorChirho::VerseOutOfBoundsChirho {
            book_chirho: "Genesis".to_string(),
            chapter_chirho: 1,
            verse_chirho: 100,
        };
        assert_eq!(error_chirho.to_string(), "Verse out of bounds: Genesis 1:100");
    }
}
