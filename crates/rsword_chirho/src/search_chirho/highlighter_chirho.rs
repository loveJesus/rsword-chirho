// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Search result highlighting.
//!
//! Provides functionality to highlight matched terms in search results.
//!
//! # Highlighting Styles
//!
//! | Style | Description | Example Output |
//! |-------|-------------|----------------|
//! | HTML | HTML `<mark>` tags | `For <mark>God</mark> so loved` |
//! | ANSI | Terminal color codes | `For \x1b[1;33mGod\x1b[0m so loved` |
//! | Plain | Asterisk markers | `For *God* so loved` |
//! | Custom | User-defined delimiters | Configurable |
//!
//! # Usage
//!
//! ```rust,ignore
//! use rsword_chirho::search_chirho::highlighter_chirho::{
//!     HighlighterChirho, HighlightStyleChirho, HighlightOptionsChirho,
//! };
//!
//! let highlighter_chirho = HighlighterChirho::new_chirho(HighlightStyleChirho::HtmlChirho);
//!
//! let text_chirho = "For God so loved the world";
//! let terms_chirho = vec!["God", "loved"];
//! let highlighted_chirho = highlighter_chirho.highlight_chirho(text_chirho, &terms_chirho);
//!
//! assert_eq!(highlighted_chirho, "For <mark>God</mark> so <mark>loved</mark> the world");
//! ```

use std::collections::HashSet;

/// Highlighting style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HighlightStyleChirho {
    /// HTML `<mark>` tags.
    #[default]
    HtmlChirho,
    /// Terminal ANSI color codes (yellow bold).
    AnsiChirho,
    /// Plain text with asterisks.
    PlainChirho,
    /// Custom delimiters.
    CustomChirho,
}

/// Options for highlighting.
#[derive(Debug, Clone)]
pub struct HighlightOptionsChirho {
    /// Highlighting style.
    pub style_chirho: HighlightStyleChirho,
    /// Custom prefix (for CustomChirho style).
    pub prefix_chirho: String,
    /// Custom suffix (for CustomChirho style).
    pub suffix_chirho: String,
    /// Case-insensitive matching.
    pub case_insensitive_chirho: bool,
    /// Maximum snippet length (0 = no limit).
    pub max_snippet_length_chirho: usize,
    /// Context words to include around matches.
    pub context_words_chirho: usize,
}

impl Default for HighlightOptionsChirho {
    fn default() -> Self {
        Self {
            style_chirho: HighlightStyleChirho::HtmlChirho,
            prefix_chirho: String::new(),
            suffix_chirho: String::new(),
            case_insensitive_chirho: true,
            max_snippet_length_chirho: 0,
            context_words_chirho: 10,
        }
    }
}

impl HighlightOptionsChirho {
    /// Create options with HTML highlighting.
    pub fn html_chirho() -> Self {
        Self {
            style_chirho: HighlightStyleChirho::HtmlChirho,
            ..Default::default()
        }
    }

    /// Create options with ANSI highlighting.
    pub fn ansi_chirho() -> Self {
        Self {
            style_chirho: HighlightStyleChirho::AnsiChirho,
            ..Default::default()
        }
    }

    /// Create options with plain text highlighting.
    pub fn plain_chirho() -> Self {
        Self {
            style_chirho: HighlightStyleChirho::PlainChirho,
            ..Default::default()
        }
    }

    /// Create options with custom delimiters.
    pub fn custom_chirho(prefix_chirho: &str, suffix_chirho: &str) -> Self {
        Self {
            style_chirho: HighlightStyleChirho::CustomChirho,
            prefix_chirho: prefix_chirho.to_string(),
            suffix_chirho: suffix_chirho.to_string(),
            ..Default::default()
        }
    }

    /// Set case sensitivity.
    pub fn case_sensitive_chirho(mut self, sensitive_chirho: bool) -> Self {
        self.case_insensitive_chirho = !sensitive_chirho;
        self
    }

    /// Set maximum snippet length.
    pub fn max_snippet_chirho(mut self, length_chirho: usize) -> Self {
        self.max_snippet_length_chirho = length_chirho;
        self
    }

