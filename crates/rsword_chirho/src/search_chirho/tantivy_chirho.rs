// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Tantivy-based full-text search engine.
//!
//! Provides fast indexed search using the tantivy search engine library.

use std::path::{Path, PathBuf};
use std::fs;

use tantivy::collector::TopDocs;
use tantivy::query::{QueryParser, RegexQuery};
use tantivy::schema::{Schema, STORED, TEXT, Field, Value};
use tantivy::{Index, IndexWriter, TantivyDocument};

use crate::error_chirho::{ErrorChirho, ResultChirho};
use crate::keys_chirho::{ListKeyChirho, StrKeyChirho};
use super::{SearchEngineChirho, SearchOptionsChirho, SearchTypeChirho};

/// Index field names.
const KEY_FIELD_CHIRHO: &str = "key";
const TEXT_FIELD_CHIRHO: &str = "text";

/// Tantivy-based search engine.
pub struct TantivySearchChirho {
    /// Path to the index directory.
    index_path_chirho: PathBuf,
    /// Tantivy index (if open).
    index_chirho: Option<Index>,
    /// Schema.
    schema_chirho: Schema,
    /// Key field.
    key_field_chirho: Field,
    /// Text field.
    text_field_chirho: Field,
}

impl TantivySearchChirho {
    /// Create a new tantivy search engine.
    pub fn new_chirho<P: AsRef<Path>>(module_path_chirho: P) -> ResultChirho<Self> {
        let index_path_chirho = module_path_chirho.as_ref().join("lucene");

        // Build schema
        let mut schema_builder_chirho = Schema::builder();
        let key_field_chirho = schema_builder_chirho.add_text_field(KEY_FIELD_CHIRHO, TEXT | STORED);
        let text_field_chirho = schema_builder_chirho.add_text_field(TEXT_FIELD_CHIRHO, TEXT);
        let schema_chirho = schema_builder_chirho.build();

        let mut engine_chirho = Self {
            index_path_chirho,
            index_chirho: None,
            schema_chirho,
            key_field_chirho,
            text_field_chirho,
        };

        // Try to open existing index
        if engine_chirho.has_index_chirho() {
            engine_chirho.open_index_chirho()?;
        }

        Ok(engine_chirho)
    }

    /// Open an existing index.
    fn open_index_chirho(&mut self) -> ResultChirho<()> {
        let index_chirho = Index::open_in_dir(&self.index_path_chirho)
            .map_err(|e_chirho| ErrorChirho::IoChirho(std::io::Error::other(
                format!("Failed to open index: {}", e_chirho)
            )))?;

        self.index_chirho = Some(index_chirho);
        Ok(())
    }

    /// Add a document to the index.
    pub fn add_document_chirho(
        &self,
        writer_chirho: &mut IndexWriter,
        key_chirho: &str,
        text_chirho: &str,
    ) -> ResultChirho<()> {
        let mut doc_chirho = TantivyDocument::default();
        doc_chirho.add_text(self.key_field_chirho, key_chirho);
        doc_chirho.add_text(self.text_field_chirho, text_chirho);

        writer_chirho.add_document(doc_chirho)
            .map_err(|e_chirho| ErrorChirho::IoChirho(std::io::Error::other(
                format!("Failed to add document: {}", e_chirho)
            )))?;

        Ok(())
    }

    /// Create an index writer.
    pub fn create_writer_chirho(&mut self) -> ResultChirho<IndexWriter> {
        // Create index directory
        fs::create_dir_all(&self.index_path_chirho)?;

        // Create or open index
        let index_chirho = Index::create_in_dir(&self.index_path_chirho, self.schema_chirho.clone())
            .map_err(|e_chirho| ErrorChirho::IoChirho(std::io::Error::other(
                format!("Failed to create index: {}", e_chirho)
            )))?;

        // Create writer with 50MB heap
        let writer_chirho = index_chirho.writer(50_000_000)
            .map_err(|e_chirho| ErrorChirho::IoChirho(std::io::Error::other(
                format!("Failed to create writer: {}", e_chirho)
            )))?;

        self.index_chirho = Some(index_chirho);
        Ok(writer_chirho)
    }

