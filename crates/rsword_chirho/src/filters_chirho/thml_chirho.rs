// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! ThML (Theological Markup Language) filters.
//!
//! Converts ThML markup to HTML or plain text.
//! ThML was designed for theological works and includes special support
//! for scriptures, Strong's numbers, and other theological markup.

use regex::Regex;
use std::sync::LazyLock;

use crate::error_chirho::ResultChirho;
use super::{FilterChirho, FilterOptionsChirho};

/// Regex for ThML synchronization tags (Strong's numbers).
static THML_SYNC_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<sync type="Strongs" value="([^"]+)"\s*/>"#).unwrap()
});

/// Regex for ThML scripture references.
static THML_SCRIPREF_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<scripRef[^>]*>(.*?)</scripRef>"#).unwrap()
});

/// Regex for ThML notes.
static THML_NOTE_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<note[^>]*>(.*?)</note>"#).unwrap()
});

/// Regex for ThML added text (italics in KJV).
static THML_ADDED_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<added>(.*?)</added>"#).unwrap()
});

/// Regex for ThML divine name.
static THML_DIVINE_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<div type="small-caps">(.*?)</div>"#).unwrap()
});

/// Regex for ThML term definitions.
static THML_TERM_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<term>(.*?)</term>"#).unwrap()
});

/// Regex for ThML definitions.
static THML_DEF_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<def>(.*?)</def>"#).unwrap()
});

/// Convert ThML markup to HTML.
pub struct ThmlToHtmlFilterChirho {
    options_chirho: FilterOptionsChirho,
}

impl ThmlToHtmlFilterChirho {
    /// Create a new ThML to HTML filter with default options.
    pub fn new_chirho() -> Self {
        Self {
            options_chirho: FilterOptionsChirho::default(),
        }
    }

    /// Create a new ThML to HTML filter with specified options.
    pub fn with_options_chirho(options_chirho: FilterOptionsChirho) -> Self {
        Self { options_chirho }
    }
}

impl Default for ThmlToHtmlFilterChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

