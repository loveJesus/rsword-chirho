// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Global option filters for toggling module output features.
//!
//! These filters can enable/disable various rendering options:
//! - Strong's numbers
//! - Morphology tags
//! - Footnotes
//! - Cross-references
//! - Red letter words
//! - Headings
//! - Lemmas

use bitflags::bitflags;
use std::sync::atomic::{AtomicU32, Ordering};

bitflags! {
    /// Global options that can be enabled/disabled.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct GlobalOptionsChirho: u32 {
        /// Show Strong's numbers.
        const STRONGS_CHIRHO = 1 << 0;
        /// Show morphology tags.
        const MORPH_CHIRHO = 1 << 1;
        /// Show footnotes.
        const FOOTNOTES_CHIRHO = 1 << 2;
        /// Show cross-references.
        const XREFS_CHIRHO = 1 << 3;
        /// Show red letter words of Christ.
        const RED_LETTER_CHIRHO = 1 << 4;
        /// Show headings.
        const HEADINGS_CHIRHO = 1 << 5;
        /// Show lemmas.
        const LEMMAS_CHIRHO = 1 << 6;
        /// Show Hebrew vowel points.
        const HEBREW_VOWELS_CHIRHO = 1 << 7;
        /// Show Hebrew cantillation marks.
        const HEBREW_CANTILLATION_CHIRHO = 1 << 8;
        /// Show Greek accents.
        const GREEK_ACCENTS_CHIRHO = 1 << 9;
        /// Show Scripture references.
        const SCRIPREF_CHIRHO = 1 << 10;
        /// Show glosses/translations.
        const GLOSSES_CHIRHO = 1 << 11;
        /// Show textual variants.
        const VARIANTS_CHIRHO = 1 << 12;
        /// Show enumerations/numbers.
        const ENUM_CHIRHO = 1 << 13;
    }
}

impl Default for GlobalOptionsChirho {
    fn default() -> Self {
        Self::empty()
    }
}

impl GlobalOptionsChirho {
    /// Create with all options enabled.
    pub fn all_enabled_chirho() -> Self {
        Self::all()
    }

    /// Create with default SWORD options enabled.
    pub fn sword_default_chirho() -> Self {
        Self::FOOTNOTES_CHIRHO | Self::HEADINGS_CHIRHO | Self::SCRIPREF_CHIRHO
    }

    /// Parse from SWORD-style option string.
    ///
    /// Examples: "nfm" (no footnotes, morph), "sfg" (strongs, footnotes, glosses)
    pub fn from_option_string_chirho(options_chirho: &str) -> Self {
        let mut result_chirho = Self::empty();

        for ch_chirho in options_chirho.chars() {
            match ch_chirho {
                'n' | 's' => result_chirho |= Self::STRONGS_CHIRHO,
                'm' => result_chirho |= Self::MORPH_CHIRHO,
                'f' => result_chirho |= Self::FOOTNOTES_CHIRHO,
                'x' => result_chirho |= Self::XREFS_CHIRHO,
                'r' => result_chirho |= Self::RED_LETTER_CHIRHO,
                'h' => result_chirho |= Self::HEADINGS_CHIRHO,
                'l' => result_chirho |= Self::LEMMAS_CHIRHO,
                'v' => result_chirho |= Self::HEBREW_VOWELS_CHIRHO,
                'c' => result_chirho |= Self::HEBREW_CANTILLATION_CHIRHO,
                'a' => result_chirho |= Self::GREEK_ACCENTS_CHIRHO,
                'p' => result_chirho |= Self::SCRIPREF_CHIRHO,
                'g' => result_chirho |= Self::GLOSSES_CHIRHO,
                't' => result_chirho |= Self::VARIANTS_CHIRHO,
                'e' => result_chirho |= Self::ENUM_CHIRHO,
                _ => {}
            }
        }

        result_chirho
    }

