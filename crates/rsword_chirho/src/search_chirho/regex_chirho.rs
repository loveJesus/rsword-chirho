// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Regex-based search engine.
//!
//! Provides simple search functionality using regular expressions.
//! Used when no tantivy index is available.

use regex::{Regex, RegexBuilder};

use crate::error_chirho::{ErrorChirho, ResultChirho};
use crate::keys_chirho::{ListKeyChirho, StrKeyChirho};
use super::{SearchEngineChirho, SearchOptionsChirho, SearchTypeChirho};

/// Entry for regex search.
#[derive(Debug, Clone)]
pub struct SearchEntryChirho {
    /// Entry key (verse reference, lexicon key, etc.).
    pub key_chirho: String,
    /// Entry text content.
    pub text_chirho: String,
}

/// Regex-based search engine.
///
/// This engine performs a linear scan of all entries,
/// so it's slower than tantivy for large modules.
pub struct RegexSearchChirho {
    /// Entries to search.
    entries_chirho: Vec<SearchEntryChirho>,
}

impl RegexSearchChirho {
    /// Create a new regex search engine.
    pub fn new_chirho() -> Self {
        Self {
            entries_chirho: Vec::new(),
        }
    }

    /// Add an entry to the search corpus.
    pub fn add_entry_chirho(&mut self, key_chirho: String, text_chirho: String) {
        self.entries_chirho.push(SearchEntryChirho {
            key_chirho,
            text_chirho,
        });
    }

    /// Get the number of entries.
    pub fn entry_count_chirho(&self) -> usize {
        self.entries_chirho.len()
    }

    /// Clear all entries.
    pub fn clear_chirho(&mut self) {
        self.entries_chirho.clear();
    }

    /// Search for a phrase (literal text).
    fn search_phrase_chirho(
        &self,
        phrase_chirho: &str,
        case_insensitive_chirho: bool,
        max_results_chirho: usize,
    ) -> ResultChirho<ListKeyChirho> {
        let pattern_chirho = regex::escape(phrase_chirho);
        let regex_chirho = RegexBuilder::new(&pattern_chirho)
            .case_insensitive(case_insensitive_chirho)
            .build()
            .map_err(|e_chirho| ErrorChirho::invalid_config_chirho(
                format!("Invalid regex pattern: {}", e_chirho)
            ))?;

        self.search_with_regex_chirho(&regex_chirho, max_results_chirho)
    }

    /// Search with a regex pattern.
    fn search_regex_chirho(
        &self,
        pattern_chirho: &str,
        case_insensitive_chirho: bool,
        max_results_chirho: usize,
    ) -> ResultChirho<ListKeyChirho> {
        let regex_chirho = RegexBuilder::new(pattern_chirho)
            .case_insensitive(case_insensitive_chirho)
            .build()
            .map_err(|e_chirho| ErrorChirho::invalid_config_chirho(
                format!("Invalid regex pattern: {}", e_chirho)
            ))?;

        self.search_with_regex_chirho(&regex_chirho, max_results_chirho)
    }

    /// Search for multiple words (AND).
    fn search_multiword_chirho(
        &self,
        words_chirho: &str,
        case_insensitive_chirho: bool,
        max_results_chirho: usize,
    ) -> ResultChirho<ListKeyChirho> {
        let word_list_chirho: Vec<&str> = words_chirho.split_whitespace().collect();
        if word_list_chirho.is_empty() {
            return Ok(ListKeyChirho::new_chirho());
        }

        // Build regex patterns for each word
        let regexes_chirho: Vec<Regex> = word_list_chirho
            .iter()
            .map(|w_chirho| {
                RegexBuilder::new(&regex::escape(w_chirho))
                    .case_insensitive(case_insensitive_chirho)
                    .build()
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e_chirho| ErrorChirho::invalid_config_chirho(
                format!("Invalid regex pattern: {}", e_chirho)
            ))?;

        let mut results_chirho = ListKeyChirho::new_chirho();
        let mut count_chirho = 0;

        for entry_chirho in &self.entries_chirho {
            if count_chirho >= max_results_chirho {
                break;
            }

            // All words must match
            let all_match_chirho = regexes_chirho
                .iter()
                .all(|r_chirho| r_chirho.is_match(&entry_chirho.text_chirho));

            if all_match_chirho {
                let key_chirho = Box::new(StrKeyChirho::with_text_chirho(&entry_chirho.key_chirho));
                results_chirho.add_chirho(key_chirho);
                count_chirho += 1;
            }
        }

        Ok(results_chirho)
    }

    /// Search using a compiled regex.
    fn search_with_regex_chirho(
        &self,
        regex_chirho: &Regex,
        max_results_chirho: usize,
    ) -> ResultChirho<ListKeyChirho> {
        let mut results_chirho = ListKeyChirho::new_chirho();
        let mut count_chirho = 0;

        for entry_chirho in &self.entries_chirho {
            if count_chirho >= max_results_chirho {
                break;
            }

            if regex_chirho.is_match(&entry_chirho.text_chirho) {
                let key_chirho = Box::new(StrKeyChirho::with_text_chirho(&entry_chirho.key_chirho));
                results_chirho.add_chirho(key_chirho);
                count_chirho += 1;
            }
        }

        Ok(results_chirho)
    }
}

impl Default for RegexSearchChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

impl SearchEngineChirho for RegexSearchChirho {
    fn search_chirho(
        &self,
        pattern_chirho: &str,
        options_chirho: &SearchOptionsChirho,
    ) -> ResultChirho<ListKeyChirho> {
        let max_results_chirho = options_chirho.max_results_chirho.unwrap_or(1000);
        let case_insensitive_chirho = options_chirho.case_insensitive_chirho;

        let search_type_chirho = options_chirho.search_type_chirho
            .unwrap_or(SearchTypeChirho::PhraseChirho);

        match search_type_chirho {
            SearchTypeChirho::RegexChirho => {
                self.search_regex_chirho(pattern_chirho, case_insensitive_chirho, max_results_chirho)
            }
            SearchTypeChirho::PhraseChirho => {
                self.search_phrase_chirho(pattern_chirho, case_insensitive_chirho, max_results_chirho)
            }
            SearchTypeChirho::MultiWordChirho => {
                self.search_multiword_chirho(pattern_chirho, case_insensitive_chirho, max_results_chirho)
            }
            SearchTypeChirho::ExternalChirho | SearchTypeChirho::EntryAttrChirho => {
                // Default to phrase search
                self.search_phrase_chirho(pattern_chirho, case_insensitive_chirho, max_results_chirho)
            }
        }
    }

