// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Parallel search using rayon.
//!
//! This module provides parallel search functionality for:
//! - Searching across testaments (OT and NT in parallel)
//! - Searching across multiple modules
//! - Parallel regex matching within a single module
//!
//! Requires the `parallel` feature to be enabled.

#[cfg(feature = "parallel")]
use rayon::prelude::*;

use regex::{Regex, RegexBuilder};

use crate::error_chirho::{ErrorChirho, ResultChirho};
use crate::keys_chirho::{ListKeyChirho, StrKeyChirho};
use crate::versification_chirho::TestamentChirho;

use super::{SearchEntryChirho, SearchOptionsChirho, SearchTypeChirho};

/// Parallel search configuration.
#[derive(Debug, Clone)]
pub struct ParallelSearchConfigChirho {
    /// Number of threads to use (0 = auto detect).
    pub num_threads_chirho: usize,
    /// Minimum entries before enabling parallel search.
    pub min_entries_for_parallel_chirho: usize,
    /// Chunk size for parallel iteration.
    pub chunk_size_chirho: usize,
}

impl Default for ParallelSearchConfigChirho {
    fn default() -> Self {
        Self {
            num_threads_chirho: 0, // Auto-detect
            min_entries_for_parallel_chirho: 1000,
            chunk_size_chirho: 500,
        }
    }
}

/// Testament partition for parallel search.
#[derive(Debug, Clone)]
pub struct TestamentPartitionChirho {
    /// Testament type.
    pub testament_chirho: TestamentChirho,
    /// Entries belonging to this testament.
    pub entries_chirho: Vec<SearchEntryChirho>,
}

/// Parallel search engine.
///
/// Provides parallel search across entries using rayon.
pub struct ParallelSearchChirho {
    /// All entries for searching.
    entries_chirho: Vec<SearchEntryChirho>,
    /// Configuration.
    config_chirho: ParallelSearchConfigChirho,
    /// Old Testament entries (for testament-parallel search).
    ot_entries_chirho: Vec<SearchEntryChirho>,
    /// New Testament entries (for testament-parallel search).
    nt_entries_chirho: Vec<SearchEntryChirho>,
}

impl ParallelSearchChirho {
    /// Create a new parallel search engine.
    pub fn new_chirho() -> Self {
        Self {
            entries_chirho: Vec::new(),
            config_chirho: ParallelSearchConfigChirho::default(),
            ot_entries_chirho: Vec::new(),
            nt_entries_chirho: Vec::new(),
        }
    }

    /// Create with custom configuration.
    pub fn with_config_chirho(config_chirho: ParallelSearchConfigChirho) -> Self {
        Self {
            entries_chirho: Vec::new(),
            config_chirho,
            ot_entries_chirho: Vec::new(),
            nt_entries_chirho: Vec::new(),
        }
    }

    /// Add an entry to the search corpus.
    pub fn add_entry_chirho(&mut self, key_chirho: String, text_chirho: String) {
        self.entries_chirho.push(SearchEntryChirho {
            key_chirho,
            text_chirho,
        });
    }

    /// Add an entry with testament information for parallel testament search.
    pub fn add_entry_with_testament_chirho(
        &mut self,
        key_chirho: String,
        text_chirho: String,
        testament_chirho: TestamentChirho,
    ) {
        let entry_chirho = SearchEntryChirho {
            key_chirho: key_chirho.clone(),
            text_chirho: text_chirho.clone(),
        };

        self.entries_chirho.push(SearchEntryChirho {
            key_chirho,
            text_chirho,
        });

        match testament_chirho {
            TestamentChirho::OldChirho => {
                self.ot_entries_chirho.push(entry_chirho);
            }
            TestamentChirho::NewChirho => {
                self.nt_entries_chirho.push(entry_chirho);
            }
        }
    }

    /// Get the number of entries.
    pub fn entry_count_chirho(&self) -> usize {
        self.entries_chirho.len()
    }