    /// Convert to option string.
    pub fn to_option_string_chirho(&self) -> String {
        let mut result_chirho = String::new();

        if self.contains(Self::STRONGS_CHIRHO) { result_chirho.push('n'); }
        if self.contains(Self::MORPH_CHIRHO) { result_chirho.push('m'); }
        if self.contains(Self::FOOTNOTES_CHIRHO) { result_chirho.push('f'); }
        if self.contains(Self::XREFS_CHIRHO) { result_chirho.push('x'); }
        if self.contains(Self::RED_LETTER_CHIRHO) { result_chirho.push('r'); }
        if self.contains(Self::HEADINGS_CHIRHO) { result_chirho.push('h'); }
        if self.contains(Self::LEMMAS_CHIRHO) { result_chirho.push('l'); }
        if self.contains(Self::HEBREW_VOWELS_CHIRHO) { result_chirho.push('v'); }
        if self.contains(Self::HEBREW_CANTILLATION_CHIRHO) { result_chirho.push('c'); }
        if self.contains(Self::GREEK_ACCENTS_CHIRHO) { result_chirho.push('a'); }
        if self.contains(Self::SCRIPREF_CHIRHO) { result_chirho.push('p'); }
        if self.contains(Self::GLOSSES_CHIRHO) { result_chirho.push('g'); }
        if self.contains(Self::VARIANTS_CHIRHO) { result_chirho.push('t'); }
        if self.contains(Self::ENUM_CHIRHO) { result_chirho.push('e'); }

        result_chirho
    }
}

/// Thread-safe global option state manager.
#[derive(Debug)]
pub struct GlobalOptionManagerChirho {
    /// Current option state as atomic u32.
    state_chirho: AtomicU32,
}

impl Default for GlobalOptionManagerChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

impl GlobalOptionManagerChirho {
    /// Create a new option manager with default state.
    pub fn new_chirho() -> Self {
        Self {
            state_chirho: AtomicU32::new(GlobalOptionsChirho::sword_default_chirho().bits()),
        }
    }

    /// Get current options.
    pub fn get_options_chirho(&self) -> GlobalOptionsChirho {
        GlobalOptionsChirho::from_bits_truncate(self.state_chirho.load(Ordering::Relaxed))
    }

    /// Set options.
    pub fn set_options_chirho(&self, options_chirho: GlobalOptionsChirho) {
        self.state_chirho.store(options_chirho.bits(), Ordering::Relaxed);
    }

    /// Enable an option.
    pub fn enable_chirho(&self, option_chirho: GlobalOptionsChirho) {
        self.state_chirho.fetch_or(option_chirho.bits(), Ordering::Relaxed);
    }

    /// Disable an option.
    pub fn disable_chirho(&self, option_chirho: GlobalOptionsChirho) {
        self.state_chirho.fetch_and(!option_chirho.bits(), Ordering::Relaxed);
    }

    /// Toggle an option.
    pub fn toggle_chirho(&self, option_chirho: GlobalOptionsChirho) {
        self.state_chirho.fetch_xor(option_chirho.bits(), Ordering::Relaxed);
    }

    /// Check if an option is enabled.
    pub fn is_enabled_chirho(&self, option_chirho: GlobalOptionsChirho) -> bool {
        self.get_options_chirho().contains(option_chirho)
    }

    /// Set from option string.
    pub fn set_from_string_chirho(&self, options_chirho: &str) {
        self.set_options_chirho(GlobalOptionsChirho::from_option_string_chirho(options_chirho));
    }
}

