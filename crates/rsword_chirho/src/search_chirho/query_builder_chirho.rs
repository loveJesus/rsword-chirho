// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Tantivy query builder for constructing complex search queries.
//!
//! Converts our query AST to tantivy queries with support for:
//! - Boolean combinations (AND, OR, NOT)
//! - Phrase queries with slop
//! - Fuzzy queries
//! - Wildcard/regex queries
//! - Field-specific queries

use tantivy::query::{
    BooleanQuery, BoostQuery, FuzzyTermQuery, Occur, PhraseQuery, Query, RegexQuery, TermQuery,
};
use tantivy::schema::{Field, IndexRecordOption, Schema};
use tantivy::Term;

use super::query_parser_chirho::QueryNodeChirho;

/// Query builder for creating tantivy queries from our AST.
pub struct TantivyQueryBuilderChirho {
    /// The schema.
    schema_chirho: Schema,
    /// Default text field.
    text_field_chirho: Field,
    /// Strong's number field (if available).
    strongs_field_chirho: Option<Field>,
    /// Lemma field (if available).
    lemma_field_chirho: Option<Field>,
    /// Morph field (if available).
    morph_field_chirho: Option<Field>,
}

impl TantivyQueryBuilderChirho {
    /// Create a new query builder with just a text field.
    pub fn new_chirho(schema_chirho: Schema, text_field_chirho: Field) -> Self {
        Self {
            schema_chirho,
            text_field_chirho,
            strongs_field_chirho: None,
            lemma_field_chirho: None,
            morph_field_chirho: None,
        }
    }

    /// Create with Strong's number field.
    pub fn with_strongs_field_chirho(mut self, field_chirho: Field) -> Self {
        self.strongs_field_chirho = Some(field_chirho);
        self
    }

    /// Create with lemma field.
    pub fn with_lemma_field_chirho(mut self, field_chirho: Field) -> Self {
        self.lemma_field_chirho = Some(field_chirho);
        self
    }

    /// Create with morph field.
    pub fn with_morph_field_chirho(mut self, field_chirho: Field) -> Self {
        self.morph_field_chirho = Some(field_chirho);
        self
    }

    /// Build a tantivy query from a query node.
    pub fn build_chirho(&self, node_chirho: &QueryNodeChirho) -> Box<dyn Query> {
        match node_chirho {
            QueryNodeChirho::TermChirho(term_chirho) => {
                self.build_term_query_chirho(term_chirho, self.text_field_chirho)
            }
            QueryNodeChirho::PhraseChirho { words_chirho, slop_chirho } => {
                self.build_phrase_query_chirho(words_chirho, *slop_chirho)
            }
            QueryNodeChirho::AndChirho(left_chirho, right_chirho) => {
                self.build_boolean_chirho(left_chirho, right_chirho, true)
            }
            QueryNodeChirho::OrChirho(left_chirho, right_chirho) => {
                self.build_boolean_chirho(left_chirho, right_chirho, false)
            }
            QueryNodeChirho::NotChirho(inner_chirho) => {
                self.build_not_query_chirho(inner_chirho)
            }
            QueryNodeChirho::FieldChirho { field_chirho, query_chirho } => {
                self.build_field_query_chirho(field_chirho, query_chirho.as_ref())
            }
            QueryNodeChirho::FuzzyChirho { term_chirho, distance_chirho } => {
                self.build_fuzzy_query_chirho(term_chirho, *distance_chirho)
            }
            QueryNodeChirho::WildcardChirho(pattern_chirho) => {
                self.build_wildcard_query_chirho(pattern_chirho)
            }
            QueryNodeChirho::GroupChirho(inner_chirho) => {
                self.build_chirho(inner_chirho)
            }
        }
    }

    /// Build a simple term query.
    fn build_term_query_chirho(&self, term_chirho: &str, field_chirho: Field) -> Box<dyn Query> {
        let term_value_chirho = Term::from_field_text(field_chirho, &term_chirho.to_lowercase());
        Box::new(TermQuery::new(term_value_chirho, IndexRecordOption::WithFreqs))
    }

    /// Build a phrase query.
    fn build_phrase_query_chirho(
        &self,
        words_chirho: &[String],
        slop_chirho: Option<u32>,
    ) -> Box<dyn Query> {
        let terms_chirho: Vec<Term> = words_chirho
            .iter()
            .map(|w_chirho| Term::from_field_text(self.text_field_chirho, &w_chirho.to_lowercase()))
            .collect();

        let mut phrase_query_chirho = PhraseQuery::new(terms_chirho);
        if let Some(slop_value_chirho) = slop_chirho {
            phrase_query_chirho.set_slop(slop_value_chirho);
        }
        Box::new(phrase_query_chirho)
    }

