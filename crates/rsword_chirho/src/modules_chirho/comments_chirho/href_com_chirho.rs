// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! HREFCom module driver.
//!
//! HREF Commentary modules don't contain actual text - they contain
//! URL templates that point to external web resources. When accessing
//! a verse, the URL is constructed from the template and returned.

use crate::config_chirho::ModuleConfigChirho;
use crate::error_chirho::ResultChirho;
use crate::keys_chirho::sw_key_chirho::SwKeyChirho;
use crate::keys_chirho::verse_key_chirho::VerseKeyChirho;
use crate::modules_chirho::sw_module_chirho::{ModuleBaseChirho, SwModuleChirho};
use crate::{DirectionChirho, EncodingChirho, MarkupChirho};

/// URL template placeholders.
const BOOK_PLACEHOLDER_CHIRHO: &str = "$BOOK";
const CHAPTER_PLACEHOLDER_CHIRHO: &str = "$CHAPTER";
const VERSE_PLACEHOLDER_CHIRHO: &str = "$VERSE";
const OSIS_PLACEHOLDER_CHIRHO: &str = "$OSISREF";
const OSISBOOK_PLACEHOLDER_CHIRHO: &str = "$OSISBOOK";

/// HREF Commentary module.
///
/// Instead of storing verse text, this module type stores URL templates
/// that point to external web resources for commentary content.
pub struct HrefComChirho {
    /// Base module functionality.
    base_chirho: ModuleBaseChirho,
    /// URL template with placeholders.
    url_template_chirho: String,
    /// Current verse key.
    key_chirho: VerseKeyChirho,
}

impl HrefComChirho {
    /// Create a new HREF Commentary module.
    pub fn new_chirho(config_chirho: ModuleConfigChirho) -> ResultChirho<Self> {
        // Get URL template from config
        let url_template_chirho = config_chirho
            .get_chirho("DataPath")
            .or_else(|| config_chirho.get_chirho("HrefSource"))
            .unwrap_or_default()
            .to_string();

        let key_chirho = VerseKeyChirho::new_chirho();

        Ok(Self {
            base_chirho: ModuleBaseChirho::new_chirho(config_chirho),
            url_template_chirho,
            key_chirho,
        })
    }

    /// Get the URL template.
    pub fn url_template_chirho(&self) -> &str {
        &self.url_template_chirho
    }

    /// Set a new URL template.
    pub fn set_url_template_chirho(&mut self, template_chirho: &str) {
        self.url_template_chirho = template_chirho.to_string();
    }

    /// Generate a URL for a specific verse.
    pub fn get_url_chirho(&self, key_chirho: &VerseKeyChirho) -> String {
        self.expand_template_chirho(&self.url_template_chirho, key_chirho)
    }

    /// Expand URL template with verse reference values.
    fn expand_template_chirho(&self, template_chirho: &str, key_chirho: &VerseKeyChirho) -> String {
        let book_name_chirho = key_chirho.get_book_name_chirho();
        let osis_book_chirho = key_chirho.get_book_osis_chirho();

        template_chirho
            .replace(BOOK_PLACEHOLDER_CHIRHO, book_name_chirho)
            .replace(OSISBOOK_PLACEHOLDER_CHIRHO, osis_book_chirho)
            .replace(CHAPTER_PLACEHOLDER_CHIRHO, &key_chirho.get_chapter_chirho().to_string())
            .replace(VERSE_PLACEHOLDER_CHIRHO, &key_chirho.get_verse_chirho().to_string())
            .replace(OSIS_PLACEHOLDER_CHIRHO, &key_chirho.get_osis_ref_chirho())
    }

    /// Get HTML link for a verse.
    pub fn get_link_chirho(&self, key_chirho: &VerseKeyChirho) -> String {
        let url_chirho = self.get_url_chirho(key_chirho);
        format!(
            "<a href=\"{}\" target=\"_blank\">{}</a>",
            url_chirho,
            key_chirho.get_text_chirho()
        )
    }
}

impl SwModuleChirho for HrefComChirho {
    fn name_chirho(&self) -> &str {
        self.base_chirho.name_chirho()
    }

    fn description_chirho(&self) -> &str {
        self.base_chirho.description_chirho()
    }

    fn module_type_chirho(&self) -> &str {
        "Commentaries"
    }

    fn language_chirho(&self) -> &str {
        self.base_chirho.language_chirho()
    }

    fn direction_chirho(&self) -> DirectionChirho {
        self.base_chirho.direction_chirho()
    }

    fn encoding_chirho(&self) -> EncodingChirho {
        self.base_chirho.encoding_chirho()
    }

    fn markup_chirho(&self) -> MarkupChirho {
        MarkupChirho::HtmlChirho
    }