impl Clone for GlobalOptionManagerChirho {
    fn clone(&self) -> Self {
        Self {
            state_chirho: AtomicU32::new(self.state_chirho.load(Ordering::Relaxed)),
        }
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_options_default_chirho() {
        let opts_chirho = GlobalOptionsChirho::default();
        assert!(opts_chirho.is_empty());
    }

    #[test]
    fn test_options_all_enabled_chirho() {
        let opts_chirho = GlobalOptionsChirho::all_enabled_chirho();
        assert!(opts_chirho.contains(GlobalOptionsChirho::STRONGS_CHIRHO));
        assert!(opts_chirho.contains(GlobalOptionsChirho::FOOTNOTES_CHIRHO));
    }

    #[test]
    fn test_options_sword_default_chirho() {
        let opts_chirho = GlobalOptionsChirho::sword_default_chirho();
        assert!(opts_chirho.contains(GlobalOptionsChirho::FOOTNOTES_CHIRHO));
        assert!(opts_chirho.contains(GlobalOptionsChirho::HEADINGS_CHIRHO));
        assert!(opts_chirho.contains(GlobalOptionsChirho::SCRIPREF_CHIRHO));
        assert!(!opts_chirho.contains(GlobalOptionsChirho::STRONGS_CHIRHO));
    }

    #[test]
    fn test_from_option_string_chirho() {
        let opts_chirho = GlobalOptionsChirho::from_option_string_chirho("nfm");
        assert!(opts_chirho.contains(GlobalOptionsChirho::STRONGS_CHIRHO));
        assert!(opts_chirho.contains(GlobalOptionsChirho::FOOTNOTES_CHIRHO));
        assert!(opts_chirho.contains(GlobalOptionsChirho::MORPH_CHIRHO));
        assert!(!opts_chirho.contains(GlobalOptionsChirho::XREFS_CHIRHO));
    }

    #[test]
    fn test_to_option_string_chirho() {
        let opts_chirho = GlobalOptionsChirho::STRONGS_CHIRHO | GlobalOptionsChirho::FOOTNOTES_CHIRHO;
        let str_chirho = opts_chirho.to_option_string_chirho();
        assert!(str_chirho.contains('n'));
        assert!(str_chirho.contains('f'));
    }

    #[test]
    fn test_manager_enable_disable_chirho() {
        let manager_chirho = GlobalOptionManagerChirho::new_chirho();

        manager_chirho.enable_chirho(GlobalOptionsChirho::STRONGS_CHIRHO);
        assert!(manager_chirho.is_enabled_chirho(GlobalOptionsChirho::STRONGS_CHIRHO));

        manager_chirho.disable_chirho(GlobalOptionsChirho::STRONGS_CHIRHO);
        assert!(!manager_chirho.is_enabled_chirho(GlobalOptionsChirho::STRONGS_CHIRHO));
    }

    #[test]
    fn test_manager_toggle_chirho() {
        let manager_chirho = GlobalOptionManagerChirho::new_chirho();

        let initial_chirho = manager_chirho.is_enabled_chirho(GlobalOptionsChirho::MORPH_CHIRHO);
        manager_chirho.toggle_chirho(GlobalOptionsChirho::MORPH_CHIRHO);
        assert_ne!(manager_chirho.is_enabled_chirho(GlobalOptionsChirho::MORPH_CHIRHO), initial_chirho);
    }

    #[test]
    fn test_manager_clone_chirho() {
        let manager_chirho = GlobalOptionManagerChirho::new_chirho();
        manager_chirho.enable_chirho(GlobalOptionsChirho::STRONGS_CHIRHO);

        let cloned_chirho = manager_chirho.clone();
        assert!(cloned_chirho.is_enabled_chirho(GlobalOptionsChirho::STRONGS_CHIRHO));
    }

    #[test]
    fn test_manager_set_from_string_chirho() {
        let manager_chirho = GlobalOptionManagerChirho::new_chirho();
        manager_chirho.set_from_string_chirho("nfmxr");

        assert!(manager_chirho.is_enabled_chirho(GlobalOptionsChirho::STRONGS_CHIRHO));
        assert!(manager_chirho.is_enabled_chirho(GlobalOptionsChirho::FOOTNOTES_CHIRHO));
        assert!(manager_chirho.is_enabled_chirho(GlobalOptionsChirho::MORPH_CHIRHO));
        assert!(manager_chirho.is_enabled_chirho(GlobalOptionsChirho::XREFS_CHIRHO));
        assert!(manager_chirho.is_enabled_chirho(GlobalOptionsChirho::RED_LETTER_CHIRHO));
    }
}
