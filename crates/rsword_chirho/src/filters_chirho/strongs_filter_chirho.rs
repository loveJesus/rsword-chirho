// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Filter for Strong's number handling.
//!
//! Strong's numbers are references to the original Hebrew (H) and Greek (G)
//! words in the Bible. This filter can either preserve, strip, or format them.

use regex::Regex;
use std::sync::LazyLock;

use crate::error_chirho::ResultChirho;
use crate::filters_chirho::FilterChirho;

/// Regex for Strong's number patterns.
static STRONGS_PATTERN_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<w\s+lemma="[^"]*"[^>]*>([^<]*)</w>"#).unwrap()
});

/// Regex for lemma attribute.
static LEMMA_ATTR_PATTERN_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"\s*lemma="[^"]*""#).unwrap()
});

/// Filter that strips Strong's numbers from text.
#[derive(Debug, Clone, Default)]
pub struct StrongsStripFilterChirho;

impl FilterChirho for StrongsStripFilterChirho {
    fn process_chirho(&self, text_chirho: &str) -> ResultChirho<String> {
        // Remove lemma attributes from <w> elements
        let result_chirho = LEMMA_ATTR_PATTERN_CHIRHO.replace_all(text_chirho, "");
        Ok(result_chirho.to_string())
    }

    fn name_chirho(&self) -> &str {
        "StrongsStrip"
    }
}

/// Filter that formats Strong's numbers for display.
#[derive(Debug, Clone)]
pub struct StrongsFormatFilterChirho {
    /// Format template for Strong's display.
    format_chirho: StrongsFormatChirho,
}

/// Strong's number display format.
#[derive(Debug, Clone, Copy, Default)]
pub enum StrongsFormatChirho {
    /// Show as subscript (e.g., word<sub>H1234</sub>).
    SubscriptChirho,
    /// Show in parentheses (e.g., word (H1234)).
    ParenthesesChirho,
    /// Show in brackets (e.g., word \[H1234\]).
    BracketsChirho,
    /// Show with hover/tooltip.
    #[default]
    TooltipChirho,
}

impl Default for StrongsFormatFilterChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

impl StrongsFormatFilterChirho {
    /// Create a new Strong's format filter with default format.
    pub fn new_chirho() -> Self {
        Self {
            format_chirho: StrongsFormatChirho::TooltipChirho,
        }
    }

    /// Create with specific format.
    pub fn with_format_chirho(format_chirho: StrongsFormatChirho) -> Self {
        Self { format_chirho }
    }

    /// Extract Strong's number from lemma attribute.
    fn extract_strongs_chirho(lemma_chirho: &str) -> Option<String> {
        // Lemma can be "strong:H1234" or just "H1234"
        let parts_chirho: Vec<&str> = lemma_chirho.split(':').collect();
        let number_chirho = if parts_chirho.len() > 1 {
            parts_chirho[1]
        } else {
            parts_chirho[0]
        };

        if number_chirho.starts_with('H') || number_chirho.starts_with('G') {
            Some(number_chirho.to_string())
        } else {
            None
        }
    }
}

impl FilterChirho for StrongsFormatFilterChirho {
    fn process_chirho(&self, text_chirho: &str) -> ResultChirho<String> {
        let result_chirho = STRONGS_PATTERN_CHIRHO.replace_all(text_chirho, |caps_chirho: &regex::Captures| {
            let word_chirho = caps_chirho.get(1).map_or("", |m| m.as_str());
            let full_match_chirho = caps_chirho.get(0).map_or("", |m| m.as_str());

            // Extract lemma from original match
            if let Some(lemma_start_chirho) = full_match_chirho.find("lemma=\"") {
                let lemma_start_chirho = lemma_start_chirho + 7;
                if let Some(lemma_end_chirho) = full_match_chirho[lemma_start_chirho..].find('"') {
                    let lemma_chirho = &full_match_chirho[lemma_start_chirho..lemma_start_chirho + lemma_end_chirho];
                    if let Some(strongs_chirho) = Self::extract_strongs_chirho(lemma_chirho) {
                        return match self.format_chirho {
                            StrongsFormatChirho::SubscriptChirho => {
                                format!("{}<sub>{}</sub>", word_chirho, strongs_chirho)
                            }
                            StrongsFormatChirho::ParenthesesChirho => {
                                format!("{} ({})", word_chirho, strongs_chirho)
                            }
                            StrongsFormatChirho::BracketsChirho => {
                                format!("{} [{}]", word_chirho, strongs_chirho)
                            }
                            StrongsFormatChirho::TooltipChirho => {
                                format!("<span title=\"{}\">{}</span>", strongs_chirho, word_chirho)
                            }
                        };
                    }
                }
            }

            word_chirho.to_string()
        });

        Ok(result_chirho.to_string())
    }

    fn name_chirho(&self) -> &str {
        "StrongsFormat"
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_strongs_strip_chirho() {
        let filter_chirho = StrongsStripFilterChirho;
        let input_chirho = r#"<w lemma="strong:H7225">In the beginning</w>"#;

        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(!result_chirho.contains("lemma"));
    }

    #[test]
    fn test_extract_strongs_chirho() {
        assert_eq!(
            StrongsFormatFilterChirho::extract_strongs_chirho("strong:H1234"),
            Some("H1234".to_string())
        );
        assert_eq!(
            StrongsFormatFilterChirho::extract_strongs_chirho("H5678"),
            Some("H5678".to_string())
        );
        assert_eq!(
            StrongsFormatFilterChirho::extract_strongs_chirho("G3056"),
            Some("G3056".to_string())
        );
        assert_eq!(
            StrongsFormatFilterChirho::extract_strongs_chirho("other"),
            None
        );
    }

    #[test]
    fn test_strongs_format_subscript_chirho() {
        let filter_chirho = StrongsFormatFilterChirho::with_format_chirho(StrongsFormatChirho::SubscriptChirho);
        let input_chirho = r#"<w lemma="H7225">beginning</w>"#;

        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(result_chirho.contains("<sub>H7225</sub>"));
    }

    #[test]
    fn test_strongs_format_parentheses_chirho() {
        let filter_chirho = StrongsFormatFilterChirho::with_format_chirho(StrongsFormatChirho::ParenthesesChirho);
        let input_chirho = r#"<w lemma="G26">love</w>"#;

        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(result_chirho.contains("(G26)"));
    }

    #[test]
    fn test_strongs_format_tooltip_chirho() {
        let filter_chirho = StrongsFormatFilterChirho::new_chirho();
        let input_chirho = r#"<w lemma="H1234">word</w>"#;

        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(result_chirho.contains("title=\"H1234\""));
    }
}