    fn create_index_chirho(&mut self) -> ResultChirho<()> {
        // Regex search doesn't use a persistent index
        Ok(())
    }

    fn has_index_chirho(&self) -> bool {
        // Regex search always has "index" (in-memory entries)
        !self.entries_chirho.is_empty()
    }

    fn delete_index_chirho(&mut self) -> ResultChirho<()> {
        self.entries_chirho.clear();
        Ok(())
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    fn create_test_search_chirho() -> RegexSearchChirho {
        let mut search_chirho = RegexSearchChirho::new_chirho();

        search_chirho.add_entry_chirho(
            "Gen.1.1".to_string(),
            "In the beginning God created the heaven and the earth".to_string(),
        );

        search_chirho.add_entry_chirho(
            "John.3.16".to_string(),
            "For God so loved the world that he gave his only begotten Son".to_string(),
        );

        search_chirho.add_entry_chirho(
            "John.14.6".to_string(),
            "I am the way the truth and the life no man cometh unto the Father but by me".to_string(),
        );

        search_chirho
    }

    #[test]
    fn test_phrase_search_chirho() {
        let search_chirho = create_test_search_chirho();
        let options_chirho = SearchOptionsChirho {
            search_type_chirho: Some(SearchTypeChirho::PhraseChirho),
            max_results_chirho: Some(10),
            ..Default::default()
        };

        let results_chirho = search_chirho.search_chirho("God", &options_chirho).unwrap();
        assert_eq!(results_chirho.count_chirho(), 2);

        let results_chirho = search_chirho.search_chirho("the way", &options_chirho).unwrap();
        assert_eq!(results_chirho.count_chirho(), 1);
    }

    #[test]
    fn test_regex_search_chirho() {
        let search_chirho = create_test_search_chirho();
        let options_chirho = SearchOptionsChirho {
            search_type_chirho: Some(SearchTypeChirho::RegexChirho),
            max_results_chirho: Some(10),
            ..Default::default()
        };

        // Match "God" or "gave"
        let results_chirho = search_chirho.search_chirho("G[ao][dv]e?", &options_chirho).unwrap();
        assert_eq!(results_chirho.count_chirho(), 2);
    }

    #[test]
    fn test_multiword_search_chirho() {
        let search_chirho = create_test_search_chirho();
        let options_chirho = SearchOptionsChirho {
            search_type_chirho: Some(SearchTypeChirho::MultiWordChirho),
            max_results_chirho: Some(10),
            ..Default::default()
        };

        // Both words must be present
        let results_chirho = search_chirho.search_chirho("God loved", &options_chirho).unwrap();
        assert_eq!(results_chirho.count_chirho(), 1);

        let results_chirho = search_chirho.search_chirho("the", &options_chirho).unwrap();
        assert_eq!(results_chirho.count_chirho(), 3);
    }

    #[test]
    fn test_case_insensitive_chirho() {
        let search_chirho = create_test_search_chirho();
        let options_chirho = SearchOptionsChirho {
            search_type_chirho: Some(SearchTypeChirho::PhraseChirho),
            case_insensitive_chirho: true,
            max_results_chirho: Some(10),
            ..Default::default()
        };

        let results_chirho = search_chirho.search_chirho("god", &options_chirho).unwrap();
        assert_eq!(results_chirho.count_chirho(), 2);
    }

    #[test]
    fn test_max_results_chirho() {
        let search_chirho = create_test_search_chirho();
        let options_chirho = SearchOptionsChirho {
            search_type_chirho: Some(SearchTypeChirho::PhraseChirho),
            max_results_chirho: Some(1),
            ..Default::default()
        };

        let results_chirho = search_chirho.search_chirho("the", &options_chirho).unwrap();
        assert_eq!(results_chirho.count_chirho(), 1);
    }
}
