// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Locale support for internationalized book names and abbreviations.
//!
//! SWORD locales provide translations of book names, abbreviations, and other
//! strings used in Bible references. This module implements locale loading and
//! management compatible with SWORD's locale format.

mod locale_data_chirho;
mod locale_manager_chirho;

pub use locale_data_chirho::{LocaleChirho, LocaleBookChirho};
pub use locale_manager_chirho::LocaleManagerChirho;

/// Default locale code.
pub const DEFAULT_LOCALE_CHIRHO: &str = "en";

/// Get the built-in English locale.
pub fn english_locale_chirho() -> LocaleChirho {
    locale_data_chirho::create_english_locale_chirho()
}

/// Get a locale by code, falling back to English if not found.
pub fn get_locale_chirho(code_chirho: &str) -> LocaleChirho {
    LocaleManagerChirho::global_chirho().get_locale_chirho(code_chirho)
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_english_locale_chirho() {
        let locale_chirho = english_locale_chirho();
        assert_eq!(locale_chirho.code_chirho, "en");
        assert!(!locale_chirho.books_chirho.is_empty());
    }

    #[test]
    fn test_get_locale_chirho() {
        let locale_chirho = get_locale_chirho("en");
        assert_eq!(locale_chirho.code_chirho, "en");
    }

    #[test]
    fn test_fallback_locale_chirho() {
        let locale_chirho = get_locale_chirho("nonexistent");
        assert_eq!(locale_chirho.code_chirho, "en"); // Falls back to English
    }
}
