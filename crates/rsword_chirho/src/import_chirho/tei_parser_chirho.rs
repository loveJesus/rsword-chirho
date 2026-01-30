// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! TEI XML parser for importing dictionary/lexicon modules.
//!
//! TEI (Text Encoding Initiative) is an XML schema commonly used for
//! dictionaries and lexicons. This parser extracts entry content from
//! TEI documents for creating SWORD lexicon modules.
//!
//! Reference: <https://tei-c.org/>

use std::io::BufRead;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::error_chirho::{ErrorChirho, ResultChirho};
use crate::import_chirho::ImpEntryChirho;

/// A TEI document with extracted content.
#[derive(Debug, Clone)]
pub struct TeiDocumentChirho {
    /// Dictionary/lexicon title.
    pub title_chirho: Option<String>,
    /// Source language (e.g., "he" for Hebrew).
    pub source_language_chirho: Option<String>,
    /// Target language (e.g., "en" for English).
    pub target_language_chirho: Option<String>,
    /// Extracted entries.
    pub entries_chirho: Vec<TeiEntryChirho>,
}

impl TeiDocumentChirho {
    /// Create a new empty TEI document.
    pub fn new_chirho() -> Self {
        Self {
            title_chirho: None,
            source_language_chirho: None,
            target_language_chirho: None,
            entries_chirho: Vec::new(),
        }
    }

    /// Convert to IMP entries for module creation.
    pub fn to_imp_entries_chirho(&self) -> Vec<ImpEntryChirho> {
        self.entries_chirho
            .iter()
            .map(|e| ImpEntryChirho {
                key_chirho: e.headword_chirho.clone(),
                content_chirho: e.to_content_chirho(),
            })
            .collect()
    }
}

/// A single entry extracted from TEI.
#[derive(Debug, Clone)]
pub struct TeiEntryChirho {
    /// Entry ID/key.
    pub id_chirho: Option<String>,
    /// Headword (orth element).
    pub headword_chirho: String,
    /// Pronunciation (pron element).
    pub pronunciation_chirho: Option<String>,
    /// Part of speech (pos element).
    pub part_of_speech_chirho: Option<String>,
    /// Grammatical information (gram element).
    pub grammar_chirho: Option<String>,
    /// Definitions/senses.
    pub senses_chirho: Vec<TeiSenseChirho>,
    /// Etymology information.
    pub etymology_chirho: Option<String>,
    /// Cross-references to other entries.
    pub cross_refs_chirho: Vec<String>,
    /// Raw content if preserve_markup is enabled.
    pub raw_content_chirho: Option<String>,
}

impl TeiEntryChirho {
    /// Create a new TEI entry with a headword.
    pub fn new_chirho(headword_chirho: &str) -> Self {
        Self {
            id_chirho: None,
            headword_chirho: headword_chirho.to_string(),
            pronunciation_chirho: None,
            part_of_speech_chirho: None,
            grammar_chirho: None,
            senses_chirho: Vec::new(),
            etymology_chirho: None,
            cross_refs_chirho: Vec::new(),
            raw_content_chirho: None,
        }
    }

    /// Convert to content string for IMP format.
    pub fn to_content_chirho(&self) -> String {
        // If raw content is available, use it
        if let Some(ref raw_chirho) = self.raw_content_chirho {
            return raw_chirho.clone();
        }

        let mut content_chirho = String::new();

        // Add pronunciation if available
        if let Some(ref pron_chirho) = self.pronunciation_chirho {
            content_chirho.push_str(&format!("<pron>{}</pron> ", pron_chirho));
        }

        // Add part of speech
        if let Some(ref pos_chirho) = self.part_of_speech_chirho {
            content_chirho.push_str(&format!("<pos>{}</pos> ", pos_chirho));
        }

        // Add grammatical info
        if let Some(ref gram_chirho) = self.grammar_chirho {
            content_chirho.push_str(&format!("<gram>{}</gram> ", gram_chirho));
        }

        // Add etymology
        if let Some(ref etym_chirho) = self.etymology_chirho {
            content_chirho.push_str(&format!("<etym>{}</etym> ", etym_chirho));
        }

        // Add senses/definitions
        for (i_chirho, sense_chirho) in self.senses_chirho.iter().enumerate() {
            if self.senses_chirho.len() > 1 {
                content_chirho.push_str(&format!("{}. ", i_chirho + 1));
            }
            content_chirho.push_str(&sense_chirho.definition_chirho);

            // Add usage examples
            for example_chirho in &sense_chirho.examples_chirho {
                content_chirho.push_str(&format!(" <i>{}</i>", example_chirho));
            }

            content_chirho.push(' ');
        }

        // Add cross-references
        if !self.cross_refs_chirho.is_empty() {
            content_chirho.push_str("See also: ");
            content_chirho.push_str(&self.cross_refs_chirho.join(", "));
        }

        content_chirho.trim().to_string()
    }
}