    /// Clear all entries.
    pub fn clear_chirho(&mut self) {
        self.entries_chirho.clear();
        self.ot_entries_chirho.clear();
        self.nt_entries_chirho.clear();
    }

    /// Check if parallel search should be used based on entry count.
    pub fn should_use_parallel_chirho(&self) -> bool {
        self.entries_chirho.len() >= self.config_chirho.min_entries_for_parallel_chirho
    }

    /// Search for a pattern using parallel or sequential depending on size.
    pub fn search_chirho(
        &self,
        pattern_chirho: &str,
        options_chirho: &SearchOptionsChirho,
    ) -> ResultChirho<ListKeyChirho> {
        #[cfg(feature = "parallel")]
        {
            if self.should_use_parallel_chirho() {
                return self.search_parallel_chirho(pattern_chirho, options_chirho);
            }
        }

        // Fall back to sequential search
        self.search_sequential_chirho(pattern_chirho, options_chirho)
    }

    /// Search OT and NT in parallel (requires testament partitioning).
    #[cfg(feature = "parallel")]
    pub fn search_by_testament_parallel_chirho(
        &self,
        pattern_chirho: &str,
        options_chirho: &SearchOptionsChirho,
    ) -> ResultChirho<ListKeyChirho> {
        let max_results_chirho = options_chirho.max_results_chirho.unwrap_or(1000);
        let case_insensitive_chirho = options_chirho.case_insensitive_chirho;
        let search_type_chirho = options_chirho.search_type_chirho
            .unwrap_or(SearchTypeChirho::PhraseChirho);

        let regex_chirho = build_regex_chirho(pattern_chirho, search_type_chirho, case_insensitive_chirho)?;

        // Search both testaments in parallel
        let partitions_chirho = vec![
            (&self.ot_entries_chirho, "OT"),
            (&self.nt_entries_chirho, "NT"),
        ];

        let results_per_testament_chirho: Vec<Vec<String>> = partitions_chirho
            .into_par_iter()
            .map(|(entries_chirho, _testament_name_chirho)| {
                search_entries_parallel_chirho(
                    entries_chirho,
                    &regex_chirho,
                    max_results_chirho / 2, // Split results between testaments
                    self.config_chirho.chunk_size_chirho,
                )
            })
            .collect();

        // Merge results
        let mut combined_chirho = ListKeyChirho::new_chirho();
        let mut count_chirho = 0;

        for testament_results_chirho in results_per_testament_chirho {
            for key_str_chirho in testament_results_chirho {
                if count_chirho >= max_results_chirho {
                    break;
                }
                combined_chirho.add_chirho(Box::new(StrKeyChirho::with_text_chirho(&key_str_chirho)));
                count_chirho += 1;
            }
        }

        Ok(combined_chirho)
    }

    /// Search OT and NT in parallel (non-parallel fallback).
    #[cfg(not(feature = "parallel"))]
    pub fn search_by_testament_parallel_chirho(
        &self,
        pattern_chirho: &str,
        options_chirho: &SearchOptionsChirho,
    ) -> ResultChirho<ListKeyChirho> {
        // Without rayon, fall back to sequential search
        self.search_sequential_chirho(pattern_chirho, options_chirho)
    }

    /// Perform parallel search using rayon.
    #[cfg(feature = "parallel")]
    fn search_parallel_chirho(
        &self,
        pattern_chirho: &str,
        options_chirho: &SearchOptionsChirho,
    ) -> ResultChirho<ListKeyChirho> {
        let max_results_chirho = options_chirho.max_results_chirho.unwrap_or(1000);
        let case_insensitive_chirho = options_chirho.case_insensitive_chirho;
        let search_type_chirho = options_chirho.search_type_chirho
            .unwrap_or(SearchTypeChirho::PhraseChirho);

        let regex_chirho = build_regex_chirho(pattern_chirho, search_type_chirho, case_insensitive_chirho)?;

        let matched_keys_chirho = search_entries_parallel_chirho(
            &self.entries_chirho,
            &regex_chirho,
            max_results_chirho,
            self.config_chirho.chunk_size_chirho,
        );

        let mut results_chirho = ListKeyChirho::new_chirho();
        for key_str_chirho in matched_keys_chirho {
            results_chirho.add_chirho(Box::new(StrKeyChirho::with_text_chirho(&key_str_chirho)));
        }

        Ok(results_chirho)
    }

