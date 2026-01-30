// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Locale manager for loading and caching locales.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::locale_data_chirho::{
    LocaleChirho, create_english_locale_chirho, create_spanish_locale_chirho, create_german_locale_chirho,
};
use super::DEFAULT_LOCALE_CHIRHO;

/// Global locale manager singleton.
static GLOBAL_MANAGER_CHIRHO: std::sync::OnceLock<LocaleManagerChirho> = std::sync::OnceLock::new();

/// Manager for loading and caching locales.
#[derive(Debug)]
pub struct LocaleManagerChirho {
    /// Cached locales by code.
    cache_chirho: Arc<RwLock<HashMap<String, LocaleChirho>>>,
    /// Default locale code.
    default_locale_chirho: String,
}

impl Default for LocaleManagerChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

impl LocaleManagerChirho {
    /// Create a new locale manager.
    pub fn new_chirho() -> Self {
        let manager_chirho = Self {
            cache_chirho: Arc::new(RwLock::new(HashMap::new())),
            default_locale_chirho: DEFAULT_LOCALE_CHIRHO.to_string(),
        };
        manager_chirho.load_builtin_locales_chirho();
        manager_chirho
    }

    /// Get the global locale manager instance.
    pub fn global_chirho() -> &'static LocaleManagerChirho {
        GLOBAL_MANAGER_CHIRHO.get_or_init(LocaleManagerChirho::new_chirho)
    }

    /// Load built-in locales.
    fn load_builtin_locales_chirho(&self) {
        let mut cache_chirho = self.cache_chirho.write().unwrap();
        cache_chirho.insert("en".to_string(), create_english_locale_chirho());
        cache_chirho.insert("es".to_string(), create_spanish_locale_chirho());
        cache_chirho.insert("de".to_string(), create_german_locale_chirho());
    }

    /// Get a locale by code, falling back to the default locale.
    pub fn get_locale_chirho(&self, code_chirho: &str) -> LocaleChirho {
        let cache_chirho = self.cache_chirho.read().unwrap();

        // Try exact match
        if let Some(locale_chirho) = cache_chirho.get(code_chirho) {
            return locale_chirho.clone();
        }

        // Try language code only (e.g., "en-US" -> "en")
        if let Some(lang_chirho) = code_chirho.split('-').next() {
            if let Some(locale_chirho) = cache_chirho.get(lang_chirho) {
                return locale_chirho.clone();
            }
        }

        // Fall back to default
        cache_chirho
            .get(&self.default_locale_chirho)
            .cloned()
            .unwrap_or_else(create_english_locale_chirho)
    }

    /// Check if a locale is available.
    pub fn has_locale_chirho(&self, code_chirho: &str) -> bool {
        let cache_chirho = self.cache_chirho.read().unwrap();
        cache_chirho.contains_key(code_chirho)
    }

    /// Get all available locale codes.
    pub fn available_locales_chirho(&self) -> Vec<String> {
        let cache_chirho = self.cache_chirho.read().unwrap();
        cache_chirho.keys().cloned().collect()
    }

    /// Register a custom locale.
    pub fn register_locale_chirho(&self, locale_chirho: LocaleChirho) {
        let mut cache_chirho = self.cache_chirho.write().unwrap();
        cache_chirho.insert(locale_chirho.code_chirho.clone(), locale_chirho);
    }

    /// Set the default locale.
    pub fn set_default_locale_chirho(&mut self, code_chirho: &str) {
        self.default_locale_chirho = code_chirho.to_string();
    }

    /// Get the default locale code.
    pub fn default_locale_chirho(&self) -> &str {
        &self.default_locale_chirho
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_manager_creation_chirho() {
        let manager_chirho = LocaleManagerChirho::new_chirho();
        assert!(manager_chirho.has_locale_chirho("en"));
        assert!(manager_chirho.has_locale_chirho("es"));
        assert!(manager_chirho.has_locale_chirho("de"));
    }

    #[test]
    fn test_get_locale_chirho() {
        let manager_chirho = LocaleManagerChirho::new_chirho();
        let locale_chirho = manager_chirho.get_locale_chirho("en");
        assert_eq!(locale_chirho.code_chirho, "en");
    }

    #[test]
    fn test_fallback_to_language_code_chirho() {
        let manager_chirho = LocaleManagerChirho::new_chirho();
        let locale_chirho = manager_chirho.get_locale_chirho("en-US");
        assert_eq!(locale_chirho.code_chirho, "en");
    }

    #[test]
    fn test_fallback_to_default_chirho() {
        let manager_chirho = LocaleManagerChirho::new_chirho();
        let locale_chirho = manager_chirho.get_locale_chirho("nonexistent");
        assert_eq!(locale_chirho.code_chirho, "en");
    }

    #[test]
    fn test_available_locales_chirho() {
        let manager_chirho = LocaleManagerChirho::new_chirho();
        let locales_chirho = manager_chirho.available_locales_chirho();
        assert!(locales_chirho.contains(&"en".to_string()));
        assert!(locales_chirho.contains(&"es".to_string()));
        assert!(locales_chirho.contains(&"de".to_string()));
    }

    #[test]
    fn test_register_custom_locale_chirho() {
        let manager_chirho = LocaleManagerChirho::new_chirho();
        let custom_chirho = LocaleChirho::new_chirho("fr", "French");
        manager_chirho.register_locale_chirho(custom_chirho);
        assert!(manager_chirho.has_locale_chirho("fr"));
    }

    #[test]
    fn test_global_manager_chirho() {
        let manager_chirho = LocaleManagerChirho::global_chirho();
        assert!(manager_chirho.has_locale_chirho("en"));
    }
}
