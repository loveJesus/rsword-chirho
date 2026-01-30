// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Search query parser for SWORD-compatible syntax.
//!
//! Supports:
//! - Boolean operators: AND, OR, NOT
//! - Phrase proximity: "word1 word2"~N
//! - Wildcards: lov*, ?ove
//! - Field-specific search: strongs:G26
//! - Fuzzy search: love~, love~2
//! - Grouping: (word1 OR word2) AND word3

use regex::Regex;
use std::sync::LazyLock;

/// Pattern for quoted phrases with optional proximity.
static PHRASE_PATTERN_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#""([^"]+)"(?:~(\d+))?"#).unwrap()
});

/// Pattern for field-specific searches.
static FIELD_PATTERN_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(\w+):(\S+)"#).unwrap()
});

/// Pattern for fuzzy searches.
static FUZZY_PATTERN_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(\w+)~(\d*)"#).unwrap()
});

/// Pattern for wildcard searches.
static WILDCARD_PATTERN_CHIRHO: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"[\w]*[*?][\w]*"#).unwrap()
});

/// Parsed search query node.
#[derive(Debug, Clone, PartialEq)]
pub enum QueryNodeChirho {
    /// Simple term.
    TermChirho(String),
    /// Phrase (multiple words in order).
    PhraseChirho {
        /// The phrase words.
        words_chirho: Vec<String>,
        /// Optional proximity (slop).
        slop_chirho: Option<u32>,
    },
    /// Boolean AND.
    AndChirho(Box<QueryNodeChirho>, Box<QueryNodeChirho>),
    /// Boolean OR.
    OrChirho(Box<QueryNodeChirho>, Box<QueryNodeChirho>),
    /// Boolean NOT.
    NotChirho(Box<QueryNodeChirho>),
    /// Field-specific search.
    FieldChirho {
        /// Field name.
        field_chirho: String,
        /// Query for that field.
        query_chirho: Box<QueryNodeChirho>,
    },
    /// Fuzzy search.
    FuzzyChirho {
        /// The term.
        term_chirho: String,
        /// Edit distance (default 1-2).
        distance_chirho: u8,
    },
    /// Wildcard search.
    WildcardChirho(String),
    /// Grouped query.
    GroupChirho(Box<QueryNodeChirho>),
}

/// Search query parser.
#[derive(Debug, Default)]
pub struct QueryParserChirho {
    /// Default field for searches.
    #[allow(dead_code)]
    default_field_chirho: String,
}

impl QueryParserChirho {
    /// Create a new query parser.
    pub fn new_chirho() -> Self {
        Self {
            default_field_chirho: "text".to_string(),
        }
    }

    /// Create with a specific default field.
    pub fn with_default_field_chirho(field_chirho: &str) -> Self {
        Self {
            default_field_chirho: field_chirho.to_string(),
        }
    }

    /// Parse a query string into a query node.
    pub fn parse_chirho(&self, query_chirho: &str) -> Result<QueryNodeChirho, String> {
        let trimmed_chirho = query_chirho.trim();
        if trimmed_chirho.is_empty() {
            return Err("Empty query".to_string());
        }

        self.parse_or_chirho(trimmed_chirho)
    }

    /// Parse OR expressions (lowest precedence).
    fn parse_or_chirho(&self, query_chirho: &str) -> Result<QueryNodeChirho, String> {
        // Split on OR but not inside quotes
        let parts_chirho = self.split_operator_chirho(query_chirho, " OR ");
        if parts_chirho.len() > 1 {
            let mut result_chirho = self.parse_and_chirho(parts_chirho[0].trim())?;
            for part_chirho in &parts_chirho[1..] {
                let right_chirho = self.parse_and_chirho(part_chirho.trim())?;
                result_chirho = QueryNodeChirho::OrChirho(
                    Box::new(result_chirho),
                    Box::new(right_chirho),
                );
            }
            return Ok(result_chirho);
        }

        self.parse_and_chirho(query_chirho)
    }

    /// Parse AND expressions.
    fn parse_and_chirho(&self, query_chirho: &str) -> Result<QueryNodeChirho, String> {
        // Split on AND
        let parts_chirho = self.split_operator_chirho(query_chirho, " AND ");
        if parts_chirho.len() > 1 {
            let mut result_chirho = self.parse_not_chirho(parts_chirho[0].trim())?;
            for part_chirho in &parts_chirho[1..] {
                let right_chirho = self.parse_not_chirho(part_chirho.trim())?;
                result_chirho = QueryNodeChirho::AndChirho(
                    Box::new(result_chirho),
                    Box::new(right_chirho),
                );
            }
            return Ok(result_chirho);
        }

        self.parse_not_chirho(query_chirho)
    }

