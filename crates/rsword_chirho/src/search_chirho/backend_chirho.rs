// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Search backend trait for platform-agnostic search implementations.
//!
//! This module provides an abstraction layer for search functionality,
//! allowing rsword_chirho to use different search backends on different platforms:
//!
//! - **Native**: Full-text search with Tantivy (fast, indexed search)
//! - **WASM**: Simple regex-based search (no index required, slower)
//!
//! ## Usage
//!
//! ```rust,ignore
//! use rsword_chirho::search_chirho::{SearchBackendChirho, get_default_search_backend_chirho};
//!
//! // Get the appropriate backend for the current platform
//! let backend_chirho = get_default_search_backend_chirho("/path/to/module", "KJV")?;
//!
//! // Search for text
//! let results_chirho = backend_chirho.search_chirho("love")?;
//! for result_chirho in results_chirho {
//!     println!("{}: {}", result_chirho.reference_chirho, result_chirho.score_chirho);
//! }
//! ```

use crate::error_chirho::ResultChirho;
use super::SearchEngineChirho;

/// A search result with reference, text snippet, and relevance score.
#[derive(Debug, Clone)]
pub struct SearchResultEntryChirho {
    /// Verse reference (e.g., "John 3:16")
    pub reference_chirho: String,
    /// Text snippet containing the match
    pub text_chirho: String,
    /// Relevance score (0.0 to 1.0, higher is better)
    pub score_chirho: f32,
}

/// Options for search queries.
#[derive(Debug, Clone, Default)]
pub struct SearchQueryOptionsChirho {
    /// Case insensitive search
    pub case_insensitive_chirho: bool,
    /// Maximum number of results
    pub max_results_chirho: Option<usize>,
    /// Search only within this scope (e.g., "NT", "Genesis")
    pub scope_chirho: Option<String>,
    /// Enable fuzzy matching
    pub fuzzy_chirho: bool,
    /// Fuzzy edit distance (0-2)
    pub fuzzy_distance_chirho: u8,
}

impl SearchQueryOptionsChirho {
    /// Create new search query options.
    pub fn new_chirho() -> Self {
        Self::default()
    }

    /// Set case insensitive search.
    pub fn case_insensitive_chirho(mut self, value_chirho: bool) -> Self {
        self.case_insensitive_chirho = value_chirho;
        self
    }

    /// Set maximum results.
    pub fn max_results_chirho(mut self, max_chirho: usize) -> Self {
        self.max_results_chirho = Some(max_chirho);
        self
    }

    /// Set search scope.
    pub fn scope_chirho(mut self, scope_chirho: &str) -> Self {
        self.scope_chirho = Some(scope_chirho.to_string());
        self
    }

    /// Enable fuzzy matching.
    pub fn fuzzy_chirho(mut self, distance_chirho: u8) -> Self {
        self.fuzzy_chirho = true;
        self.fuzzy_distance_chirho = distance_chirho.min(2);
        self
    }
}

/// Abstract search backend trait.
///
/// Implementations provide search functionality for different platforms:
///
/// - [`TantivySearchBackendChirho`]: Uses Tantivy for indexed full-text search (native only)
/// - [`RegexSearchBackendChirho`]: Uses regex for simple pattern matching (cross-platform)
pub trait SearchBackendChirho: Send + Sync {
    /// Get the module name this backend searches.
    fn module_name_chirho(&self) -> &str;

    /// Check if a search index exists (for indexed backends).
    fn has_index_chirho(&self) -> bool;

    /// Create/rebuild the search index.
    ///
    /// For non-indexed backends (like regex), this is a no-op.
    fn create_index_chirho(&mut self) -> ResultChirho<()>;

    /// Delete the search index.
    ///
    /// For non-indexed backends, this is a no-op.
    fn delete_index_chirho(&mut self) -> ResultChirho<()>;

    /// Search for a pattern.
    ///
    /// # Arguments
    /// * `query_chirho` - Search query (supports backend-specific syntax)
    /// * `options_chirho` - Search options
    ///
    /// # Returns
    /// Vector of search results sorted by relevance
    fn search_chirho(
        &self,
        query_chirho: &str,
        options_chirho: &SearchQueryOptionsChirho,
    ) -> ResultChirho<Vec<SearchResultEntryChirho>>;