/// A sense/definition within an entry.
#[derive(Debug, Clone)]
pub struct TeiSenseChirho {
    /// Sense number/ID.
    pub number_chirho: Option<String>,
    /// Definition text.
    pub definition_chirho: String,
    /// Usage examples.
    pub examples_chirho: Vec<String>,
    /// Grammatical restrictions.
    pub usage_chirho: Option<String>,
}

impl TeiSenseChirho {
    /// Create a new sense with a definition.
    pub fn new_chirho(definition_chirho: &str) -> Self {
        Self {
            number_chirho: None,
            definition_chirho: definition_chirho.to_string(),
            examples_chirho: Vec::new(),
            usage_chirho: None,
        }
    }
}

/// Parser state for tracking context.
#[derive(Debug, Default)]
struct ParserStateChirho {
    /// Current entry being parsed.
    current_entry_chirho: Option<TeiEntryChirho>,
    /// Current sense being parsed.
    current_sense_chirho: Option<TeiSenseChirho>,
    /// Current text accumulator.
    current_text_chirho: String,
    /// Current element we're inside.
    current_element_chirho: Option<String>,
    /// Stack of nested elements.
    element_stack_chirho: Vec<String>,
    /// Whether to preserve raw markup.
    preserve_markup_chirho: bool,
    /// Raw markup accumulator.
    raw_markup_chirho: String,
    /// Inside entry flag.
    in_entry_chirho: bool,
    /// Inside title flag for header.
    in_title_chirho: bool,
    /// Inside cit (example container) flag to avoid double counting.
    in_cit_chirho: bool,
}

/// TEI XML parser configuration.
#[derive(Debug, Clone)]
pub struct TeiParserConfigChirho {
    /// Whether to preserve inline markup in entry content.
    pub preserve_markup_chirho: bool,
    /// Whether to include cross-references.
    pub include_xrefs_chirho: bool,
    /// Whether to include examples.
    pub include_examples_chirho: bool,
}

impl Default for TeiParserConfigChirho {
    fn default() -> Self {
        Self {
            preserve_markup_chirho: true,
            include_xrefs_chirho: true,
            include_examples_chirho: true,
        }
    }
}

/// TEI XML parser.
#[derive(Debug)]
pub struct TeiParserChirho {
    config_chirho: TeiParserConfigChirho,
}

impl Default for TeiParserChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

impl TeiParserChirho {
    /// Create a new TEI parser with default configuration.
    pub fn new_chirho() -> Self {
        Self {
            config_chirho: TeiParserConfigChirho::default(),
        }
    }

    /// Create a new TEI parser with custom configuration.
    pub fn with_config_chirho(config_chirho: TeiParserConfigChirho) -> Self {
        Self { config_chirho }
    }

    /// Parse TEI XML content from a string.
    pub fn parse_str_chirho(&self, content_chirho: &str) -> ResultChirho<TeiDocumentChirho> {
        let reader_chirho = Reader::from_str(content_chirho);
        self.parse_reader_chirho(reader_chirho)
    }

