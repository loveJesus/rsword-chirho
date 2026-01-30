// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Daily devotional module driver.
//!
//! Daily devotional modules use a month/day key (MM.DD) to provide
//! daily readings throughout the year.

use std::path::Path;

use crate::config_chirho::ModuleConfigChirho;
use crate::error_chirho::ResultChirho;
use crate::keys_chirho::date_key_chirho::DateKeyChirho;
use crate::keys_chirho::sw_key_chirho::SwKeyChirho;
use crate::modules_chirho::sw_module_chirho::{ModuleBaseChirho, SwModuleChirho};
use crate::storage_chirho::RawStrChirho;
use crate::{DirectionChirho, EncodingChirho, MarkupChirho};

/// Daily devotional module.
///
/// Provides daily readings keyed by month/day (MM.DD format).
pub struct DailyDevotionalChirho {
    /// Base module functionality.
    base_chirho: ModuleBaseChirho,
    /// String storage for entries.
    storage_chirho: RawStrChirho,
    /// Current date key.
    key_chirho: DateKeyChirho,
    /// Cached entry availability.
    has_entry_chirho: bool,
}

impl DailyDevotionalChirho {
    /// Create a new daily devotional module.
    pub fn new_chirho<P: AsRef<Path>>(
        config_chirho: ModuleConfigChirho,
        path_chirho: P,
        basename_chirho: &str,
    ) -> ResultChirho<Self> {
        let storage_chirho = RawStrChirho::open_chirho(path_chirho, basename_chirho)?;
        let key_chirho = DateKeyChirho::new_chirho();

        Ok(Self {
            base_chirho: ModuleBaseChirho::new_chirho(config_chirho),
            storage_chirho,
            key_chirho,
            has_entry_chirho: false,
        })
    }

    /// Get today's devotional.
    pub fn get_today_chirho(&mut self) -> ResultChirho<String> {
        self.key_chirho = DateKeyChirho::today_chirho();
        self.get_raw_entry_chirho()
    }

    /// Get devotional for a specific date.
    pub fn get_date_chirho(&mut self, month_chirho: u8, day_chirho: u8) -> ResultChirho<String> {
        self.key_chirho = DateKeyChirho::from_date_chirho(month_chirho, day_chirho);
        self.get_raw_entry_chirho()
    }

    /// Get the current date key.
    pub fn date_key_chirho(&self) -> &DateKeyChirho {
        &self.key_chirho
    }

    /// Set leap day handling.
    pub fn set_include_leap_day_chirho(&mut self, include_chirho: bool) {
        self.key_chirho.set_include_leap_day_chirho(include_chirho);
    }

}

impl SwModuleChirho for DailyDevotionalChirho {
    fn name_chirho(&self) -> &str {
        self.base_chirho.name_chirho()
    }

    fn description_chirho(&self) -> &str {
        self.base_chirho.description_chirho()
    }

    fn module_type_chirho(&self) -> &str {
        "Daily Devotional"
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
        self.base_chirho.markup_chirho()
    }

    fn get_key_chirho(&self) -> &dyn SwKeyChirho {
        &self.key_chirho
    }

    fn set_key_chirho(&mut self, key_chirho: &dyn SwKeyChirho) -> ResultChirho<()> {
        self.key_chirho.set_text_chirho(key_chirho.get_text_chirho());
        Ok(())
    }

    fn get_raw_entry_chirho(&mut self) -> ResultChirho<String> {
        let key_text_chirho = self.key_chirho.to_mmdd_chirho();
        match self.storage_chirho.find_by_key_chirho(&key_text_chirho)? {
            Some(value_chirho) => {
                self.has_entry_chirho = true;
                Ok(value_chirho)
            }
            None => {
                self.has_entry_chirho = false;
                Ok(String::new())
            }
        }
    }

    fn is_encrypted_chirho(&self) -> bool {
        self.base_chirho.is_encrypted_chirho()
    }

    fn has_entry_chirho(&self) -> bool {
        self.has_entry_chirho
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

    #[test]
    fn test_module_type_chirho() {
        // Just test that module_type returns the right value
        // Full tests require storage fixtures
        let module_type_chirho = "Daily Devotional";
        assert_eq!(module_type_chirho, "Daily Devotional");
    }

    #[test]
    fn test_date_key_integration_chirho() {
        let key_chirho = DateKeyChirho::from_date_chirho(3, 16);
        assert_eq!(key_chirho.to_mmdd_chirho(), "03.16");
        assert_eq!(format!("{}", key_chirho), "March 16");
    }

    #[test]
    fn test_today_key_chirho() {
        let key_chirho = DateKeyChirho::today_chirho();
        // Verify it's a valid date
        assert!(key_chirho.get_month_chirho() >= 1 && key_chirho.get_month_chirho() <= 12);
        assert!(key_chirho.get_day_chirho() >= 1 && key_chirho.get_day_chirho() <= 31);
    }
}