    /// Set context words around matches.
    pub fn context_chirho(mut self, words_chirho: usize) -> Self {
        self.context_words_chirho = words_chirho;
        self
    }
}

/// Text highlighter for search results.
#[derive(Debug, Clone)]
pub struct HighlighterChirho {
    /// Highlighting options.
    options_chirho: HighlightOptionsChirho,
}

impl Default for HighlighterChirho {
    fn default() -> Self {
        Self::new_chirho(HighlightStyleChirho::HtmlChirho)
    }
}

impl HighlighterChirho {
    /// Create a new highlighter with the specified style.
    pub fn new_chirho(style_chirho: HighlightStyleChirho) -> Self {
        Self {
            options_chirho: HighlightOptionsChirho {
                style_chirho,
                ..Default::default()
            },
        }
    }

    /// Create a highlighter with custom options.
    pub fn with_options_chirho(options_chirho: HighlightOptionsChirho) -> Self {
        Self { options_chirho }
    }

    /// Get the prefix for the current style.
    fn prefix_chirho(&self) -> &str {
        match self.options_chirho.style_chirho {
            HighlightStyleChirho::HtmlChirho => "<mark>",
            HighlightStyleChirho::AnsiChirho => "\x1b[1;33m", // Yellow bold
            HighlightStyleChirho::PlainChirho => "*",
            HighlightStyleChirho::CustomChirho => &self.options_chirho.prefix_chirho,
        }
    }

    /// Get the suffix for the current style.
    fn suffix_chirho(&self) -> &str {
        match self.options_chirho.style_chirho {
            HighlightStyleChirho::HtmlChirho => "</mark>",
            HighlightStyleChirho::AnsiChirho => "\x1b[0m", // Reset
            HighlightStyleChirho::PlainChirho => "*",
            HighlightStyleChirho::CustomChirho => &self.options_chirho.suffix_chirho,
        }
    }

    /// Highlight matching terms in text.
    ///
    /// # Arguments
    ///
    /// * `text_chirho` - The text to highlight
    /// * `terms_chirho` - Terms to highlight
    ///
    /// # Returns
    ///
    /// Text with highlighted terms.
    pub fn highlight_chirho(&self, text_chirho: &str, terms_chirho: &[&str]) -> String {
        if terms_chirho.is_empty() {
            return text_chirho.to_string();
        }

        // Build set of terms for efficient lookup
        let term_set_chirho: HashSet<String> = if self.options_chirho.case_insensitive_chirho {
            terms_chirho.iter().map(|t_chirho| t_chirho.to_lowercase()).collect()
        } else {
            terms_chirho.iter().map(|t_chirho| t_chirho.to_string()).collect()
        };

        let mut result_chirho = String::with_capacity(text_chirho.len() * 2);
        let mut word_start_chirho = 0;
        let mut in_word_chirho = false;

        for (i_chirho, c_chirho) in text_chirho.char_indices() {
            if c_chirho.is_alphanumeric() || c_chirho == '\'' {
                if !in_word_chirho {
                    word_start_chirho = i_chirho;
                    in_word_chirho = true;
                }
            } else {
                if in_word_chirho {
                    let word_chirho = &text_chirho[word_start_chirho..i_chirho];
                    let lookup_word_chirho = if self.options_chirho.case_insensitive_chirho {
                        word_chirho.to_lowercase()
                    } else {
                        word_chirho.to_string()
                    };

                    if term_set_chirho.contains(&lookup_word_chirho) {
                        result_chirho.push_str(self.prefix_chirho());
                        result_chirho.push_str(word_chirho);
                        result_chirho.push_str(self.suffix_chirho());
                    } else {
                        result_chirho.push_str(word_chirho);
                    }
                    in_word_chirho = false;
                }
                result_chirho.push(c_chirho);
            }
        }

        // Handle final word if text doesn't end with separator
        if in_word_chirho {
            let word_chirho = &text_chirho[word_start_chirho..];
            let lookup_word_chirho = if self.options_chirho.case_insensitive_chirho {
                word_chirho.to_lowercase()
            } else {
                word_chirho.to_string()
            };

            if term_set_chirho.contains(&lookup_word_chirho) {
                result_chirho.push_str(self.prefix_chirho());
                result_chirho.push_str(word_chirho);
                result_chirho.push_str(self.suffix_chirho());
            } else {
                result_chirho.push_str(word_chirho);
            }
        }

        result_chirho
    }

