// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Versification systems for Bible navigation.
//!
//! Different Bible translations use different verse numbering systems.
//! This module provides support for various versification schemes.
//!
//! ## Available Systems
//!
//! | System | Aliases | Description |
//! |--------|---------|-------------|
//! | KJV | - | King James Version (default, Protestant) |
//! | Catholic | - | Roman Catholic canon (includes deuterocanonicals) |
//! | LXX | Septuagint | Greek Septuagint ordering |
//! | Synodal | SynodalProt | Russian Synodal translation |
//! | Luther | German | German Lutheran Bible ordering |
//! | Vulgate | Vulg | Latin Vulgate versification |
//! | NRSV | - | New Revised Standard Version |
//! | Leningrad | MT, Hebrew | Leningrad Codex (Hebrew Bible, OT only) |
//! | Ethiopian | Ethiopic | Ethiopian Orthodox canon (largest canon) |
//!
//! ## Usage
//!
//! ```rust
//! use rsword_chirho::versification_chirho::{get_versification_chirho, kjv_chirho};
//! use rsword_chirho::versification_chirho::TestamentChirho;
//!
//! // Get a versification system
//! let kjv_chirho = get_versification_chirho("KJV").unwrap();
//! let catholic_chirho = get_versification_chirho("Catholic").unwrap();
//!
//! // Query book information
//! let (testament_chirho, book_idx_chirho) = kjv_chirho.lookup_book_chirho("Genesis").unwrap();
//! assert_eq!(testament_chirho, TestamentChirho::OldChirho);
//!
//! // Get verse counts
//! let genesis_chirho = kjv_chirho.get_book_chirho(testament_chirho, book_idx_chirho).unwrap();
//! assert_eq!(genesis_chirho.chapter_count_chirho, 50);
//! assert_eq!(genesis_chirho.max_verse_chirho(1), Some(31)); // Gen 1 has 31 verses
//! ```
//!
//! ## Book Order Differences
//!
//! Different versifications may have different book orders or include different books:
//!
//! - **KJV/Protestant**: 39 OT + 27 NT = 66 books
//! - **Catholic**: Includes Tobit, Judith, Wisdom, Sirach, Baruch, 1-2 Maccabees
//! - **LXX**: Greek ordering with additions (1 Esdras, Psalm 151, etc.)
//! - **Leningrad**: Hebrew Bible order (Torah, Nevi'im, Ketuvim), OT only
//! - **Ethiopian**: Largest canon with 1 Enoch, Jubilees, and more
//!
//! ## Manager
//!
//! Use [`VersificationManagerChirho`] to cache and reuse versification instances:
//!
//! ```rust
//! use rsword_chirho::versification_chirho::VersificationManagerChirho;
//!
//! let manager_chirho = VersificationManagerChirho::new_chirho();
//!
//! // List available systems
//! for name_chirho in manager_chirho.list_available_chirho() {
//!     println!("{}", name_chirho);
//! }
//!
//! // Get a versification (cached)
//! let kjv_chirho = manager_chirho.get_chirho("KJV").unwrap();
//! let kjv2_chirho = manager_chirho.get_chirho("KJV").unwrap();
//! // Both point to same Arc instance
//! ```

pub mod canons_chirho;
pub mod manager_chirho;

use std::collections::HashMap;
use std::sync::LazyLock;

pub use manager_chirho::VersificationManagerChirho;

/// Book information.
#[derive(Debug, Clone)]
pub struct BookInfoChirho {
    /// Full book name.
    pub name_chirho: String,
    /// OSIS abbreviation.
    pub osis_chirho: String,
    /// Short abbreviation.
    pub abbrev_chirho: String,
    /// Number of chapters.
    pub chapter_count_chirho: u8,
    /// Maximum verses per chapter.
    pub verse_max_chirho: Vec<u8>,
}

impl BookInfoChirho {
    /// Create a new book info.
    pub fn new_chirho(
        name_chirho: &str,
        osis_chirho: &str,
        abbrev_chirho: &str,
        verse_max_chirho: Vec<u8>,
    ) -> Self {
        Self {
            name_chirho: name_chirho.to_string(),
            osis_chirho: osis_chirho.to_string(),
            abbrev_chirho: abbrev_chirho.to_string(),
            chapter_count_chirho: verse_max_chirho.len() as u8,
            verse_max_chirho,
        }
    }

    /// Get the maximum verse for a chapter (1-indexed).
    pub fn max_verse_chirho(&self, chapter_chirho: u8) -> Option<u8> {
        if chapter_chirho == 0 || chapter_chirho > self.chapter_count_chirho {
            None
        } else {
            Some(self.verse_max_chirho[(chapter_chirho - 1) as usize])
        }
    }
}