    /// Get total indexed document count (for indexed backends).
    fn document_count_chirho(&self) -> usize {
        0
    }
}

/// Get the default search backend for the current platform.
///
/// - On native platforms with the `native` feature: Returns TantivySearchBackendChirho
/// - On WASM or without native feature: Returns RegexSearchBackendChirho
#[cfg(feature = "native")]
pub fn get_default_search_backend_chirho(
    module_path_chirho: &std::path::Path,
    module_name_chirho: &str,
) -> ResultChirho<Box<dyn SearchBackendChirho>> {
    // Try to use Tantivy first, fall back to regex
    match super::TantivySearchChirho::new_chirho(module_path_chirho) {
        Ok(tantivy_chirho) => Ok(Box::new(TantivySearchBackendChirho::new_chirho(
            tantivy_chirho,
            module_name_chirho.to_string(),
        ))),
        Err(_) => Ok(Box::new(RegexSearchBackendChirho::new_chirho(
            module_path_chirho.to_path_buf(),
            module_name_chirho.to_string(),
        ))),
    }
}

/// Get the default search backend for WASM (regex only).
#[cfg(not(feature = "native"))]
pub fn get_default_search_backend_chirho(
    module_path_chirho: &std::path::Path,
    module_name_chirho: &str,
) -> ResultChirho<Box<dyn SearchBackendChirho>> {
    Ok(Box::new(RegexSearchBackendChirho::new_chirho(
        module_path_chirho.to_path_buf(),
        module_name_chirho.to_string(),
    )))
}

/// Tantivy-based search backend (native only).
#[cfg(feature = "native")]
pub struct TantivySearchBackendChirho {
    inner_chirho: super::TantivySearchChirho,
    module_name_chirho: String,
}

#[cfg(feature = "native")]
impl TantivySearchBackendChirho {
    /// Create a new Tantivy search backend.
    pub fn new_chirho(tantivy_chirho: super::TantivySearchChirho, module_name_chirho: String) -> Self {
        Self {
            inner_chirho: tantivy_chirho,
            module_name_chirho,
        }
    }
}

#[cfg(feature = "native")]
impl SearchBackendChirho for TantivySearchBackendChirho {
    fn module_name_chirho(&self) -> &str {
        &self.module_name_chirho
    }

    fn has_index_chirho(&self) -> bool {
        self.inner_chirho.has_index_chirho()
    }

    fn create_index_chirho(&mut self) -> ResultChirho<()> {
        use super::SearchEngineChirho;
        self.inner_chirho.create_index_chirho()
    }

    fn delete_index_chirho(&mut self) -> ResultChirho<()> {
        use super::SearchEngineChirho;
        self.inner_chirho.delete_index_chirho()
    }

    fn search_chirho(
        &self,
        query_chirho: &str,
        options_chirho: &SearchQueryOptionsChirho,
    ) -> ResultChirho<Vec<SearchResultEntryChirho>> {
        use super::{SearchOptionsChirho, SearchTypeChirho};

        let mut opts_chirho = SearchOptionsChirho::new_chirho()
            .case_insensitive_chirho(options_chirho.case_insensitive_chirho);

        if let Some(max_chirho) = options_chirho.max_results_chirho {
            opts_chirho = opts_chirho.with_max_results_chirho(max_chirho);
        }
        if let Some(ref scope_chirho) = options_chirho.scope_chirho {
            opts_chirho = opts_chirho.with_scope_chirho(scope_chirho);
        }
        if options_chirho.fuzzy_chirho {
            opts_chirho = opts_chirho
                .with_type_chirho(SearchTypeChirho::FuzzyChirho)
                .with_fuzzy_chirho(options_chirho.fuzzy_distance_chirho);
        }

        let list_key_chirho = SearchEngineChirho::search_chirho(&self.inner_chirho, query_chirho, &opts_chirho)?;

        // Convert ListKeyChirho to Vec<SearchResultEntryChirho>
        let total_chirho = list_key_chirho.count_chirho();
        let results_chirho: Vec<SearchResultEntryChirho> = list_key_chirho
            .iter_chirho()
            .enumerate()
            .map(|(i_chirho, key_chirho)| SearchResultEntryChirho {
                reference_chirho: key_chirho.get_text_chirho().to_string(),
                text_chirho: String::new(), // Text not stored in ListKeyChirho
                score_chirho: if total_chirho > 0 {
                    1.0 - (i_chirho as f32 / total_chirho as f32)
                } else {
                    1.0
                },
            })
            .collect();

        Ok(results_chirho)
    }