    /// Commit the index writer.
    pub fn commit_chirho(&self, writer_chirho: &mut IndexWriter) -> ResultChirho<()> {
        writer_chirho.commit()
            .map_err(|e_chirho| ErrorChirho::IoChirho(std::io::Error::other(
                format!("Failed to commit: {}", e_chirho)
            )))?;
        Ok(())
    }

    /// Search with a query string.
    fn search_query_chirho(
        &self,
        query_str_chirho: &str,
        max_results_chirho: usize,
    ) -> ResultChirho<ListKeyChirho> {
        let index_chirho = self.index_chirho.as_ref()
            .ok_or_else(|| ErrorChirho::ModuleNotFoundChirho {
                module_name_chirho: "Search index not open".to_string(),
            })?;

        let reader_chirho = index_chirho.reader()
            .map_err(|e_chirho| ErrorChirho::IoChirho(std::io::Error::other(
                format!("Failed to create reader: {}", e_chirho)
            )))?;

        let searcher_chirho = reader_chirho.searcher();

        let query_parser_chirho = QueryParser::for_index(index_chirho, vec![self.text_field_chirho]);
        let query_chirho = query_parser_chirho.parse_query(query_str_chirho)
            .map_err(|e_chirho| ErrorChirho::IoChirho(std::io::Error::other(
                format!("Failed to parse query: {}", e_chirho)
            )))?;

        let top_docs_chirho = searcher_chirho.search(&query_chirho, &TopDocs::with_limit(max_results_chirho))
            .map_err(|e_chirho| ErrorChirho::IoChirho(std::io::Error::other(
                format!("Search failed: {}", e_chirho)
            )))?;

        let mut results_chirho = ListKeyChirho::new_chirho();

        for (_score_chirho, doc_address_chirho) in top_docs_chirho {
            let doc_chirho: TantivyDocument = searcher_chirho.doc(doc_address_chirho)
                .map_err(|e_chirho| ErrorChirho::IoChirho(std::io::Error::other(
                    format!("Failed to retrieve doc: {}", e_chirho)
                )))?;

            if let Some(key_value_chirho) = doc_chirho.get_first(self.key_field_chirho) {
                if let Some(key_text_chirho) = key_value_chirho.as_str() {
                    let key_chirho = Box::new(StrKeyChirho::with_text_chirho(key_text_chirho));
                    results_chirho.add_chirho(key_chirho);
                }
            }
        }

        Ok(results_chirho)
    }

    /// Search with a regex pattern.
    fn search_regex_chirho(
        &self,
        pattern_chirho: &str,
        max_results_chirho: usize,
    ) -> ResultChirho<ListKeyChirho> {
        let index_chirho = self.index_chirho.as_ref()
            .ok_or_else(|| ErrorChirho::ModuleNotFoundChirho {
                module_name_chirho: "Search index not open".to_string(),
            })?;

        let reader_chirho = index_chirho.reader()
            .map_err(|e_chirho| ErrorChirho::IoChirho(std::io::Error::other(
                format!("Failed to create reader: {}", e_chirho)
            )))?;

        let searcher_chirho = reader_chirho.searcher();

        let query_chirho = RegexQuery::from_pattern(pattern_chirho, self.text_field_chirho)
            .map_err(|e_chirho| ErrorChirho::IoChirho(std::io::Error::other(
                format!("Invalid regex: {}", e_chirho)
            )))?;

        let top_docs_chirho = searcher_chirho.search(&query_chirho, &TopDocs::with_limit(max_results_chirho))
            .map_err(|e_chirho| ErrorChirho::IoChirho(std::io::Error::other(
                format!("Search failed: {}", e_chirho)
            )))?;

        let mut results_chirho = ListKeyChirho::new_chirho();

        for (_score_chirho, doc_address_chirho) in top_docs_chirho {
            let doc_chirho: TantivyDocument = searcher_chirho.doc(doc_address_chirho)
                .map_err(|e_chirho| ErrorChirho::IoChirho(std::io::Error::other(
                    format!("Failed to retrieve doc: {}", e_chirho)
                )))?;

            if let Some(key_value_chirho) = doc_chirho.get_first(self.key_field_chirho) {
                if let Some(key_text_chirho) = key_value_chirho.as_str() {
                    let key_chirho = Box::new(StrKeyChirho::with_text_chirho(key_text_chirho));
                    results_chirho.add_chirho(key_chirho);
                }
            }
        }

        Ok(results_chirho)
    }
}