impl FilterChirho for ThmlToHtmlFilterChirho {
    fn process_chirho(&self, text_chirho: &str) -> ResultChirho<String> {
        let mut result_chirho = text_chirho.to_string();

        // Process Strong's synchronization tags
        if self.options_chirho.strongs_chirho {
            result_chirho = THML_SYNC_REGEX_CHIRHO
                .replace_all(&result_chirho, |caps_chirho: &regex::Captures| {
                    format!(
                        r#"<sup class="strongs"><a href="strongs://{0}">{0}</a></sup>"#,
                        &caps_chirho[1]
                    )
                })
                .to_string();
        } else {
            result_chirho = THML_SYNC_REGEX_CHIRHO.replace_all(&result_chirho, "").to_string();
        }

        // Process scripture references
        if self.options_chirho.scripref_chirho {
            result_chirho = THML_SCRIPREF_REGEX_CHIRHO
                .replace_all(&result_chirho, |caps_chirho: &regex::Captures| {
                    format!(
                        r#"<a class="scripref" href="bible://{0}">{0}</a>"#,
                        &caps_chirho[1]
                    )
                })
                .to_string();
        } else {
            result_chirho = THML_SCRIPREF_REGEX_CHIRHO
                .replace_all(&result_chirho, "$1")
                .to_string();
        }

        // Process notes
        if self.options_chirho.footnotes_chirho {
            result_chirho = THML_NOTE_REGEX_CHIRHO
                .replace_all(&result_chirho, |caps_chirho: &regex::Captures| {
                    format!(r#"<span class="footnote">[{}]</span>"#, &caps_chirho[1])
                })
                .to_string();
        } else {
            result_chirho = THML_NOTE_REGEX_CHIRHO.replace_all(&result_chirho, "").to_string();
        }

        // Process added text (italics)
        result_chirho = THML_ADDED_REGEX_CHIRHO
            .replace_all(&result_chirho, r#"<em class="added">$1</em>"#)
            .to_string();

        // Process divine name (small caps -> span)
        result_chirho = THML_DIVINE_REGEX_CHIRHO
            .replace_all(&result_chirho, r#"<span class="divine-name">$1</span>"#)
            .to_string();

        // Process terms and definitions
        result_chirho = THML_TERM_REGEX_CHIRHO
            .replace_all(&result_chirho, r#"<dfn class="term">$1</dfn>"#)
            .to_string();

        result_chirho = THML_DEF_REGEX_CHIRHO
            .replace_all(&result_chirho, r#"<span class="definition">$1</span>"#)
            .to_string();

        // Convert basic ThML elements to HTML equivalents
        result_chirho = result_chirho
            .replace("<br/>", "<br/>")
            .replace("<br />", "<br/>")
            .replace("<pb/>", "<hr class=\"page-break\"/>")
            .replace("<pb />", "<hr class=\"page-break\"/>");

        // Remove ThML-specific container tags
        result_chirho = Regex::new(r#"</?ThML[^>]*>"#)
            .unwrap()
            .replace_all(&result_chirho, "")
            .to_string();

        result_chirho = Regex::new(r#"</?ThML\.body[^>]*>"#)
            .unwrap()
            .replace_all(&result_chirho, "")
            .to_string();

        Ok(result_chirho)
    }

    fn name_chirho(&self) -> &str {
        "ThML to HTML"
    }
}

/// Convert ThML markup to plain text.
pub struct ThmlToPlainFilterChirho {
    options_chirho: FilterOptionsChirho,
}

impl ThmlToPlainFilterChirho {
    /// Create a new ThML to plain text filter.
    pub fn new_chirho() -> Self {
        Self {
            options_chirho: FilterOptionsChirho::none_chirho(),
        }
    }

    /// Create with options.
    pub fn with_options_chirho(options_chirho: FilterOptionsChirho) -> Self {
        Self { options_chirho }
    }
}

impl Default for ThmlToPlainFilterChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

impl FilterChirho for ThmlToPlainFilterChirho {
    fn process_chirho(&self, text_chirho: &str) -> ResultChirho<String> {
        let mut result_chirho = text_chirho.to_string();

        // Process Strong's numbers
        if self.options_chirho.strongs_chirho {
            result_chirho = THML_SYNC_REGEX_CHIRHO
                .replace_all(&result_chirho, |caps_chirho: &regex::Captures| {
                    format!(" ({})", &caps_chirho[1])
                })
                .to_string();
        } else {
            result_chirho = THML_SYNC_REGEX_CHIRHO.replace_all(&result_chirho, "").to_string();
        }

        // Process notes
        if self.options_chirho.footnotes_chirho {
            result_chirho = THML_NOTE_REGEX_CHIRHO
                .replace_all(&result_chirho, |caps_chirho: &regex::Captures| {
                    format!(" [Note: {}]", &caps_chirho[1])
                })
                .to_string();
        } else {
            result_chirho = THML_NOTE_REGEX_CHIRHO.replace_all(&result_chirho, "").to_string();
        }

        // Extract text from various elements
        result_chirho = THML_SCRIPREF_REGEX_CHIRHO
            .replace_all(&result_chirho, "$1")
            .to_string();

        result_chirho = THML_ADDED_REGEX_CHIRHO
            .replace_all(&result_chirho, "$1")
            .to_string();

        result_chirho = THML_DIVINE_REGEX_CHIRHO
            .replace_all(&result_chirho, "$1")
            .to_string();

        result_chirho = THML_TERM_REGEX_CHIRHO
            .replace_all(&result_chirho, "$1")
            .to_string();

        result_chirho = THML_DEF_REGEX_CHIRHO
            .replace_all(&result_chirho, "$1")
            .to_string();

        // Convert line breaks
        result_chirho = result_chirho
            .replace("<br/>", "\n")
            .replace("<br />", "\n")
            .replace("<pb/>", "\n---\n")
            .replace("<pb />", "\n---\n");

        // Remove all remaining tags
        result_chirho = Regex::new(r#"<[^>]+>"#)
            .unwrap()
            .replace_all(&result_chirho, "")
            .to_string();

        // Clean up whitespace
        result_chirho = Regex::new(r#"\s+"#)
            .unwrap()
            .replace_all(&result_chirho, " ")
            .to_string()
            .trim()
            .to_string();

        Ok(result_chirho)
    }

    fn name_chirho(&self) -> &str {
        "ThML to Plain"
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_thml_to_plain_basic_chirho() {
        let filter_chirho = ThmlToPlainFilterChirho::new_chirho();
        let input_chirho = "<p>In the beginning</p>";
        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert_eq!(result_chirho, "In the beginning");
    }

    #[test]
    fn test_thml_to_plain_strongs_chirho() {
        let filter_chirho = ThmlToPlainFilterChirho::with_options_chirho(FilterOptionsChirho {
            strongs_chirho: true,
            ..Default::default()
        });
        let input_chirho = r#"God<sync type="Strongs" value="G2316"/>"#;
        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(result_chirho.contains("God"));
        assert!(result_chirho.contains("G2316"));
    }

    #[test]
    fn test_thml_to_html_strongs_chirho() {
        let filter_chirho = ThmlToHtmlFilterChirho::with_options_chirho(FilterOptionsChirho {
            strongs_chirho: true,
            ..Default::default()
        });
        let input_chirho = r#"God<sync type="Strongs" value="G2316"/>"#;
        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(result_chirho.contains("strongs://G2316"));
    }

    #[test]
    fn test_thml_added_text_chirho() {
        let filter_chirho = ThmlToHtmlFilterChirho::new_chirho();
        let input_chirho = "<added>supplied text</added>";
        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(result_chirho.contains("<em"));
        assert!(result_chirho.contains("supplied text"));
    }
}
