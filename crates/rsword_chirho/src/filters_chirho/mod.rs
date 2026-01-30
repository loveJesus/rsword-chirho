// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Text filters for markup conversion.
//!
//! Filters transform module text between different formats (OSIS, ThML, GBF, etc.)
//! and apply options like Strong's numbers, footnotes, and morphology.
//!
//! ## Supported Formats
//!
//! | Source | To HTML | To Plain |
//! |--------|---------|----------|
//! | OSIS | [`OsisToHtmlFilterChirho`] | [`OsisToPlainFilterChirho`] |
//! | ThML | [`ThmlToHtmlFilterChirho`] | [`ThmlToPlainFilterChirho`] |
//! | GBF | [`GbfToHtmlFilterChirho`] | [`GbfToPlainFilterChirho`] |
//! | TEI | [`TeiToHtmlFilterChirho`] | [`TeiToPlainFilterChirho`] |
//!
//! ## Basic Usage
//!
//! ```rust
//! use rsword_chirho::filters_chirho::{FilterChirho, OsisToHtmlFilterChirho, OsisToPlainFilterChirho};
//!
//! // Create filters
//! let html_filter_chirho = OsisToHtmlFilterChirho::new_chirho();
//! let plain_filter_chirho = OsisToPlainFilterChirho::new_chirho();
//!
//! let osis_text_chirho = r#"<verse osisID="John.3.16">For God so loved the world</verse>"#;
//!
//! // Convert to HTML
//! let html_chirho = html_filter_chirho.process_chirho(osis_text_chirho).unwrap();
//!
//! // Convert to plain text
//! let plain_chirho = plain_filter_chirho.process_chirho(osis_text_chirho).unwrap();
//! ```
//!
//! ## Filter Options
//!
//! Control what markup features are included in output:
//!
//! ```rust
//! use rsword_chirho::filters_chirho::FilterOptionsChirho;
//!
//! // Enable all features
//! let all_chirho = FilterOptionsChirho::all_chirho();
//!
//! // Minimal output
//! let none_chirho = FilterOptionsChirho::none_chirho();
//!
//! // Custom options
//! let custom_chirho = FilterOptionsChirho {
//!     strongs_chirho: true,      // Strong's numbers
//!     footnotes_chirho: true,    // Footnotes
//!     red_letter_chirho: true,   // Words of Christ in red
//!     ..Default::default()
//! };
//! ```
//!
//! ## Filter Chains
//!
//! Apply multiple filters in sequence:
//!
//! ```rust
//! use rsword_chirho::filters_chirho::{FilterChainChirho, StripFilterChirho, PlainFilterChirho};
//!
//! let mut chain_chirho = FilterChainChirho::new_chirho();
//! chain_chirho.add_chirho(Box::new(StripFilterChirho));
//! chain_chirho.add_chirho(Box::new(PlainFilterChirho));
//!
//! let result_chirho = chain_chirho.process_chirho("<p>Text with <b>markup</b></p>").unwrap();
//! assert_eq!(result_chirho, "Text with markup");
//! ```
//!
//! ## HTML Safety
//!
//! Use [`escape_html_chirho`] to prevent XSS when rendering user content:
//!
//! ```rust
//! use rsword_chirho::filters_chirho::escape_html_chirho;
//!
//! let unsafe_chirho = "<script>alert('xss')</script>";
//! let safe_chirho = escape_html_chirho(unsafe_chirho);
//! assert_eq!(safe_chirho, "&lt;script&gt;alert(&#39;xss&#39;)&lt;/script&gt;");
//! ```

mod osis_chirho;
mod thml_chirho;
mod gbf_chirho;
mod tei_filter_chirho;
mod cipher_filter_chirho;
mod global_options_chirho;
mod strongs_filter_chirho;
mod footnotes_filter_chirho;
mod redletter_filter_chirho;

pub use osis_chirho::{OsisToHtmlFilterChirho, OsisToPlainFilterChirho};
pub use thml_chirho::{ThmlToHtmlFilterChirho, ThmlToPlainFilterChirho};
pub use gbf_chirho::{GbfToHtmlFilterChirho, GbfToPlainFilterChirho};
pub use tei_filter_chirho::{TeiToHtmlFilterChirho, TeiToPlainFilterChirho};
pub use cipher_filter_chirho::{CipherFilterChirho, CipherBytesFilterChirho};
pub use global_options_chirho::{GlobalOptionsChirho, GlobalOptionManagerChirho};
pub use strongs_filter_chirho::{StrongsStripFilterChirho, StrongsFormatFilterChirho, StrongsFormatChirho};
pub use footnotes_filter_chirho::{FootnotesStripFilterChirho, FootnotesFormatFilterChirho, FootnoteFormatChirho};
pub use redletter_filter_chirho::{RedLetterStripFilterChirho, RedLetterFormatFilterChirho};

use crate::error_chirho::ResultChirho;

/// Escape HTML special characters to prevent XSS.
///
/// This function escapes the five HTML special characters that could be used
/// for injection attacks: `<`, `>`, `&`, `"`, and `'`.
#[inline]
pub fn escape_html_chirho(text_chirho: &str) -> String {
    let mut result_chirho = String::with_capacity(text_chirho.len());
    for ch_chirho in text_chirho.chars() {
        match ch_chirho {
            '<' => result_chirho.push_str("&lt;"),
            '>' => result_chirho.push_str("&gt;"),
            '&' => result_chirho.push_str("&amp;"),
            '"' => result_chirho.push_str("&quot;"),
            '\'' => result_chirho.push_str("&#39;"),
            _ => result_chirho.push(ch_chirho),
        }
    }
    result_chirho
}