    /// Parse TEI XML from a BufRead source.
    pub fn parse_reader_chirho<R: BufRead>(&self, mut reader_chirho: Reader<R>) -> ResultChirho<TeiDocumentChirho> {
        let mut document_chirho = TeiDocumentChirho::new_chirho();
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
                    self.handle_empty_chirho(&mut state_chirho, e_chirho)?;
                }
                Ok(Event::End(ref e_chirho)) => {
                    self.handle_end_chirho(&mut document_chirho, &mut state_chirho, e_chirho)?;
                }
                Ok(Event::Text(ref e_chirho)) => {
                    let text_chirho = e_chirho.unescape()
                        .map_err(|e| ErrorChirho::xml_parse_chirho(e.to_string()))?;

                    // Capture text for header title
                    if state_chirho.in_title_chirho && !state_chirho.in_entry_chirho {
                        state_chirho.current_text_chirho.push_str(&text_chirho);
                    }

                    if state_chirho.in_entry_chirho {
                        state_chirho.current_text_chirho.push_str(&text_chirho);
                        if state_chirho.preserve_markup_chirho {
                            // Escape for XML preservation
                            let escaped_chirho = text_chirho
                                .replace('&', "&amp;")
                                .replace('<', "&lt;")
                                .replace('>', "&gt;");
                            state_chirho.raw_markup_chirho.push_str(&escaped_chirho);
                        }
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

        // Handle any remaining entry
        self.finish_entry_chirho(&mut document_chirho, &mut state_chirho);

        Ok(document_chirho)
    }

    /// Handle a start element.
    fn handle_start_chirho(
        &self,
        document_chirho: &mut TeiDocumentChirho,
        state_chirho: &mut ParserStateChirho,
        event_chirho: &BytesStart,
    ) -> ResultChirho<()> {
        let name_binding_chirho = event_chirho.name();
        let name_chirho = std::str::from_utf8(name_binding_chirho.as_ref())
            .map_err(|e| ErrorChirho::xml_parse_chirho(e.to_string()))?;

        state_chirho.element_stack_chirho.push(name_chirho.to_string());
        state_chirho.current_element_chirho = Some(name_chirho.to_string());

        match name_chirho {
            "title" if !state_chirho.in_entry_chirho => {
                // Document title in header
                state_chirho.current_text_chirho.clear();
                state_chirho.in_title_chirho = true;
            }
            "entry" => {
                // Start new dictionary entry
                self.finish_entry_chirho(document_chirho, state_chirho);

                let mut entry_chirho = TeiEntryChirho::new_chirho("");

                // Get entry ID if present
                if let Some(id_chirho) = self.get_attribute_chirho(event_chirho, "xml:id")? {
                    entry_chirho.id_chirho = Some(id_chirho);
                } else if let Some(id_chirho) = self.get_attribute_chirho(event_chirho, "n")? {
                    entry_chirho.id_chirho = Some(id_chirho);
                }

                state_chirho.current_entry_chirho = Some(entry_chirho);
                state_chirho.in_entry_chirho = true;
                state_chirho.current_text_chirho.clear();
                state_chirho.raw_markup_chirho.clear();
            }
            "orth" => {
                // Headword/orthographic form
                state_chirho.current_text_chirho.clear();
                if state_chirho.preserve_markup_chirho {
                    state_chirho.raw_markup_chirho.push_str("<orth>");
                }
            }
            "pron" => {
                // Pronunciation
                state_chirho.current_text_chirho.clear();
                if state_chirho.preserve_markup_chirho {
                    state_chirho.raw_markup_chirho.push_str("<pron>");
                }
            }
            "pos" => {
                // Part of speech
                state_chirho.current_text_chirho.clear();
                if state_chirho.preserve_markup_chirho {
                    state_chirho.raw_markup_chirho.push_str("<pos>");
                }
            }
            "gram" => {
                // Grammatical information
                state_chirho.current_text_chirho.clear();
                if state_chirho.preserve_markup_chirho {
                    state_chirho.raw_markup_chirho.push_str("<gram>");
                }
            }
            "sense" => {
                // Start new sense/definition
                self.finish_sense_chirho(state_chirho);

                let mut sense_chirho = TeiSenseChirho::new_chirho("");
                if let Some(n_chirho) = self.get_attribute_chirho(event_chirho, "n")? {
                    sense_chirho.number_chirho = Some(n_chirho);
                }

                state_chirho.current_sense_chirho = Some(sense_chirho);
                state_chirho.current_text_chirho.clear();
                if state_chirho.preserve_markup_chirho {
                    state_chirho.raw_markup_chirho.push_str("<sense>");
                }
            }
            "def" => {
                // Definition
                state_chirho.current_text_chirho.clear();
                if state_chirho.preserve_markup_chirho {
                    state_chirho.raw_markup_chirho.push_str("<def>");
                }
            }
            "cit" => {
                // Citation container (contains quote)
                state_chirho.in_cit_chirho = true;
                state_chirho.current_text_chirho.clear();
                if state_chirho.preserve_markup_chirho {
                    state_chirho.raw_markup_chirho.push_str("<cit>");
                }
            }
            "quote" => {
                // Quote/example text
                state_chirho.current_text_chirho.clear();
                if state_chirho.preserve_markup_chirho {
                    state_chirho.raw_markup_chirho.push_str("<quote>");
                }
            }
            "etym" => {
                // Etymology
                state_chirho.current_text_chirho.clear();
                if state_chirho.preserve_markup_chirho {
                    state_chirho.raw_markup_chirho.push_str("<etym>");
                }
            }
            "xr" | "ref" => {
                // Cross-reference
                state_chirho.current_text_chirho.clear();
                if state_chirho.preserve_markup_chirho {
                    state_chirho.raw_markup_chirho.push_str(&format!("<{}", name_chirho));
                    // Copy target attribute
                    if let Some(target_chirho) = self.get_attribute_chirho(event_chirho, "target")? {
                        state_chirho.raw_markup_chirho.push_str(&format!(" target=\"{}\"", target_chirho));
                    }
                    state_chirho.raw_markup_chirho.push('>');
                }
            }
            "form" | "gramGrp" | "usg" | "hi" | "foreign" | "mentioned" => {
                // Other common TEI elements
                if state_chirho.preserve_markup_chirho && state_chirho.in_entry_chirho {
                    state_chirho.raw_markup_chirho.push_str(&format!("<{}>", name_chirho));
                }
            }
            _ => {
                // Generic element handling
                if state_chirho.preserve_markup_chirho && state_chirho.in_entry_chirho {
                    state_chirho.raw_markup_chirho.push_str(&format!("<{}>", name_chirho));
                }
            }
        }

        Ok(())
    }

    /// Handle an empty/self-closing element.
    fn handle_empty_chirho(
        &self,
        state_chirho: &mut ParserStateChirho,
        event_chirho: &BytesStart,
    ) -> ResultChirho<()> {
        let name_binding_chirho = event_chirho.name();
        let name_chirho = std::str::from_utf8(name_binding_chirho.as_ref())
            .map_err(|e| ErrorChirho::xml_parse_chirho(e.to_string()))?;

        if state_chirho.preserve_markup_chirho && state_chirho.in_entry_chirho {
            state_chirho.raw_markup_chirho.push_str(&format!("<{}/>", name_chirho));
        }

        // Handle ref elements
        if (name_chirho == "ref" || name_chirho == "xr") && self.config_chirho.include_xrefs_chirho {
            if let Some(target_chirho) = self.get_attribute_chirho(event_chirho, "target")? {
                if let Some(ref mut entry_chirho) = state_chirho.current_entry_chirho {
                    entry_chirho.cross_refs_chirho.push(target_chirho);
                }
            }
        }

        Ok(())
    }

    /// Handle an end element.
    fn handle_end_chirho(
        &self,
        document_chirho: &mut TeiDocumentChirho,
        state_chirho: &mut ParserStateChirho,
        event_chirho: &quick_xml::events::BytesEnd,
    ) -> ResultChirho<()> {
        let name_binding_chirho = event_chirho.name();
        let name_chirho = std::str::from_utf8(name_binding_chirho.as_ref())
            .map_err(|e| ErrorChirho::xml_parse_chirho(e.to_string()))?;

        state_chirho.element_stack_chirho.pop();

        match name_chirho {
            "title" if !state_chirho.in_entry_chirho => {
                document_chirho.title_chirho = Some(state_chirho.current_text_chirho.trim().to_string());
                state_chirho.in_title_chirho = false;
            }
            "entry" => {
                self.finish_entry_chirho(document_chirho, state_chirho);
                state_chirho.in_entry_chirho = false;
            }
            "orth" => {
                if let Some(ref mut entry_chirho) = state_chirho.current_entry_chirho {
                    if entry_chirho.headword_chirho.is_empty() {
                        entry_chirho.headword_chirho = state_chirho.current_text_chirho.trim().to_string();
                    }
                }
                if state_chirho.preserve_markup_chirho {
                    state_chirho.raw_markup_chirho.push_str("</orth>");
                }
            }
            "pron" => {
                if let Some(ref mut entry_chirho) = state_chirho.current_entry_chirho {
                    entry_chirho.pronunciation_chirho = Some(state_chirho.current_text_chirho.trim().to_string());
                }
                if state_chirho.preserve_markup_chirho {
                    state_chirho.raw_markup_chirho.push_str("</pron>");
                }
            }
            "pos" => {
                if let Some(ref mut entry_chirho) = state_chirho.current_entry_chirho {
                    entry_chirho.part_of_speech_chirho = Some(state_chirho.current_text_chirho.trim().to_string());
                }
                if state_chirho.preserve_markup_chirho {
                    state_chirho.raw_markup_chirho.push_str("</pos>");
                }
            }
            "gram" => {
                if let Some(ref mut entry_chirho) = state_chirho.current_entry_chirho {
                    entry_chirho.grammar_chirho = Some(state_chirho.current_text_chirho.trim().to_string());
                }
                if state_chirho.preserve_markup_chirho {
                    state_chirho.raw_markup_chirho.push_str("</gram>");
                }
            }
            "sense" => {
                self.finish_sense_chirho(state_chirho);
                if state_chirho.preserve_markup_chirho {
                    state_chirho.raw_markup_chirho.push_str("</sense>");
                }
            }
            "def" => {
                if let Some(ref mut sense_chirho) = state_chirho.current_sense_chirho {
                    sense_chirho.definition_chirho = state_chirho.current_text_chirho.trim().to_string();
                }
                if state_chirho.preserve_markup_chirho {
                    state_chirho.raw_markup_chirho.push_str("</def>");
                }
            }
            "cit" => {
                // End of cit container - don't add example here, it's added in quote
                state_chirho.in_cit_chirho = false;
                if state_chirho.preserve_markup_chirho {
                    state_chirho.raw_markup_chirho.push_str("</cit>");
                }
            }
            "quote" => {
                // End of quote - add example here
                if self.config_chirho.include_examples_chirho {
                    let example_chirho = state_chirho.current_text_chirho.trim().to_string();
                    if !example_chirho.is_empty() {
                        if let Some(ref mut sense_chirho) = state_chirho.current_sense_chirho {
                            sense_chirho.examples_chirho.push(example_chirho);
                        }
                    }
                }
                if state_chirho.preserve_markup_chirho {
                    state_chirho.raw_markup_chirho.push_str("</quote>");
                }
            }
            "etym" => {
                if let Some(ref mut entry_chirho) = state_chirho.current_entry_chirho {
                    entry_chirho.etymology_chirho = Some(state_chirho.current_text_chirho.trim().to_string());
                }
                if state_chirho.preserve_markup_chirho {
                    state_chirho.raw_markup_chirho.push_str("</etym>");
                }
            }
            "xr" | "ref" => {
                if self.config_chirho.include_xrefs_chirho {
                    let xref_chirho = state_chirho.current_text_chirho.trim().to_string();
                    if !xref_chirho.is_empty() {
                        if let Some(ref mut entry_chirho) = state_chirho.current_entry_chirho {
                            entry_chirho.cross_refs_chirho.push(xref_chirho);
                        }
                    }
                }
                if state_chirho.preserve_markup_chirho {
                    state_chirho.raw_markup_chirho.push_str(&format!("</{}>", name_chirho));
                }
            }
            _ => {
                if state_chirho.preserve_markup_chirho && state_chirho.in_entry_chirho {
                    state_chirho.raw_markup_chirho.push_str(&format!("</{}>", name_chirho));
                }
            }
        }

        state_chirho.current_element_chirho = state_chirho.element_stack_chirho.last().cloned();

        Ok(())
    }

    /// Finish the current sense and add it to the entry.
    fn finish_sense_chirho(&self, state_chirho: &mut ParserStateChirho) {
        if let Some(sense_chirho) = state_chirho.current_sense_chirho.take() {
            if !sense_chirho.definition_chirho.is_empty() || !sense_chirho.examples_chirho.is_empty() {
                if let Some(ref mut entry_chirho) = state_chirho.current_entry_chirho {
                    entry_chirho.senses_chirho.push(sense_chirho);
                }
            }
        }
    }

    /// Finish the current entry and add it to the document.
    fn finish_entry_chirho(
        &self,
        document_chirho: &mut TeiDocumentChirho,
        state_chirho: &mut ParserStateChirho,
    ) {
        // Finish any remaining sense
        self.finish_sense_chirho(state_chirho);

        if let Some(mut entry_chirho) = state_chirho.current_entry_chirho.take() {
            // Don't add entries without headwords
            if !entry_chirho.headword_chirho.is_empty() {
                // Store raw markup if configured
                if state_chirho.preserve_markup_chirho && !state_chirho.raw_markup_chirho.is_empty() {
                    entry_chirho.raw_content_chirho = Some(state_chirho.raw_markup_chirho.trim().to_string());
                }
                document_chirho.entries_chirho.push(entry_chirho);
            }
        }

        state_chirho.current_text_chirho.clear();
        state_chirho.raw_markup_chirho.clear();
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

    /// Parse TEI content and convert directly to IMP entries.
    pub fn parse_to_imp_chirho(&self, content_chirho: &str) -> ResultChirho<Vec<ImpEntryChirho>> {
        let document_chirho = self.parse_str_chirho(content_chirho)?;
        Ok(document_chirho.to_imp_entries_chirho())
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    const SAMPLE_TEI_CHIRHO: &str = r##"<?xml version="1.0" encoding="UTF-8"?>
<TEI xmlns="http://www.tei-c.org/ns/1.0">
  <teiHeader>
    <fileDesc>
      <titleStmt>
        <title>Sample Dictionary</title>
      </titleStmt>
    </fileDesc>
  </teiHeader>
  <text>
    <body>
      <entry xml:id="love">
        <form><orth>love</orth></form>
        <gramGrp><pos>noun</pos></gramGrp>
        <sense n="1">
          <def>A deep affection for another person.</def>
          <cit><quote>God is love.</quote></cit>
        </sense>
        <sense n="2">
          <def>An intense feeling of romantic attachment.</def>
        </sense>
      </entry>
      <entry xml:id="faith">
        <form><orth>faith</orth></form>
        <gramGrp><pos>noun</pos></gramGrp>
        <sense>
          <def>Complete trust or confidence in something.</def>
        </sense>
        <xr>See also: <ref target="#belief">belief</ref></xr>
      </entry>
    </body>
  </text>
</TEI>"##;

    #[test]
    fn test_parse_simple_tei_chirho() {
        let parser_chirho = TeiParserChirho::new_chirho();
        let doc_chirho = parser_chirho.parse_str_chirho(SAMPLE_TEI_CHIRHO).unwrap();

        assert_eq!(doc_chirho.title_chirho, Some("Sample Dictionary".to_string()));
        assert_eq!(doc_chirho.entries_chirho.len(), 2);
    }

    #[test]
    fn test_entry_structure_chirho() {
        let parser_chirho = TeiParserChirho::new_chirho();
        let doc_chirho = parser_chirho.parse_str_chirho(SAMPLE_TEI_CHIRHO).unwrap();

        let love_entry_chirho = &doc_chirho.entries_chirho[0];
        assert_eq!(love_entry_chirho.headword_chirho, "love");
        assert_eq!(love_entry_chirho.part_of_speech_chirho, Some("noun".to_string()));
        assert_eq!(love_entry_chirho.senses_chirho.len(), 2);
    }

    #[test]
    fn test_sense_with_example_chirho() {
        let parser_chirho = TeiParserChirho::new_chirho();
        let doc_chirho = parser_chirho.parse_str_chirho(SAMPLE_TEI_CHIRHO).unwrap();

        let love_entry_chirho = &doc_chirho.entries_chirho[0];
        let first_sense_chirho = &love_entry_chirho.senses_chirho[0];

        assert!(first_sense_chirho.definition_chirho.contains("deep affection"));
        assert_eq!(first_sense_chirho.examples_chirho.len(), 1);
        assert!(first_sense_chirho.examples_chirho[0].contains("God is love"));
    }

    #[test]
    fn test_cross_references_chirho() {
        let parser_chirho = TeiParserChirho::new_chirho();
        let doc_chirho = parser_chirho.parse_str_chirho(SAMPLE_TEI_CHIRHO).unwrap();

        let faith_entry_chirho = &doc_chirho.entries_chirho[1];
        assert!(!faith_entry_chirho.cross_refs_chirho.is_empty());
        assert!(faith_entry_chirho.cross_refs_chirho.iter().any(|x| x.contains("belief")));
    }

    #[test]
    fn test_tei_to_imp_chirho() {
        let parser_chirho = TeiParserChirho::new_chirho();
        let entries_chirho = parser_chirho.parse_to_imp_chirho(SAMPLE_TEI_CHIRHO).unwrap();

        assert_eq!(entries_chirho.len(), 2);
        assert_eq!(entries_chirho[0].key_chirho, "love");
        assert_eq!(entries_chirho[1].key_chirho, "faith");
    }

    #[test]
    fn test_entry_to_content_chirho() {
        let mut entry_chirho = TeiEntryChirho::new_chirho("test");
        entry_chirho.part_of_speech_chirho = Some("verb".to_string());

        let mut sense_chirho = TeiSenseChirho::new_chirho("to examine");
        sense_chirho.examples_chirho.push("Test the software.".to_string());
        entry_chirho.senses_chirho.push(sense_chirho);

        let content_chirho = entry_chirho.to_content_chirho();
        assert!(content_chirho.contains("<pos>verb</pos>"));
        assert!(content_chirho.contains("to examine"));
        assert!(content_chirho.contains("<i>Test the software.</i>"));
    }

    #[test]
    fn test_entry_id_chirho() {
        let parser_chirho = TeiParserChirho::new_chirho();
        let doc_chirho = parser_chirho.parse_str_chirho(SAMPLE_TEI_CHIRHO).unwrap();

        assert_eq!(doc_chirho.entries_chirho[0].id_chirho, Some("love".to_string()));
        assert_eq!(doc_chirho.entries_chirho[1].id_chirho, Some("faith".to_string()));
    }

    #[test]
    fn test_empty_document_chirho() {
        let parser_chirho = TeiParserChirho::new_chirho();
        let doc_chirho = parser_chirho.parse_str_chirho("<TEI><text><body></body></text></TEI>").unwrap();

        assert!(doc_chirho.entries_chirho.is_empty());
    }

    #[test]
    fn test_config_no_xrefs_chirho() {
        let config_chirho = TeiParserConfigChirho {
            include_xrefs_chirho: false,
            ..Default::default()
        };
        let parser_chirho = TeiParserChirho::with_config_chirho(config_chirho);
        let doc_chirho = parser_chirho.parse_str_chirho(SAMPLE_TEI_CHIRHO).unwrap();

        let faith_entry_chirho = &doc_chirho.entries_chirho[1];
        assert!(faith_entry_chirho.cross_refs_chirho.is_empty());
    }

    #[test]
    fn test_config_no_examples_chirho() {
        let config_chirho = TeiParserConfigChirho {
            include_examples_chirho: false,
            ..Default::default()
        };
        let parser_chirho = TeiParserChirho::with_config_chirho(config_chirho);
        let doc_chirho = parser_chirho.parse_str_chirho(SAMPLE_TEI_CHIRHO).unwrap();

        let love_entry_chirho = &doc_chirho.entries_chirho[0];
        let first_sense_chirho = &love_entry_chirho.senses_chirho[0];
        assert!(first_sense_chirho.examples_chirho.is_empty());
    }
}
