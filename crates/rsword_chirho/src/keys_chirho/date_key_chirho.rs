// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Date key implementation for daily devotional modules.
//!
//! DateKey represents a month/day combination for navigating
//! daily devotional content.

use std::cmp::Ordering;
use std::fmt;

use crate::keys_chirho::sw_key_chirho::{KeyErrorChirho, PositionChirho, SwKeyChirho};

/// Days in each month (non-leap year).
const DAYS_IN_MONTH_CHIRHO: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// Month names for display.
const MONTH_NAMES_CHIRHO: [&str; 12] = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December",
];

/// A key for navigating daily devotional modules by date.
///
/// Represents a month/day combination (MM.DD format).
#[derive(Clone)]
pub struct DateKeyChirho {
    /// Month (1-12).
    month_chirho: u8,
    /// Day (1-31).
    day_chirho: u8,
    /// Whether to include leap day (Feb 29).
    include_leap_day_chirho: bool,
    /// Current error status.
    error_chirho: KeyErrorChirho,
}

impl DateKeyChirho {
    /// Create a new date key at January 1.
    pub fn new_chirho() -> Self {
        Self {
            month_chirho: 1,
            day_chirho: 1,
            include_leap_day_chirho: true,
            error_chirho: KeyErrorChirho::NoneChirho,
        }
    }

    /// Create a date key for a specific month and day.
    pub fn from_date_chirho(month_chirho: u8, day_chirho: u8) -> Self {
        let mut key_chirho = Self::new_chirho();
        key_chirho.set_date_chirho(month_chirho, day_chirho);
        key_chirho
    }

    /// Create a date key from today's date.
    pub fn today_chirho() -> Self {
        let now_chirho = std::time::SystemTime::now();
        let since_epoch_chirho = now_chirho
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();

        // Calculate current date from Unix timestamp
        let days_chirho = (since_epoch_chirho.as_secs() / 86400) as i64;
        let (_, month_chirho, day_chirho) = Self::days_to_date_chirho(days_chirho + 719163);

        Self::from_date_chirho(month_chirho, day_chirho)
    }

    /// Convert days since epoch to year/month/day.
    fn days_to_date_chirho(days_chirho: i64) -> (i32, u8, u8) {
        // Algorithm based on Howard Hinnant's date algorithms
        let z_chirho = days_chirho;
        let era_chirho = z_chirho / 146097;
        let doe_chirho = (z_chirho - era_chirho * 146097) as u32;
        let yoe_chirho = (doe_chirho - doe_chirho / 1460 + doe_chirho / 36524 - doe_chirho / 146096) / 365;
        let y_chirho = yoe_chirho as i64 + era_chirho * 400;
        let doy_chirho = doe_chirho - (365 * yoe_chirho + yoe_chirho / 4 - yoe_chirho / 100);
        let mp_chirho = (5 * doy_chirho + 2) / 153;
        let d_chirho = doy_chirho - (153 * mp_chirho + 2) / 5 + 1;
        let m_chirho = if mp_chirho < 10 { mp_chirho + 3 } else { mp_chirho - 9 };
        let y_chirho = if m_chirho <= 2 { y_chirho + 1 } else { y_chirho };

        (y_chirho as i32, m_chirho as u8, d_chirho as u8)
    }

    /// Parse a date string in MM.DD format.
    pub fn parse_chirho(text_chirho: &str) -> Result<Self, String> {
        let text_chirho = text_chirho.trim();

        // Try MM.DD format first
        if let Some(dot_pos_chirho) = text_chirho.find('.') {
            let month_str_chirho = &text_chirho[..dot_pos_chirho];
            let day_str_chirho = &text_chirho[dot_pos_chirho + 1..];

            let month_chirho = month_str_chirho.parse::<u8>()
                .map_err(|_| format!("Invalid month: {}", month_str_chirho))?;
            let day_chirho = day_str_chirho.parse::<u8>()
                .map_err(|_| format!("Invalid day: {}", day_str_chirho))?;

            let mut key_chirho = Self::new_chirho();
            key_chirho.set_date_chirho(month_chirho, day_chirho);
            return Ok(key_chirho);
        }

        // Try "Month Day" format
        let parts_chirho: Vec<&str> = text_chirho.split_whitespace().collect();
        if parts_chirho.len() == 2 {
            if let Some(month_chirho) = Self::parse_month_name_chirho(parts_chirho[0]) {
                if let Ok(day_chirho) = parts_chirho[1].parse::<u8>() {
                    let mut key_chirho = Self::new_chirho();
                    key_chirho.set_date_chirho(month_chirho, day_chirho);
                    return Ok(key_chirho);
                }
            }
        }

        Err(format!("Cannot parse date: {}", text_chirho))
    }