/// Testament enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TestamentChirho {
    /// Old Testament.
    OldChirho = 1,
    /// New Testament.
    NewChirho = 2,
}

impl TestamentChirho {
    /// Get from numeric value.
    pub fn from_u8_chirho(value_chirho: u8) -> Option<Self> {
        match value_chirho {
            1 => Some(Self::OldChirho),
            2 => Some(Self::NewChirho),
            _ => None,
        }
    }
}

/// A versification system.
#[derive(Debug, Clone)]
pub struct VersificationChirho {
    /// Name of this versification system.
    pub name_chirho: String,
    /// Old Testament books.
    pub ot_books_chirho: Vec<BookInfoChirho>,
    /// New Testament books.
    pub nt_books_chirho: Vec<BookInfoChirho>,
    /// Precomputed offset table for OT.
    ot_offsets_chirho: Vec<u32>,
    /// Precomputed offset table for NT.
    nt_offsets_chirho: Vec<u32>,
    /// Total verses in OT.
    ot_total_chirho: u32,
    /// Total verses in NT.
    nt_total_chirho: u32,
    /// Book name to index mapping.
    book_lookup_chirho: HashMap<String, (TestamentChirho, usize)>,
}

impl VersificationChirho {
    /// Create a new versification system.
    pub fn new_chirho(
        name_chirho: &str,
        ot_books_chirho: Vec<BookInfoChirho>,
        nt_books_chirho: Vec<BookInfoChirho>,
    ) -> Self {
        let mut v11n_chirho = Self {
            name_chirho: name_chirho.to_string(),
            ot_books_chirho,
            nt_books_chirho,
            ot_offsets_chirho: Vec::new(),
            nt_offsets_chirho: Vec::new(),
            ot_total_chirho: 0,
            nt_total_chirho: 0,
            book_lookup_chirho: HashMap::new(),
        };
        v11n_chirho.build_offsets_chirho();
        v11n_chirho.build_lookup_chirho();
        v11n_chirho
    }

    fn build_offsets_chirho(&mut self) {
        // Build OT offsets
        // Start at 1 to account for testament intro at index 0
        let mut offset_chirho = 1u32;
        for book_chirho in &self.ot_books_chirho {
            self.ot_offsets_chirho.push(offset_chirho);
            for &max_v_chirho in &book_chirho.verse_max_chirho {
                offset_chirho += max_v_chirho as u32 + 1; // +1 for chapter intro
            }
            offset_chirho += 1; // +1 for book intro
        }
        self.ot_total_chirho = offset_chirho;

        // Build NT offsets
        // Start at 1 to account for testament intro at index 0
        offset_chirho = 1;
        for book_chirho in &self.nt_books_chirho {
            self.nt_offsets_chirho.push(offset_chirho);
            for &max_v_chirho in &book_chirho.verse_max_chirho {
                offset_chirho += max_v_chirho as u32 + 1; // +1 for chapter intro
            }
            offset_chirho += 1; // +1 for book intro
        }
        self.nt_total_chirho = offset_chirho;
    }

    fn build_lookup_chirho(&mut self) {
        // Collect book entries to avoid borrow issues
        let mut entries_chirho: Vec<(String, (TestamentChirho, usize))> = Vec::new();

        // Add OT books
        for (idx_chirho, book_chirho) in self.ot_books_chirho.iter().enumerate() {
            Self::collect_book_entries_chirho(
                &mut entries_chirho,
                TestamentChirho::OldChirho,
                idx_chirho,
                book_chirho,
            );
        }

        // Add NT books
        for (idx_chirho, book_chirho) in self.nt_books_chirho.iter().enumerate() {
            Self::collect_book_entries_chirho(
                &mut entries_chirho,
                TestamentChirho::NewChirho,
                idx_chirho,
                book_chirho,
            );
        }

        // Insert all entries
        for (key_chirho, value_chirho) in entries_chirho {
            self.book_lookup_chirho.insert(key_chirho, value_chirho);
        }
    }

