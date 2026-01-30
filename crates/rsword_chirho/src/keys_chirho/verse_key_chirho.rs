// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! VerseKey implementation for Bible verse navigation.
//!
//! [`VerseKeyChirho`] is the primary key type for Bible texts and commentaries.
//! It handles book, chapter, and verse references with support for
//! different versification systems.
//!
//! ## Parsing References
//!
//! ```rust
//! use rsword_chirho::VerseKeyChirho;
//!
//! // Various reference formats are supported
//! let key1_chirho = VerseKeyChirho::from_str_chirho("Genesis 1:1").unwrap();
//! let key2_chirho = VerseKeyChirho::from_str_chirho("Gen 1:1").unwrap();       // Abbreviation
//! let key3_chirho = VerseKeyChirho::from_str_chirho("1 John 3:16").unwrap();   // Numbered book
//! let key4_chirho = VerseKeyChirho::from_str_chirho("I John 3:16").unwrap();   // Roman numeral
//! let key5_chirho = VerseKeyChirho::from_str_chirho("Ps 23").unwrap();         // Chapter only
//! ```
//!
//! ## Navigation
//!
//! ```rust,ignore
//! use rsword_chirho::VerseKeyChirho;
//!
//! let mut key_chirho = VerseKeyChirho::from_str_chirho("Genesis 1:1").unwrap();
//!
//! // Move forward
//! key_chirho.increment_chirho(1);
//! println!("Now at: {}", key_chirho); // Genesis 1:2
//!
//! // Crosses chapter boundary automatically
//! key_chirho.increment_chirho(30);
//! println!("Chapter: {}", key_chirho.get_chapter_chirho());
//!
//! // Move backward
//! key_chirho.decrement_chirho(1);
//! ```
//!
//! ## Verse Ranges
//!
//! Set bounds for iteration:
//!
//! ```rust,ignore
//! use rsword_chirho::VerseKeyChirho;
//!
//! let mut key_chirho = VerseKeyChirho::from_str_chirho("Genesis 1:1").unwrap();
//!
//! // Set bounds for iteration
//! key_chirho.set_lower_bound_chirho(&VerseKeyChirho::from_str_chirho("Genesis 1:1").unwrap());
//! key_chirho.set_upper_bound_chirho(&VerseKeyChirho::from_str_chirho("Genesis 1:10").unwrap());
//!
//! // Iterate through verses 1-10
//! let mut count_chirho = 0;
//! while !key_chirho.pop_error_chirho() {
//!     count_chirho += 1;
//!     key_chirho.increment_chirho(1);
//! }
//! println!("Total verses: {}", count_chirho);
//! ```
//!
//! ## Versification Support
//!
//! ```rust
//! use std::sync::Arc;
//! use rsword_chirho::VerseKeyChirho;
//! use rsword_chirho::versification_chirho::catholic_chirho;
//!
//! // Use Catholic versification
//! let key_chirho = VerseKeyChirho::with_versification_chirho(Arc::new(catholic_chirho().clone()));
//! ```

use std::cmp::Ordering;
use std::fmt;
use std::sync::Arc;

use crate::error_chirho::ResultChirho;
use crate::keys_chirho::sw_key_chirho::{KeyErrorChirho, PositionChirho, SwKeyChirho};
use crate::versification_chirho::{
    kjv_chirho, TestamentChirho, VersificationChirho,
};

/// A key for navigating Bible verses.
#[derive(Clone)]
pub struct VerseKeyChirho {
    /// Cached text representation.
    text_chirho: String,
    /// Current testament (1=OT, 2=NT).
    testament_chirho: u8,
    /// Book index within testament (0-based).
    book_chirho: u8,
    /// Chapter number (1-based, 0=book intro).
    chapter_chirho: u16,
    /// Verse number (1-based, 0=chapter intro).
    verse_chirho: u16,
    /// Verse suffix (e.g., 'a', 'b').
    suffix_chirho: Option<char>,
    /// Versification system.
    v11n_chirho: Arc<VersificationChirho>,
    /// Error status.
    error_chirho: KeyErrorChirho,
    /// Auto-normalize flag.
    auto_normalize_chirho: bool,
    /// Include intros flag.
    intros_chirho: bool,
    /// Lower bound for iteration.
    lower_bound_chirho: Option<Box<VerseKeyChirho>>,
    /// Upper bound for iteration.
    upper_bound_chirho: Option<Box<VerseKeyChirho>>,
}

