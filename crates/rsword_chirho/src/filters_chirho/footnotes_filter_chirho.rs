// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Filter for footnote handling.
//!
//! Footnotes in Bible modules can contain variant readings, translator notes,
//! and other supplementary information.

use regex::Regex;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::error_chirho::ResultChirho;
use crate::filters_chirho::FilterChirho;

/// Regex for note elements (OSIS/ThML).
static NOTE_PATTERN_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<note[^>]*>(.*?)</note>"#).unwrap()
});

/// Filter that strips footnotes from text.
#[derive(Debug, Clone, Default)]
pub struct FootnotesStripFilterChirho;

impl FilterChirho for FootnotesStripFilterChirho {
    fn process_chirho(&self, text_chirho: &str) -> ResultChirho<String> {
        let result_chirho = NOTE_PATTERN_CHIRHO.replace_all(text_chirho, "");
        Ok(result_chirho.to_string())
    }

    fn name_chirho(&self) -> &str {
        "FootnotesStrip"
    }
}

/// Filter that formats footnotes for display.
#[derive(Debug)]
pub struct FootnotesFormatFilterChirho {
    /// Format template for footnote display.
    format_chirho: FootnoteFormatChirho,
    /// Counter for footnote numbering.
    counter_chirho: AtomicU32,
}

/// Footnote display format.
#[derive(Debug, Clone, Copy, Default)]
pub enum FootnoteFormatChirho {
    /// Show inline in parentheses.
    InlineChirho,
    /// Show as superscript marker with popup/hover.
    #[default]
    PopupChirho,
    /// Show at end of text.
    EndnotesChirho,
    /// Hide footnotes.
    HiddenChirho,
}

impl Default for FootnotesFormatFilterChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

impl FootnotesFormatFilterChirho {
    /// Create a new footnotes format filter with default format.
    pub fn new_chirho() -> Self {
        Self {
            format_chirho: FootnoteFormatChirho::PopupChirho,
            counter_chirho: AtomicU32::new(0),
        }
    }

    /// Create with specific format.
    pub fn with_format_chirho(format_chirho: FootnoteFormatChirho) -> Self {
        Self {
            format_chirho,
            counter_chirho: AtomicU32::new(0),
        }
    }

    /// Reset the footnote counter.
    pub fn reset_counter_chirho(&self) {
        self.counter_chirho.store(0, Ordering::Relaxed);
    }

    /// Get next footnote number.
    fn next_number_chirho(&self) -> u32 {
        self.counter_chirho.fetch_add(1, Ordering::Relaxed) + 1
    }
}

impl FilterChirho for FootnotesFormatFilterChirho {
    fn process_chirho(&self, text_chirho: &str) -> ResultChirho<String> {
        // Reset counter for each text block
        self.reset_counter_chirho();

        let result_chirho = NOTE_PATTERN_CHIRHO.replace_all(text_chirho, |caps_chirho: &regex::Captures| {
            let content_chirho = caps_chirho.get(1).map_or("", |m| m.as_str());

            match self.format_chirho {
                FootnoteFormatChirho::InlineChirho => {
                    format!(" ({})", content_chirho)
                }
                FootnoteFormatChirho::PopupChirho => {
                    let num_chirho = self.next_number_chirho();
                    format!(
                        "<sup class=\"footnote\" title=\"{}\"><a href=\"#fn{}\">[{}]</a></sup>",
                        content_chirho, num_chirho, num_chirho
                    )
                }
                FootnoteFormatChirho::EndnotesChirho => {
                    let num_chirho = self.next_number_chirho();
                    format!(r#"<sup>[{}]</sup>"#, num_chirho)
                }
                FootnoteFormatChirho::HiddenChirho => String::new(),
            }
        });

        Ok(result_chirho.to_string())
    }

    fn name_chirho(&self) -> &str {
        "FootnotesFormat"
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_footnotes_strip_chirho() {
        let filter_chirho = FootnotesStripFilterChirho;
        let input_chirho = "In the beginning<note>Or: When God began</note> God created";

        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert_eq!(result_chirho, "In the beginning God created");
    }

    #[test]
    fn test_footnotes_inline_chirho() {
        let filter_chirho = FootnotesFormatFilterChirho::with_format_chirho(FootnoteFormatChirho::InlineChirho);
        let input_chirho = "In the beginning<note>Or: When God began</note>";

        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(result_chirho.contains("(Or: When God began)"));
    }

    #[test]
    fn test_footnotes_popup_chirho() {
        let filter_chirho = FootnotesFormatFilterChirho::new_chirho();
        let input_chirho = "word<note>note text</note>";

        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(result_chirho.contains("class=\"footnote\""));
        assert!(result_chirho.contains("title=\"note text\""));
    }

    #[test]
    fn test_footnotes_hidden_chirho() {
        let filter_chirho = FootnotesFormatFilterChirho::with_format_chirho(FootnoteFormatChirho::HiddenChirho);
        let input_chirho = "word<note>hidden note</note>";

        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert_eq!(result_chirho, "word");
    }

    #[test]
    fn test_multiple_footnotes_chirho() {
        let filter_chirho = FootnotesFormatFilterChirho::new_chirho();
        let input_chirho = "first<note>note1</note> second<note>note2</note>";

        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(result_chirho.contains("[1]"));
        assert!(result_chirho.contains("[2]"));
    }
}
