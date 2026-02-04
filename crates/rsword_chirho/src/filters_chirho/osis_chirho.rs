// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! OSIS (Open Scripture Information Standard) filters.
//!
//! Converts OSIS XML markup to HTML or plain text.
//! OSIS is the most common format for modern SWORD Bible modules.

use regex::Regex;
use std::sync::LazyLock;

use crate::error_chirho::ResultChirho;
use super::{escape_html_chirho, escape_html_attr_chirho, FilterChirho, FilterOptionsChirho};

/// Compiled regex patterns for OSIS parsing.
static OSIS_WORD_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<w\s+([^>]*)>(.*?)</w>"#).unwrap()
});

// Self-closing word tags like <w lemma="..." /> that have no content
static OSIS_WORD_SELF_CLOSING_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<w\s+[^>]*/>"#).unwrap()
});

static OSIS_NOTE_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<note[^>]*>(.*?)</note>"#).unwrap()
});

static OSIS_TITLE_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<title[^>]*>(.*?)</title>"#).unwrap()
});

static OSIS_REFERENCE_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<reference[^>]*>(.*?)</reference>"#).unwrap()
});

static OSIS_DIVINE_NAME_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<divineName>(.*?)</divineName>"#).unwrap()
});

static OSIS_JESUS_WORDS_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<q who="Jesus"[^>]*>(.*?)</q>"#).unwrap()
});

static OSIS_LEMMA_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"lemma="([^"]*)""#).unwrap()
});

static OSIS_STRONGS_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"strong:([GH]\d+)"#).unwrap()
});

static OSIS_MORPH_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"morph="([^"]*)""#).unwrap()
});

static OSIS_GLOSS_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"gloss="([^"]*)""#).unwrap()
});

static OSIS_XLIT_REGEX_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"xlit="([^"]*)""#).unwrap()
});

/// Interlinear word data extracted from OSIS markup.
///
/// Represents a single word with its original text, transliteration,
/// morphology, Strong's numbers, and gloss (English meaning).
#[derive(Debug, Clone, Default)]
pub struct InterlinearWordDataChirho {
    /// Original word text (Hebrew/Greek).
    pub original_chirho: String,
    /// Transliteration (Latin characters).
    pub transliteration_chirho: String,
    /// Morphology code (e.g., "V-AAI-3S").
    pub morphology_chirho: String,
    /// Strong's number(s) (e.g., "H430", "G2316").
    pub strongs_chirho: String,
    /// English gloss/meaning.
    pub gloss_chirho: String,
    /// Part of speech extracted from morphology.
    pub part_of_speech_chirho: String,
    /// True if Hebrew (H prefix), false if Greek (G prefix).
    pub is_hebrew_chirho: bool,
    /// Lemma (dictionary form).
    pub lemma_chirho: String,
}