impl VerseKeyChirho {
    /// Create a new VerseKey at Genesis 1:1.
    pub fn new_chirho() -> Self {
        Self::with_versification_chirho(Arc::new(kjv_chirho().clone()))
    }

    /// Create a new VerseKey with a specific versification.
    pub fn with_versification_chirho(v11n_chirho: Arc<VersificationChirho>) -> Self {
        let mut key_chirho = Self {
            text_chirho: String::new(),
            testament_chirho: 1,
            book_chirho: 0,
            chapter_chirho: 1,
            verse_chirho: 1,
            suffix_chirho: None,
            v11n_chirho,
            error_chirho: KeyErrorChirho::NoneChirho,
            auto_normalize_chirho: true,
            intros_chirho: false,
            lower_bound_chirho: None,
            upper_bound_chirho: None,
        };
        key_chirho.refresh_text_chirho();
        key_chirho
    }

    /// Parse a verse reference string.
    pub fn from_str_chirho(text_chirho: &str) -> ResultChirho<Self> {
        let mut key_chirho = Self::new_chirho();
        key_chirho.parse_chirho(text_chirho)?;
        Ok(key_chirho)
    }

    /// Parse a verse reference and update this key.
    pub fn parse_chirho(&mut self, text_chirho: &str) -> ResultChirho<()> {
        self.error_chirho = KeyErrorChirho::NoneChirho;

        let text_chirho = text_chirho.trim();
        if text_chirho.is_empty() {
            return Ok(());
        }

        // Parse the reference: "Book Chapter:Verse" or "Book Chapter" or "Book"
        let (book_part_chirho, ref_part_chirho) = self.split_book_ref_chirho(text_chirho);

        // Look up the book
        if let Some((testament_chirho, book_idx_chirho)) = self.v11n_chirho.lookup_book_chirho(book_part_chirho) {
            self.testament_chirho = match testament_chirho {
                TestamentChirho::OldChirho => 1,
                TestamentChirho::NewChirho => 2,
            };
            self.book_chirho = book_idx_chirho as u8;

            // Parse chapter:verse if present
            if let Some(ref_chirho) = ref_part_chirho {
                self.parse_chapter_verse_chirho(ref_chirho);
            } else {
                self.chapter_chirho = 1;
                self.verse_chirho = 1;
            }

            if self.auto_normalize_chirho {
                self.normalize_chirho();
            }

            self.refresh_text_chirho();
            Ok(())
        } else {
            self.error_chirho = KeyErrorChirho::ParseErrorChirho;
            Err(crate::error_chirho::ErrorChirho::invalid_verse_reference_chirho(text_chirho))
        }
    }