    /// Create a snippet with highlighted terms.
    ///
    /// Extracts a portion of text around matched terms with context.
    pub fn snippet_chirho(&self, text_chirho: &str, terms_chirho: &[&str]) -> String {
        if terms_chirho.is_empty() || self.options_chirho.max_snippet_length_chirho == 0 {
            return self.highlight_chirho(text_chirho, terms_chirho);
        }

        let words_chirho: Vec<&str> = text_chirho.split_whitespace().collect();
        if words_chirho.is_empty() {
            return String::new();
        }

        // Find first matching word index
        let term_set_chirho: HashSet<String> = if self.options_chirho.case_insensitive_chirho {
            terms_chirho.iter().map(|t_chirho| t_chirho.to_lowercase()).collect()
        } else {
            terms_chirho.iter().map(|t_chirho| t_chirho.to_string()).collect()
        };

        let first_match_chirho = words_chirho.iter().position(|w_chirho| {
            let clean_chirho = w_chirho.trim_matches(|c_chirho: char| !c_chirho.is_alphanumeric());
            let lookup_chirho = if self.options_chirho.case_insensitive_chirho {
                clean_chirho.to_lowercase()
            } else {
                clean_chirho.to_string()
            };
            term_set_chirho.contains(&lookup_chirho)
        });

        let center_chirho = first_match_chirho.unwrap_or(0);
        let context_chirho = self.options_chirho.context_words_chirho;

        let start_chirho = center_chirho.saturating_sub(context_chirho);
        let end_chirho = (center_chirho + context_chirho + 1).min(words_chirho.len());

        let snippet_words_chirho = &words_chirho[start_chirho..end_chirho];
        let mut snippet_chirho = String::new();

        if start_chirho > 0 {
            snippet_chirho.push_str("...");
        }

        snippet_chirho.push_str(&snippet_words_chirho.join(" "));

        if end_chirho < words_chirho.len() {
            snippet_chirho.push_str("...");
        }

        self.highlight_chirho(&snippet_chirho, terms_chirho)
    }

    /// Highlight a phrase match in text.
    ///
    /// Highlights the entire phrase as a unit rather than individual words.
    pub fn highlight_phrase_chirho(&self, text_chirho: &str, phrase_chirho: &str) -> String {
        let (search_text_chirho, search_phrase_chirho) = if self.options_chirho.case_insensitive_chirho {
            (text_chirho.to_lowercase(), phrase_chirho.to_lowercase())
        } else {
            (text_chirho.to_string(), phrase_chirho.to_string())
        };

        let mut result_chirho = String::with_capacity(text_chirho.len() * 2);
        let mut last_end_chirho = 0;

        for (start_chirho, _) in search_text_chirho.match_indices(&search_phrase_chirho) {
            let end_chirho = start_chirho + phrase_chirho.len();

            // Add text before match
            result_chirho.push_str(&text_chirho[last_end_chirho..start_chirho]);

            // Add highlighted match (using original case from text)
            result_chirho.push_str(self.prefix_chirho());
            result_chirho.push_str(&text_chirho[start_chirho..end_chirho]);
            result_chirho.push_str(self.suffix_chirho());

            last_end_chirho = end_chirho;
        }

        // Add remaining text
        result_chirho.push_str(&text_chirho[last_end_chirho..]);

        result_chirho
    }
}

/// Search result with highlighted text.
#[derive(Debug, Clone)]
pub struct HighlightedResultChirho {
    /// The key (verse reference or entry key).
    pub key_chirho: String,
    /// Original text content.
    pub original_chirho: String,
    /// Highlighted text.
    pub highlighted_chirho: String,
    /// Snippet with context.
    pub snippet_chirho: Option<String>,
    /// Relevance score.
    pub score_chirho: f32,
}

impl HighlightedResultChirho {
    /// Create a new highlighted result.
    pub fn new_chirho(
        key_chirho: String,
        original_chirho: String,
        highlighted_chirho: String,
        score_chirho: f32,
    ) -> Self {
        Self {
            key_chirho,
            original_chirho,
            highlighted_chirho,
            snippet_chirho: None,
            score_chirho,
        }
    }