    /// Parse NOT expressions.
    fn parse_not_chirho(&self, query_chirho: &str) -> Result<QueryNodeChirho, String> {
        let trimmed_chirho = query_chirho.trim();

        if let Some(rest_chirho) = trimmed_chirho.strip_prefix("NOT ") {
            let parsed_chirho = self.parse_primary_chirho(rest_chirho.trim())?;
            return Ok(QueryNodeChirho::NotChirho(Box::new(parsed_chirho)));
        }

        if let Some(rest_chirho) = trimmed_chirho.strip_prefix('-') {
            let parsed_chirho = self.parse_primary_chirho(rest_chirho.trim())?;
            return Ok(QueryNodeChirho::NotChirho(Box::new(parsed_chirho)));
        }

        self.parse_primary_chirho(trimmed_chirho)
    }

    /// Parse primary expressions (terms, phrases, groups).
    fn parse_primary_chirho(&self, query_chirho: &str) -> Result<QueryNodeChirho, String> {
        let trimmed_chirho = query_chirho.trim();

        // Handle grouped expressions
        if trimmed_chirho.starts_with('(') && trimmed_chirho.ends_with(')') {
            let inner_chirho = &trimmed_chirho[1..trimmed_chirho.len() - 1];
            let parsed_chirho = self.parse_or_chirho(inner_chirho)?;
            return Ok(QueryNodeChirho::GroupChirho(Box::new(parsed_chirho)));
        }

        // Handle quoted phrases with optional proximity
        if let Some(captures_chirho) = PHRASE_PATTERN_CHIRHO.captures(trimmed_chirho) {
            let phrase_chirho = captures_chirho.get(1).unwrap().as_str();
            let slop_chirho = captures_chirho.get(2)
                .and_then(|m_chirho| m_chirho.as_str().parse::<u32>().ok());

            let words_chirho: Vec<String> = phrase_chirho
                .split_whitespace()
                .map(|s_chirho| s_chirho.to_string())
                .collect();

            return Ok(QueryNodeChirho::PhraseChirho {
                words_chirho,
                slop_chirho,
            });
        }

        // Handle field-specific search
        if let Some(captures_chirho) = FIELD_PATTERN_CHIRHO.captures(trimmed_chirho) {
            let field_chirho = captures_chirho.get(1).unwrap().as_str().to_string();
            let value_chirho = captures_chirho.get(2).unwrap().as_str();
            let inner_query_chirho = self.parse_primary_chirho(value_chirho)?;
            return Ok(QueryNodeChirho::FieldChirho {
                field_chirho,
                query_chirho: Box::new(inner_query_chirho),
            });
        }

        // Handle fuzzy search
        if let Some(captures_chirho) = FUZZY_PATTERN_CHIRHO.captures(trimmed_chirho) {
            if !WILDCARD_PATTERN_CHIRHO.is_match(trimmed_chirho) {
                let term_chirho = captures_chirho.get(1).unwrap().as_str().to_string();
                let distance_str_chirho = captures_chirho.get(2).unwrap().as_str();
                let distance_chirho = if distance_str_chirho.is_empty() {
                    2
                } else {
                    distance_str_chirho.parse::<u8>().unwrap_or(2).min(2)
                };
                return Ok(QueryNodeChirho::FuzzyChirho {
                    term_chirho,
                    distance_chirho,
                });
            }
        }

        // Handle wildcard search
        if WILDCARD_PATTERN_CHIRHO.is_match(trimmed_chirho) {
            return Ok(QueryNodeChirho::WildcardChirho(trimmed_chirho.to_string()));
        }

        // Handle implicit AND for multiple space-separated terms
        let words_chirho: Vec<&str> = trimmed_chirho.split_whitespace().collect();
        if words_chirho.len() > 1 {
            let mut result_chirho = QueryNodeChirho::TermChirho(words_chirho[0].to_string());
            for word_chirho in &words_chirho[1..] {
                result_chirho = QueryNodeChirho::AndChirho(
                    Box::new(result_chirho),
                    Box::new(QueryNodeChirho::TermChirho(word_chirho.to_string())),
                );
            }
            return Ok(result_chirho);
        }

        // Simple term
        Ok(QueryNodeChirho::TermChirho(trimmed_chirho.to_string()))
    }