    fn collect_book_entries_chirho(
        entries_chirho: &mut Vec<(String, (TestamentChirho, usize))>,
        testament_chirho: TestamentChirho,
        idx_chirho: usize,
        book_chirho: &BookInfoChirho,
    ) {
        let name_lower_chirho = book_chirho.name_chirho.to_lowercase();
        let osis_lower_chirho = book_chirho.osis_chirho.to_lowercase();
        let abbrev_lower_chirho = book_chirho.abbrev_chirho.to_lowercase();

        entries_chirho.push((name_lower_chirho.clone(), (testament_chirho, idx_chirho)));
        entries_chirho.push((osis_lower_chirho.clone(), (testament_chirho, idx_chirho)));
        entries_chirho.push((abbrev_lower_chirho.clone(), (testament_chirho, idx_chirho)));

        // Add numeric aliases for Roman numeral books
        // e.g., "I John" -> "1 John", "1john", "1jn"
        let roman_to_arabic_chirho = [
            ("i ", "1 "),
            ("ii ", "2 "),
            ("iii ", "3 "),
            ("iv ", "4 "),
        ];

        for (roman_chirho, arabic_chirho) in &roman_to_arabic_chirho {
            if let Some(suffix_chirho) = name_lower_chirho.strip_prefix(roman_chirho) {
                let alias_chirho = format!("{}{}", arabic_chirho, suffix_chirho);
                entries_chirho.push((alias_chirho.clone(), (testament_chirho, idx_chirho)));
                // Also add without space: "1john"
                let no_space_chirho = alias_chirho.replace(' ', "");
                entries_chirho.push((no_space_chirho, (testament_chirho, idx_chirho)));
            }
            if let Some(suffix_chirho) = osis_lower_chirho.strip_prefix(roman_chirho.trim()) {
                let alias_chirho = format!("{}{}", arabic_chirho.trim(), suffix_chirho);
                entries_chirho.push((alias_chirho, (testament_chirho, idx_chirho)));
            }
        }
    }

    /// Look up a book by name or abbreviation.
    pub fn lookup_book_chirho(&self, name_chirho: &str) -> Option<(TestamentChirho, usize)> {
        let normalized_chirho = name_chirho.to_lowercase();
        self.book_lookup_chirho.get(&normalized_chirho).copied()
    }

    /// Get a book by testament and index.
    pub fn get_book_chirho(&self, testament_chirho: TestamentChirho, index_chirho: usize) -> Option<&BookInfoChirho> {
        match testament_chirho {
            TestamentChirho::OldChirho => self.ot_books_chirho.get(index_chirho),
            TestamentChirho::NewChirho => self.nt_books_chirho.get(index_chirho),
        }
    }

    /// Get the number of books in a testament.
    pub fn book_count_chirho(&self, testament_chirho: TestamentChirho) -> usize {
        match testament_chirho {
            TestamentChirho::OldChirho => self.ot_books_chirho.len(),
            TestamentChirho::NewChirho => self.nt_books_chirho.len(),
        }
    }

    /// Get the total number of entries in a testament (including intros).
    pub fn testament_size_chirho(&self, testament_chirho: TestamentChirho) -> u32 {
        match testament_chirho {
            TestamentChirho::OldChirho => self.ot_total_chirho,
            TestamentChirho::NewChirho => self.nt_total_chirho,
        }
    }

    /// Calculate the index offset for a verse.
    pub fn calculate_index_chirho(
        &self,
        testament_chirho: TestamentChirho,
        book_chirho: usize,
        chapter_chirho: u8,
        verse_chirho: u8,
    ) -> Option<u32> {
        let book_info_chirho = self.get_book_chirho(testament_chirho, book_chirho)?;
        let offsets_chirho = match testament_chirho {
            TestamentChirho::OldChirho => &self.ot_offsets_chirho,
            TestamentChirho::NewChirho => &self.nt_offsets_chirho,
        };

        if book_chirho >= offsets_chirho.len() {
            return None;
        }

        let mut index_chirho = offsets_chirho[book_chirho];

        // Add book intro
        index_chirho += 1;

        // Add chapters
        if chapter_chirho > 0 {
            for ch_chirho in 0..(chapter_chirho as usize - 1).min(book_info_chirho.verse_max_chirho.len()) {
                index_chirho += book_info_chirho.verse_max_chirho[ch_chirho] as u32 + 1;
            }
            // Add chapter intro
            index_chirho += 1;
            // Add verse
            index_chirho += verse_chirho as u32;
        }

        Some(index_chirho)
    }
}

/// Get the KJV versification system.
pub fn kjv_chirho() -> &'static VersificationChirho {
    static KJV_CHIRHO: LazyLock<VersificationChirho> = LazyLock::new(|| {
        canons_chirho::kjv_chirho::create_kjv_chirho()
    });
    &KJV_CHIRHO
}

/// Get the Catholic versification system.
pub fn catholic_chirho() -> &'static VersificationChirho {
    static CATHOLIC_CHIRHO: LazyLock<VersificationChirho> = LazyLock::new(|| {
        canons_chirho::catholic_chirho::create_catholic_chirho()
    });
    &CATHOLIC_CHIRHO
}

