// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! TEI markup to HTML/plain text conversion filters.
//!
//! Converts TEI (Text Encoding Initiative) markup used in dictionary/lexicon
//! modules to HTML or plain text.

use regex::Regex;
use std::sync::LazyLock;

use crate::error_chirho::ResultChirho;
use crate::filters_chirho::FilterChirho;

/// Pattern for TEI orth (headword) elements.
static TEI_ORTH_PATTERN_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<orth(?:\s[^>]*)?>(.+?)</orth>"#).unwrap()
});

/// Pattern for TEI sense elements.
static TEI_SENSE_PATTERN_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<sense(?:\s[^>]*)?>(.+?)</sense>"#).unwrap()
});

/// Pattern for TEI def (definition) elements.
static TEI_DEF_PATTERN_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<def(?:\s[^>]*)?>(.+?)</def>"#).unwrap()
});

/// Pattern for TEI quote elements.
static TEI_QUOTE_PATTERN_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<quote(?:\s[^>]*)?>(.+?)</quote>"#).unwrap()
});

/// Pattern for TEI cit (citation) elements.
static TEI_CIT_PATTERN_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<cit(?:\s[^>]*)?>(.+?)</cit>"#).unwrap()
});

/// Pattern for TEI gram (grammar) elements.
static TEI_GRAM_PATTERN_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<gram(?:\s[^>]*)?>(.+?)</gram>"#).unwrap()
});

/// Pattern for TEI ref (reference) elements.
static TEI_REF_PATTERN_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<ref\s+target="([^"]+)"[^>]*>(.+?)</ref>"#).unwrap()
});

/// Pattern for TEI note elements.
static TEI_NOTE_PATTERN_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<note(?:\s[^>]*)?>(.+?)</note>"#).unwrap()
});

/// Pattern for TEI entry elements (opening tag).
static TEI_ENTRY_PATTERN_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"<entry(?:\s[^>]*)?>"#).unwrap()
});

/// Pattern for any remaining TEI tags (only TEI-specific, not HTML).
/// This matches common TEI elements that aren't already handled.
static TEI_TAG_PATTERN_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"</?(?:entry|orth|sense|def|quote|cit|gram|ref|note|form|usg|etym|pos|pron|hi|pb|lb|foreign|gloss|bibl|author|title|date|milestone|seg|c|m|w|pc|ab|ptr)(?:\s[^>]*)?/?>"#).unwrap()
});

/// Filter to convert TEI markup to HTML.
#[derive(Debug, Clone, Default)]
pub struct TeiToHtmlFilterChirho {
    /// Show grammatical information.
    show_grammar_chirho: bool,
    /// Show examples/citations.
    show_examples_chirho: bool,
    /// Show cross-references as links.
    link_refs_chirho: bool,
}

impl TeiToHtmlFilterChirho {
    /// Create a new TEI to HTML filter.
    pub fn new_chirho() -> Self {
        Self {
            show_grammar_chirho: true,
            show_examples_chirho: true,
            link_refs_chirho: true,
        }
    }

    /// Set whether to show grammatical information.
    pub fn set_show_grammar_chirho(&mut self, show_chirho: bool) {
        self.show_grammar_chirho = show_chirho;
    }

    /// Set whether to show examples/citations.
    pub fn set_show_examples_chirho(&mut self, show_chirho: bool) {
        self.show_examples_chirho = show_chirho;
    }

    /// Set whether to link references.
    pub fn set_link_refs_chirho(&mut self, link_chirho: bool) {
        self.link_refs_chirho = link_chirho;
    }
}

impl FilterChirho for TeiToHtmlFilterChirho {
    fn name_chirho(&self) -> &str {
        "TEI to HTML"
    }

