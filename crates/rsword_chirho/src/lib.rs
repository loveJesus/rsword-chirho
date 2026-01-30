// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! # rsword_chirho
//!
//! Pure Rust port of the SWORD Bible software library.
//!
//! This crate provides full compatibility with SWORD module binary formats,
//! including reading and writing Bible texts, commentaries, lexicons, and
//! general books.

#![warn(missing_docs)]

pub mod error_chirho;
pub mod byte_order_chirho;
pub mod config_chirho;
pub mod keys_chirho;
pub mod versification_chirho;
pub mod storage_chirho;
pub mod compression_chirho;
pub mod cipher_chirho;
pub mod cache_chirho;
pub mod modules_chirho;
pub mod filters_chirho;
pub mod search_chirho;
pub mod manager_chirho;
pub mod transport_chirho;
pub mod import_chirho;
pub mod locale_chirho;
pub mod parallels_chirho;

// Re-exports
pub use error_chirho::{ErrorChirho, ResultChirho};
pub use keys_chirho::sw_key_chirho::SwKeyChirho;
pub use keys_chirho::verse_key_chirho::VerseKeyChirho;
pub use keys_chirho::list_key_chirho::ListKeyChirho;
pub use modules_chirho::sw_module_chirho::SwModuleChirho;
pub use modules_chirho::texts_chirho::RawTextChirho;
pub use manager_chirho::sw_mgr_chirho::SwMgrChirho;
pub use versification_chirho::{
    VersificationChirho, TestamentChirho, BookInfoChirho,
    kjv_chirho, catholic_chirho, lxx_chirho, synodal_chirho, get_versification_chirho,
};
pub use search_chirho::{
    SearchEngineChirho, SearchOptionsChirho, SearchTypeChirho,
    TantivySearchChirho, RegexSearchChirho,
};
pub use filters_chirho::{
    FilterChirho, FilterOptionsChirho, FilterChainChirho,
    OsisToHtmlFilterChirho, OsisToPlainFilterChirho,
    ThmlToHtmlFilterChirho, ThmlToPlainFilterChirho,
    GbfToHtmlFilterChirho, GbfToPlainFilterChirho,
};
pub use locale_chirho::{
    LocaleChirho, LocaleBookChirho, LocaleManagerChirho,
    english_locale_chirho, get_locale_chirho, DEFAULT_LOCALE_CHIRHO,
};
pub use cipher_chirho::{
    CipherChirho, CipherTypeChirho, SwCipherChirho,
    SapphireCipherChirho, XorCipherChirho, NullCipherChirho,
    create_cipher_chirho,
};
pub use parallels_chirho::{
    ParallelPassageChirho, ParallelSetChirho, ParallelTypeChirho,
    ParallelManagerChirho,
};

/// Library version
pub const VERSION_CHIRHO: &str = env!("CARGO_PKG_VERSION");

/// Default versification system
pub const DEFAULT_V11N_CHIRHO: &str = "KJV";

/// Module types
pub mod module_type_chirho {
    /// Biblical text module
    pub const BIBLICAL_TEXT_CHIRHO: &str = "Biblical Texts";
    /// Commentary module
    pub const COMMENTARY_CHIRHO: &str = "Commentaries";
    /// Lexicon/dictionary module
    pub const LEXICON_CHIRHO: &str = "Lexicons / Dictionaries";
    /// General book module
    pub const GENBOOK_CHIRHO: &str = "Generic Books";
    /// Daily devotional module
    pub const DAILY_DEVOTIONAL_CHIRHO: &str = "Daily Devotional";
    /// Glossary module
    pub const GLOSSARY_CHIRHO: &str = "Glossaries";
    /// Image/map module
    pub const IMAGES_CHIRHO: &str = "Images";
}

/// Text direction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DirectionChirho {
    /// Left to right (default)
    #[default]
    LeftToRightChirho,
    /// Right to left (Hebrew, Arabic)
    RightToLeftChirho,
    /// Bidirectional
    BidirectionalChirho,
}

/// Text encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EncodingChirho {
    /// Unknown encoding
    UnknownChirho,
    /// Latin-1 (ISO-8859-1)
    Latin1Chirho,
    /// UTF-8 (default)
    #[default]
    Utf8Chirho,
    /// SCSU (Standard Compression Scheme for Unicode)
    ScsuChirho,
    /// UTF-16
    Utf16Chirho,
}

/// Source markup format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MarkupChirho {
    /// Unknown markup
    #[default]
    UnknownChirho,
    /// Plain text
    PlainChirho,
    /// ThML (Theological Markup Language)
    ThmlChirho,
    /// GBF (General Bible Format)
    GbfChirho,
    /// HTML
    HtmlChirho,
    /// OSIS (Open Scripture Information Standard)
    OsisChirho,
    /// TEI (Text Encoding Initiative)
    TeiChirho,
}

/// Block types for compressed modules
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BlockTypeChirho {
    /// Each verse compressed separately (value: 2)
    VerseBlocksChirho = 2,
    /// Each chapter compressed together (value: 3)
    #[default]
    ChapterBlocksChirho = 3,
    /// Each book compressed together (value: 4)
    BookBlocksChirho = 4,
}

/// Compression types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CompressionTypeChirho {
    /// No compression
    #[default]
    NoneChirho,
    /// ZIP/zlib compression
    ZipChirho,
    /// LZSS compression (legacy)
    LzssChirho,
    /// bzip2 compression
    Bzip2Chirho,
    /// XZ/LZMA compression
    XzChirho,
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_version_chirho() {
        assert!(!VERSION_CHIRHO.is_empty());
    }

    #[test]
    fn test_block_type_values_chirho() {
        assert_eq!(BlockTypeChirho::VerseBlocksChirho as u8, 2);
        assert_eq!(BlockTypeChirho::ChapterBlocksChirho as u8, 3);
        assert_eq!(BlockTypeChirho::BookBlocksChirho as u8, 4);
    }
}

#[cfg(test)]
mod test_indices_chirho;