    /// Split on an operator, respecting quoted strings.
    fn split_operator_chirho<'a>(&self, query_chirho: &'a str, operator_chirho: &str) -> Vec<&'a str> {
        let mut parts_chirho = Vec::new();
        let mut in_quote_chirho = false;
        let mut paren_depth_chirho: u32 = 0;
        let mut last_split_chirho = 0;

        let chars_chirho: Vec<char> = query_chirho.chars().collect();
        let op_len_chirho = operator_chirho.len();

        for (i_chirho, ch_chirho) in chars_chirho.iter().enumerate() {
            match ch_chirho {
                '"' => in_quote_chirho = !in_quote_chirho,
                '(' if !in_quote_chirho => paren_depth_chirho += 1,
                ')' if !in_quote_chirho => paren_depth_chirho = paren_depth_chirho.saturating_sub(1),
                _ => {}
            }

            if !in_quote_chirho && paren_depth_chirho == 0
                && query_chirho[i_chirho..].starts_with(operator_chirho)
            {
                parts_chirho.push(&query_chirho[last_split_chirho..i_chirho]);
                last_split_chirho = i_chirho + op_len_chirho;
            }
        }

        parts_chirho.push(&query_chirho[last_split_chirho..]);
        parts_chirho
    }
}

/// Convert a query node to tantivy query syntax.
pub fn to_tantivy_query_chirho(node_chirho: &QueryNodeChirho) -> String {
    match node_chirho {
        QueryNodeChirho::TermChirho(term_chirho) => {
            term_chirho.to_lowercase()
        }
        QueryNodeChirho::PhraseChirho { words_chirho, slop_chirho } => {
            let phrase_chirho = words_chirho.join(" ");
            if let Some(slop_chirho) = slop_chirho {
                format!("\"{}\"~{}", phrase_chirho, slop_chirho)
            } else {
                format!("\"{}\"", phrase_chirho)
            }
        }
        QueryNodeChirho::AndChirho(left_chirho, right_chirho) => {
            format!(
                "({} AND {})",
                to_tantivy_query_chirho(left_chirho),
                to_tantivy_query_chirho(right_chirho)
            )
        }
        QueryNodeChirho::OrChirho(left_chirho, right_chirho) => {
            format!(
                "({} OR {})",
                to_tantivy_query_chirho(left_chirho),
                to_tantivy_query_chirho(right_chirho)
            )
        }
        QueryNodeChirho::NotChirho(inner_chirho) => {
            format!("-{}", to_tantivy_query_chirho(inner_chirho))
        }
        QueryNodeChirho::FieldChirho { field_chirho, query_chirho } => {
            format!("{}:{}", field_chirho, to_tantivy_query_chirho(query_chirho))
        }
        QueryNodeChirho::FuzzyChirho { term_chirho, distance_chirho } => {
            format!("{}~{}", term_chirho.to_lowercase(), distance_chirho)
        }
        QueryNodeChirho::WildcardChirho(pattern_chirho) => {
            pattern_chirho.to_lowercase()
        }
        QueryNodeChirho::GroupChirho(inner_chirho) => {
            format!("({})", to_tantivy_query_chirho(inner_chirho))
        }
    }
}

/// Search scope for limiting search range.
#[derive(Debug, Clone, PartialEq)]
pub enum SearchScopeChirho {
    /// Search entire module.
    AllChirho,
    /// Search Old Testament only.
    OldTestamentChirho,
    /// Search New Testament only.
    NewTestamentChirho,
    /// Search a specific book.
    BookChirho(String),
    /// Search a range of books.
    BookRangeChirho(String, String),
    /// Search a specific chapter.
    ChapterChirho(String, u32),
    /// Custom range (e.g., "Gen 1-3").
    CustomChirho(String),
}