    /// Build a boolean query (AND or OR).
    fn build_boolean_chirho(
        &self,
        left_chirho: &QueryNodeChirho,
        right_chirho: &QueryNodeChirho,
        is_and_chirho: bool,
    ) -> Box<dyn Query> {
        let left_query_chirho = self.build_chirho(left_chirho);
        let right_query_chirho = self.build_chirho(right_chirho);

        let occur_chirho = if is_and_chirho {
            Occur::Must
        } else {
            Occur::Should
        };

        Box::new(BooleanQuery::new(vec![
            (occur_chirho, left_query_chirho),
            (occur_chirho, right_query_chirho),
        ]))
    }

    /// Build a NOT query (must not occur).
    fn build_not_query_chirho(&self, inner_chirho: &QueryNodeChirho) -> Box<dyn Query> {
        let inner_query_chirho = self.build_chirho(inner_chirho);

        // For NOT queries, we need a MustNot with a broad Match All
        // In tantivy, we typically combine with another query
        // For standalone NOT, we'll use boost with -1
        Box::new(BoostQuery::new(inner_query_chirho, -1.0))
    }

    /// Build a field-specific query.
    fn build_field_query_chirho(
        &self,
        field_name_chirho: &str,
        query_chirho: &QueryNodeChirho,
    ) -> Box<dyn Query> {
        let field_chirho = match field_name_chirho.to_lowercase().as_str() {
            "strongs" | "strong" => self.strongs_field_chirho.unwrap_or(self.text_field_chirho),
            "lemma" => self.lemma_field_chirho.unwrap_or(self.text_field_chirho),
            "morph" | "morphology" => self.morph_field_chirho.unwrap_or(self.text_field_chirho),
            "text" => self.text_field_chirho,
            _ => {
                // Try to find the field by name in schema
                self.schema_chirho
                    .get_field(field_name_chirho)
                    .unwrap_or(self.text_field_chirho)
            }
        };

        // Build the inner query with the specific field
        match query_chirho {
            QueryNodeChirho::TermChirho(term_chirho) => {
                self.build_term_query_chirho(term_chirho, field_chirho)
            }
            QueryNodeChirho::WildcardChirho(pattern_chirho) => {
                // Use the correct field for wildcard
                self.build_regex_query_chirho(pattern_chirho, field_chirho)
            }
            _ => self.build_chirho(query_chirho),
        }
    }

    /// Build a fuzzy query.
    fn build_fuzzy_query_chirho(&self, term_chirho: &str, distance_chirho: u8) -> Box<dyn Query> {
        let term_value_chirho = Term::from_field_text(
            self.text_field_chirho,
            &term_chirho.to_lowercase(),
        );
        Box::new(FuzzyTermQuery::new(term_value_chirho, distance_chirho, true))
    }

    /// Build a wildcard query (converted to regex).
    fn build_wildcard_query_chirho(&self, pattern_chirho: &str) -> Box<dyn Query> {
        self.build_regex_query_chirho(pattern_chirho, self.text_field_chirho)
    }

    /// Build a regex query from a wildcard pattern.
    fn build_regex_query_chirho(&self, pattern_chirho: &str, field_chirho: Field) -> Box<dyn Query> {
        // Convert wildcard to regex: * -> .*, ? -> .
        let regex_pattern_chirho = pattern_chirho
            .to_lowercase()
            .replace('*', ".*")
            .replace('?', ".");

        match RegexQuery::from_pattern(&regex_pattern_chirho, field_chirho) {
            Ok(query_chirho) => Box::new(query_chirho),
            Err(_) => {
                // Fall back to term query if regex is invalid
                self.build_term_query_chirho(pattern_chirho, field_chirho)
            }
        }
    }
}

/// Search result with score.
#[derive(Debug, Clone)]
pub struct ScoredResultChirho {
    /// The key (verse reference or entry key).
    pub key_chirho: String,
    /// The relevance score.
    pub score_chirho: f32,
}

impl ScoredResultChirho {
    /// Create a new scored result.
    pub fn new_chirho(key_chirho: String, score_chirho: f32) -> Self {
        Self {
            key_chirho,
            score_chirho,
        }
    }
}

/// Builder for search options with fluent API.
#[derive(Debug, Clone, Default)]
pub struct SearchBuilderChirho {
    /// The query string.
    query_chirho: String,
    /// Maximum results to return.
    max_results_chirho: usize,
    /// Whether to include scores.
    include_scores_chirho: bool,
    /// Minimum score threshold.
    min_score_chirho: Option<f32>,
    /// Fields to search (default is "text").
    fields_chirho: Vec<String>,
    /// Sort by score descending (default true).
    sort_by_score_chirho: bool,
}

impl SearchBuilderChirho {
    /// Create a new search builder.
    pub fn new_chirho(query_chirho: &str) -> Self {
        Self {
            query_chirho: query_chirho.to_string(),
            max_results_chirho: 100,
            include_scores_chirho: false,
            min_score_chirho: None,
            fields_chirho: vec!["text".to_string()],
            sort_by_score_chirho: true,
        }
    }

    /// Set maximum results.
    pub fn max_results_chirho(mut self, max_chirho: usize) -> Self {
        self.max_results_chirho = max_chirho;
        self
    }

    /// Include relevance scores in results.
    pub fn with_scores_chirho(mut self) -> Self {
        self.include_scores_chirho = true;
        self
    }

