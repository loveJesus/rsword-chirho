// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! VPL (Verse-Per-Line) format parser.
//!
//! VPL is a simple text format where each line contains a verse reference
//! followed by the verse text. This is commonly used for simple Bible imports.
//!
//! Format: `BookAbbrev Chapter:Verse Text...`
//! Example:
//! ```text
//! Gen 1:1 In the beginning God created the heaven and the earth.
//! Gen 1:2 And the earth was without form, and void;
//! ```

use crate::error_chirho::{ErrorChirho, ResultChirho};
use crate::import_chirho::ImpEntryChirho;

/// A parsed VPL entry.
#[derive(Debug, Clone)]
pub struct VplEntryChirho {
    /// Book name or abbreviation.
    pub book_chirho: String,
    /// Chapter number.
    pub chapter_chirho: u16,
    /// Verse number.
    pub verse_chirho: u16,
    /// Verse text content.
    pub text_chirho: String,
}

impl VplEntryChirho {
    /// Create a new VPL entry.
    pub fn new_chirho(book_chirho: &str, chapter_chirho: u16, verse_chirho: u16, text_chirho: &str) -> Self {
        Self {
            book_chirho: book_chirho.to_string(),
            chapter_chirho,
            verse_chirho,
            text_chirho: text_chirho.to_string(),
        }
    }

    /// Get the full reference string (e.g., "Gen 1:1").
    pub fn reference_chirho(&self) -> String {
        format!("{} {}:{}", self.book_chirho, self.chapter_chirho, self.verse_chirho)
    }

    /// Convert to IMP entry format.
    pub fn to_imp_entry_chirho(&self) -> ImpEntryChirho {
        ImpEntryChirho {
            key_chirho: self.reference_chirho(),
            content_chirho: self.text_chirho.clone(),
        }
    }
}

/// VPL parser configuration.
#[derive(Debug, Clone)]
pub struct VplParserConfigChirho {
    /// Delimiter between reference and text (default: whitespace after verse number).
    pub delimiter_chirho: Option<char>,
    /// Allow continuation lines (lines starting with whitespace are appended to previous).
    pub allow_continuation_chirho: bool,
    /// Skip empty lines.
    pub skip_empty_chirho: bool,
    /// Skip comment lines (lines starting with #).
    pub skip_comments_chirho: bool,
}

impl Default for VplParserConfigChirho {
    fn default() -> Self {
        Self {
            delimiter_chirho: None,
            allow_continuation_chirho: true,
            skip_empty_chirho: true,
            skip_comments_chirho: true,
        }
    }
}

/// VPL format parser.
#[derive(Debug, Clone, Default)]
pub struct VplParserChirho {
    config_chirho: VplParserConfigChirho,
}

impl VplParserChirho {
    /// Create a new VPL parser with default configuration.
    pub fn new_chirho() -> Self {
        Self::default()
    }

    /// Create a new VPL parser with custom configuration.
    pub fn with_config_chirho(config_chirho: VplParserConfigChirho) -> Self {
        Self { config_chirho }
    }

    /// Parse VPL content into entries.
    pub fn parse_chirho(&self, content_chirho: &str) -> ResultChirho<Vec<VplEntryChirho>> {
        let mut entries_chirho: Vec<VplEntryChirho> = Vec::new();

        for line_chirho in content_chirho.lines() {
            let line_chirho = line_chirho.trim_end();

            // Skip empty lines
            if self.config_chirho.skip_empty_chirho && line_chirho.trim().is_empty() {
                continue;
            }

            // Skip comment lines
            if self.config_chirho.skip_comments_chirho && line_chirho.trim_start().starts_with('#') {
                continue;
            }

            // Check for continuation line
            if self.config_chirho.allow_continuation_chirho
                && !line_chirho.is_empty()
                && (line_chirho.starts_with(' ') || line_chirho.starts_with('\t'))
                && !entries_chirho.is_empty()
            {
                // Append to previous entry
                let last_chirho = entries_chirho.last_mut().unwrap();
                last_chirho.text_chirho.push(' ');
                last_chirho.text_chirho.push_str(line_chirho.trim());
                continue;
            }

            // Parse as new entry
            if let Some(entry_chirho) = self.parse_line_chirho(line_chirho)? {
                entries_chirho.push(entry_chirho);
            }
        }

        Ok(entries_chirho)
    }

