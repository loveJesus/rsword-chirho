// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! OSIS XML parser for importing Bible modules.
//!
//! OSIS (Open Scripture Information Standard) is an XML schema for encoding
//! scripture texts. This parser extracts verse content from OSIS documents
//! for creating SWORD modules.
//!
//! Reference: <https://crosswire.org/osis/>

use std::io::BufRead;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::error_chirho::{ErrorChirho, ResultChirho};
use crate::import_chirho::ImpEntryChirho;

/// An OSIS document with extracted content.
#[derive(Debug, Clone)]
pub struct OsisDocumentChirho {
    /// Work identifier from osisWork attribute.
    pub work_id_chirho: Option<String>,
    /// Work title from header.
    pub title_chirho: Option<String>,
    /// Work language.
    pub language_chirho: Option<String>,
    /// Extracted verses.
    pub verses_chirho: Vec<OsisVerseChirho>,
}

impl OsisDocumentChirho {
    /// Create a new empty OSIS document.
    pub fn new_chirho() -> Self {
        Self {
            work_id_chirho: None,
            title_chirho: None,
            language_chirho: None,
            verses_chirho: Vec::new(),
        }
    }

    /// Convert to IMP entries for module creation.
    pub fn to_imp_entries_chirho(&self) -> Vec<ImpEntryChirho> {
        self.verses_chirho
            .iter()
            .map(|v| ImpEntryChirho {
                key_chirho: v.reference_chirho(),
                content_chirho: v.text_chirho.clone(),
            })
            .collect()
    }
}

/// A single verse extracted from OSIS.
#[derive(Debug, Clone)]
pub struct OsisVerseChirho {
    /// OSIS ID (e.g., "Gen.1.1").
    pub osis_id_chirho: String,
    /// Book OSIS abbreviation.
    pub book_chirho: String,
    /// Chapter number.
    pub chapter_chirho: u16,
    /// Verse number.
    pub verse_chirho: u16,
    /// Verse text content (may include markup).
    pub text_chirho: String,
}

impl OsisVerseChirho {
    /// Create a new OSIS verse.
    pub fn new_chirho(osis_id_chirho: &str, text_chirho: &str) -> ResultChirho<Self> {
        let parts_chirho: Vec<&str> = osis_id_chirho.split('.').collect();
        if parts_chirho.len() < 3 {
            return Err(ErrorChirho::ParseChirho {
                message_chirho: format!("Invalid OSIS ID: {}", osis_id_chirho),
            });
        }

        let book_chirho = parts_chirho[0].to_string();
        let chapter_chirho = parts_chirho[1].parse::<u16>().map_err(|_| ErrorChirho::ParseChirho {
            message_chirho: format!("Invalid chapter in OSIS ID: {}", osis_id_chirho),
        })?;
        let verse_chirho = parts_chirho[2].parse::<u16>().map_err(|_| ErrorChirho::ParseChirho {
            message_chirho: format!("Invalid verse in OSIS ID: {}", osis_id_chirho),
        })?;

        Ok(Self {
            osis_id_chirho: osis_id_chirho.to_string(),
            book_chirho,
            chapter_chirho,
            verse_chirho,
            text_chirho: text_chirho.to_string(),
        })
    }

    /// Get the verse reference in standard format (e.g., "Gen 1:1").
    pub fn reference_chirho(&self) -> String {
        format!("{} {}:{}", self.book_chirho, self.chapter_chirho, self.verse_chirho)
    }
}

/// Parser state for tracking context.
#[derive(Debug, Default)]
struct ParserStateChirho {
    /// Current book OSIS ID.
    current_book_chirho: Option<String>,
    /// Current chapter number.
    current_chapter_chirho: u16,
    /// Current verse OSIS ID.
    current_verse_id_chirho: Option<String>,
    /// Accumulated text for current verse.
    current_text_chirho: String,
    /// Stack of open elements for text accumulation.
    element_depth_chirho: u32,
    /// Whether we're inside a verse.
    in_verse_chirho: bool,
    /// Whether to preserve markup.
    preserve_markup_chirho: bool,
}