impl SearchEngineChirho for TantivySearchChirho {
    fn search_chirho(
        &self,
        pattern_chirho: &str,
        options_chirho: &SearchOptionsChirho,
    ) -> ResultChirho<ListKeyChirho> {
        let max_results_chirho = options_chirho.max_results_chirho.unwrap_or(1000);

        let search_type_chirho = options_chirho.search_type_chirho
            .unwrap_or(SearchTypeChirho::PhraseChirho);

        match search_type_chirho {
            SearchTypeChirho::RegexChirho => {
                self.search_regex_chirho(pattern_chirho, max_results_chirho)
            }
            SearchTypeChirho::PhraseChirho => {
                // Quote the phrase for exact matching
                let query_chirho = format!("\"{}\"", pattern_chirho);
                self.search_query_chirho(&query_chirho, max_results_chirho)
            }
            SearchTypeChirho::MultiWordChirho => {
                // AND all words together
                let words_chirho: Vec<&str> = pattern_chirho.split_whitespace().collect();
                let query_chirho = words_chirho.join(" AND ");
                self.search_query_chirho(&query_chirho, max_results_chirho)
            }
            SearchTypeChirho::ExternalChirho | SearchTypeChirho::EntryAttrChirho => {
                // Default to phrase search
                self.search_query_chirho(pattern_chirho, max_results_chirho)
            }
        }
    }

    fn create_index_chirho(&mut self) -> ResultChirho<()> {
        // Create index directory and schema
        fs::create_dir_all(&self.index_path_chirho)?;

        let index_chirho = Index::create_in_dir(&self.index_path_chirho, self.schema_chirho.clone())
            .map_err(|e_chirho| ErrorChirho::IoChirho(std::io::Error::other(
                format!("Failed to create index: {}", e_chirho)
            )))?;

        self.index_chirho = Some(index_chirho);
        Ok(())
    }

    fn has_index_chirho(&self) -> bool {
        self.index_path_chirho.exists() && self.index_path_chirho.is_dir()
    }

    fn delete_index_chirho(&mut self) -> ResultChirho<()> {
        if self.index_path_chirho.exists() {
            fs::remove_dir_all(&self.index_path_chirho)?;
        }
        self.index_chirho = None;
        Ok(())
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_create_index_chirho() {
        let temp_dir_chirho = TempDir::new().unwrap();
        let mut search_chirho = TantivySearchChirho::new_chirho(temp_dir_chirho.path()).unwrap();

        assert!(!search_chirho.has_index_chirho());

        search_chirho.create_index_chirho().unwrap();

        assert!(search_chirho.has_index_chirho());
    }

    #[test]
    fn test_add_and_search_chirho() {
        let temp_dir_chirho = TempDir::new().unwrap();
        let mut search_chirho = TantivySearchChirho::new_chirho(temp_dir_chirho.path()).unwrap();

        // Create index and add documents
        let mut writer_chirho = search_chirho.create_writer_chirho().unwrap();

        search_chirho.add_document_chirho(
            &mut writer_chirho,
            "Gen.1.1",
            "In the beginning God created the heaven and the earth",
        ).unwrap();

        search_chirho.add_document_chirho(
            &mut writer_chirho,
            "John.3.16",
            "For God so loved the world that he gave his only begotten Son",
        ).unwrap();

        search_chirho.commit_chirho(&mut writer_chirho).unwrap();

        // Search for "God"
        let options_chirho = SearchOptionsChirho {
            search_type_chirho: Some(SearchTypeChirho::PhraseChirho),
            max_results_chirho: Some(10),
            ..Default::default()
        };

        let results_chirho = search_chirho.search_chirho("God", &options_chirho).unwrap();
        assert_eq!(results_chirho.count_chirho(), 2);
    }

    #[test]
    fn test_delete_index_chirho() {
        let temp_dir_chirho = TempDir::new().unwrap();
        let mut search_chirho = TantivySearchChirho::new_chirho(temp_dir_chirho.path()).unwrap();

        search_chirho.create_index_chirho().unwrap();
        assert!(search_chirho.has_index_chirho());

        search_chirho.delete_index_chirho().unwrap();
        assert!(!search_chirho.has_index_chirho());
    }
}