    /// Parse a single VPL line.
    fn parse_line_chirho(&self, line_chirho: &str) -> ResultChirho<Option<VplEntryChirho>> {
        let line_chirho = line_chirho.trim();
        if line_chirho.is_empty() {
            return Ok(None);
        }

        // Parse formats:
        // 1. "Gen 1:1 In the beginning..."
        // 2. "1John 1:1 That which was..."
        // 3. "Genesis 1:1 In the beginning..."

        // Find the chapter:verse pattern
        let parts_chirho: Vec<&str> = line_chirho.splitn(2, ':').collect();
        if parts_chirho.len() != 2 {
            return Err(ErrorChirho::ParseChirho {
                message_chirho: format!("Invalid VPL line (no chapter:verse): {}", line_chirho),
            });
        }

        // Parse the book and chapter from the first part
        let (book_chirho, chapter_chirho) = self.parse_book_chapter_chirho(parts_chirho[0])?;

        // Parse the verse and text from the second part
        let (verse_chirho, text_chirho) = self.parse_verse_text_chirho(parts_chirho[1])?;

        Ok(Some(VplEntryChirho::new_chirho(&book_chirho, chapter_chirho, verse_chirho, &text_chirho)))
    }

    /// Parse book name and chapter number from text like "Gen 1" or "1John 3".
    fn parse_book_chapter_chirho(&self, text_chirho: &str) -> ResultChirho<(String, u16)> {
        let text_chirho = text_chirho.trim();

        // Find the last whitespace to split book from chapter
        if let Some(last_space_chirho) = text_chirho.rfind(' ') {
            let book_chirho = text_chirho[..last_space_chirho].trim();
            let chapter_str_chirho = text_chirho[last_space_chirho..].trim();

            if let Ok(chapter_chirho) = chapter_str_chirho.parse::<u16>() {
                return Ok((book_chirho.to_string(), chapter_chirho));
            }
        }

        // Try parsing without space (e.g., "Gen1")
        // Find where digits start
        if let Some(digit_start_chirho) = text_chirho.find(|c: char| c.is_ascii_digit()) {
            // Make sure we're not at a numbered book like "1John"
            if digit_start_chirho > 0 {
                let book_chirho = text_chirho[..digit_start_chirho].trim();
                let chapter_str_chirho = text_chirho[digit_start_chirho..].trim();

                if let Ok(chapter_chirho) = chapter_str_chirho.parse::<u16>() {
                    return Ok((book_chirho.to_string(), chapter_chirho));
                }
            }
        }

        Err(ErrorChirho::ParseChirho {
            message_chirho: format!("Could not parse book and chapter from: {}", text_chirho),
        })
    }

    /// Parse verse number and text from text like "1 In the beginning...".
    fn parse_verse_text_chirho(&self, text_chirho: &str) -> ResultChirho<(u16, String)> {
        let text_chirho = text_chirho.trim();

        // Find where the verse number ends (first non-digit)
        let verse_end_chirho = text_chirho
            .find(|c: char| !c.is_ascii_digit())
            .unwrap_or(text_chirho.len());

        if verse_end_chirho == 0 {
            return Err(ErrorChirho::ParseChirho {
                message_chirho: format!("Could not parse verse number from: {}", text_chirho),
            });
        }

        let verse_str_chirho = &text_chirho[..verse_end_chirho];
        let verse_chirho = verse_str_chirho.parse::<u16>().map_err(|_| ErrorChirho::ParseChirho {
            message_chirho: format!("Invalid verse number: {}", verse_str_chirho),
        })?;

        let text_chirho = text_chirho[verse_end_chirho..].trim();

        Ok((verse_chirho, text_chirho.to_string()))
    }