/// OSIS XML parser configuration.
#[derive(Debug, Clone)]
pub struct OsisParserConfigChirho {
    /// Whether to preserve inline markup in verse text.
    pub preserve_markup_chirho: bool,
    /// Whether to include chapter/book introductions.
    pub include_intros_chirho: bool,
    /// Whether to strip Strong's numbers.
    pub strip_strongs_chirho: bool,
    /// Whether to strip morphology.
    pub strip_morph_chirho: bool,
}

impl Default for OsisParserConfigChirho {
    fn default() -> Self {
        Self {
            preserve_markup_chirho: true,
            include_intros_chirho: false,
            strip_strongs_chirho: false,
            strip_morph_chirho: false,
        }
    }
}

/// OSIS XML parser.
#[derive(Debug)]
pub struct OsisParserChirho {
    config_chirho: OsisParserConfigChirho,
}

impl Default for OsisParserChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

impl OsisParserChirho {
    /// Create a new OSIS parser with default configuration.
    pub fn new_chirho() -> Self {
        Self {
            config_chirho: OsisParserConfigChirho::default(),
        }
    }

    /// Create a new OSIS parser with custom configuration.
    pub fn with_config_chirho(config_chirho: OsisParserConfigChirho) -> Self {
        Self { config_chirho }
    }

    /// Parse OSIS XML content from a string.
    pub fn parse_str_chirho(&self, content_chirho: &str) -> ResultChirho<OsisDocumentChirho> {
        let reader_chirho = Reader::from_str(content_chirho);
        self.parse_reader_chirho(reader_chirho)
    }

    /// Parse OSIS XML from a BufRead source.
    pub fn parse_reader_chirho<R: BufRead>(&self, mut reader_chirho: Reader<R>) -> ResultChirho<OsisDocumentChirho> {
        let mut document_chirho = OsisDocumentChirho::new_chirho();
        let mut state_chirho = ParserStateChirho {
            preserve_markup_chirho: self.config_chirho.preserve_markup_chirho,
            ..Default::default()
        };
        let mut buf_chirho = Vec::new();

        loop {
            match reader_chirho.read_event_into(&mut buf_chirho) {
                Ok(Event::Start(ref e_chirho)) => {
                    self.handle_start_chirho(&mut document_chirho, &mut state_chirho, e_chirho)?;
                }
                Ok(Event::Empty(ref e_chirho)) => {
                    // Handle milestone elements like <verse sID="..." />
                    self.handle_empty_chirho(&mut document_chirho, &mut state_chirho, e_chirho)?;
                }
                Ok(Event::End(ref e_chirho)) => {
                    self.handle_end_chirho(&mut document_chirho, &mut state_chirho, e_chirho)?;
                }
                Ok(Event::Text(ref e_chirho)) => {
                    if state_chirho.in_verse_chirho {
                        let text_chirho = e_chirho.unescape()
                            .map_err(|e| ErrorChirho::xml_parse_chirho(e.to_string()))?;
                        state_chirho.current_text_chirho.push_str(&text_chirho);
                    }
                }
                Ok(Event::Eof) => break,
                Err(e_chirho) => {
                    return Err(ErrorChirho::xml_parse_chirho(format!(
                        "Error at position {}: {:?}",
                        reader_chirho.buffer_position(),
                        e_chirho
                    )));
                }
                _ => {}
            }
            buf_chirho.clear();
        }

        // Handle any remaining verse
        self.finish_verse_chirho(&mut document_chirho, &mut state_chirho);

        Ok(document_chirho)
    }