    fn process_chirho(&self, input_chirho: &str) -> ResultChirho<String> {
        let mut result_chirho = input_chirho.to_string();

        // Convert orth (headword) to bold
        result_chirho = TEI_ORTH_PATTERN_CHIRHO
            .replace_all(&result_chirho, "<strong class=\"headword\">$1</strong>")
            .to_string();

        // Convert def (definition) to span
        result_chirho = TEI_DEF_PATTERN_CHIRHO
            .replace_all(&result_chirho, "<span class=\"definition\">$1</span>")
            .to_string();

        // Convert gram (grammar) to italic
        if self.show_grammar_chirho {
            result_chirho = TEI_GRAM_PATTERN_CHIRHO
                .replace_all(&result_chirho, "<em class=\"grammar\">$1</em>")
                .to_string();
        } else {
            result_chirho = TEI_GRAM_PATTERN_CHIRHO
                .replace_all(&result_chirho, "")
                .to_string();
        }

        // Convert quote to blockquote
        if self.show_examples_chirho {
            result_chirho = TEI_QUOTE_PATTERN_CHIRHO
                .replace_all(&result_chirho, "<blockquote class=\"example\">$1</blockquote>")
                .to_string();

            result_chirho = TEI_CIT_PATTERN_CHIRHO
                .replace_all(&result_chirho, "<cite>$1</cite>")
                .to_string();
        } else {
            result_chirho = TEI_QUOTE_PATTERN_CHIRHO
                .replace_all(&result_chirho, "")
                .to_string();

            result_chirho = TEI_CIT_PATTERN_CHIRHO
                .replace_all(&result_chirho, "")
                .to_string();
        }

        // Convert ref to hyperlink
        if self.link_refs_chirho {
            result_chirho = TEI_REF_PATTERN_CHIRHO
                .replace_all(&result_chirho, "<a href=\"$1\" class=\"crossref\">$2</a>")
                .to_string();
        } else {
            result_chirho = TEI_REF_PATTERN_CHIRHO
                .replace_all(&result_chirho, "$2")
                .to_string();
        }

        // Convert note to superscript note indicator
        result_chirho = TEI_NOTE_PATTERN_CHIRHO
            .replace_all(&result_chirho, "<sup class=\"note\">[$1]</sup>")
            .to_string();

        // Convert sense to numbered list item
        result_chirho = TEI_SENSE_PATTERN_CHIRHO
            .replace_all(&result_chirho, "<li class=\"sense\">$1</li>")
            .to_string();

        // Convert entry to div
        result_chirho = TEI_ENTRY_PATTERN_CHIRHO
            .replace_all(&result_chirho, "<div class=\"entry\">")
            .to_string();
        result_chirho = result_chirho.replace("</entry>", "</div>");

        // Remove remaining TEI tags
        result_chirho = TEI_TAG_PATTERN_CHIRHO
            .replace_all(&result_chirho, "")
            .to_string();

        Ok(result_chirho)
    }
}

/// Filter to convert TEI markup to plain text.
#[derive(Debug, Clone, Default)]
pub struct TeiToPlainFilterChirho {
    /// Include grammar info in parentheses.
    include_grammar_chirho: bool,
    /// Include examples.
    include_examples_chirho: bool,
}

impl TeiToPlainFilterChirho {
    /// Create a new TEI to plain filter.
    pub fn new_chirho() -> Self {
        Self {
            include_grammar_chirho: true,
            include_examples_chirho: true,
        }
    }

    /// Set whether to include grammar info.
    pub fn set_include_grammar_chirho(&mut self, include_chirho: bool) {
        self.include_grammar_chirho = include_chirho;
    }

    /// Set whether to include examples.
    pub fn set_include_examples_chirho(&mut self, include_chirho: bool) {
        self.include_examples_chirho = include_chirho;
    }
}

impl FilterChirho for TeiToPlainFilterChirho {
    fn name_chirho(&self) -> &str {
        "TEI to Plain"
    }