    /// Split a reference into book name and chapter:verse parts.
    fn split_book_ref_chirho<'a>(&self, text_chirho: &'a str) -> (&'a str, Option<&'a str>) {
        // Find where the book name ends and the reference begins
        // Book names can contain numbers at the start (1 John) or letters only
        // References start with a digit after the book name

        let bytes_chirho = text_chirho.as_bytes();

        // Skip leading numbers and spaces (e.g., "1 John", "2 Kings")
        let mut book_end_chirho = 0;
        let mut found_alpha_chirho = false;

        for (i_chirho, &ch_chirho) in bytes_chirho.iter().enumerate() {
            if ch_chirho.is_ascii_alphabetic() {
                found_alpha_chirho = true;
            }

            if found_alpha_chirho && ch_chirho.is_ascii_digit() {
                // Found a digit after alphabetic chars - this is the chapter
                book_end_chirho = i_chirho;
                break;
            }

            if found_alpha_chirho && ch_chirho == b' ' {
                // Check if next non-space char is a digit
                let rest_chirho = &text_chirho[i_chirho..].trim_start();
                if !rest_chirho.is_empty() && rest_chirho.chars().next().unwrap().is_ascii_digit() {
                    book_end_chirho = i_chirho;
                    break;
                }
            }
        }

        if book_end_chirho == 0 {
            (text_chirho, None)
        } else {
            let book_chirho = text_chirho[..book_end_chirho].trim();
            let ref_chirho = text_chirho[book_end_chirho..].trim();
            if ref_chirho.is_empty() {
                (book_chirho, None)
            } else {
                (book_chirho, Some(ref_chirho))
            }
        }
    }

    /// Parse a chapter:verse reference.
    fn parse_chapter_verse_chirho(&mut self, ref_chirho: &str) {
        let ref_chirho = ref_chirho.trim();

        if let Some(colon_pos_chirho) = ref_chirho.find(':') {
            // Chapter:Verse format
            if let Ok(ch_chirho) = ref_chirho[..colon_pos_chirho].trim().parse::<u16>() {
                self.chapter_chirho = ch_chirho;
            }

            let verse_part_chirho = &ref_chirho[colon_pos_chirho + 1..].trim();
            self.parse_verse_with_suffix_chirho(verse_part_chirho);
        } else {
            // Just chapter number
            if let Ok(ch_chirho) = ref_chirho.parse::<u16>() {
                self.chapter_chirho = ch_chirho;
                self.verse_chirho = 1;
            }
        }
    }

    /// Parse a verse number with optional suffix.
    fn parse_verse_with_suffix_chirho(&mut self, verse_str_chirho: &str) {
        let verse_str_chirho = verse_str_chirho.trim();

        // Check for suffix like "16a" or "16b"
        if verse_str_chirho.len() >= 2 {
            let last_char_chirho = verse_str_chirho.chars().last().unwrap();
            if last_char_chirho.is_ascii_lowercase() {
                let num_part_chirho = &verse_str_chirho[..verse_str_chirho.len() - 1];
                if let Ok(v_chirho) = num_part_chirho.parse::<u16>() {
                    self.verse_chirho = v_chirho;
                    self.suffix_chirho = Some(last_char_chirho);
                    return;
                }
            }
        }

        // No suffix
        if let Ok(v_chirho) = verse_str_chirho.parse::<u16>() {
            self.verse_chirho = v_chirho;
        }
        self.suffix_chirho = None;
    }

    /// Refresh the text representation.
    fn refresh_text_chirho(&mut self) {
        let testament_chirho = if self.testament_chirho == 1 {
            TestamentChirho::OldChirho
        } else {
            TestamentChirho::NewChirho
        };

        if let Some(book_chirho) = self.v11n_chirho.get_book_chirho(testament_chirho, self.book_chirho as usize) {
            self.text_chirho = if let Some(suffix_chirho) = self.suffix_chirho {
                format!(
                    "{} {}:{}{}",
                    book_chirho.name_chirho, self.chapter_chirho, self.verse_chirho, suffix_chirho
                )
            } else {
                format!(
                    "{} {}:{}",
                    book_chirho.name_chirho, self.chapter_chirho, self.verse_chirho
                )
            };
        } else {
            self.text_chirho = format!(
                "Unknown {}:{}",
                self.chapter_chirho, self.verse_chirho
            );
        }
    }

    /// Normalize the key to valid bounds.
    pub fn normalize_chirho(&mut self) {
        let testament_chirho = if self.testament_chirho == 1 {
            TestamentChirho::OldChirho
        } else {
            TestamentChirho::NewChirho
        };

        // Ensure testament is valid (clamp to 1-2)
        self.testament_chirho = self.testament_chirho.clamp(1, 2);

        // Ensure book is valid
        let max_book_chirho = self.v11n_chirho.book_count_chirho(testament_chirho);
        if self.book_chirho as usize >= max_book_chirho {
            if self.testament_chirho == 1 && !self.v11n_chirho.nt_books_chirho.is_empty() {
                // Move to NT
                self.testament_chirho = 2;
                self.book_chirho = 0;
            } else {
                self.book_chirho = (max_book_chirho - 1) as u8;
                self.error_chirho = KeyErrorChirho::OutOfBoundsChirho;
            }
        }

        // Get book info for chapter/verse validation
        let testament_chirho = if self.testament_chirho == 1 {
            TestamentChirho::OldChirho
        } else {
            TestamentChirho::NewChirho
        };

        if let Some(book_info_chirho) = self.v11n_chirho.get_book_chirho(testament_chirho, self.book_chirho as usize) {
            // Handle chapter intro
            if self.chapter_chirho == 0 && !self.intros_chirho {
                self.chapter_chirho = 1;
            }

            // Ensure chapter is valid
            if self.chapter_chirho > book_info_chirho.chapter_count_chirho as u16 {
                // Move to next book
                self.chapter_chirho = book_info_chirho.chapter_count_chirho as u16;
                self.verse_chirho = book_info_chirho.max_verse_chirho(self.chapter_chirho as u8).unwrap_or(1) as u16;
            }

            // Handle verse intro
            if self.verse_chirho == 0 && !self.intros_chirho && self.chapter_chirho > 0 {
                self.verse_chirho = 1;
            }

            // Ensure verse is valid
            if self.chapter_chirho > 0 {
                if let Some(max_verse_chirho) = book_info_chirho.max_verse_chirho(self.chapter_chirho as u8) {
                    if self.verse_chirho > max_verse_chirho as u16 {
                        self.verse_chirho = max_verse_chirho as u16;
                    }
                }
            }
        }
    }

    /// Get the current testament.
    pub fn get_testament_chirho(&self) -> u8 {
        self.testament_chirho
    }

    /// Set the testament.
    pub fn set_testament_chirho(&mut self, testament_chirho: u8) {
        self.testament_chirho = testament_chirho;
        if self.auto_normalize_chirho {
            self.normalize_chirho();
        }
        self.refresh_text_chirho();
    }

    /// Get the current book index.
    pub fn get_book_chirho(&self) -> u8 {
        self.book_chirho
    }

    /// Set the book index.
    pub fn set_book_chirho(&mut self, book_chirho: u8) {
        self.book_chirho = book_chirho;
        if self.auto_normalize_chirho {
            self.normalize_chirho();
        }
        self.refresh_text_chirho();
    }

    /// Get the current chapter.
    pub fn get_chapter_chirho(&self) -> u16 {
        self.chapter_chirho
    }

    /// Set the chapter.
    pub fn set_chapter_chirho(&mut self, chapter_chirho: u16) {
        self.chapter_chirho = chapter_chirho;
        if self.auto_normalize_chirho {
            self.normalize_chirho();
        }
        self.refresh_text_chirho();
    }

    /// Get the current verse.
    pub fn get_verse_chirho(&self) -> u16 {
        self.verse_chirho
    }

    /// Set the verse.
    pub fn set_verse_chirho(&mut self, verse_chirho: u16) {
        self.verse_chirho = verse_chirho;
        if self.auto_normalize_chirho {
            self.normalize_chirho();
        }
        self.refresh_text_chirho();
    }

    /// Get the verse suffix.
    pub fn get_suffix_chirho(&self) -> Option<char> {
        self.suffix_chirho
    }

    /// Set the verse suffix.
    pub fn set_suffix_chirho(&mut self, suffix_chirho: Option<char>) {
        self.suffix_chirho = suffix_chirho;
        self.refresh_text_chirho();
    }

    /// Get the book name.
    pub fn get_book_name_chirho(&self) -> &str {
        let testament_chirho = if self.testament_chirho == 1 {
            TestamentChirho::OldChirho
        } else {
            TestamentChirho::NewChirho
        };

        self.v11n_chirho
            .get_book_chirho(testament_chirho, self.book_chirho as usize)
            .map(|b| b.name_chirho.as_str())
            .unwrap_or("Unknown")
    }

    /// Get the OSIS book abbreviation.
    pub fn get_book_osis_chirho(&self) -> &str {
        let testament_chirho = if self.testament_chirho == 1 {
            TestamentChirho::OldChirho
        } else {
            TestamentChirho::NewChirho
        };

        self.v11n_chirho
            .get_book_chirho(testament_chirho, self.book_chirho as usize)
            .map(|b| b.osis_chirho.as_str())
            .unwrap_or("Unknown")
    }

    /// Get maximum chapter for current book.
    pub fn get_chapter_max_chirho(&self) -> u16 {
        let testament_chirho = if self.testament_chirho == 1 {
            TestamentChirho::OldChirho
        } else {
            TestamentChirho::NewChirho
        };

        self.v11n_chirho
            .get_book_chirho(testament_chirho, self.book_chirho as usize)
            .map(|b| b.chapter_count_chirho as u16)
            .unwrap_or(1)
    }

    /// Get maximum verse for current chapter.
    pub fn get_verse_max_chirho(&self) -> u16 {
        let testament_chirho = if self.testament_chirho == 1 {
            TestamentChirho::OldChirho
        } else {
            TestamentChirho::NewChirho
        };

        self.v11n_chirho
            .get_book_chirho(testament_chirho, self.book_chirho as usize)
            .and_then(|b| b.max_verse_chirho(self.chapter_chirho as u8))
            .map(|v| v as u16)
            .unwrap_or(1)
    }

    /// Enable or disable auto-normalization.
    pub fn set_auto_normalize_chirho(&mut self, auto_chirho: bool) {
        self.auto_normalize_chirho = auto_chirho;
    }

    /// Check if auto-normalization is enabled.
    pub fn is_auto_normalize_chirho(&self) -> bool {
        self.auto_normalize_chirho
    }

    /// Enable or disable intro verses.
    pub fn set_intros_chirho(&mut self, intros_chirho: bool) {
        self.intros_chirho = intros_chirho;
    }

    /// Check if intro verses are enabled.
    pub fn is_intros_chirho(&self) -> bool {
        self.intros_chirho
    }

    /// Get the versification system.
    pub fn get_versification_chirho(&self) -> &VersificationChirho {
        &self.v11n_chirho
    }

    /// Set the versification system.
    pub fn set_versification_chirho(&mut self, v11n_chirho: &'static VersificationChirho) {
        self.v11n_chirho = Arc::new(v11n_chirho.clone());
        if self.auto_normalize_chirho {
            self.normalize_chirho();
        }
        self.refresh_text_chirho();
    }

    /// Get the testament as a TestamentChirho enum.
    pub fn testament_enum_chirho(&self) -> TestamentChirho {
        if self.testament_chirho == 1 {
            TestamentChirho::OldChirho
        } else {
            TestamentChirho::NewChirho
        }
    }

    /// Calculate the index for this verse position.
    pub fn calculate_index_chirho(&self) -> i64 {
        let testament_chirho = if self.testament_chirho == 1 {
            TestamentChirho::OldChirho
        } else {
            TestamentChirho::NewChirho
        };

        let testament_offset_chirho = if self.testament_chirho == 2 {
            self.v11n_chirho.testament_size_chirho(TestamentChirho::OldChirho) as i64
        } else {
            0
        };

        let verse_offset_chirho = self.v11n_chirho.calculate_index_chirho(
            testament_chirho,
            self.book_chirho as usize,
            self.chapter_chirho as u8,
            self.verse_chirho as u8,
        ).unwrap_or(0) as i64;

        testament_offset_chirho + verse_offset_chirho
    }

    /// Set the lower bound for iteration.
    pub fn set_lower_bound_chirho(&mut self, bound_chirho: &VerseKeyChirho) {
        self.lower_bound_chirho = Some(Box::new(bound_chirho.clone()));
    }

    /// Set the upper bound for iteration.
    pub fn set_upper_bound_chirho(&mut self, bound_chirho: &VerseKeyChirho) {
        self.upper_bound_chirho = Some(Box::new(bound_chirho.clone()));
    }

    /// Clear bounds.
    pub fn clear_bounds_chirho(&mut self) {
        self.lower_bound_chirho = None;
        self.upper_bound_chirho = None;
    }
}