    /// Handle a start element.
    fn handle_start_chirho(
        &self,
        document_chirho: &mut OsisDocumentChirho,
        state_chirho: &mut ParserStateChirho,
        event_chirho: &BytesStart,
    ) -> ResultChirho<()> {
        let name_binding_chirho = event_chirho.name();
        let name_chirho = std::str::from_utf8(name_binding_chirho.as_ref())
            .map_err(|e| ErrorChirho::xml_parse_chirho(e.to_string()))?;

        match name_chirho {
            "osisText" => {
                // Extract work ID
                if let Some(attr_chirho) = self.get_attribute_chirho(event_chirho, "osisIDWork")? {
                    document_chirho.work_id_chirho = Some(attr_chirho);
                }
            }
            "work" => {
                // Start of header work element
            }
            "title" => {
                // Could capture title, but we'll use osisIDWork for now
            }
            "language" => {
                // Language will be captured from attribute
            }
            "div" => {
                // Check for book division
                if let Some(osis_id_chirho) = self.get_attribute_chirho(event_chirho, "osisID")? {
                    if self.get_attribute_chirho(event_chirho, "type")?.as_deref() == Some("book") {
                        state_chirho.current_book_chirho = Some(osis_id_chirho);
                    }
                }
            }
            "chapter" => {
                // Finish any pending verse before chapter change
                self.finish_verse_chirho(document_chirho, state_chirho);

                if let Some(osis_id_chirho) = self.get_attribute_chirho(event_chirho, "osisID")? {
                    // Parse chapter number from osisID like "Gen.1"
                    if let Some(ch_str_chirho) = osis_id_chirho.split('.').next_back() {
                        if let Ok(ch_chirho) = ch_str_chirho.parse::<u16>() {
                            state_chirho.current_chapter_chirho = ch_chirho;
                        }
                    }
                }
            }
            "verse" => {
                // Finish previous verse if any
                self.finish_verse_chirho(document_chirho, state_chirho);

                // Start new verse
                if let Some(osis_id_chirho) = self.get_attribute_chirho(event_chirho, "osisID")? {
                    state_chirho.current_verse_id_chirho = Some(osis_id_chirho);
                    state_chirho.in_verse_chirho = true;
                    state_chirho.current_text_chirho.clear();
                }
            }
            "w" | "transChange" | "hi" | "q" | "note" | "reference" | "divineName" | "foreign" => {
                // Inline elements - preserve markup if configured
                if state_chirho.in_verse_chirho && state_chirho.preserve_markup_chirho {
                    state_chirho.current_text_chirho.push('<');
                    state_chirho.current_text_chirho.push_str(name_chirho);

                    // Copy relevant attributes
                    for attr_chirho in event_chirho.attributes().flatten() {
                        let key_chirho = std::str::from_utf8(attr_chirho.key.as_ref())
                            .unwrap_or("");
                        let value_chirho = attr_chirho.unescape_value()
                            .unwrap_or_default();

                        // Skip Strong's numbers if configured
                        if self.config_chirho.strip_strongs_chirho && key_chirho == "lemma" {
                            continue;
                        }
                        // Skip morphology if configured
                        if self.config_chirho.strip_morph_chirho && key_chirho == "morph" {
                            continue;
                        }

                        state_chirho.current_text_chirho.push(' ');
                        state_chirho.current_text_chirho.push_str(key_chirho);
                        state_chirho.current_text_chirho.push_str("=\"");
                        state_chirho.current_text_chirho.push_str(&value_chirho);
                        state_chirho.current_text_chirho.push('"');
                    }
                    state_chirho.current_text_chirho.push('>');
                }
                state_chirho.element_depth_chirho += 1;
            }
            _ => {
                if state_chirho.in_verse_chirho {
                    state_chirho.element_depth_chirho += 1;
                }
            }
        }

        Ok(())
    }