impl SearchScopeChirho {
    /// Parse a scope string.
    pub fn parse_chirho(scope_str_chirho: &str) -> Self {
        let trimmed_chirho = scope_str_chirho.trim();

        if trimmed_chirho.is_empty() || trimmed_chirho.eq_ignore_ascii_case("all") {
            return SearchScopeChirho::AllChirho;
        }

        if trimmed_chirho.eq_ignore_ascii_case("ot") {
            return SearchScopeChirho::OldTestamentChirho;
        }

        if trimmed_chirho.eq_ignore_ascii_case("nt") {
            return SearchScopeChirho::NewTestamentChirho;
        }

        // Check for range (e.g., "Gen-Exod")
        if trimmed_chirho.contains('-') && !trimmed_chirho.contains(' ') {
            let parts_chirho: Vec<&str> = trimmed_chirho.split('-').collect();
            if parts_chirho.len() == 2 {
                return SearchScopeChirho::BookRangeChirho(
                    parts_chirho[0].to_string(),
                    parts_chirho[1].to_string(),
                );
            }
        }

        // Check for chapter (e.g., "Gen 1")
        if let Some(space_pos_chirho) = trimmed_chirho.find(' ') {
            let book_chirho = &trimmed_chirho[..space_pos_chirho];
            let chapter_str_chirho = &trimmed_chirho[space_pos_chirho + 1..];
            if let Ok(chapter_chirho) = chapter_str_chirho.parse::<u32>() {
                return SearchScopeChirho::ChapterChirho(book_chirho.to_string(), chapter_chirho);
            }
        }

        // Assume single book
        SearchScopeChirho::BookChirho(trimmed_chirho.to_string())
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_parse_simple_term_chirho() {
        let parser_chirho = QueryParserChirho::new_chirho();
        let result_chirho = parser_chirho.parse_chirho("love").unwrap();
        assert_eq!(result_chirho, QueryNodeChirho::TermChirho("love".to_string()));
    }

    #[test]
    fn test_parse_phrase_chirho() {
        let parser_chirho = QueryParserChirho::new_chirho();
        let result_chirho = parser_chirho.parse_chirho("\"God is love\"").unwrap();

        if let QueryNodeChirho::PhraseChirho { words_chirho, slop_chirho } = result_chirho {
            assert_eq!(words_chirho, vec!["God", "is", "love"]);
            assert_eq!(slop_chirho, None);
        } else {
            panic!("Expected phrase node");
        }
    }

    #[test]
    fn test_parse_phrase_with_slop_chirho() {
        let parser_chirho = QueryParserChirho::new_chirho();
        let result_chirho = parser_chirho.parse_chirho("\"God love\"~5").unwrap();

        if let QueryNodeChirho::PhraseChirho { words_chirho, slop_chirho } = result_chirho {
            assert_eq!(words_chirho, vec!["God", "love"]);
            assert_eq!(slop_chirho, Some(5));
        } else {
            panic!("Expected phrase node");
        }
    }

    #[test]
    fn test_parse_and_chirho() {
        let parser_chirho = QueryParserChirho::new_chirho();
        let result_chirho = parser_chirho.parse_chirho("God AND love").unwrap();

        if let QueryNodeChirho::AndChirho(left_chirho, right_chirho) = result_chirho {
            assert_eq!(*left_chirho, QueryNodeChirho::TermChirho("God".to_string()));
            assert_eq!(*right_chirho, QueryNodeChirho::TermChirho("love".to_string()));
        } else {
            panic!("Expected AND node");
        }
    }

    #[test]
    fn test_parse_or_chirho() {
        let parser_chirho = QueryParserChirho::new_chirho();
        let result_chirho = parser_chirho.parse_chirho("God OR Lord").unwrap();

        if let QueryNodeChirho::OrChirho(left_chirho, right_chirho) = result_chirho {
            assert_eq!(*left_chirho, QueryNodeChirho::TermChirho("God".to_string()));
            assert_eq!(*right_chirho, QueryNodeChirho::TermChirho("Lord".to_string()));
        } else {
            panic!("Expected OR node");
        }
    }

    #[test]
    fn test_parse_not_chirho() {
        let parser_chirho = QueryParserChirho::new_chirho();
        let result_chirho = parser_chirho.parse_chirho("NOT evil").unwrap();

        if let QueryNodeChirho::NotChirho(inner_chirho) = result_chirho {
            assert_eq!(*inner_chirho, QueryNodeChirho::TermChirho("evil".to_string()));
        } else {
            panic!("Expected NOT node");
        }
    }

    #[test]
    fn test_parse_field_chirho() {
        let parser_chirho = QueryParserChirho::new_chirho();
        let result_chirho = parser_chirho.parse_chirho("strongs:G26").unwrap();

        if let QueryNodeChirho::FieldChirho { field_chirho, query_chirho } = result_chirho {
            assert_eq!(field_chirho, "strongs");
            assert_eq!(*query_chirho, QueryNodeChirho::TermChirho("G26".to_string()));
        } else {
            panic!("Expected Field node");
        }
    }

    #[test]
    fn test_parse_fuzzy_chirho() {
        let parser_chirho = QueryParserChirho::new_chirho();
        let result_chirho = parser_chirho.parse_chirho("love~").unwrap();

        if let QueryNodeChirho::FuzzyChirho { term_chirho, distance_chirho } = result_chirho {
            assert_eq!(term_chirho, "love");
            assert_eq!(distance_chirho, 2);
        } else {
            panic!("Expected Fuzzy node");
        }
    }

    #[test]
    fn test_parse_fuzzy_with_distance_chirho() {
        let parser_chirho = QueryParserChirho::new_chirho();
        let result_chirho = parser_chirho.parse_chirho("love~1").unwrap();

        if let QueryNodeChirho::FuzzyChirho { term_chirho, distance_chirho } = result_chirho {
            assert_eq!(term_chirho, "love");
            assert_eq!(distance_chirho, 1);
        } else {
            panic!("Expected Fuzzy node");
        }
    }

    #[test]
    fn test_parse_wildcard_chirho() {
        let parser_chirho = QueryParserChirho::new_chirho();
        let result_chirho = parser_chirho.parse_chirho("lov*").unwrap();

        if let QueryNodeChirho::WildcardChirho(pattern_chirho) = result_chirho {
            assert_eq!(pattern_chirho, "lov*");
        } else {
            panic!("Expected Wildcard node");
        }
    }

    #[test]
    fn test_parse_complex_chirho() {
        let parser_chirho = QueryParserChirho::new_chirho();
        let result_chirho = parser_chirho.parse_chirho("(God OR Lord) AND love").unwrap();

        // Should be AND with grouped OR on left
        if let QueryNodeChirho::AndChirho(left_chirho, right_chirho) = result_chirho {
            assert!(matches!(*left_chirho, QueryNodeChirho::GroupChirho(_)));
            assert_eq!(*right_chirho, QueryNodeChirho::TermChirho("love".to_string()));
        } else {
            panic!("Expected AND node");
        }
    }

    #[test]
    fn test_to_tantivy_query_chirho() {
        let node_chirho = QueryNodeChirho::AndChirho(
            Box::new(QueryNodeChirho::TermChirho("God".to_string())),
            Box::new(QueryNodeChirho::TermChirho("love".to_string())),
        );
        let tantivy_query_chirho = to_tantivy_query_chirho(&node_chirho);
        assert_eq!(tantivy_query_chirho, "(god AND love)");
    }

    #[test]
    fn test_scope_parse_chirho() {
        assert_eq!(SearchScopeChirho::parse_chirho(""), SearchScopeChirho::AllChirho);
        assert_eq!(SearchScopeChirho::parse_chirho("OT"), SearchScopeChirho::OldTestamentChirho);
        assert_eq!(SearchScopeChirho::parse_chirho("NT"), SearchScopeChirho::NewTestamentChirho);
        assert_eq!(
            SearchScopeChirho::parse_chirho("Gen-Exod"),
            SearchScopeChirho::BookRangeChirho("Gen".to_string(), "Exod".to_string())
        );
        assert_eq!(
            SearchScopeChirho::parse_chirho("John"),
            SearchScopeChirho::BookChirho("John".to_string())
        );
        assert_eq!(
            SearchScopeChirho::parse_chirho("Gen 1"),
            SearchScopeChirho::ChapterChirho("Gen".to_string(), 1)
        );
    }

    #[test]
    fn test_implicit_and_chirho() {
        let parser_chirho = QueryParserChirho::new_chirho();
        let result_chirho = parser_chirho.parse_chirho("God love world").unwrap();

        // Should be nested ANDs
        if let QueryNodeChirho::AndChirho(_, right_chirho) = result_chirho {
            assert_eq!(*right_chirho, QueryNodeChirho::TermChirho("world".to_string()));
        } else {
            panic!("Expected AND node for implicit AND");
        }
    }
}