impl fmt::Display for VerseKeyChirho {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text_chirho)
    }
}

impl fmt::Debug for VerseKeyChirho {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VerseKeyChirho")
            .field("text_chirho", &self.text_chirho)
            .field("testament_chirho", &self.testament_chirho)
            .field("book_chirho", &self.book_chirho)
            .field("chapter_chirho", &self.chapter_chirho)
            .field("verse_chirho", &self.verse_chirho)
            .field("suffix_chirho", &self.suffix_chirho)
            .finish()
    }
}

impl SwKeyChirho for VerseKeyChirho {
    fn get_text_chirho(&self) -> &str {
        &self.text_chirho
    }

    fn set_text_chirho(&mut self, text_chirho: &str) {
        let _ = self.parse_chirho(text_chirho);
    }

    fn get_short_text_chirho(&self) -> String {
        format!(
            "{} {}:{}",
            self.get_book_osis_chirho(),
            self.chapter_chirho,
            self.verse_chirho
        )
    }

    fn get_osis_ref_chirho(&self) -> String {
        format!(
            "{}.{}.{}",
            self.get_book_osis_chirho(),
            self.chapter_chirho,
            self.verse_chirho
        )
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
        self.calculate_index_chirho()
    }

    fn set_index_chirho(&mut self, _index_chirho: i64) {
        // TODO: Implement reverse lookup from index
    }