/// Escape HTML for attribute values (also escapes backticks and newlines).
#[inline]
pub fn escape_html_attr_chirho(text_chirho: &str) -> String {
    let mut result_chirho = String::with_capacity(text_chirho.len());
    for ch_chirho in text_chirho.chars() {
        match ch_chirho {
            '<' => result_chirho.push_str("&lt;"),
            '>' => result_chirho.push_str("&gt;"),
            '&' => result_chirho.push_str("&amp;"),
            '"' => result_chirho.push_str("&quot;"),
            '\'' => result_chirho.push_str("&#39;"),
            '`' => result_chirho.push_str("&#96;"),
            '\n' => result_chirho.push_str("&#10;"),
            '\r' => result_chirho.push_str("&#13;"),
            _ => result_chirho.push(ch_chirho),
        }
    }
    result_chirho
}

/// Filter trait for text transformation.
pub trait FilterChirho: Send + Sync {
    /// Process text through this filter.
    fn process_chirho(&self, text_chirho: &str) -> ResultChirho<String>;

    /// Get the filter name.
    fn name_chirho(&self) -> &str;
}

/// Filter options for controlling output.
#[derive(Debug, Clone, Default)]
pub struct FilterOptionsChirho {
    /// Include Strong's numbers.
    pub strongs_chirho: bool,
    /// Include morphology tags.
    pub morph_chirho: bool,
    /// Include footnotes.
    pub footnotes_chirho: bool,
    /// Include headings.
    pub headings_chirho: bool,
    /// Include cross-references.
    pub xrefs_chirho: bool,
    /// Include red letter words of Christ.
    pub red_letter_chirho: bool,
    /// Include lemmas.
    pub lemmas_chirho: bool,
    /// Include scripture references.
    pub scripref_chirho: bool,
}

impl FilterOptionsChirho {
    /// Create options with all features enabled.
    pub fn all_chirho() -> Self {
        Self {
            strongs_chirho: true,
            morph_chirho: true,
            footnotes_chirho: true,
            headings_chirho: true,
            xrefs_chirho: true,
            red_letter_chirho: true,
            lemmas_chirho: true,
            scripref_chirho: true,
        }
    }

    /// Create options with no features (plain text).
    pub fn none_chirho() -> Self {
        Self::default()
    }
}

/// Strip filter - removes all markup.
pub struct StripFilterChirho;

impl FilterChirho for StripFilterChirho {
    fn process_chirho(&self, text_chirho: &str) -> ResultChirho<String> {
        // Simple tag removal
        let mut result_chirho = String::with_capacity(text_chirho.len());
        let mut in_tag_chirho = false;

        for ch_chirho in text_chirho.chars() {
            if ch_chirho == '<' {
                in_tag_chirho = true;
            } else if ch_chirho == '>' {
                in_tag_chirho = false;
            } else if !in_tag_chirho {
                result_chirho.push(ch_chirho);
            }
        }

        Ok(result_chirho)
    }

    fn name_chirho(&self) -> &str {
        "Strip"
    }
}

/// Plain text filter - minimal processing.
pub struct PlainFilterChirho;

impl FilterChirho for PlainFilterChirho {
    fn process_chirho(&self, text_chirho: &str) -> ResultChirho<String> {
        Ok(text_chirho.to_string())
    }

    fn name_chirho(&self) -> &str {
        "Plain"
    }
}

/// UTF-8 encoding filter.
pub struct Utf8FilterChirho;

impl FilterChirho for Utf8FilterChirho {
    fn process_chirho(&self, text_chirho: &str) -> ResultChirho<String> {
        Ok(text_chirho.to_string())
    }

    fn name_chirho(&self) -> &str {
        "UTF8"
    }
}

/// Filter chain that applies multiple filters in sequence.
pub struct FilterChainChirho {
    filters_chirho: Vec<Box<dyn FilterChirho>>,
}

impl FilterChainChirho {
    /// Create a new empty filter chain.
    pub fn new_chirho() -> Self {
        Self {
            filters_chirho: Vec::new(),
        }
    }

    /// Add a filter to the chain.
    pub fn add_chirho(&mut self, filter_chirho: Box<dyn FilterChirho>) {
        self.filters_chirho.push(filter_chirho);
    }

    /// Process text through all filters in the chain.
    pub fn process_chirho(&self, text_chirho: &str) -> ResultChirho<String> {
        let mut result_chirho = text_chirho.to_string();
        for filter_chirho in &self.filters_chirho {
            result_chirho = filter_chirho.process_chirho(&result_chirho)?;
        }
        Ok(result_chirho)
    }
}

impl Default for FilterChainChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_strip_filter_chirho() {
        let filter_chirho = StripFilterChirho;
        let result_chirho = filter_chirho.process_chirho("<p>Hello <b>World</b></p>").unwrap();
        assert_eq!(result_chirho, "Hello World");
    }

    #[test]
    fn test_filter_chain_chirho() {
        let mut chain_chirho = FilterChainChirho::new_chirho();
        chain_chirho.add_chirho(Box::new(StripFilterChirho));
        chain_chirho.add_chirho(Box::new(PlainFilterChirho));

        let result_chirho = chain_chirho.process_chirho("<p>Test</p>").unwrap();
        assert_eq!(result_chirho, "Test");
    }

    #[test]
    fn test_filter_options_chirho() {
        let opts_chirho = FilterOptionsChirho::all_chirho();
        assert!(opts_chirho.strongs_chirho);
        assert!(opts_chirho.footnotes_chirho);

        let opts_none_chirho = FilterOptionsChirho::none_chirho();
        assert!(!opts_none_chirho.strongs_chirho);
    }
}
