// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Full-text search functionality using tantivy.

use crate::error_chirho::ResultChirho;
use crate::keys_chirho::ListKeyChirho;

/// Search type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchTypeChirho {
    /// Regex search.
    RegexChirho,
    /// Phrase search.
    PhraseChirho,
    /// Multi-word search (AND).
    MultiWordChirho,
    /// Entry attribute search.
    EntryAttrChirho,
    /// External search (tantivy).
    ExternalChirho,
}

/// Search options.
#[derive(Debug, Clone, Default)]
pub struct SearchOptionsChirho {
    /// Search type.
    pub search_type_chirho: Option<SearchTypeChirho>,
    /// Case insensitive.
    pub case_insensitive_chirho: bool,
    /// Scope key (optional).
    pub scope_chirho: Option<String>,
    /// Maximum results.
    pub max_results_chirho: Option<usize>,
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

// TODO: Implement TantivySearchChirho using tantivy crate

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_search_options_default_chirho() {
        let options_chirho = SearchOptionsChirho::default();
        assert!(options_chirho.search_type_chirho.is_none());
        assert!(!options_chirho.case_insensitive_chirho);
    }
}