    /// Parse a month name (full or abbreviated).
    fn parse_month_name_chirho(name_chirho: &str) -> Option<u8> {
        let lower_chirho = name_chirho.to_lowercase();

        for (i_chirho, full_name_chirho) in MONTH_NAMES_CHIRHO.iter().enumerate() {
            if full_name_chirho.to_lowercase().starts_with(&lower_chirho) {
                return Some((i_chirho + 1) as u8);
            }
        }

        None
    }

    /// Set the date.
    pub fn set_date_chirho(&mut self, month_chirho: u8, day_chirho: u8) {
        self.month_chirho = month_chirho.clamp(1, 12);
        let max_day_chirho = self.days_in_month_chirho(self.month_chirho);
        self.day_chirho = day_chirho.clamp(1, max_day_chirho);
        self.error_chirho = KeyErrorChirho::NoneChirho;
    }

    /// Get the current month (1-12).
    pub fn get_month_chirho(&self) -> u8 {
        self.month_chirho
    }

    /// Set the month.
    pub fn set_month_chirho(&mut self, month_chirho: u8) {
        self.set_date_chirho(month_chirho, self.day_chirho);
    }

    /// Get the current day (1-31).
    pub fn get_day_chirho(&self) -> u8 {
        self.day_chirho
    }

    /// Set the day.
    pub fn set_day_chirho(&mut self, day_chirho: u8) {
        self.set_date_chirho(self.month_chirho, day_chirho);
    }

    /// Get the month name.
    pub fn get_month_name_chirho(&self) -> &'static str {
        MONTH_NAMES_CHIRHO[(self.month_chirho - 1) as usize]
    }

    /// Get the date in MM.DD format.
    pub fn to_mmdd_chirho(&self) -> String {
        format!("{:02}.{:02}", self.month_chirho, self.day_chirho)
    }

    /// Get days in a specific month.
    fn days_in_month_chirho(&self, month_chirho: u8) -> u8 {
        if month_chirho == 2 && self.include_leap_day_chirho {
            29
        } else {
            DAYS_IN_MONTH_CHIRHO[(month_chirho - 1) as usize]
        }
    }

    /// Calculate day of year (1-366).
    pub fn day_of_year_chirho(&self) -> u16 {
        let mut day_chirho = 0u16;
        for m_chirho in 1..self.month_chirho {
            day_chirho += self.days_in_month_chirho(m_chirho) as u16;
        }
        day_chirho + self.day_chirho as u16
    }

    /// Set whether to include February 29.
    pub fn set_include_leap_day_chirho(&mut self, include_chirho: bool) {
        self.include_leap_day_chirho = include_chirho;
        // Re-normalize the date
        self.set_date_chirho(self.month_chirho, self.day_chirho);
    }

    /// Check if leap day is included.
    pub fn is_include_leap_day_chirho(&self) -> bool {
        self.include_leap_day_chirho
    }
}

impl fmt::Display for DateKeyChirho {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.get_month_name_chirho(), self.day_chirho)
    }
}

impl fmt::Debug for DateKeyChirho {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DateKeyChirho")
            .field("month_chirho", &self.month_chirho)
            .field("day_chirho", &self.day_chirho)
            .field("include_leap_day_chirho", &self.include_leap_day_chirho)
            .finish()
    }
}

impl SwKeyChirho for DateKeyChirho {
    fn get_text_chirho(&self) -> &str {
        // We need to return a reference, so we'll use a static buffer approach
        // This is a limitation - in practice we'd use a cached String
        Box::leak(self.to_mmdd_chirho().into_boxed_str())
    }