    fn increment_chirho(&mut self, steps_chirho: i32) {
        for _ in 0..steps_chirho.abs() {
            if steps_chirho > 0 {
                self.verse_chirho += 1;
                if self.verse_chirho > self.get_verse_max_chirho() {
                    self.verse_chirho = 1;
                    self.chapter_chirho += 1;
                    if self.chapter_chirho > self.get_chapter_max_chirho() {
                        self.chapter_chirho = 1;
                        self.book_chirho += 1;

                        let testament_chirho = if self.testament_chirho == 1 {
                            TestamentChirho::OldChirho
                        } else {
                            TestamentChirho::NewChirho
                        };
                        let max_book_chirho = self.v11n_chirho.book_count_chirho(testament_chirho);

                        if self.book_chirho as usize >= max_book_chirho {
                            if self.testament_chirho == 1 {
                                self.testament_chirho = 2;
                                self.book_chirho = 0;
                            } else {
                                // End of Bible
                                self.book_chirho = (max_book_chirho - 1) as u8;
                                self.chapter_chirho = self.get_chapter_max_chirho();
                                self.verse_chirho = self.get_verse_max_chirho();
                                self.error_chirho = KeyErrorChirho::OutOfBoundsChirho;
                            }
                        }
                    }
                }
            } else {
                self.decrement_chirho(1);
            }
        }

        // Check bounds
        if let Some(upper_chirho) = &self.upper_bound_chirho {
            if self.calculate_index_chirho() > upper_chirho.calculate_index_chirho() {
                self.error_chirho = KeyErrorChirho::OutOfBoundsChirho;
            }
        }

        self.refresh_text_chirho();
    }

