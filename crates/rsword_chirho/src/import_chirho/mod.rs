// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Module import functionality.
//!
//! Parsers for OSIS, TEI, IMP, VPL, and other input formats.

mod vpl_parser_chirho;
mod osis_parser_chirho;
mod tei_parser_chirho;

use std::path::Path;

use crate::error_chirho::ResultChirho;

pub use vpl_parser_chirho::{VplParserChirho, VplParserConfigChirho, VplEntryChirho};
pub use osis_parser_chirho::{OsisParserChirho, OsisParserConfigChirho, OsisDocumentChirho, OsisVerseChirho};
pub use tei_parser_chirho::{TeiParserChirho, TeiParserConfigChirho, TeiDocumentChirho, TeiEntryChirho, TeiSenseChirho};

/// Importer trait for creating modules from various formats.
pub trait ImporterChirho {
    /// Import content from a file.
    fn import_file_chirho(&mut self, input_chirho: &Path, output_chirho: &Path) -> ResultChirho<()>;

    /// Get the importer name.
    fn name_chirho(&self) -> &str;
}

/// IMP (SWORD Import) format line.
#[derive(Debug, Clone)]
pub struct ImpEntryChirho {
    /// Entry key.
    pub key_chirho: String,
    /// Entry content.
    pub content_chirho: String,
}

/// Parse IMP format content.
pub fn parse_imp_chirho(content_chirho: &str) -> Vec<ImpEntryChirho> {
    let mut entries_chirho = Vec::new();
    let mut current_key_chirho: Option<String> = None;
    let mut current_content_chirho = String::new();

    for line_chirho in content_chirho.lines() {
        if let Some(stripped_chirho) = line_chirho.strip_prefix("$$$") {
            // Save previous entry
            if let Some(key_chirho) = current_key_chirho.take() {
                entries_chirho.push(ImpEntryChirho {
                    key_chirho,
                    content_chirho: current_content_chirho.trim().to_string(),
                });
            }

            // Start new entry
            current_key_chirho = Some(stripped_chirho.trim().to_string());
            current_content_chirho.clear();
        } else if current_key_chirho.is_some() {
            current_content_chirho.push_str(line_chirho);
            current_content_chirho.push('\n');
        }
    }

    // Save last entry
    if let Some(key_chirho) = current_key_chirho {
        entries_chirho.push(ImpEntryChirho {
            key_chirho,
            content_chirho: current_content_chirho.trim().to_string(),
        });
    }

    entries_chirho
}

// TODO: Implement OSIS parser
// TODO: Implement TEI parser

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_parse_imp_chirho() {
        let content_chirho = r#"$$$Genesis 1:1
In the beginning God created the heaven and the earth.
$$$Genesis 1:2
And the earth was without form, and void.
"#;

        let entries_chirho = parse_imp_chirho(content_chirho);
        assert_eq!(entries_chirho.len(), 2);
        assert_eq!(entries_chirho[0].key_chirho, "Genesis 1:1");
        assert!(entries_chirho[0].content_chirho.contains("In the beginning"));
    }
}