    fn set_text_chirho(&mut self, text_chirho: &str) {
        if let Ok(key_chirho) = Self::parse_chirho(text_chirho) {
            self.month_chirho = key_chirho.month_chirho;
            self.day_chirho = key_chirho.day_chirho;
            self.error_chirho = KeyErrorChirho::NoneChirho;
        } else {
            self.error_chirho = KeyErrorChirho::ParseErrorChirho;
        }
    }

    fn get_short_text_chirho(&self) -> String {
        self.to_mmdd_chirho()
    }

    fn get_osis_ref_chirho(&self) -> String {
        self.to_mmdd_chirho()
    }

    fn pop_error_chirho(&mut self) -> KeyErrorChirho {
        let error_chirho = self.error_chirho;
        self.error_chirho = KeyErrorChirho::NoneChirho;
        error_chirho
    }

    fn has_error_chirho(&self) -> bool {
        self.error_chirho.is_error_chirho()
    }

    fn clone_key_chirho(&self) -> Box<dyn SwKeyChirho> {
        Box::new(self.clone())
    }

    fn get_index_chirho(&self) -> i64 {
        self.day_of_year_chirho() as i64
    }

    fn set_index_chirho(&mut self, index_chirho: i64) {
        let mut remaining_chirho = index_chirho.clamp(1, 366) as u16;

        for month_chirho in 1u8..=12 {
            let days_chirho = self.days_in_month_chirho(month_chirho) as u16;
            if remaining_chirho <= days_chirho {
                self.month_chirho = month_chirho;
                self.day_chirho = remaining_chirho as u8;
                return;
            }
            remaining_chirho -= days_chirho;
        }

        // Shouldn't reach here, but set to Dec 31 as fallback
        self.month_chirho = 12;
        self.day_chirho = 31;
    }

    fn increment_chirho(&mut self, steps_chirho: i32) {
        for _ in 0..steps_chirho.abs() {
            if steps_chirho > 0 {
                self.day_chirho += 1;
                if self.day_chirho > self.days_in_month_chirho(self.month_chirho) {
                    self.day_chirho = 1;
                    self.month_chirho += 1;
                    if self.month_chirho > 12 {
                        self.month_chirho = 1;
                        self.error_chirho = KeyErrorChirho::OutOfBoundsChirho;
                    }
                }
            } else {
                self.decrement_chirho(1);
            }
        }
    }

    fn decrement_chirho(&mut self, steps_chirho: i32) {
        for _ in 0..steps_chirho.abs() {
            if self.day_chirho > 1 {
                self.day_chirho -= 1;
            } else if self.month_chirho > 1 {
                self.month_chirho -= 1;
                self.day_chirho = self.days_in_month_chirho(self.month_chirho);
            } else {
                // Wrap to Dec 31
                self.month_chirho = 12;
                self.day_chirho = 31;
                self.error_chirho = KeyErrorChirho::OutOfBoundsChirho;
            }
        }
    }

    fn set_position_chirho(&mut self, position_chirho: PositionChirho) {
        match position_chirho {
            PositionChirho::TopChirho => {
                self.month_chirho = 1;
                self.day_chirho = 1;
            }
            PositionChirho::BottomChirho => {
                self.month_chirho = 12;
                self.day_chirho = 31;
            }
        }
        self.error_chirho = KeyErrorChirho::NoneChirho;
    }