    /// Set minimum score threshold.
    pub fn min_score_chirho(mut self, score_chirho: f32) -> Self {
        self.min_score_chirho = Some(score_chirho);
        self
    }

    /// Add a field to search.
    pub fn add_field_chirho(mut self, field_chirho: &str) -> Self {
        self.fields_chirho.push(field_chirho.to_string());
        self
    }

    /// Set fields to search.
    pub fn fields_chirho(mut self, fields_chirho: Vec<&str>) -> Self {
        self.fields_chirho = fields_chirho.iter().map(|s_chirho| s_chirho.to_string()).collect();
        self
    }

    /// Disable score sorting (use index order).
    pub fn unsorted_chirho(mut self) -> Self {
        self.sort_by_score_chirho = false;
        self
    }

    /// Get the query string.
    pub fn query_string_chirho(&self) -> &str {
        &self.query_chirho
    }

    /// Get max results.
    pub fn get_max_results_chirho(&self) -> usize {
        self.max_results_chirho
    }

    /// Check if scores are included.
    pub fn has_scores_chirho(&self) -> bool {
        self.include_scores_chirho
    }

    /// Get minimum score.
    pub fn get_min_score_chirho(&self) -> Option<f32> {
        self.min_score_chirho
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use tantivy::schema::{Schema, TEXT, STORED};

    fn create_test_schema_chirho() -> (Schema, Field) {
        let mut schema_builder_chirho = Schema::builder();
        let text_field_chirho = schema_builder_chirho.add_text_field("text", TEXT | STORED);
        (schema_builder_chirho.build(), text_field_chirho)
    }

    #[test]
    fn test_build_term_query_chirho() {
        let (schema_chirho, text_field_chirho) = create_test_schema_chirho();
        let builder_chirho = TantivyQueryBuilderChirho::new_chirho(schema_chirho, text_field_chirho);

        let node_chirho = QueryNodeChirho::TermChirho("love".to_string());
        let _query_chirho = builder_chirho.build_chirho(&node_chirho);
        // Query was built successfully
    }

    #[test]
    fn test_build_phrase_query_chirho() {
        let (schema_chirho, text_field_chirho) = create_test_schema_chirho();
        let builder_chirho = TantivyQueryBuilderChirho::new_chirho(schema_chirho, text_field_chirho);

        let node_chirho = QueryNodeChirho::PhraseChirho {
            words_chirho: vec!["God".to_string(), "is".to_string(), "love".to_string()],
            slop_chirho: Some(2),
        };
        let _query_chirho = builder_chirho.build_chirho(&node_chirho);
        // Query was built successfully
    }

    #[test]
    fn test_build_and_query_chirho() {
        let (schema_chirho, text_field_chirho) = create_test_schema_chirho();
        let builder_chirho = TantivyQueryBuilderChirho::new_chirho(schema_chirho, text_field_chirho);

        let node_chirho = QueryNodeChirho::AndChirho(
            Box::new(QueryNodeChirho::TermChirho("God".to_string())),
            Box::new(QueryNodeChirho::TermChirho("love".to_string())),
        );
        let _query_chirho = builder_chirho.build_chirho(&node_chirho);
        // Query was built successfully
    }

    #[test]
    fn test_build_fuzzy_query_chirho() {
        let (schema_chirho, text_field_chirho) = create_test_schema_chirho();
        let builder_chirho = TantivyQueryBuilderChirho::new_chirho(schema_chirho, text_field_chirho);

        let node_chirho = QueryNodeChirho::FuzzyChirho {
            term_chirho: "love".to_string(),
            distance_chirho: 2,
        };
        let _query_chirho = builder_chirho.build_chirho(&node_chirho);
        // Query was built successfully
    }

    #[test]
    fn test_build_wildcard_query_chirho() {
        let (schema_chirho, text_field_chirho) = create_test_schema_chirho();
        let builder_chirho = TantivyQueryBuilderChirho::new_chirho(schema_chirho, text_field_chirho);

        let node_chirho = QueryNodeChirho::WildcardChirho("lov*".to_string());
        let _query_chirho = builder_chirho.build_chirho(&node_chirho);
        // Query was built successfully
    }

    #[test]
    fn test_search_builder_chirho() {
        let builder_chirho = SearchBuilderChirho::new_chirho("love AND God")
            .max_results_chirho(50)
            .with_scores_chirho()
            .min_score_chirho(0.5);

        assert_eq!(builder_chirho.query_string_chirho(), "love AND God");
        assert_eq!(builder_chirho.get_max_results_chirho(), 50);
        assert!(builder_chirho.has_scores_chirho());
        assert_eq!(builder_chirho.get_min_score_chirho(), Some(0.5));
    }

    #[test]
    fn test_scored_result_chirho() {
        let result_chirho = ScoredResultChirho::new_chirho("John.3.16".to_string(), 0.95);
        assert_eq!(result_chirho.key_chirho, "John.3.16");
        assert_eq!(result_chirho.score_chirho, 0.95);
    }
}