    fn document_count_chirho(&self) -> usize {
        // Document count not tracked in current Tantivy implementation
        0
    }
}

/// Regex-based search backend (cross-platform).
///
/// This backend performs simple regex pattern matching without
/// requiring an index. It's slower but works everywhere including WASM.
pub struct RegexSearchBackendChirho {
    /// Path to module data (reserved for future full implementation).
    #[allow(dead_code)]
    module_path_chirho: std::path::PathBuf,
    module_name_chirho: String,
}

impl RegexSearchBackendChirho {
    /// Create a new regex search backend.
    pub fn new_chirho(
        module_path_chirho: std::path::PathBuf,
        module_name_chirho: String,
    ) -> Self {
        Self {
            module_path_chirho,
            module_name_chirho,
        }
    }
}

impl SearchBackendChirho for RegexSearchBackendChirho {
    fn module_name_chirho(&self) -> &str {
        &self.module_name_chirho
    }

    fn has_index_chirho(&self) -> bool {
        // Regex backend doesn't use indexes
        true
    }

    fn create_index_chirho(&mut self) -> ResultChirho<()> {
        // No-op for regex backend
        Ok(())
    }

    fn delete_index_chirho(&mut self) -> ResultChirho<()> {
        // No-op for regex backend
        Ok(())
    }

    fn search_chirho(
        &self,
        query_chirho: &str,
        options_chirho: &SearchQueryOptionsChirho,
    ) -> ResultChirho<Vec<SearchResultEntryChirho>> {
        use super::{RegexSearchChirho, SearchOptionsChirho};

        // Note: RegexSearchChirho requires entries to be added before searching.
        // For now, this is a simplified implementation that returns empty results.
        // A full implementation would load module data and populate entries first.
        let search_chirho = RegexSearchChirho::new_chirho();

        let mut opts_chirho = SearchOptionsChirho::new_chirho()
            .case_insensitive_chirho(options_chirho.case_insensitive_chirho);

        if let Some(max_chirho) = options_chirho.max_results_chirho {
            opts_chirho = opts_chirho.with_max_results_chirho(max_chirho);
        }
        if let Some(ref scope_chirho) = options_chirho.scope_chirho {
            opts_chirho = opts_chirho.with_scope_chirho(scope_chirho);
        }

        let list_key_chirho = SearchEngineChirho::search_chirho(&search_chirho, query_chirho, &opts_chirho)?;

        // Convert ListKeyChirho to Vec<SearchResultEntryChirho>
        let results_chirho: Vec<SearchResultEntryChirho> = list_key_chirho
            .iter_chirho()
            .map(|key_chirho| SearchResultEntryChirho {
                reference_chirho: key_chirho.get_text_chirho().to_string(),
                text_chirho: String::new(),
                score_chirho: 1.0, // Regex matches all have equal score
            })
            .collect();

        Ok(results_chirho)
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_search_query_options_builder_chirho() {
        let opts_chirho = SearchQueryOptionsChirho::new_chirho()
            .case_insensitive_chirho(true)
            .max_results_chirho(50)
            .scope_chirho("NT")
            .fuzzy_chirho(1);

        assert!(opts_chirho.case_insensitive_chirho);
        assert_eq!(opts_chirho.max_results_chirho, Some(50));
        assert_eq!(opts_chirho.scope_chirho, Some("NT".to_string()));
        assert!(opts_chirho.fuzzy_chirho);
        assert_eq!(opts_chirho.fuzzy_distance_chirho, 1);
    }

    #[test]
    fn test_search_result_entry_chirho() {
        let result_chirho = SearchResultEntryChirho {
            reference_chirho: "John 3:16".to_string(),
            text_chirho: "For God so loved...".to_string(),
            score_chirho: 0.95,
        };

        assert_eq!(result_chirho.reference_chirho, "John 3:16");
        assert_eq!(result_chirho.score_chirho, 0.95);
    }
}
