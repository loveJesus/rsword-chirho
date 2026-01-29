// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Text filters for markup conversion.
//!
//! Filters transform module text between different formats (OSIS, ThML, GBF, etc.)
//! and apply options like Strong's numbers, footnotes, and morphology.

use crate::error_chirho::ResultChirho;

/// Filter trait for text transformation.
pub trait FilterChirho: Send + Sync {
    /// Process text through this filter.
    fn process_chirho(&self, text_chirho: &str) -> ResultChirho<String>;

    /// Get the filter name.
    fn name_chirho(&self) -> &str;
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

// TODO: Implement OSIS, ThML, GBF to HTML/plain filters
// TODO: Implement Strong's, footnotes, morphology option filters

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_strip_filter_chirho() {
        let filter_chirho = StripFilterChirho;
        let result_chirho = filter_chirho.process_chirho("<p>Hello <b>World</b></p>").unwrap();
        assert_eq!(result_chirho, "Hello World");
    }
}