    fn get_key_chirho(&self) -> &dyn SwKeyChirho {
        &self.key_chirho
    }

    fn set_key_chirho(&mut self, key_chirho: &dyn SwKeyChirho) -> ResultChirho<()> {
        // Parse the key text as a verse reference
        self.key_chirho.set_text_chirho(key_chirho.get_text_chirho());
        Ok(())
    }

    fn get_raw_entry_chirho(&mut self) -> ResultChirho<String> {
        // Return the URL as the "content"
        Ok(self.get_url_chirho(&self.key_chirho))
    }

    fn render_text_chirho(&mut self) -> ResultChirho<String> {
        // Return formatted link
        Ok(self.get_link_chirho(&self.key_chirho))
    }

    fn is_encrypted_chirho(&self) -> bool {
        false
    }

    fn has_entry_chirho(&self) -> bool {
        // HREF modules always "have" an entry since they generate URLs
        true
    }

    fn get_config_entry_chirho(&self, key_chirho: &str) -> Option<&str> {
        self.base_chirho.get_config_entry_chirho(key_chirho)
    }

    fn increment_chirho(&mut self, steps_chirho: i32) {
        self.key_chirho.increment_chirho(steps_chirho);
    }

    fn decrement_chirho(&mut self, steps_chirho: i32) {
        self.key_chirho.decrement_chirho(steps_chirho);
    }

    fn pop_error_chirho(&mut self) -> bool {
        self.base_chirho.pop_error_chirho()
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    fn create_test_module_chirho(template_chirho: &str) -> HrefComChirho {
        let mut config_chirho = ModuleConfigChirho::new_chirho("TestHREFCom".to_string());
        config_chirho.set_chirho("DataPath", template_chirho);
        config_chirho.set_chirho("Description", "Test HREF Commentary");
        config_chirho.set_chirho("ModDrv", "HREFCom");

        HrefComChirho::new_chirho(config_chirho).unwrap()
    }

    fn create_verse_key_chirho(reference_chirho: &str) -> VerseKeyChirho {
        VerseKeyChirho::from_str_chirho(reference_chirho).unwrap()
    }

    #[test]
    fn test_url_generation_basic_chirho() {
        let module_chirho = create_test_module_chirho(
            "https://example.com/commentary/$OSISBOOK/$CHAPTER/$VERSE"
        );

        // Genesis 1:1
        let key_chirho = create_verse_key_chirho("Genesis 1:1");
        let url_chirho = module_chirho.get_url_chirho(&key_chirho);

        assert!(url_chirho.contains("Gen"));
        assert!(url_chirho.contains("/1/1"));
    }

    #[test]
    fn test_url_generation_osis_ref_chirho() {
        let module_chirho = create_test_module_chirho(
            "https://bible.org/$OSISREF"
        );

        // John 3:16
        let key_chirho = create_verse_key_chirho("John 3:16");
        let url_chirho = module_chirho.get_url_chirho(&key_chirho);

        assert!(url_chirho.contains("John.3.16"));
    }

    #[test]
    fn test_get_link_chirho() {
        let module_chirho = create_test_module_chirho("https://example.com/$OSISREF");

        let key_chirho = create_verse_key_chirho("Genesis 1:1");
        let link_chirho = module_chirho.get_link_chirho(&key_chirho);

        assert!(link_chirho.contains("<a href="));
        assert!(link_chirho.contains("target=\"_blank\""));
    }

    #[test]
    fn test_module_type_chirho() {
        let module_chirho = create_test_module_chirho("https://example.com/");
        assert_eq!(module_chirho.module_type_chirho(), "Commentaries");
    }

    #[test]
    fn test_has_entry_chirho() {
        let module_chirho = create_test_module_chirho("https://example.com/");
        // HREF modules always have entries
        assert!(module_chirho.has_entry_chirho());
    }

    #[test]
    fn test_url_template_setter_chirho() {
        let mut module_chirho = create_test_module_chirho("https://old.url/");
        module_chirho.set_url_template_chirho("https://new.url/$OSISREF");

        let key_chirho = create_verse_key_chirho("Genesis 1:1");
        let url_chirho = module_chirho.get_url_chirho(&key_chirho);

        assert!(url_chirho.starts_with("https://new.url/"));
    }

    #[test]
    fn test_render_text_chirho() {
        let mut module_chirho = create_test_module_chirho("https://example.com/$OSISREF");
        let key_chirho = create_verse_key_chirho("Genesis 1:1");
        module_chirho.key_chirho = key_chirho;

        let rendered_chirho = module_chirho.render_text_chirho().unwrap();
        assert!(rendered_chirho.contains("https://example.com/"));
        assert!(rendered_chirho.contains("<a href="));
    }
}