/// Get the LXX (Septuagint) versification system.
pub fn lxx_chirho() -> &'static VersificationChirho {
    static LXX_CHIRHO: LazyLock<VersificationChirho> = LazyLock::new(|| {
        canons_chirho::lxx_chirho::create_lxx_chirho()
    });
    &LXX_CHIRHO
}

/// Get the Synodal versification system.
pub fn synodal_chirho() -> &'static VersificationChirho {
    static SYNODAL_CHIRHO: LazyLock<VersificationChirho> = LazyLock::new(|| {
        canons_chirho::synodal_chirho::create_synodal_chirho()
    });
    &SYNODAL_CHIRHO
}

/// Get the Luther versification system.
pub fn luther_chirho() -> &'static VersificationChirho {
    static LUTHER_CHIRHO: LazyLock<VersificationChirho> = LazyLock::new(|| {
        canons_chirho::luther_chirho::create_luther_chirho()
    });
    &LUTHER_CHIRHO
}

/// Get the Vulgate versification system.
pub fn vulgate_chirho() -> &'static VersificationChirho {
    static VULGATE_CHIRHO: LazyLock<VersificationChirho> = LazyLock::new(|| {
        canons_chirho::vulgate_chirho::create_vulgate_chirho()
    });
    &VULGATE_CHIRHO
}

/// Get the NRSV versification system.
pub fn nrsv_chirho() -> &'static VersificationChirho {
    static NRSV_CHIRHO: LazyLock<VersificationChirho> = LazyLock::new(|| {
        canons_chirho::nrsv_chirho::create_nrsv_chirho()
    });
    &NRSV_CHIRHO
}

/// Get the Leningrad (Hebrew/MT) versification system.
pub fn leningrad_chirho() -> &'static VersificationChirho {
    static LENINGRAD_CHIRHO: LazyLock<VersificationChirho> = LazyLock::new(|| {
        canons_chirho::leningrad_chirho::create_leningrad_chirho()
    });
    &LENINGRAD_CHIRHO
}

/// Get the Ethiopian Orthodox versification system.
pub fn ethiopian_chirho() -> &'static VersificationChirho {
    static ETHIOPIAN_CHIRHO: LazyLock<VersificationChirho> = LazyLock::new(|| {
        canons_chirho::ethiopian_chirho::create_ethiopian_chirho()
    });
    &ETHIOPIAN_CHIRHO
}

/// Get a versification system by name.
pub fn get_versification_chirho(name_chirho: &str) -> Option<&'static VersificationChirho> {
    match name_chirho.to_lowercase().as_str() {
        "kjv" => Some(kjv_chirho()),
        "catholic" => Some(catholic_chirho()),
        "lxx" | "septuagint" => Some(lxx_chirho()),
        "synodal" | "synodalprot" => Some(synodal_chirho()),
        "luther" | "german" => Some(luther_chirho()),
        "vulgate" | "vulg" => Some(vulgate_chirho()),
        "nrsv" => Some(nrsv_chirho()),
        "leningrad" | "mt" | "hebrew" => Some(leningrad_chirho()),
        "ethiopian" | "ethiopic" => Some(ethiopian_chirho()),
        _ => None,
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_kjv_versification_chirho() {
        let kjv_chirho = kjv_chirho();

        assert_eq!(kjv_chirho.name_chirho, "KJV");
        assert_eq!(kjv_chirho.ot_books_chirho.len(), 39);
        assert_eq!(kjv_chirho.nt_books_chirho.len(), 27);
    }

    #[test]
    fn test_book_lookup_chirho() {
        let kjv_chirho = kjv_chirho();

        let genesis_chirho = kjv_chirho.lookup_book_chirho("Genesis");
        assert!(genesis_chirho.is_some());
        let (testament_chirho, idx_chirho) = genesis_chirho.unwrap();
        assert_eq!(testament_chirho, TestamentChirho::OldChirho);
        assert_eq!(idx_chirho, 0);

        let john_chirho = kjv_chirho.lookup_book_chirho("John");
        assert!(john_chirho.is_some());
        let (testament_chirho, _) = john_chirho.unwrap();
        assert_eq!(testament_chirho, TestamentChirho::NewChirho);
    }

    #[test]
    fn test_max_verse_chirho() {
        let kjv_chirho = kjv_chirho();

        let genesis_chirho = kjv_chirho.get_book_chirho(TestamentChirho::OldChirho, 0).unwrap();
        assert_eq!(genesis_chirho.name_chirho, "Genesis");
        assert_eq!(genesis_chirho.chapter_count_chirho, 50);
        assert_eq!(genesis_chirho.max_verse_chirho(1), Some(31)); // Genesis 1 has 31 verses
    }
}