    fn decrement_chirho(&mut self, steps_chirho: i32) {
        for _ in 0..steps_chirho.abs() {
            if self.verse_chirho > 1 {
                self.verse_chirho -= 1;
            } else if self.chapter_chirho > 1 {
                self.chapter_chirho -= 1;
                self.verse_chirho = self.get_verse_max_chirho();
            } else if self.book_chirho > 0 {
                self.book_chirho -= 1;
                self.chapter_chirho = self.get_chapter_max_chirho();
                self.verse_chirho = self.get_verse_max_chirho();
            } else if self.testament_chirho == 2 {
                self.testament_chirho = 1;
                self.book_chirho = (self.v11n_chirho.book_count_chirho(TestamentChirho::OldChirho) - 1) as u8;
                self.chapter_chirho = self.get_chapter_max_chirho();
                self.verse_chirho = self.get_verse_max_chirho();
            } else {
                // Beginning of Bible
                self.error_chirho = KeyErrorChirho::OutOfBoundsChirho;
            }
        }

        // Check bounds
        if let Some(lower_chirho) = &self.lower_bound_chirho {
            if self.calculate_index_chirho() < lower_chirho.calculate_index_chirho() {
                self.error_chirho = KeyErrorChirho::OutOfBoundsChirho;
            }
        }

        self.refresh_text_chirho();
    }

    fn set_position_chirho(&mut self, position_chirho: PositionChirho) {
        match position_chirho {
            PositionChirho::TopChirho => {
                if let Some(lower_chirho) = &self.lower_bound_chirho {
                    self.testament_chirho = lower_chirho.testament_chirho;
                    self.book_chirho = lower_chirho.book_chirho;
                    self.chapter_chirho = lower_chirho.chapter_chirho;
                    self.verse_chirho = lower_chirho.verse_chirho;
                } else {
                    self.testament_chirho = 1;
                    self.book_chirho = 0;
                    self.chapter_chirho = 1;
                    self.verse_chirho = 1;
                }
            }
            PositionChirho::BottomChirho => {
                if let Some(upper_chirho) = &self.upper_bound_chirho {
                    self.testament_chirho = upper_chirho.testament_chirho;
                    self.book_chirho = upper_chirho.book_chirho;
                    self.chapter_chirho = upper_chirho.chapter_chirho;
                    self.verse_chirho = upper_chirho.verse_chirho;
                } else {
                    self.testament_chirho = 2;
                    let max_book_chirho = self.v11n_chirho.book_count_chirho(TestamentChirho::NewChirho);
                    self.book_chirho = (max_book_chirho - 1) as u8;
                    self.chapter_chirho = self.get_chapter_max_chirho();
                    self.verse_chirho = self.get_verse_max_chirho();
                }
            }
        }
        self.error_chirho = KeyErrorChirho::NoneChirho;
        self.refresh_text_chirho();
    }