    /// Perform sequential search.
    fn search_sequential_chirho(
        &self,
        pattern_chirho: &str,
        options_chirho: &SearchOptionsChirho,
    ) -> ResultChirho<ListKeyChirho> {
        let max_results_chirho = options_chirho.max_results_chirho.unwrap_or(1000);
        let case_insensitive_chirho = options_chirho.case_insensitive_chirho;
        let search_type_chirho = options_chirho.search_type_chirho
            .unwrap_or(SearchTypeChirho::PhraseChirho);

        let regex_chirho = build_regex_chirho(pattern_chirho, search_type_chirho, case_insensitive_chirho)?;

        let mut results_chirho = ListKeyChirho::new_chirho();
        let mut count_chirho = 0;

        for entry_chirho in &self.entries_chirho {
            if count_chirho >= max_results_chirho {
                break;
            }

            if regex_chirho.is_match(&entry_chirho.text_chirho) {
                results_chirho.add_chirho(Box::new(StrKeyChirho::with_text_chirho(&entry_chirho.key_chirho)));
                count_chirho += 1;
            }
        }

        Ok(results_chirho)
    }
}

impl Default for ParallelSearchChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

/// Build a regex pattern based on search type.
fn build_regex_chirho(
    pattern_chirho: &str,
    search_type_chirho: SearchTypeChirho,
    case_insensitive_chirho: bool,
) -> ResultChirho<Regex> {
    let pattern_str_chirho = match search_type_chirho {
        SearchTypeChirho::RegexChirho => pattern_chirho.to_string(),
        SearchTypeChirho::PhraseChirho => regex::escape(pattern_chirho),
        SearchTypeChirho::MultiWordChirho => {
            // Build alternation pattern for multiword
            let words_chirho: Vec<&str> = pattern_chirho.split_whitespace().collect();
            if words_chirho.is_empty() {
                return Err(ErrorChirho::invalid_config_chirho("Empty search pattern"));
            }
            // For multiword, we need all words to match, so we'll build a complex pattern
            // This is a simplified approach - each word as a lookahead
            let lookaheads_chirho: Vec<String> = words_chirho
                .iter()
                .map(|w_chirho| format!("(?=.*{})", regex::escape(w_chirho)))
                .collect();
            format!("^{}.*$", lookaheads_chirho.join(""))
        }
        _ => regex::escape(pattern_chirho),
    };

    RegexBuilder::new(&pattern_str_chirho)
        .case_insensitive(case_insensitive_chirho)
        .dot_matches_new_line(true)
        .build()
        .map_err(|e_chirho| ErrorChirho::invalid_config_chirho(
            format!("Invalid regex pattern: {}", e_chirho)
        ))
}

/// Search entries in parallel using rayon.
#[cfg(feature = "parallel")]
fn search_entries_parallel_chirho(
    entries_chirho: &[SearchEntryChirho],
    regex_chirho: &Regex,
    max_results_chirho: usize,
    chunk_size_chirho: usize,
) -> Vec<String> {
    use std::sync::atomic::{AtomicUsize, Ordering};

    let found_count_chirho = AtomicUsize::new(0);

    entries_chirho
        .par_chunks(chunk_size_chirho)
        .flat_map(|chunk_chirho| {
            let mut local_matches_chirho = Vec::new();

            for entry_chirho in chunk_chirho {
                // Check if we've hit the limit
                if found_count_chirho.load(Ordering::Relaxed) >= max_results_chirho {
                    break;
                }

                if regex_chirho.is_match(&entry_chirho.text_chirho) {
                    let current_chirho = found_count_chirho.fetch_add(1, Ordering::Relaxed);
                    if current_chirho < max_results_chirho {
                        local_matches_chirho.push(entry_chirho.key_chirho.clone());
                    }
                }
            }

            local_matches_chirho
        })
        .collect::<Vec<_>>()
        .into_iter()
        .take(max_results_chirho)
        .collect()
}

