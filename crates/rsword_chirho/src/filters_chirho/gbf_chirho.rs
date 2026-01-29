// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! GBF (General Bible Format) filters.
//!
//! Converts GBF markup to HTML or plain text.
//! GBF uses special codes enclosed in angle brackets for formatting.

use regex::Regex;
use std::sync::LazyLock;

use crate::error_chirho::ResultChirho;
use super::{FilterChirho, FilterOptionsChirho};

/// Regex for GBF formatting codes.
static GBF_TAG_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<([A-Z]{1,2})([^>]*)>"#).unwrap()
});

/// GBF Strong's number pattern.
static GBF_STRONGS_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<W([GH])(\d+)>"#).unwrap()
});

/// GBF morphology pattern.
static GBF_MORPH_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<WT([^>]+)>"#).unwrap()
});

/// GBF footnote pattern.
static GBF_FOOTNOTE_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<RF>([^<]*)<Rf>"#).unwrap()
});

/// GBF cross-reference pattern.
static GBF_XREF_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<RX>([^<]*)<Rx>"#).unwrap()
});

/// Convert GBF markup to HTML.
pub struct GbfToHtmlFilterChirho {
    options_chirho: FilterOptionsChirho,
}

impl GbfToHtmlFilterChirho {
    /// Create a new GBF to HTML filter with default options.
    pub fn new_chirho() -> Self {
        Self {
            options_chirho: FilterOptionsChirho::default(),
        }
    }

    /// Create a new GBF to HTML filter with specified options.
    pub fn with_options_chirho(options_chirho: FilterOptionsChirho) -> Self {
        Self { options_chirho }
    }
}

impl Default for GbfToHtmlFilterChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

