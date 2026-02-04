// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Full-text search functionality.
//!
//! Provides two search backends:
//! - [`TantivySearchChirho`]: Fast indexed search using Tantivy (requires index creation)
//! - [`RegexSearchChirho`]: Simple regex-based search (no index required)
//!
//! ## Search Types
//!
//! | Type | Description | Example |
//! |------|-------------|---------|
//! | [`SearchTypeChirho::PhraseChirho`] | Exact phrase match | "for God so loved" |
//! | [`SearchTypeChirho::MultiWordChirho`] | All words present | love world God |
//! | [`SearchTypeChirho::RegexChirho`] | Regular expression | lov(e\|ing) |
//! | [`SearchTypeChirho::ExternalChirho`] | Tantivy indexed | Complex queries |
//!
//! ## Basic Search
//!
//! ```rust,ignore
//! use rsword_chirho::{SearchOptionsChirho, SearchTypeChirho, RegexSearchChirho, SearchEngineChirho};
//! use rsword_chirho::SwMgrChirho;
//! use std::path::Path;
//!
//! let mgr_chirho = SwMgrChirho::with_system_paths_chirho().unwrap();
//!
//! // Get data path for module
//! if let Some(data_path_chirho) = mgr_chirho.get_module_data_path_chirho("KJV") {
//!     // Create regex search engine
//!     let search_chirho = RegexSearchChirho::new_chirho(&data_path_chirho);
//!
//!     // Search with options
//!     let options_chirho = SearchOptionsChirho::new_chirho()
//!         .with_type_chirho(SearchTypeChirho::PhraseChirho)
//!         .case_insensitive_chirho(true)
//!         .with_max_results_chirho(50);
//!
//!     let results_chirho = search_chirho.search_chirho("for God so loved", &options_chirho).unwrap();
//!     println!("Found {} results", results_chirho.count_chirho());
//! }
//! ```
//!
//! ## Indexed Search with Tantivy
//!
//! For large modules, create and use a search index:
//!
//! ```rust,ignore
//! use rsword_chirho::{TantivySearchChirho, SearchEngineChirho, SearchOptionsChirho};
//! use std::path::Path;
//!
//! // Create search engine for module
//! let mut search_chirho = TantivySearchChirho::new_chirho(
//!     Path::new("/path/to/module"),
//!     "KJV"
//! ).unwrap();
//!
//! // Build search index (one-time operation)
//! if !search_chirho.has_index_chirho() {
//!     search_chirho.create_index_chirho().unwrap();
//! }
//!
//! // Search using the index
//! let results_chirho = search_chirho.search_chirho("love", &SearchOptionsChirho::default()).unwrap();
//! ```
//!
//! ## Advanced Query Syntax
//!
//! The query parser supports advanced search syntax:
//!
//! - Boolean operators: `love AND world`, `mercy OR grace`, `NOT sin`
//! - Phrase proximity: `"word1 word2"~5` (words within 5 positions)
//! - Wildcards: `lov*` (love, loving, loved), `?ove` (love, dove, etc.)
//! - Field-specific: `strongs:G26` (Strong's number search)
//! - Fuzzy matching: `love~` (matches similar words)
//!
//! ## Parallel Search
//!
//! Search across multiple modules or partitions simultaneously:
//!
//! ```rust,ignore
//! use rsword_chirho::search_chirho::{ParallelSearchChirho, ParallelSearchConfigChirho};
//!
//! let config_chirho = ParallelSearchConfigChirho::default();
//! let parallel_chirho = ParallelSearchChirho::new_chirho(config_chirho);
//! // Search multiple modules in parallel...
//! ```

// Tantivy-based search (native only, requires indexing)
#[cfg(feature = "native")]
mod tantivy_chirho;
#[cfg(feature = "native")]
mod query_builder_chirho;
#[cfg(feature = "native")]
mod parallel_chirho;

// Cross-platform modules
mod regex_chirho;
mod query_parser_chirho;
pub mod highlighter_chirho;
pub mod backend_chirho;

// Native-only exports
#[cfg(feature = "native")]
pub use tantivy_chirho::TantivySearchChirho;
#[cfg(feature = "native")]
pub use query_builder_chirho::{
    TantivyQueryBuilderChirho, SearchBuilderChirho, ScoredResultChirho,
};
#[cfg(feature = "native")]
pub use parallel_chirho::{
    ParallelSearchChirho, ParallelSearchConfigChirho, TestamentPartitionChirho,
    MultiModuleResultChirho, search_multi_module_parallel_chirho,
};

// Cross-platform exports
pub use regex_chirho::{RegexSearchChirho, SearchEntryChirho};
#[cfg(feature = "native")]
pub use query_parser_chirho::{
    QueryParserChirho, QueryNodeChirho, SearchScopeChirho, to_tantivy_query_chirho,
};
#[cfg(not(feature = "native"))]
pub use query_parser_chirho::{
    QueryParserChirho, QueryNodeChirho, SearchScopeChirho,
};
pub use highlighter_chirho::{
    HighlighterChirho, HighlightStyleChirho, HighlightOptionsChirho, HighlightedResultChirho,
};

// Search backend abstraction (cross-platform)
pub use backend_chirho::{
    SearchBackendChirho, SearchResultEntryChirho, SearchQueryOptionsChirho,
    RegexSearchBackendChirho, get_default_search_backend_chirho,
};
#[cfg(feature = "native")]
pub use backend_chirho::TantivySearchBackendChirho;