    /// Create with a snippet.
    pub fn with_snippet_chirho(mut self, snippet_chirho: String) -> Self {
        self.snippet_chirho = Some(snippet_chirho);
        self
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_highlight_html_chirho() {
        let highlighter_chirho = HighlighterChirho::new_chirho(HighlightStyleChirho::HtmlChirho);
        let text_chirho = "For God so loved the world";
        let result_chirho = highlighter_chirho.highlight_chirho(text_chirho, &["God", "loved"]);

        assert_eq!(result_chirho, "For <mark>God</mark> so <mark>loved</mark> the world");
    }

    #[test]
    fn test_highlight_ansi_chirho() {
        let highlighter_chirho = HighlighterChirho::new_chirho(HighlightStyleChirho::AnsiChirho);
        let text_chirho = "For God so loved";
        let result_chirho = highlighter_chirho.highlight_chirho(text_chirho, &["God"]);

        assert!(result_chirho.contains("\x1b[1;33m"));
        assert!(result_chirho.contains("\x1b[0m"));
    }

    #[test]
    fn test_highlight_plain_chirho() {
        let highlighter_chirho = HighlighterChirho::new_chirho(HighlightStyleChirho::PlainChirho);
        let text_chirho = "For God so loved";
        let result_chirho = highlighter_chirho.highlight_chirho(text_chirho, &["God"]);

        assert_eq!(result_chirho, "For *God* so loved");
    }

    #[test]
    fn test_highlight_custom_chirho() {
        let options_chirho = HighlightOptionsChirho::custom_chirho("[", "]");
        let highlighter_chirho = HighlighterChirho::with_options_chirho(options_chirho);
        let text_chirho = "For God so loved";
        let result_chirho = highlighter_chirho.highlight_chirho(text_chirho, &["God"]);

        assert_eq!(result_chirho, "For [God] so loved");
    }

    #[test]
    fn test_highlight_case_insensitive_chirho() {
        let highlighter_chirho = HighlighterChirho::default();
        let text_chirho = "GOD is good, God is great";
        let result_chirho = highlighter_chirho.highlight_chirho(text_chirho, &["god"]);

        assert_eq!(result_chirho, "<mark>GOD</mark> is good, <mark>God</mark> is great");
    }

    #[test]
    fn test_highlight_phrase_chirho() {
        let highlighter_chirho = HighlighterChirho::default();
        let text_chirho = "For God so loved the world that he gave";
        let result_chirho = highlighter_chirho.highlight_phrase_chirho(text_chirho, "God so loved");

        assert_eq!(result_chirho, "For <mark>God so loved</mark> the world that he gave");
    }

    #[test]
    fn test_snippet_chirho() {
        let mut options_chirho = HighlightOptionsChirho::html_chirho();
        options_chirho.max_snippet_length_chirho = 100;
        options_chirho.context_words_chirho = 3;

        let highlighter_chirho = HighlighterChirho::with_options_chirho(options_chirho);
        let text_chirho = "In the beginning God created the heaven and the earth and everything in it";
        let snippet_chirho = highlighter_chirho.snippet_chirho(text_chirho, &["God"]);

        assert!(snippet_chirho.contains("<mark>God</mark>"));
        assert!(snippet_chirho.contains("...") || !snippet_chirho.contains("everything"));
    }

    #[test]
    fn test_no_terms_chirho() {
        let highlighter_chirho = HighlighterChirho::default();
        let text_chirho = "For God so loved the world";
        let result_chirho = highlighter_chirho.highlight_chirho(text_chirho, &[]);

        assert_eq!(result_chirho, text_chirho);
    }

    #[test]
    fn test_highlighted_result_chirho() {
        let result_chirho = HighlightedResultChirho::new_chirho(
            "John.3.16".to_string(),
            "For God so loved".to_string(),
            "For <mark>God</mark> so <mark>loved</mark>".to_string(),
            0.95,
        );

        assert_eq!(result_chirho.key_chirho, "John.3.16");
        assert_eq!(result_chirho.score_chirho, 0.95);
    }
}