    fn compare_chirho(&self, other_chirho: &dyn SwKeyChirho) -> Ordering {
        self.get_index_chirho().cmp(&other_chirho.get_index_chirho())
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_new_verse_key_chirho() {
        let key_chirho = VerseKeyChirho::new_chirho();
        assert_eq!(key_chirho.get_text_chirho(), "Genesis 1:1");
        assert_eq!(key_chirho.get_testament_chirho(), 1);
        assert_eq!(key_chirho.get_book_chirho(), 0);
        assert_eq!(key_chirho.get_chapter_chirho(), 1);
        assert_eq!(key_chirho.get_verse_chirho(), 1);
    }

    #[test]
    fn test_parse_simple_reference_chirho() {
        let key_chirho = VerseKeyChirho::from_str_chirho("John 3:16").unwrap();
        assert_eq!(key_chirho.get_book_name_chirho(), "John");
        assert_eq!(key_chirho.get_chapter_chirho(), 3);
        assert_eq!(key_chirho.get_verse_chirho(), 16);
        assert_eq!(key_chirho.get_testament_chirho(), 2);
    }

    #[test]
    fn test_parse_ot_reference_chirho() {
        let key_chirho = VerseKeyChirho::from_str_chirho("Genesis 1:1").unwrap();
        assert_eq!(key_chirho.get_book_name_chirho(), "Genesis");
        assert_eq!(key_chirho.get_chapter_chirho(), 1);
        assert_eq!(key_chirho.get_verse_chirho(), 1);
        assert_eq!(key_chirho.get_testament_chirho(), 1);
    }

    #[test]
    fn test_parse_numbered_book_chirho() {
        let key_chirho = VerseKeyChirho::from_str_chirho("1 John 1:1").unwrap();
        assert_eq!(key_chirho.get_book_name_chirho(), "I John");
        assert_eq!(key_chirho.get_chapter_chirho(), 1);
        assert_eq!(key_chirho.get_verse_chirho(), 1);
    }

    #[test]
    fn test_osis_ref_chirho() {
        let key_chirho = VerseKeyChirho::from_str_chirho("John 3:16").unwrap();
        assert_eq!(key_chirho.get_osis_ref_chirho(), "John.3.16");
    }

    #[test]
    fn test_increment_chirho() {
        let mut key_chirho = VerseKeyChirho::from_str_chirho("Genesis 1:31").unwrap();
        key_chirho.increment_chirho(1);
        assert_eq!(key_chirho.get_chapter_chirho(), 2);
        assert_eq!(key_chirho.get_verse_chirho(), 1);
    }

    #[test]
    fn test_decrement_chirho() {
        let mut key_chirho = VerseKeyChirho::from_str_chirho("Genesis 2:1").unwrap();
        key_chirho.decrement_chirho(1);
        assert_eq!(key_chirho.get_chapter_chirho(), 1);
        assert_eq!(key_chirho.get_verse_chirho(), 31); // Genesis 1 has 31 verses
    }

    #[test]
    fn test_set_position_chirho() {
        let mut key_chirho = VerseKeyChirho::new_chirho();

        key_chirho.set_position_chirho(PositionChirho::BottomChirho);
        assert_eq!(key_chirho.get_book_name_chirho(), "Revelation of John");
        assert_eq!(key_chirho.get_chapter_chirho(), 22);
        assert_eq!(key_chirho.get_verse_chirho(), 21);

        key_chirho.set_position_chirho(PositionChirho::TopChirho);
        assert_eq!(key_chirho.get_book_name_chirho(), "Genesis");
        assert_eq!(key_chirho.get_chapter_chirho(), 1);
        assert_eq!(key_chirho.get_verse_chirho(), 1);
    }

    #[test]
    fn test_verse_max_chirho() {
        let key_chirho = VerseKeyChirho::from_str_chirho("Psalms 119:1").unwrap();
        assert_eq!(key_chirho.get_verse_max_chirho(), 176);
    }
}