    fn process_chirho(&self, input_chirho: &str) -> ResultChirho<String> {
        let mut result_chirho = input_chirho.to_string();

        // Keep orth text
        result_chirho = TEI_ORTH_PATTERN_CHIRHO
            .replace_all(&result_chirho, "$1")
            .to_string();

        // Keep def text
        result_chirho = TEI_DEF_PATTERN_CHIRHO
            .replace_all(&result_chirho, "$1")
            .to_string();

        // Convert gram to parentheses or remove
        if self.include_grammar_chirho {
            result_chirho = TEI_GRAM_PATTERN_CHIRHO
                .replace_all(&result_chirho, "($1)")
                .to_string();
        } else {
            result_chirho = TEI_GRAM_PATTERN_CHIRHO
                .replace_all(&result_chirho, "")
                .to_string();
        }

        // Convert quotes or remove
        if self.include_examples_chirho {
            result_chirho = TEI_QUOTE_PATTERN_CHIRHO
                .replace_all(&result_chirho, "\"$1\"")
                .to_string();

            result_chirho = TEI_CIT_PATTERN_CHIRHO
                .replace_all(&result_chirho, "($1)")
                .to_string();
        } else {
            result_chirho = TEI_QUOTE_PATTERN_CHIRHO
                .replace_all(&result_chirho, "")
                .to_string();

            result_chirho = TEI_CIT_PATTERN_CHIRHO
                .replace_all(&result_chirho, "")
                .to_string();
        }

        // Keep ref text only
        result_chirho = TEI_REF_PATTERN_CHIRHO
            .replace_all(&result_chirho, "$2")
            .to_string();

        // Convert note to brackets
        result_chirho = TEI_NOTE_PATTERN_CHIRHO
            .replace_all(&result_chirho, "[$1]")
            .to_string();

        // Keep sense text with numbers
        let mut sense_count_chirho = 0u32;
        result_chirho = TEI_SENSE_PATTERN_CHIRHO
            .replace_all(&result_chirho, |_caps_chirho: &regex::Captures| {
                sense_count_chirho += 1;
                format!("{}. ", sense_count_chirho)
            })
            .to_string();

        // Remove remaining tags
        result_chirho = TEI_TAG_PATTERN_CHIRHO
            .replace_all(&result_chirho, "")
            .to_string();

        // Clean up whitespace
        result_chirho = result_chirho
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");

        Ok(result_chirho)
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_tei_to_html_orth_chirho() {
        let filter_chirho = TeiToHtmlFilterChirho::new_chirho();
        let input_chirho = "<orth>love</orth>";

        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(result_chirho.contains("<strong"));
        assert!(result_chirho.contains("love"));
    }

    #[test]
    fn test_tei_to_html_def_chirho() {
        let filter_chirho = TeiToHtmlFilterChirho::new_chirho();
        let input_chirho = "<def>a strong affection</def>";

        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(result_chirho.contains("definition"));
        assert!(result_chirho.contains("a strong affection"));
    }

    #[test]
    fn test_tei_to_html_ref_chirho() {
        let filter_chirho = TeiToHtmlFilterChirho::new_chirho();
        let input_chirho = r#"<ref target="G26">agape</ref>"#;

        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(result_chirho.contains("href=\"G26\""));
        assert!(result_chirho.contains("agape"));
    }

    #[test]
    fn test_tei_to_plain_chirho() {
        let filter_chirho = TeiToPlainFilterChirho::new_chirho();
        let input_chirho = "<orth>love</orth> <gram>noun</gram> <def>affection</def>";

        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(result_chirho.contains("love"));
        assert!(result_chirho.contains("(noun)"));
        assert!(result_chirho.contains("affection"));
    }

    #[test]
    fn test_tei_to_plain_no_grammar_chirho() {
        let mut filter_chirho = TeiToPlainFilterChirho::new_chirho();
        filter_chirho.set_include_grammar_chirho(false);

        let input_chirho = "<orth>love</orth> <gram>noun</gram>";

        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(!result_chirho.contains("noun"));
    }

    #[test]
    fn test_tei_quote_html_chirho() {
        let filter_chirho = TeiToHtmlFilterChirho::new_chirho();
        let input_chirho = "<quote>God is love</quote>";

        let result_chirho = filter_chirho.process_chirho(input_chirho).unwrap();
        assert!(result_chirho.contains("blockquote"));
        assert!(result_chirho.contains("God is love"));
    }

    #[test]
    fn test_filter_name_chirho() {
        let html_filter_chirho = TeiToHtmlFilterChirho::new_chirho();
        let plain_filter_chirho = TeiToPlainFilterChirho::new_chirho();

        assert_eq!(html_filter_chirho.name_chirho(), "TEI to HTML");
        assert_eq!(plain_filter_chirho.name_chirho(), "TEI to Plain");
    }
}