use crate::error_chirho::ResultChirho;
use crate::keys_chirho::ListKeyChirho;

/// Search type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub enum SearchTypeChirho {
    /// Regex search.
    RegexChirho,
    /// Phrase search (literal text).
    #[default]
    PhraseChirho,
    /// Phrase search with proximity (slop).
    ProximityChirho,
    /// Multi-word search (all words must be present).
    MultiWordChirho,
    /// Fuzzy search (tolerates typos).
    FuzzyChirho,
    /// Entry attribute search.
    EntryAttrChirho,
    /// External search (tantivy indexed).
    ExternalChirho,
}


/// Search options.
#[derive(Debug, Clone, Default)]
pub struct SearchOptionsChirho {
    /// Search type.
    pub search_type_chirho: Option<SearchTypeChirho>,
    /// Case insensitive.
    pub case_insensitive_chirho: bool,
    /// Scope key (optional range to search within).
    pub scope_chirho: Option<String>,
    /// Maximum results.
    pub max_results_chirho: Option<usize>,
    /// Phrase slop (words allowed between phrase terms for proximity search).
    pub slop_chirho: Option<u32>,
    /// Fuzzy distance (edit distance for fuzzy matching, 0-2).
    pub fuzzy_distance_chirho: Option<u8>,
    /// Include scores in results.
    pub include_scores_chirho: bool,
    /// Minimum score threshold.
    pub min_score_chirho: Option<f32>,
    /// Highlight matched terms.
    pub highlight_chirho: bool,
}

impl SearchOptionsChirho {
    /// Create new search options.
    pub fn new_chirho() -> Self {
        Self::default()
    }

    /// Set the search type.
    pub fn with_type_chirho(mut self, search_type_chirho: SearchTypeChirho) -> Self {
        self.search_type_chirho = Some(search_type_chirho);
        self
    }

    /// Set case insensitive.
    pub fn case_insensitive_chirho(mut self, value_chirho: bool) -> Self {
        self.case_insensitive_chirho = value_chirho;
        self
    }

    /// Set the scope.
    pub fn with_scope_chirho(mut self, scope_chirho: &str) -> Self {
        self.scope_chirho = Some(scope_chirho.to_string());
        self
    }

    /// Set maximum results.
    pub fn with_max_results_chirho(mut self, max_chirho: usize) -> Self {
        self.max_results_chirho = Some(max_chirho);
        self
    }

    /// Set phrase slop (proximity).
    ///
    /// Slop defines how many intervening words are allowed between
    /// phrase terms. For example, slop=2 allows "God so loved" to match
    /// "God loved" or "God really loved".
    pub fn with_slop_chirho(mut self, slop_chirho: u32) -> Self {
        self.slop_chirho = Some(slop_chirho);
        self
    }

    /// Set fuzzy distance.
    ///
    /// Distance defines how many character edits (insertions, deletions,
    /// substitutions) are allowed. Valid values are 0-2.
    /// For example, distance=1 allows "love" to match "dove" or "lov".
    pub fn with_fuzzy_chirho(mut self, distance_chirho: u8) -> Self {
        self.fuzzy_distance_chirho = Some(distance_chirho.min(2));
        self
    }

    /// Include relevance scores in results.
    pub fn with_scores_chirho(mut self) -> Self {
        self.include_scores_chirho = true;
        self
    }

    /// Set minimum score threshold.
    pub fn with_min_score_chirho(mut self, score_chirho: f32) -> Self {
        self.min_score_chirho = Some(score_chirho);
        self
    }

    /// Enable highlighting of matched terms.
    pub fn with_highlight_chirho(mut self) -> Self {
        self.highlight_chirho = true;
        self
    }
}

/// Search engine trait.
pub trait SearchEngineChirho {
    /// Search for a pattern.
    fn search_chirho(
        &self,
        pattern_chirho: &str,
        options_chirho: &SearchOptionsChirho,
    ) -> ResultChirho<ListKeyChirho>;

    /// Create a search index for a module.
    fn create_index_chirho(&mut self) -> ResultChirho<()>;

    /// Check if a search index exists.
    fn has_index_chirho(&self) -> bool;

    /// Delete the search index.
    fn delete_index_chirho(&mut self) -> ResultChirho<()>;
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_search_options_default_chirho() {
        let options_chirho = SearchOptionsChirho::default();
        assert!(options_chirho.search_type_chirho.is_none());
        assert!(!options_chirho.case_insensitive_chirho);
    }

    #[test]
    fn test_search_options_builder_chirho() {
        let options_chirho = SearchOptionsChirho::new_chirho()
            .with_type_chirho(SearchTypeChirho::RegexChirho)
            .case_insensitive_chirho(true)
            .with_max_results_chirho(100)
            .with_scope_chirho("Gen-Exod");

        assert_eq!(options_chirho.search_type_chirho, Some(SearchTypeChirho::RegexChirho));
        assert!(options_chirho.case_insensitive_chirho);
        assert_eq!(options_chirho.max_results_chirho, Some(100));
        assert_eq!(options_chirho.scope_chirho, Some("Gen-Exod".to_string()));
    }

    #[test]
    fn test_search_type_default_chirho() {
        assert_eq!(SearchTypeChirho::default(), SearchTypeChirho::PhraseChirho);
    }
}