/// Extract interlinear word data from OSIS text.
///
/// Parses `<w>` elements with their attributes (lemma, morph, gloss, xlit)
/// and returns a vector of word data for interlinear display.
///
/// # Example
///
/// ```rust
/// use rsword_chirho::filters_chirho::extract_interlinear_words_chirho;
///
/// let osis_chirho = r#"<w lemma="strong:H7225" morph="oshm:HNcfsa" gloss="In [the] beginning">בְּרֵאשִׁית</w>"#;
/// let words_chirho = extract_interlinear_words_chirho(osis_chirho);
/// assert_eq!(words_chirho.len(), 1);
/// assert_eq!(words_chirho[0].gloss_chirho, "In [the] beginning");
/// assert_eq!(words_chirho[0].strongs_chirho, "H7225");
/// ```
pub fn extract_interlinear_words_chirho(osis_text_chirho: &str) -> Vec<InterlinearWordDataChirho> {
    let mut words_chirho = Vec::new();

    for cap_chirho in OSIS_WORD_REGEX_CHIRHO.captures_iter(osis_text_chirho) {
        let attrs_chirho = &cap_chirho[1];
        let word_text_chirho = &cap_chirho[2];

        // Extract gloss (English meaning)
        let gloss_chirho = OSIS_GLOSS_REGEX_CHIRHO
            .captures(attrs_chirho)
            .map(|c_chirho| c_chirho[1].to_string())
            .unwrap_or_default();

        // Extract Strong's numbers
        let strongs_list_chirho: Vec<String> = OSIS_STRONGS_REGEX_CHIRHO
            .captures_iter(attrs_chirho)
            .map(|c_chirho| c_chirho[1].to_string())
            .collect();
        let strongs_chirho = strongs_list_chirho.join(", ");

        // Extract morphology (strip scheme prefix like "robinson:", "oshm:", etc.)
        let morphology_raw_chirho = OSIS_MORPH_REGEX_CHIRHO
            .captures(attrs_chirho)
            .map(|c_chirho| c_chirho[1].to_string())
            .unwrap_or_default();
        let morphology_chirho = morphology_raw_chirho
            .split(':')
            .last()
            .unwrap_or(&morphology_raw_chirho)
            .to_string();

        // Extract transliteration
        let transliteration_chirho = OSIS_XLIT_REGEX_CHIRHO
            .captures(attrs_chirho)
            .map(|c_chirho| c_chirho[1].to_string())
            .unwrap_or_default();

        // Extract lemma
        let lemma_chirho = OSIS_LEMMA_REGEX_CHIRHO
            .captures(attrs_chirho)
            .map(|c_chirho| c_chirho[1].to_string())
            .unwrap_or_default();

        // Determine if Hebrew (H prefix) or Greek (G prefix)
        let is_hebrew_chirho = strongs_list_chirho
            .first()
            .map(|s_chirho| s_chirho.starts_with('H'))
            .unwrap_or(false);

        // Extract part of speech from morphology
        let part_of_speech_chirho = if !morphology_chirho.is_empty() {
            extract_pos_from_morph_chirho(&morphology_chirho)
        } else {
            String::new()
        };

        words_chirho.push(InterlinearWordDataChirho {
            original_chirho: word_text_chirho.to_string(),
            transliteration_chirho,
            morphology_chirho,
            strongs_chirho,
            gloss_chirho,
            part_of_speech_chirho,
            is_hebrew_chirho,
            lemma_chirho,
        });
    }

    words_chirho
}

/// Extract part of speech from morphology code.
fn extract_pos_from_morph_chirho(morph_chirho: &str) -> String {
    // Handle Robinson morphology (Greek): V-AAI-3S, N-NSM, etc.
    // Handle OSHM morphology (Hebrew): HNcfsa, HVqp3ms, etc.
    let code_chirho = morph_chirho.split(':').next_back().unwrap_or(morph_chirho);

    if code_chirho.starts_with('V') || code_chirho.contains("Vq") || code_chirho.contains("Vh") {
        "Verb".to_string()
    } else if code_chirho.starts_with('N') || code_chirho.starts_with("HN") {
        "Noun".to_string()
    } else if code_chirho.starts_with('A') || code_chirho.starts_with("HA") {
        "Adjective".to_string()
    } else if code_chirho.starts_with('P') || code_chirho.starts_with("HP") {
        if code_chirho.contains("prep") || code_chirho.starts_with("PREP") {
            "Preposition".to_string()
        } else {
            "Pronoun".to_string()
        }
    } else if code_chirho.contains("Conj") || code_chirho.starts_with('C') || code_chirho.starts_with("HC") {
        "Conjunction".to_string()
    } else if code_chirho.contains("Adv") || code_chirho.starts_with("ADV") {
        "Adverb".to_string()
    } else if code_chirho.contains("Art") || code_chirho.starts_with("T") {
        "Article".to_string()
    } else if code_chirho.contains("Part") || code_chirho.starts_with("PRT") {
        "Particle".to_string()
    } else if code_chirho.contains("Intj") || code_chirho.starts_with("INJ") {
        "Interjection".to_string()
    } else if !code_chirho.is_empty() {
        code_chirho.chars().take(4).collect()
    } else {
        String::new()
    }
}

