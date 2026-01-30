// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Full-text search functionality.
//!
//! Provides two search backends:
//! - `TantivySearchChirho`: Fast indexed search using tantivy
//! - `RegexSearchChirho`: Simple regex-based search (no index required)
//!
//! Also provides query parsing for advanced search syntax:
//! - Boolean operators: AND, OR, NOT
//! - Phrase proximity: "word1 word2"~N
//! - Wildcards: lov*, ?ove
//! - Field-specific: strongs:G26
//! - Fuzzy: love~

mod tantivy_chirho;
mod regex_chirho;
mod query_parser_chirho;
mod query_builder_chirho;
mod parallel_chirho;

pub use tantivy_chirho::TantivySearchChirho;
pub use regex_chirho::{RegexSearchChirho, SearchEntryChirho};
pub use query_parser_chirho::{
    QueryParserChirho, QueryNodeChirho, SearchScopeChirho, to_tantivy_query_chirho,
};
pub use query_builder_chirho::{
    TantivyQueryBuilderChirho, SearchBuilderChirho, ScoredResultChirho,
};
pub use parallel_chirho::{
    ParallelSearchChirho, ParallelSearchConfigChirho, TestamentPartitionChirho,
    MultiModuleResultChirho, search_multi_module_parallel_chirho,
};

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
    /// Multi-word search (all words must be present).
    MultiWordChirho,
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