impl FilterChirho for GbfToHtmlFilterChirho {
    fn process_chirho(&self, text_chirho: &str) -> ResultChirho<String> {
        let mut result_chirho = text_chirho.to_string();

        // Process Strong's numbers
        if self.options_chirho.strongs_chirho {
            result_chirho = GBF_STRONGS_REGEX_CHIRHO
                .replace_all(&result_chirho, |caps_chirho: &regex::Captures| {
                    let prefix_chirho = &caps_chirho[1];
                    let num_chirho = &caps_chirho[2];
                    format!(
                        r#"<sup class="strongs"><a href="strongs://{0}{1}">{0}{1}</a></sup>"#,
                        prefix_chirho, num_chirho
                    )
                })
                .to_string();
        } else {
            result_chirho = GBF_STRONGS_REGEX_CHIRHO.replace_all(&result_chirho, "").to_string();
        }

        // Process morphology
        if self.options_chirho.morph_chirho {
            result_chirho = GBF_MORPH_REGEX_CHIRHO
                .replace_all(&result_chirho, |caps_chirho: &regex::Captures| {
                    format!(r#"<sup class="morph">{}</sup>"#, &caps_chirho[1])
                })
                .to_string();
        } else {
            result_chirho = GBF_MORPH_REGEX_CHIRHO.replace_all(&result_chirho, "").to_string();
        }

        // Process footnotes
        if self.options_chirho.footnotes_chirho {
            result_chirho = GBF_FOOTNOTE_REGEX_CHIRHO
                .replace_all(&result_chirho, |caps_chirho: &regex::Captures| {
                    format!(r#"<span class="footnote">[{}]</span>"#, &caps_chirho[1])
                })
                .to_string();
        } else {
            result_chirho = GBF_FOOTNOTE_REGEX_CHIRHO.replace_all(&result_chirho, "").to_string();
        }

        // Process cross-references
        if self.options_chirho.xrefs_chirho {
            result_chirho = GBF_XREF_REGEX_CHIRHO
                .replace_all(&result_chirho, |caps_chirho: &regex::Captures| {
                    format!(
                        r#"<span class="xref"><a href="bible://{0}">{0}</a></span>"#,
                        &caps_chirho[1]
                    )
                })
                .to_string();
        } else {
            result_chirho = GBF_XREF_REGEX_CHIRHO.replace_all(&result_chirho, "").to_string();
        }

        // Convert GBF formatting codes to HTML
        // Bold
        result_chirho = result_chirho
            .replace("<FB>", "<b>")
            .replace("<Fb>", "</b>");

        // Italic
        result_chirho = result_chirho
            .replace("<FI>", "<i>")
            .replace("<Fi>", "</i>");

        // Underline
        result_chirho = result_chirho
            .replace("<FU>", "<u>")
            .replace("<Fu>", "</u>");

        // Superscript
        result_chirho = result_chirho
            .replace("<FS>", "<sup>")
            .replace("<Fs>", "</sup>");

        // Red letter (words of Christ)
        if self.options_chirho.red_letter_chirho {
            result_chirho = result_chirho
                .replace("<FR>", r#"<span class="jesus-words">"#)
                .replace("<Fr>", "</span>");
        } else {
            result_chirho = result_chirho
                .replace("<FR>", "")
                .replace("<Fr>", "");
        }

        // Paragraph
        result_chirho = result_chirho
            .replace("<CM>", "<p>")
            .replace("<CL>", "<br/>");

        // Poetry/verse
        result_chirho = result_chirho
            .replace("<PI>", r#"<div class="poetry">"#)
            .replace("<Pi>", "</div>");

        // Title/heading
        if self.options_chirho.headings_chirho {
            result_chirho = result_chirho
                .replace("<TS>", r#"<h3 class="heading">"#)
                .replace("<Ts>", "</h3>");
        } else {
            result_chirho = result_chirho
                .replace("<TS>", "")
                .replace("<Ts>", "");
        }

        // Small caps (divine name)
        result_chirho = result_chirho
            .replace("<SC>", r#"<span class="divine-name">"#)
            .replace("<Sc>", "</span>");

        // Remove remaining GBF tags
        result_chirho = GBF_TAG_REGEX_CHIRHO.replace_all(&result_chirho, "").to_string();

        Ok(result_chirho)
    }

    fn name_chirho(&self) -> &str {
        "GBF to HTML"
    }
}

/// Convert GBF markup to plain text.
pub struct GbfToPlainFilterChirho {
    options_chirho: FilterOptionsChirho,
}

impl GbfToPlainFilterChirho {
    /// Create a new GBF to plain text filter.
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

impl Default for GbfToPlainFilterChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

impl FilterChirho for GbfToPlainFilterChirho {
    fn process_chirho(&self, text_chirho: &str) -> ResultChirho<String> {
        let mut result_chirho = text_chirho.to_string();

        // Process Strong's numbers
        if self.options_chirho.strongs_chirho {
            result_chirho = GBF_STRONGS_REGEX_CHIRHO
                .replace_all(&result_chirho, |caps_chirho: &regex::Captures| {
                    format!(" ({}{})", &caps_chirho[1], &caps_chirho[2])
                })
                .to_string();
        } else {
            result_chirho = GBF_STRONGS_REGEX_CHIRHO.replace_all(&result_chirho, "").to_string();
        }

        // Process morphology
        if self.options_chirho.morph_chirho {
            result_chirho = GBF_MORPH_REGEX_CHIRHO
                .replace_all(&result_chirho, |caps_chirho: &regex::Captures| {
                    format!(" ({})", &caps_chirho[1])
                })
                .to_string();
        } else {
            result_chirho = GBF_MORPH_REGEX_CHIRHO.replace_all(&result_chirho, "").to_string();
        }

        // Process footnotes
        if self.options_chirho.footnotes_chirho {
            result_chirho = GBF_FOOTNOTE_REGEX_CHIRHO
                .replace_all(&result_chirho, |caps_chirho: &regex::Captures| {
                    format!(" [Note: {}]", &caps_chirho[1])
                })
                .to_string();
        } else {
            result_chirho = GBF_FOOTNOTE_REGEX_CHIRHO.replace_all(&result_chirho, "").to_string();
        }

        // Process cross-references
        if self.options_chirho.xrefs_chirho {
            result_chirho = GBF_XREF_REGEX_CHIRHO
                .replace_all(&result_chirho, " [See: $1]")
                .to_string();
        } else {
            result_chirho = GBF_XREF_REGEX_CHIRHO.replace_all(&result_chirho, "").to_string();
        }

        // Convert line breaks
        result_chirho = result_chirho
            .replace("<CL>", "\n")
            .replace("<CM>", "\n\n");

        // Process headings
        if self.options_chirho.headings_chirho {
            // Extract heading text between <TS> and <Ts>
            result_chirho = Regex::new(r#"<TS>([^<]*)<Ts>"#)
                .unwrap()
                .replace_all(&result_chirho, "\n=== $1 ===\n")
                .to_string();
        }

        // Remove all remaining GBF tags
        result_chirho = GBF_TAG_REGEX_CHIRHO.replace_all(&result_chirho, "").to_string();

        // Also remove closing tags
        result_chirho = Regex::new(r#"<[A-Z][a-z]>"#)
            .unwrap()
            .replace_all(&result_chirho, "")
            .to_string();

        // Clean up whitespace
        result_chirho = Regex::new(r#"[ \t]+"#)
            .unwrap()
            .replace_all(&result_chirho, " ")
            .to_string();

        result_chirho = Regex::new(r#"\n{3,}"#)
            .unwrap()
            .replace_all(&result_chirho, "\n\n")
            .to_string()
            .trim()
            .to_string();

        Ok(result_chirho)
    }

    fn name_chirho(&self) -> &str {
        "GBF to Plain"
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_gbf_to_plain_basic_chirho() {
        let filter_chirho = GbfToPlainFilterChirho::new_chirho();
        let input_chirho = "<FB>Bold<Fb> and <FI>italic<Fi>";
        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert_eq!(result_chirho, "Bold and italic");
    }

    #[test]
    fn test_gbf_to_plain_strongs_chirho() {
        let filter_chirho = GbfToPlainFilterChirho::with_options_chirho(FilterOptionsChirho {
            strongs_chirho: true,
            ..Default::default()
        });
        let input_chirho = "God<WG2316>";
        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(result_chirho.contains("God"));
        assert!(result_chirho.contains("G2316"));
    }

    #[test]
    fn test_gbf_to_html_basic_chirho() {
        let filter_chirho = GbfToHtmlFilterChirho::new_chirho();
        let input_chirho = "<FB>Bold<Fb>";
        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert_eq!(result_chirho, "<b>Bold</b>");
    }

    #[test]
    fn test_gbf_to_html_strongs_chirho() {
        let filter_chirho = GbfToHtmlFilterChirho::with_options_chirho(FilterOptionsChirho {
            strongs_chirho: true,
            ..Default::default()
        });
        let input_chirho = "God<WG2316>";
        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(result_chirho.contains("strongs://G2316"));
    }

    #[test]
    fn test_gbf_footnotes_chirho() {
        let filter_chirho = GbfToHtmlFilterChirho::with_options_chirho(FilterOptionsChirho {
            footnotes_chirho: true,
            ..Default::default()
        });
        let input_chirho = "text<RF>footnote<Rf>more";
        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(result_chirho.contains("footnote"));
    }

    #[test]
    fn test_gbf_red_letter_chirho() {
        let filter_chirho = GbfToHtmlFilterChirho::with_options_chirho(FilterOptionsChirho {
            red_letter_chirho: true,
            ..Default::default()
        });
        let input_chirho = "<FR>I am the way<Fr>";
        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(result_chirho.contains("jesus-words"));
    }
}