/// Convert OSIS markup to HTML.
pub struct OsisToHtmlFilterChirho {
    options_chirho: FilterOptionsChirho,
}

impl OsisToHtmlFilterChirho {
    /// Create a new OSIS to HTML filter with default options.
    pub fn new_chirho() -> Self {
        Self {
            options_chirho: FilterOptionsChirho::default(),
        }
    }

    /// Create a new OSIS to HTML filter with specified options.
    pub fn with_options_chirho(options_chirho: FilterOptionsChirho) -> Self {
        Self { options_chirho }
    }

    /// Process a word element with Strong's and morphology.
    fn process_word_chirho(&self, attrs_chirho: &str, content_chirho: &str) -> String {
        let mut result_chirho = String::new();
        let escaped_content_chirho = escape_html_chirho(content_chirho);

        // Extract Strong's numbers
        let strongs_chirho: Vec<String> = OSIS_STRONGS_REGEX_CHIRHO
            .captures_iter(attrs_chirho)
            .map(|cap_chirho| cap_chirho[1].to_string())
            .collect();

        // Extract morphology
        let morph_chirho = OSIS_MORPH_REGEX_CHIRHO
            .captures(attrs_chirho)
            .map(|cap_chirho| cap_chirho[1].to_string());

        // Build HTML output
        if self.options_chirho.strongs_chirho && !strongs_chirho.is_empty() {
            let strongs_str_chirho = strongs_chirho.join(" ");
            result_chirho.push_str(&format!(
                r#"<span class="word" data-strongs="{}">{}</span>"#,
                escape_html_attr_chirho(&strongs_str_chirho),
                escaped_content_chirho
            ));

            // Add Strong's number superscript
            for s_chirho in &strongs_chirho {
                let escaped_strongs_chirho = escape_html_attr_chirho(s_chirho);
                result_chirho.push_str(&format!(
                    r#"<sup class="strongs"><a href="strongs://{}">{}</a></sup>"#,
                    escaped_strongs_chirho,
                    escape_html_chirho(s_chirho)
                ));
            }
        } else {
            result_chirho.push_str(&escaped_content_chirho);
        }

        // Add morphology
        if self.options_chirho.morph_chirho {
            if let Some(m_chirho) = morph_chirho {
                result_chirho.push_str(&format!(
                    r#"<sup class="morph">{}</sup>"#,
                    escape_html_chirho(&m_chirho)
                ));
            }
        }

        result_chirho
    }
}

impl Default for OsisToHtmlFilterChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