    /// Handle an empty/self-closing element.
    fn handle_empty_chirho(
        &self,
        document_chirho: &mut OsisDocumentChirho,
        state_chirho: &mut ParserStateChirho,
        event_chirho: &BytesStart,
    ) -> ResultChirho<()> {
        let name_binding_chirho = event_chirho.name();
        let name_chirho = std::str::from_utf8(name_binding_chirho.as_ref())
            .map_err(|e| ErrorChirho::xml_parse_chirho(e.to_string()))?;

        match name_chirho {
            "verse" => {
                // Milestone verse marker
                if let Some(sid_chirho) = self.get_attribute_chirho(event_chirho, "sID")? {
                    // Start of verse milestone
                    self.finish_verse_chirho(document_chirho, state_chirho);
                    state_chirho.current_verse_id_chirho = Some(sid_chirho);
                    state_chirho.in_verse_chirho = true;
                    state_chirho.current_text_chirho.clear();
                } else if self.get_attribute_chirho(event_chirho, "eID")?.is_some() {
                    // End of verse milestone
                    self.finish_verse_chirho(document_chirho, state_chirho);
                }
            }
            "chapter" => {
                // Milestone chapter marker
                if let Some(sid_chirho) = self.get_attribute_chirho(event_chirho, "sID")? {
                    self.finish_verse_chirho(document_chirho, state_chirho);
                    if let Some(ch_str_chirho) = sid_chirho.split('.').next_back() {
                        if let Ok(ch_chirho) = ch_str_chirho.parse::<u16>() {
                            state_chirho.current_chapter_chirho = ch_chirho;
                        }
                    }
                }
            }
            "lb" => {
                // Line break
                if state_chirho.in_verse_chirho {
                    if state_chirho.preserve_markup_chirho {
                        state_chirho.current_text_chirho.push_str("<lb/>");
                    } else {
                        state_chirho.current_text_chirho.push('\n');
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// Handle an end element.
    fn handle_end_chirho(
        &self,
        document_chirho: &mut OsisDocumentChirho,
        state_chirho: &mut ParserStateChirho,
        event_chirho: &quick_xml::events::BytesEnd,
    ) -> ResultChirho<()> {
        let name_binding_chirho = event_chirho.name();
        let name_chirho = std::str::from_utf8(name_binding_chirho.as_ref())
            .map_err(|e| ErrorChirho::xml_parse_chirho(e.to_string()))?;

        match name_chirho {
            "verse" => {
                self.finish_verse_chirho(document_chirho, state_chirho);
            }
            "w" | "transChange" | "hi" | "q" | "note" | "reference" | "divineName" | "foreign" => {
                if state_chirho.in_verse_chirho && state_chirho.preserve_markup_chirho {
                    state_chirho.current_text_chirho.push_str("</");
                    state_chirho.current_text_chirho.push_str(name_chirho);
                    state_chirho.current_text_chirho.push('>');
                }
                if state_chirho.element_depth_chirho > 0 {
                    state_chirho.element_depth_chirho -= 1;
                }
            }
            _ => {
                if state_chirho.in_verse_chirho && state_chirho.element_depth_chirho > 0 {
                    state_chirho.element_depth_chirho -= 1;
                }
            }
        }

        Ok(())
    }

    /// Finish the current verse and add it to the document.
    fn finish_verse_chirho(
        &self,
        document_chirho: &mut OsisDocumentChirho,
        state_chirho: &mut ParserStateChirho,
    ) {
        if let Some(osis_id_chirho) = state_chirho.current_verse_id_chirho.take() {
            let text_chirho = state_chirho.current_text_chirho.trim().to_string();
            if !text_chirho.is_empty() || self.config_chirho.include_intros_chirho {
                if let Ok(verse_chirho) = OsisVerseChirho::new_chirho(&osis_id_chirho, &text_chirho) {
                    document_chirho.verses_chirho.push(verse_chirho);
                }
            }
        }
        state_chirho.in_verse_chirho = false;
        state_chirho.current_text_chirho.clear();
    }

    /// Get an attribute value from an element.
    fn get_attribute_chirho(
        &self,
        event_chirho: &BytesStart,
        name_chirho: &str,
    ) -> ResultChirho<Option<String>> {
        for attr_chirho in event_chirho.attributes().flatten() {
            let key_chirho = std::str::from_utf8(attr_chirho.key.as_ref())
                .map_err(|e| ErrorChirho::xml_parse_chirho(e.to_string()))?;
            if key_chirho == name_chirho {
                let value_chirho = attr_chirho.unescape_value()
                    .map_err(|e| ErrorChirho::xml_parse_chirho(e.to_string()))?;
                return Ok(Some(value_chirho.to_string()));
            }
        }
        Ok(None)
    }

    /// Parse OSIS content and convert directly to IMP entries.
    pub fn parse_to_imp_chirho(&self, content_chirho: &str) -> ResultChirho<Vec<ImpEntryChirho>> {
        let document_chirho = self.parse_str_chirho(content_chirho)?;
        Ok(document_chirho.to_imp_entries_chirho())
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    const SAMPLE_OSIS_CHIRHO: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<osis xmlns="http://www.bibletechnologies.net/2003/OSIS/namespace">
  <osisText osisIDWork="KJV" xml:lang="en">
    <div type="book" osisID="Gen">
      <chapter osisID="Gen.1">
        <verse osisID="Gen.1.1">In the beginning God created the heaven and the earth.</verse>
        <verse osisID="Gen.1.2">And the earth was without form, and void.</verse>
      </chapter>
    </div>
  </osisText>
</osis>"#;

    const MILESTONE_OSIS_CHIRHO: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<osis xmlns="http://www.bibletechnologies.net/2003/OSIS/namespace">
  <osisText osisIDWork="KJV">
    <div type="book" osisID="Gen">
      <chapter sID="Gen.1"/>
      <verse sID="Gen.1.1"/>In the beginning God created the heaven and the earth.<verse eID="Gen.1.1"/>
      <verse sID="Gen.1.2"/>And the earth was without form, and void.<verse eID="Gen.1.2"/>
      <chapter eID="Gen.1"/>
    </div>
  </osisText>
</osis>"#;

    #[test]
    fn test_parse_simple_osis_chirho() {
        let parser_chirho = OsisParserChirho::new_chirho();
        let doc_chirho = parser_chirho.parse_str_chirho(SAMPLE_OSIS_CHIRHO).unwrap();

        assert_eq!(doc_chirho.work_id_chirho, Some("KJV".to_string()));
        assert_eq!(doc_chirho.verses_chirho.len(), 2);

        assert_eq!(doc_chirho.verses_chirho[0].book_chirho, "Gen");
        assert_eq!(doc_chirho.verses_chirho[0].chapter_chirho, 1);
        assert_eq!(doc_chirho.verses_chirho[0].verse_chirho, 1);
        assert!(doc_chirho.verses_chirho[0].text_chirho.contains("In the beginning"));
    }

    #[test]
    fn test_parse_milestone_osis_chirho() {
        let parser_chirho = OsisParserChirho::new_chirho();
        let doc_chirho = parser_chirho.parse_str_chirho(MILESTONE_OSIS_CHIRHO).unwrap();

        assert_eq!(doc_chirho.verses_chirho.len(), 2);
        assert_eq!(doc_chirho.verses_chirho[0].reference_chirho(), "Gen 1:1");
        assert_eq!(doc_chirho.verses_chirho[1].reference_chirho(), "Gen 1:2");
    }

    #[test]
    fn test_osis_verse_reference_chirho() {
        let verse_chirho = OsisVerseChirho::new_chirho("John.3.16", "For God so loved the world").unwrap();
        assert_eq!(verse_chirho.reference_chirho(), "John 3:16");
    }

    #[test]
    fn test_osis_to_imp_chirho() {
        let parser_chirho = OsisParserChirho::new_chirho();
        let entries_chirho = parser_chirho.parse_to_imp_chirho(SAMPLE_OSIS_CHIRHO).unwrap();

        assert_eq!(entries_chirho.len(), 2);
        assert_eq!(entries_chirho[0].key_chirho, "Gen 1:1");
        assert!(entries_chirho[0].content_chirho.contains("In the beginning"));
    }

    #[test]
    fn test_osis_with_markup_chirho() {
        let osis_chirho = r#"<?xml version="1.0"?>
<osis xmlns="http://www.bibletechnologies.net/2003/OSIS/namespace">
  <osisText osisIDWork="Test">
    <div type="book" osisID="Gen">
      <chapter osisID="Gen.1">
        <verse osisID="Gen.1.1"><w lemma="H7225">In the beginning</w> God created.</verse>
      </chapter>
    </div>
  </osisText>
</osis>"#;

        let parser_chirho = OsisParserChirho::new_chirho();
        let doc_chirho = parser_chirho.parse_str_chirho(osis_chirho).unwrap();

        assert!(doc_chirho.verses_chirho[0].text_chirho.contains("lemma"));
    }

    #[test]
    fn test_osis_strip_strongs_chirho() {
        let osis_chirho = r#"<?xml version="1.0"?>
<osis xmlns="http://www.bibletechnologies.net/2003/OSIS/namespace">
  <osisText osisIDWork="Test">
    <div type="book" osisID="Gen">
      <chapter osisID="Gen.1">
        <verse osisID="Gen.1.1"><w lemma="H7225">In the beginning</w> God created.</verse>
      </chapter>
    </div>
  </osisText>
</osis>"#;

        let config_chirho = OsisParserConfigChirho {
            strip_strongs_chirho: true,
            ..Default::default()
        };
        let parser_chirho = OsisParserChirho::with_config_chirho(config_chirho);
        let doc_chirho = parser_chirho.parse_str_chirho(osis_chirho).unwrap();

        assert!(!doc_chirho.verses_chirho[0].text_chirho.contains("lemma"));
    }
}