/// Result of multi-module parallel search.
///
/// Contains matched keys as strings (to avoid `Send` bounds on `ListKeyChirho`).
#[derive(Debug, Clone)]
pub struct MultiModuleResultChirho {
    /// Module name.
    pub module_name_chirho: String,
    /// Matched keys as strings.
    pub matched_keys_chirho: Vec<String>,
}

impl MultiModuleResultChirho {
    /// Convert matched keys to a ListKeyChirho.
    pub fn to_list_key_chirho(&self) -> ListKeyChirho {
        let mut list_chirho = ListKeyChirho::new_chirho();
        for key_chirho in &self.matched_keys_chirho {
            list_chirho.add_chirho(Box::new(StrKeyChirho::with_text_chirho(key_chirho)));
        }
        list_chirho
    }

    /// Get the count of results.
    pub fn count_chirho(&self) -> usize {
        self.matched_keys_chirho.len()
    }
}

/// Multi-module parallel search.
///
/// Searches across multiple module entries in parallel.
/// Returns `MultiModuleResultChirho` which can be converted to `ListKeyChirho`.
#[cfg(feature = "parallel")]
pub fn search_multi_module_parallel_chirho(
    module_entries_chirho: Vec<(String, Vec<SearchEntryChirho>)>,
    pattern_chirho: &str,
    options_chirho: &SearchOptionsChirho,
) -> ResultChirho<Vec<MultiModuleResultChirho>> {
    let case_insensitive_chirho = options_chirho.case_insensitive_chirho;
    let search_type_chirho = options_chirho.search_type_chirho
        .unwrap_or(SearchTypeChirho::PhraseChirho);
    let max_per_module_chirho = options_chirho.max_results_chirho.unwrap_or(100);

    let regex_chirho = build_regex_chirho(pattern_chirho, search_type_chirho, case_insensitive_chirho)?;

    let results_chirho: Vec<MultiModuleResultChirho> = module_entries_chirho
        .into_par_iter()
        .map(|(module_name_chirho, entries_chirho)| {
            let matched_keys_chirho = search_entries_parallel_chirho(
                &entries_chirho,
                &regex_chirho,
                max_per_module_chirho,
                500,
            );

            MultiModuleResultChirho {
                module_name_chirho,
                matched_keys_chirho,
            }
        })
        .collect();

    Ok(results_chirho)
}