impl FilterChirho for OsisToHtmlFilterChirho {
    fn process_chirho(&self, text_chirho: &str) -> ResultChirho<String> {
        let mut result_chirho = text_chirho.to_string();

        // Remove self-closing word tags first (e.g., <w lemma="..." />)
        result_chirho = OSIS_WORD_SELF_CLOSING_REGEX_CHIRHO
            .replace_all(&result_chirho, "")
            .to_string();

        // Process word elements with Strong's/morphology
        result_chirho = OSIS_WORD_REGEX_CHIRHO
            .replace_all(&result_chirho, |caps_chirho: &regex::Captures| {
                self.process_word_chirho(&caps_chirho[1], &caps_chirho[2])
            })
            .to_string();

        // Process notes/footnotes
        if self.options_chirho.footnotes_chirho {
            result_chirho = OSIS_NOTE_REGEX_CHIRHO
                .replace_all(&result_chirho, |caps_chirho: &regex::Captures| {
                    format!(r#"<span class="footnote">[{}]</span>"#, escape_html_chirho(&caps_chirho[1]))
                })
                .to_string();
        } else {
            result_chirho = OSIS_NOTE_REGEX_CHIRHO.replace_all(&result_chirho, "").to_string();
        }

        // Process titles/headings
        if self.options_chirho.headings_chirho {
            result_chirho = OSIS_TITLE_REGEX_CHIRHO
                .replace_all(&result_chirho, |caps_chirho: &regex::Captures| {
                    format!(r#"<h3 class="heading">{}</h3>"#, escape_html_chirho(&caps_chirho[1]))
                })
                .to_string();
        } else {
            result_chirho = OSIS_TITLE_REGEX_CHIRHO.replace_all(&result_chirho, "").to_string();
        }

        // Process scripture references
        if self.options_chirho.scripref_chirho {
            result_chirho = OSIS_REFERENCE_REGEX_CHIRHO
                .replace_all(&result_chirho, |caps_chirho: &regex::Captures| {
                    let ref_chirho = &caps_chirho[1];
                    format!(
                        r#"<a class="scripref" href="bible://{}">{}</a>"#,
                        escape_html_attr_chirho(ref_chirho),
                        escape_html_chirho(ref_chirho)
                    )
                })
                .to_string();
        } else {
            result_chirho = OSIS_REFERENCE_REGEX_CHIRHO
                .replace_all(&result_chirho, |caps_chirho: &regex::Captures| {
                    escape_html_chirho(&caps_chirho[1])
                })
                .to_string();
        }

        // Process divine name (LORD)
        result_chirho = OSIS_DIVINE_NAME_REGEX_CHIRHO
            .replace_all(&result_chirho, |caps_chirho: &regex::Captures| {
                format!(r#"<span class="divine-name">{}</span>"#, escape_html_chirho(&caps_chirho[1]))
            })
            .to_string();

        // Process words of Jesus (red letter)
        if self.options_chirho.red_letter_chirho {
            result_chirho = OSIS_JESUS_WORDS_REGEX_CHIRHO
                .replace_all(&result_chirho, |caps_chirho: &regex::Captures| {
                    format!(r#"<span class="jesus-words">{}</span>"#, escape_html_chirho(&caps_chirho[1]))
                })
                .to_string();
        }

        // Convert OSIS structural elements to HTML
        result_chirho = result_chirho
            .replace("<verse ", "<span class=\"verse\" ")
            .replace("</verse>", "</span>")
            .replace("<chapter ", "<div class=\"chapter\" ")
            .replace("</chapter>", "</div>")
            .replace("<lb/>", "<br/>")
            .replace("<lb />", "<br/>");

        // Remove remaining OSIS-specific tags
        result_chirho = Regex::new(r#"</?seg[^>]*>"#)
            .unwrap()
            .replace_all(&result_chirho, "")
            .to_string();

        result_chirho = Regex::new(r#"<milestone[^>]*/>"#)
            .unwrap()
            .replace_all(&result_chirho, "")
            .to_string();

        Ok(result_chirho)
    }

    fn name_chirho(&self) -> &str {
        "OSIS to HTML"
    }
}

/// Convert OSIS markup to plain text.
pub struct OsisToPlainFilterChirho {
    options_chirho: FilterOptionsChirho,
}

impl OsisToPlainFilterChirho {
    /// Create a new OSIS to plain text filter.
    pub fn new_chirho() -> Self {
        Self {
            options_chirho: FilterOptionsChirho::none_chirho(),
        }
    }

    /// Create with options (Strong's numbers can still be shown inline).
    pub fn with_options_chirho(options_chirho: FilterOptionsChirho) -> Self {
        Self { options_chirho }
    }
}

impl Default for OsisToPlainFilterChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

impl FilterChirho for OsisToPlainFilterChirho {
    fn process_chirho(&self, text_chirho: &str) -> ResultChirho<String> {
        let mut result_chirho = text_chirho.to_string();

        // Remove self-closing word tags first (e.g., <w lemma="..." />)
        // These are empty word markers used in some modules and contain no displayable content
        result_chirho = OSIS_WORD_SELF_CLOSING_REGEX_CHIRHO
            .replace_all(&result_chirho, "")
            .to_string();

        // Process word elements - extract text, optionally add Strong's
        result_chirho = OSIS_WORD_REGEX_CHIRHO
            .replace_all(&result_chirho, |caps_chirho: &regex::Captures| {
                let content_chirho = &caps_chirho[2];
                if self.options_chirho.strongs_chirho {
                    let strongs_chirho: Vec<String> = OSIS_STRONGS_REGEX_CHIRHO
                        .captures_iter(&caps_chirho[1])
                        .map(|cap_chirho| cap_chirho[1].to_string())
                        .collect();
                    if !strongs_chirho.is_empty() {
                        format!("{} ({})", content_chirho, strongs_chirho.join(","))
                    } else {
                        content_chirho.to_string()
                    }
                } else {
                    content_chirho.to_string()
                }
            })
            .to_string();

        // Process notes
        if self.options_chirho.footnotes_chirho {
            result_chirho = OSIS_NOTE_REGEX_CHIRHO
                .replace_all(&result_chirho, |caps_chirho: &regex::Captures| {
                    format!(" [Note: {}]", &caps_chirho[1])
                })
                .to_string();
        } else {
            result_chirho = OSIS_NOTE_REGEX_CHIRHO.replace_all(&result_chirho, "").to_string();
        }

        // Process headings
        if self.options_chirho.headings_chirho {
            result_chirho = OSIS_TITLE_REGEX_CHIRHO
                .replace_all(&result_chirho, |caps_chirho: &regex::Captures| {
                    format!("\n=== {} ===\n", &caps_chirho[1])
                })
                .to_string();
        } else {
            result_chirho = OSIS_TITLE_REGEX_CHIRHO.replace_all(&result_chirho, "").to_string();
        }

        // Extract reference text
        result_chirho = OSIS_REFERENCE_REGEX_CHIRHO
            .replace_all(&result_chirho, "$1")
            .to_string();

        // Extract divine name text
        result_chirho = OSIS_DIVINE_NAME_REGEX_CHIRHO
            .replace_all(&result_chirho, "$1")
            .to_string();

        // Extract Jesus words text
        result_chirho = OSIS_JESUS_WORDS_REGEX_CHIRHO
            .replace_all(&result_chirho, "$1")
            .to_string();

        // Convert line breaks
        result_chirho = result_chirho
            .replace("<lb/>", "\n")
            .replace("<lb />", "\n");

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
        "OSIS to Plain"
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_osis_to_plain_basic_chirho() {
        let filter_chirho = OsisToPlainFilterChirho::new_chirho();
        let input_chirho = r#"<verse osisID="Gen.1.1">In the beginning God created</verse>"#;
        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(result_chirho.contains("In the beginning"));
    }

    #[test]
    fn test_osis_to_plain_strongs_chirho() {
        let filter_chirho = OsisToPlainFilterChirho::with_options_chirho(FilterOptionsChirho {
            strongs_chirho: true,
            ..Default::default()
        });
        let input_chirho = r#"<w lemma="strong:G2316">God</w>"#;
        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(result_chirho.contains("God"));
        assert!(result_chirho.contains("G2316"));
    }

    #[test]
    fn test_osis_to_html_basic_chirho() {
        let filter_chirho = OsisToHtmlFilterChirho::new_chirho();
        let input_chirho = r#"<divineName>LORD</divineName>"#;
        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(result_chirho.contains("divine-name"));
        assert!(result_chirho.contains("LORD"));
    }

    #[test]
    fn test_osis_to_html_strongs_chirho() {
        let filter_chirho = OsisToHtmlFilterChirho::with_options_chirho(FilterOptionsChirho {
            strongs_chirho: true,
            ..Default::default()
        });
        let input_chirho = r#"<w lemma="strong:G2316">God</w>"#;
        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(result_chirho.contains("data-strongs"));
        assert!(result_chirho.contains("G2316"));
    }

    #[test]
    fn test_osis_footnotes_chirho() {
        let filter_chirho = OsisToHtmlFilterChirho::with_options_chirho(FilterOptionsChirho {
            footnotes_chirho: true,
            ..Default::default()
        });
        let input_chirho = r#"text <note>footnote text</note> more"#;
        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(result_chirho.contains("footnote"));
    }
}