    /// Parse VPL content and convert to IMP entries.
    pub fn parse_to_imp_chirho(&self, content_chirho: &str) -> ResultChirho<Vec<ImpEntryChirho>> {
        let entries_chirho = self.parse_chirho(content_chirho)?;
        Ok(entries_chirho.into_iter().map(|e| e.to_imp_entry_chirho()).collect())
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_parse_simple_vpl_chirho() {
        let parser_chirho = VplParserChirho::new_chirho();
        let content_chirho = "Gen 1:1 In the beginning God created the heaven and the earth.";

        let entries_chirho = parser_chirho.parse_chirho(content_chirho).unwrap();

        assert_eq!(entries_chirho.len(), 1);
        assert_eq!(entries_chirho[0].book_chirho, "Gen");
        assert_eq!(entries_chirho[0].chapter_chirho, 1);
        assert_eq!(entries_chirho[0].verse_chirho, 1);
        assert!(entries_chirho[0].text_chirho.contains("In the beginning"));
    }

    #[test]
    fn test_parse_multiple_verses_chirho() {
        let parser_chirho = VplParserChirho::new_chirho();
        let content_chirho = r#"Gen 1:1 In the beginning God created the heaven and the earth.
Gen 1:2 And the earth was without form, and void.
Gen 1:3 And God said, Let there be light: and there was light."#;

        let entries_chirho = parser_chirho.parse_chirho(content_chirho).unwrap();

        assert_eq!(entries_chirho.len(), 3);
        assert_eq!(entries_chirho[0].verse_chirho, 1);
        assert_eq!(entries_chirho[1].verse_chirho, 2);
        assert_eq!(entries_chirho[2].verse_chirho, 3);
    }

    #[test]
    fn test_parse_numbered_book_chirho() {
        let parser_chirho = VplParserChirho::new_chirho();
        let content_chirho = "1John 1:1 That which was from the beginning.";

        let entries_chirho = parser_chirho.parse_chirho(content_chirho).unwrap();

        assert_eq!(entries_chirho.len(), 1);
        assert_eq!(entries_chirho[0].book_chirho, "1John");
        assert_eq!(entries_chirho[0].chapter_chirho, 1);
        assert_eq!(entries_chirho[0].verse_chirho, 1);
    }

    #[test]
    fn test_parse_with_space_in_book_chirho() {
        let parser_chirho = VplParserChirho::new_chirho();
        let content_chirho = "1 John 1:1 That which was from the beginning.";

        let entries_chirho = parser_chirho.parse_chirho(content_chirho).unwrap();

        assert_eq!(entries_chirho.len(), 1);
        assert_eq!(entries_chirho[0].book_chirho, "1 John");
    }

    #[test]
    fn test_parse_continuation_lines_chirho() {
        let parser_chirho = VplParserChirho::new_chirho();
        let content_chirho = r#"Gen 1:1 In the beginning God created
  the heaven and the earth."#;

        let entries_chirho = parser_chirho.parse_chirho(content_chirho).unwrap();

        assert_eq!(entries_chirho.len(), 1);
        assert!(entries_chirho[0].text_chirho.contains("the heaven and the earth"));
    }

    #[test]
    fn test_skip_comments_chirho() {
        let parser_chirho = VplParserChirho::new_chirho();
        let content_chirho = r#"# This is a comment
Gen 1:1 In the beginning
# Another comment
Gen 1:2 And the earth was"#;

        let entries_chirho = parser_chirho.parse_chirho(content_chirho).unwrap();

        assert_eq!(entries_chirho.len(), 2);
    }

    #[test]
    fn test_skip_empty_lines_chirho() {
        let parser_chirho = VplParserChirho::new_chirho();
        let content_chirho = r#"Gen 1:1 In the beginning

Gen 1:2 And the earth was"#;

        let entries_chirho = parser_chirho.parse_chirho(content_chirho).unwrap();

        assert_eq!(entries_chirho.len(), 2);
    }

    #[test]
    fn test_to_imp_entry_chirho() {
        let entry_chirho = VplEntryChirho::new_chirho("Gen", 1, 1, "In the beginning");

        let imp_entry_chirho = entry_chirho.to_imp_entry_chirho();

        assert_eq!(imp_entry_chirho.key_chirho, "Gen 1:1");
        assert_eq!(imp_entry_chirho.content_chirho, "In the beginning");
    }

    #[test]
    fn test_reference_chirho() {
        let entry_chirho = VplEntryChirho::new_chirho("John", 3, 16, "For God so loved");

        assert_eq!(entry_chirho.reference_chirho(), "John 3:16");
    }

    #[test]
    fn test_parse_to_imp_chirho() {
        let parser_chirho = VplParserChirho::new_chirho();
        let content_chirho = "Gen 1:1 In the beginning\nGen 1:2 And the earth";

        let imp_entries_chirho = parser_chirho.parse_to_imp_chirho(content_chirho).unwrap();

        assert_eq!(imp_entries_chirho.len(), 2);
        assert_eq!(imp_entries_chirho[0].key_chirho, "Gen 1:1");
    }
}
