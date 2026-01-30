// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Filter for red letter words handling.
//!
//! Red letter words are the words of Christ in the Gospels,
//! traditionally printed in red ink.

use regex::Regex;
use std::sync::LazyLock;

use crate::error_chirho::ResultChirho;
use crate::filters_chirho::FilterChirho;

/// Regex for OSIS q (quotation) elements with who="Jesus".
static JESUS_QUOTE_PATTERN_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<q\s+[^>]*who="Jesus"[^>]*>(.*?)</q>"#).unwrap()
});

/// Regex for red letter style span.
static REDLETTER_SPAN_PATTERN_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<span[^>]*class="[^"]*red[^"]*"[^>]*>(.*?)</span>"#).unwrap()
});

/// Filter that strips red letter markup.
#[derive(Debug, Clone, Default)]
pub struct RedLetterStripFilterChirho;

impl FilterChirho for RedLetterStripFilterChirho {
    fn process_chirho(&self, text_chirho: &str) -> ResultChirho<String> {
        // Replace OSIS q elements with just their content
        let result_chirho = JESUS_QUOTE_PATTERN_CHIRHO.replace_all(text_chirho, "$1");
        // Also strip span-based red letter
        let result_chirho = REDLETTER_SPAN_PATTERN_CHIRHO.replace_all(&result_chirho, "$1");
        Ok(result_chirho.to_string())
    }

    fn name_chirho(&self) -> &str {
        "RedLetterStrip"
    }
}

/// Filter that formats red letter words for display.
#[derive(Debug, Clone)]
pub struct RedLetterFormatFilterChirho {
    /// CSS class for red letter text.
    css_class_chirho: String,
    /// Inline color style.
    color_chirho: Option<String>,
}

impl Default for RedLetterFormatFilterChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

impl RedLetterFormatFilterChirho {
    /// Create a new red letter format filter.
    pub fn new_chirho() -> Self {
        Self {
            css_class_chirho: "red-letter".to_string(),
            color_chirho: None,
        }
    }

    /// Create with custom CSS class.
    pub fn with_class_chirho(class_chirho: &str) -> Self {
        Self {
            css_class_chirho: class_chirho.to_string(),
            color_chirho: None,
        }
    }

    /// Create with inline color style.
    pub fn with_color_chirho(color_chirho: &str) -> Self {
        Self {
            css_class_chirho: String::new(),
            color_chirho: Some(color_chirho.to_string()),
        }
    }
}

impl FilterChirho for RedLetterFormatFilterChirho {
    fn process_chirho(&self, text_chirho: &str) -> ResultChirho<String> {
        let replacement_chirho = if let Some(ref color_chirho) = self.color_chirho {
            format!(r#"<span style="color: {}">$1</span>"#, color_chirho)
        } else if !self.css_class_chirho.is_empty() {
            format!(r#"<span class="{}">$1</span>"#, self.css_class_chirho)
        } else {
            "$1".to_string()
        };

        let result_chirho = JESUS_QUOTE_PATTERN_CHIRHO.replace_all(text_chirho, replacement_chirho.as_str());
        Ok(result_chirho.to_string())
    }

    fn name_chirho(&self) -> &str {
        "RedLetterFormat"
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_redletter_strip_chirho() {
        let filter_chirho = RedLetterStripFilterChirho;
        let input_chirho = r#"Jesus said, <q who="Jesus">I am the way</q>"#;

        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert_eq!(result_chirho, "Jesus said, I am the way");
    }

    #[test]
    fn test_redletter_format_class_chirho() {
        let filter_chirho = RedLetterFormatFilterChirho::new_chirho();
        let input_chirho = r#"<q who="Jesus">words of Christ</q>"#;

        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(result_chirho.contains("class=\"red-letter\""));
        assert!(result_chirho.contains("words of Christ"));
    }

    #[test]
    fn test_redletter_format_color_chirho() {
        let filter_chirho = RedLetterFormatFilterChirho::with_color_chirho("#cc0000");
        let input_chirho = r#"<q who="Jesus">I am the light</q>"#;

        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(result_chirho.contains("style=\"color: #cc0000\""));
    }

    #[test]
    fn test_redletter_custom_class_chirho() {
        let filter_chirho = RedLetterFormatFilterChirho::with_class_chirho("jesus-words");
        let input_chirho = r#"<q who="Jesus">text</q>"#;

        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(result_chirho.contains("class=\"jesus-words\""));
    }
}