/// Multi-module parallel search (non-parallel fallback).
#[cfg(not(feature = "parallel"))]
pub fn search_multi_module_parallel_chirho(
    module_entries_chirho: Vec<(String, Vec<SearchEntryChirho>)>,
    pattern_chirho: &str,
    options_chirho: &SearchOptionsChirho,
) -> ResultChirho<Vec<MultiModuleResultChirho>> {
    let case_insensitive_chirho = options_chirho.case_insensitive_chirho;
    let search_type_chirho = options_chirho.search_type_chirho
        .unwrap_or(SearchTypeChirho::PhraseChirho);
    let max_per_module_chirho = options_chirho.max_results_chirho.unwrap_or(100);

    let regex_chirho = build_regex_chirho(pattern_chirho, search_type_chirho, case_insensitive_chirho)?;

    let mut results_chirho = Vec::new();

    for (module_name_chirho, entries_chirho) in module_entries_chirho {
        let mut matched_keys_chirho = Vec::new();

        for entry_chirho in &entries_chirho {
            if matched_keys_chirho.len() >= max_per_module_chirho {
                break;
            }

            if regex_chirho.is_match(&entry_chirho.text_chirho) {
                matched_keys_chirho.push(entry_chirho.key_chirho.clone());
            }
        }

        results_chirho.push(MultiModuleResultChirho {
            module_name_chirho,
            matched_keys_chirho,
        });
    }

    Ok(results_chirho)
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    fn create_test_entries_chirho(count_chirho: usize) -> Vec<SearchEntryChirho> {
        (0..count_chirho)
            .map(|i_chirho| SearchEntryChirho {
                key_chirho: format!("Test.{}.{}", i_chirho / 100 + 1, i_chirho % 100 + 1),
                text_chirho: if i_chirho % 10 == 0 {
                    format!("This is verse {} with the word love in it", i_chirho)
                } else {
                    format!("This is verse {} without the target word", i_chirho)
                },
            })
            .collect()
    }

    #[test]
    fn test_parallel_search_config_default_chirho() {
        let config_chirho = ParallelSearchConfigChirho::default();
        assert_eq!(config_chirho.num_threads_chirho, 0);
        assert_eq!(config_chirho.min_entries_for_parallel_chirho, 1000);
        assert_eq!(config_chirho.chunk_size_chirho, 500);
    }

    #[test]
    fn test_parallel_search_creation_chirho() {
        let search_chirho = ParallelSearchChirho::new_chirho();
        assert_eq!(search_chirho.entry_count_chirho(), 0);
        assert!(!search_chirho.should_use_parallel_chirho());
    }

    #[test]
    fn test_parallel_search_add_entry_chirho() {
        let mut search_chirho = ParallelSearchChirho::new_chirho();
        search_chirho.add_entry_chirho("Gen.1.1".to_string(), "In the beginning".to_string());
        assert_eq!(search_chirho.entry_count_chirho(), 1);
    }

    #[test]
    fn test_parallel_search_sequential_chirho() {
        let mut search_chirho = ParallelSearchChirho::new_chirho();

        // Add small number of entries (below parallel threshold)
        for entry_chirho in create_test_entries_chirho(100) {
            search_chirho.add_entry_chirho(entry_chirho.key_chirho, entry_chirho.text_chirho);
        }

        let options_chirho = SearchOptionsChirho {
            search_type_chirho: Some(SearchTypeChirho::PhraseChirho),
            max_results_chirho: Some(50),
            ..Default::default()
        };

        let results_chirho = search_chirho.search_chirho("love", &options_chirho).unwrap();
        // Every 10th entry has "love", so 100/10 = 10 entries
        assert_eq!(results_chirho.count_chirho(), 10);
    }

    #[test]
    fn test_parallel_search_with_testament_chirho() {
        let mut search_chirho = ParallelSearchChirho::new_chirho();

        // Add OT entries
        for i_chirho in 0..50 {
            search_chirho.add_entry_with_testament_chirho(
                format!("Gen.{}.1", i_chirho + 1),
                format!("OT verse {} with love", i_chirho),
                TestamentChirho::OldChirho,
            );
        }

        // Add NT entries
        for i_chirho in 0..50 {
            search_chirho.add_entry_with_testament_chirho(
                format!("Matt.{}.1", i_chirho + 1),
                format!("NT verse {} with love", i_chirho),
                TestamentChirho::NewChirho,
            );
        }

        assert_eq!(search_chirho.entry_count_chirho(), 100);
    }

    #[test]
    fn test_should_use_parallel_chirho() {
        let config_chirho = ParallelSearchConfigChirho {
            min_entries_for_parallel_chirho: 100,
            ..Default::default()
        };

        let mut search_chirho = ParallelSearchChirho::with_config_chirho(config_chirho);

        // Below threshold
        for entry_chirho in create_test_entries_chirho(50) {
            search_chirho.add_entry_chirho(entry_chirho.key_chirho, entry_chirho.text_chirho);
        }
        assert!(!search_chirho.should_use_parallel_chirho());

        // At/above threshold
        for entry_chirho in create_test_entries_chirho(50) {
            search_chirho.add_entry_chirho(entry_chirho.key_chirho, entry_chirho.text_chirho);
        }
        assert!(search_chirho.should_use_parallel_chirho());
    }

    #[test]
    fn test_build_regex_phrase_chirho() {
        let regex_chirho = build_regex_chirho("test pattern", SearchTypeChirho::PhraseChirho, false).unwrap();
        assert!(regex_chirho.is_match("this is a test pattern here"));
        assert!(!regex_chirho.is_match("this is a TEST PATTERN here")); // Case sensitive
    }

    #[test]
    fn test_build_regex_case_insensitive_chirho() {
        let regex_chirho = build_regex_chirho("test pattern", SearchTypeChirho::PhraseChirho, true).unwrap();
        assert!(regex_chirho.is_match("this is a TEST PATTERN here"));
    }

    #[test]
    fn test_build_regex_actual_regex_chirho() {
        let regex_chirho = build_regex_chirho("lo+ve", SearchTypeChirho::RegexChirho, false).unwrap();
        assert!(regex_chirho.is_match("I love you"));
        assert!(regex_chirho.is_match("I looove you"));
        assert!(!regex_chirho.is_match("I lve you"));
    }

    #[test]
    fn test_clear_entries_chirho() {
        let mut search_chirho = ParallelSearchChirho::new_chirho();
        search_chirho.add_entry_chirho("key1".to_string(), "text1".to_string());
        search_chirho.add_entry_with_testament_chirho(
            "key2".to_string(),
            "text2".to_string(),
            TestamentChirho::OldChirho,
        );

        assert_eq!(search_chirho.entry_count_chirho(), 2);

        search_chirho.clear_chirho();
        assert_eq!(search_chirho.entry_count_chirho(), 0);
    }

    #[cfg(feature = "parallel")]
    #[test]
    fn test_parallel_search_large_dataset_chirho() {
        let config_chirho = ParallelSearchConfigChirho {
            min_entries_for_parallel_chirho: 100,
            chunk_size_chirho: 50,
            ..Default::default()
        };

        let mut search_chirho = ParallelSearchChirho::with_config_chirho(config_chirho);

        // Add enough entries to trigger parallel search
        for entry_chirho in create_test_entries_chirho(500) {
            search_chirho.add_entry_chirho(entry_chirho.key_chirho, entry_chirho.text_chirho);
        }

        assert!(search_chirho.should_use_parallel_chirho());

        let options_chirho = SearchOptionsChirho {
            search_type_chirho: Some(SearchTypeChirho::PhraseChirho),
            max_results_chirho: Some(100),
            ..Default::default()
        };

        let results_chirho = search_chirho.search_chirho("love", &options_chirho).unwrap();
        // Every 10th entry has "love", so 500/10 = 50 entries, but limited to 100
        assert_eq!(results_chirho.count_chirho(), 50);
    }

    #[cfg(feature = "parallel")]
    #[test]
    fn test_multi_module_parallel_search_chirho() {
        let module1_entries_chirho = create_test_entries_chirho(100);
        let module2_entries_chirho = create_test_entries_chirho(100);

        let modules_chirho = vec![
            ("KJV".to_string(), module1_entries_chirho),
            ("ESV".to_string(), module2_entries_chirho),
        ];

        let options_chirho = SearchOptionsChirho {
            search_type_chirho: Some(SearchTypeChirho::PhraseChirho),
            max_results_chirho: Some(50),
            ..Default::default()
        };

        let results_chirho = search_multi_module_parallel_chirho(
            modules_chirho,
            "love",
            &options_chirho,
        ).unwrap();

        assert_eq!(results_chirho.len(), 2);

        // Each module should have 10 results (every 10th of 100 entries)
        for result_chirho in results_chirho {
            assert!(result_chirho.module_name_chirho == "KJV" || result_chirho.module_name_chirho == "ESV");
            assert_eq!(result_chirho.count_chirho(), 10);
            // Verify conversion to ListKeyChirho works
            let list_chirho = result_chirho.to_list_key_chirho();
            assert_eq!(list_chirho.count_chirho(), 10);
        }
    }
}