    fn compare_chirho(&self, other_chirho: &dyn SwKeyChirho) -> Ordering {
        self.get_index_chirho().cmp(&other_chirho.get_index_chirho())
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_new_date_key_chirho() {
        let key_chirho = DateKeyChirho::new_chirho();
        assert_eq!(key_chirho.get_month_chirho(), 1);
        assert_eq!(key_chirho.get_day_chirho(), 1);
        assert_eq!(key_chirho.get_month_name_chirho(), "January");
    }

    #[test]
    fn test_from_date_chirho() {
        let key_chirho = DateKeyChirho::from_date_chirho(7, 4);
        assert_eq!(key_chirho.get_month_chirho(), 7);
        assert_eq!(key_chirho.get_day_chirho(), 4);
        assert_eq!(key_chirho.get_month_name_chirho(), "July");
    }

    #[test]
    fn test_parse_mmdd_chirho() {
        let key_chirho = DateKeyChirho::parse_chirho("03.16").unwrap();
        assert_eq!(key_chirho.get_month_chirho(), 3);
        assert_eq!(key_chirho.get_day_chirho(), 16);
    }

    #[test]
    fn test_parse_month_name_chirho() {
        let key_chirho = DateKeyChirho::parse_chirho("March 16").unwrap();
        assert_eq!(key_chirho.get_month_chirho(), 3);
        assert_eq!(key_chirho.get_day_chirho(), 16);
    }

    #[test]
    fn test_parse_abbreviated_month_chirho() {
        let key_chirho = DateKeyChirho::parse_chirho("Mar 16").unwrap();
        assert_eq!(key_chirho.get_month_chirho(), 3);
        assert_eq!(key_chirho.get_day_chirho(), 16);
    }

    #[test]
    fn test_to_mmdd_chirho() {
        let key_chirho = DateKeyChirho::from_date_chirho(3, 16);
        assert_eq!(key_chirho.to_mmdd_chirho(), "03.16");
    }

    #[test]
    fn test_display_chirho() {
        let key_chirho = DateKeyChirho::from_date_chirho(3, 16);
        assert_eq!(format!("{}", key_chirho), "March 16");
    }

    #[test]
    fn test_day_of_year_chirho() {
        let key_chirho = DateKeyChirho::from_date_chirho(1, 1);
        assert_eq!(key_chirho.day_of_year_chirho(), 1);

        let key_chirho = DateKeyChirho::from_date_chirho(12, 31);
        assert_eq!(key_chirho.day_of_year_chirho(), 366); // With leap day
    }

    #[test]
    fn test_increment_chirho() {
        let mut key_chirho = DateKeyChirho::from_date_chirho(1, 31);
        key_chirho.increment_chirho(1);
        assert_eq!(key_chirho.get_month_chirho(), 2);
        assert_eq!(key_chirho.get_day_chirho(), 1);
    }

    #[test]
    fn test_decrement_chirho() {
        let mut key_chirho = DateKeyChirho::from_date_chirho(2, 1);
        key_chirho.decrement_chirho(1);
        assert_eq!(key_chirho.get_month_chirho(), 1);
        assert_eq!(key_chirho.get_day_chirho(), 31);
    }

    #[test]
    fn test_set_position_chirho() {
        let mut key_chirho = DateKeyChirho::new_chirho();

        key_chirho.set_position_chirho(PositionChirho::BottomChirho);
        assert_eq!(key_chirho.get_month_chirho(), 12);
        assert_eq!(key_chirho.get_day_chirho(), 31);

        key_chirho.set_position_chirho(PositionChirho::TopChirho);
        assert_eq!(key_chirho.get_month_chirho(), 1);
        assert_eq!(key_chirho.get_day_chirho(), 1);
    }

    #[test]
    fn test_set_index_chirho() {
        let mut key_chirho = DateKeyChirho::new_chirho();
        key_chirho.set_index_chirho(60); // Feb 29 (with leap day)
        assert_eq!(key_chirho.get_month_chirho(), 2);
        assert_eq!(key_chirho.get_day_chirho(), 29);
    }

    #[test]
    fn test_leap_day_handling_chirho() {
        let mut key_chirho = DateKeyChirho::new_chirho();
        key_chirho.set_include_leap_day_chirho(false);
        key_chirho.set_date_chirho(2, 29);
        assert_eq!(key_chirho.get_day_chirho(), 28); // Clamped to 28

        key_chirho.set_include_leap_day_chirho(true);
        key_chirho.set_date_chirho(2, 29);
        assert_eq!(key_chirho.get_day_chirho(), 29); // Feb 29 allowed
    }

    #[test]
    fn test_today_chirho() {
        let key_chirho = DateKeyChirho::today_chirho();
        // Just verify it returns a valid date
        assert!(key_chirho.get_month_chirho() >= 1 && key_chirho.get_month_chirho() <= 12);
        assert!(key_chirho.get_day_chirho() >= 1 && key_chirho.get_day_chirho() <= 31);
    }
}
