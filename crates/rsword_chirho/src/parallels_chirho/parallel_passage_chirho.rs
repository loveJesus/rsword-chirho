// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Parallel passage identification and lookup.
//!
//! Provides functionality to find parallel passages such as:
//! - Synoptic Gospel parallels
//! - Chronicles/Kings parallels
//! - Related prophecy fulfillments

use std::collections::HashMap;
use std::sync::LazyLock;

/// Type of parallel relationship between passages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParallelTypeChirho {
    /// Synoptic Gospel parallel (same event in multiple Gospels).
    SynopticChirho,
    /// Old Testament parallel history (Chronicles vs Kings).
    HistoryChirho,
    /// Prophecy and fulfillment.
    ProphecyChirho,
    /// Quotation of an earlier passage.
    QuotationChirho,
    /// Thematic parallel (similar theme/topic).
    ThematicChirho,
}

/// A single parallel passage reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParallelPassageChirho {
    /// OSIS reference (e.g., "Matt.3.13-17").
    pub osis_ref_chirho: String,
    /// Book abbreviation.
    pub book_chirho: String,
    /// Chapter number.
    pub chapter_chirho: u8,
    /// Starting verse.
    pub verse_start_chirho: u8,
    /// Ending verse (same as start if single verse).
    pub verse_end_chirho: u8,
    /// Type of parallel.
    pub parallel_type_chirho: ParallelTypeChirho,
}

impl ParallelPassageChirho {
    /// Create a new parallel passage reference.
    pub fn new_chirho(
        book_chirho: &str,
        chapter_chirho: u8,
        verse_start_chirho: u8,
        verse_end_chirho: u8,
        parallel_type_chirho: ParallelTypeChirho,
    ) -> Self {
        let osis_ref_chirho = if verse_start_chirho == verse_end_chirho {
            format!("{}.{}.{}", book_chirho, chapter_chirho, verse_start_chirho)
        } else {
            format!(
                "{}.{}.{}-{}",
                book_chirho, chapter_chirho, verse_start_chirho, verse_end_chirho
            )
        };

        Self {
            osis_ref_chirho,
            book_chirho: book_chirho.to_string(),
            chapter_chirho,
            verse_start_chirho,
            verse_end_chirho,
            parallel_type_chirho,
        }
    }

    /// Create a single-verse parallel.
    pub fn single_chirho(book_chirho: &str, chapter_chirho: u8, verse_chirho: u8, parallel_type_chirho: ParallelTypeChirho) -> Self {
        Self::new_chirho(book_chirho, chapter_chirho, verse_chirho, verse_chirho, parallel_type_chirho)
    }

    /// Get a human-readable reference.
    pub fn display_ref_chirho(&self) -> String {
        if self.verse_start_chirho == self.verse_end_chirho {
            format!("{} {}:{}", self.book_chirho, self.chapter_chirho, self.verse_start_chirho)
        } else {
            format!(
                "{} {}:{}-{}",
                self.book_chirho, self.chapter_chirho, self.verse_start_chirho, self.verse_end_chirho
            )
        }
    }
}

/// A set of parallel passages (e.g., all Synoptic accounts of Jesus' baptism).
#[derive(Debug, Clone)]
pub struct ParallelSetChirho {
    /// Description of this parallel set.
    pub description_chirho: String,
    /// The passages in this parallel set.
    pub passages_chirho: Vec<ParallelPassageChirho>,
    /// The type of parallel.
    pub parallel_type_chirho: ParallelTypeChirho,
}

impl ParallelSetChirho {
    /// Create a new parallel set.
    pub fn new_chirho(
        description_chirho: &str,
        passages_chirho: Vec<ParallelPassageChirho>,
        parallel_type_chirho: ParallelTypeChirho,
    ) -> Self {
        Self {
            description_chirho: description_chirho.to_string(),
            passages_chirho,
            parallel_type_chirho,
        }
    }
}

/// Manager for parallel passage lookups.
pub struct ParallelManagerChirho {
    /// Map from OSIS reference to parallel set indices.
    ref_to_sets_chirho: HashMap<String, Vec<usize>>,
    /// All parallel sets.
    parallel_sets_chirho: Vec<ParallelSetChirho>,
}

/// Static synoptic parallel data.
static SYNOPTIC_PARALLELS_CHIRHO: LazyLock<Vec<ParallelSetChirho>> = LazyLock::new(|| {
    let synoptic_chirho = ParallelTypeChirho::SynopticChirho;

    vec![
        // Baptism of Jesus
        ParallelSetChirho::new_chirho(
            "Baptism of Jesus",
            vec![
                ParallelPassageChirho::new_chirho("Matt", 3, 13, 17, synoptic_chirho),
                ParallelPassageChirho::new_chirho("Mark", 1, 9, 11, synoptic_chirho),
                ParallelPassageChirho::new_chirho("Luke", 3, 21, 22, synoptic_chirho),
            ],
            synoptic_chirho,
        ),
        // Temptation of Jesus
        ParallelSetChirho::new_chirho(
            "Temptation of Jesus",
            vec![
                ParallelPassageChirho::new_chirho("Matt", 4, 1, 11, synoptic_chirho),
                ParallelPassageChirho::new_chirho("Mark", 1, 12, 13, synoptic_chirho),
                ParallelPassageChirho::new_chirho("Luke", 4, 1, 13, synoptic_chirho),
            ],
            synoptic_chirho,
        ),
        // Feeding of the 5000
        ParallelSetChirho::new_chirho(
            "Feeding of the 5000",
            vec![
                ParallelPassageChirho::new_chirho("Matt", 14, 13, 21, synoptic_chirho),
                ParallelPassageChirho::new_chirho("Mark", 6, 30, 44, synoptic_chirho),
                ParallelPassageChirho::new_chirho("Luke", 9, 10, 17, synoptic_chirho),
                ParallelPassageChirho::new_chirho("John", 6, 1, 14, synoptic_chirho),
            ],
            synoptic_chirho,
        ),
        // Peter's Confession
        ParallelSetChirho::new_chirho(
            "Peter's Confession",
            vec![
                ParallelPassageChirho::new_chirho("Matt", 16, 13, 20, synoptic_chirho),
                ParallelPassageChirho::new_chirho("Mark", 8, 27, 30, synoptic_chirho),
                ParallelPassageChirho::new_chirho("Luke", 9, 18, 21, synoptic_chirho),
            ],
            synoptic_chirho,
        ),
        // Transfiguration
        ParallelSetChirho::new_chirho(
            "Transfiguration",
            vec![
                ParallelPassageChirho::new_chirho("Matt", 17, 1, 8, synoptic_chirho),
                ParallelPassageChirho::new_chirho("Mark", 9, 2, 8, synoptic_chirho),
                ParallelPassageChirho::new_chirho("Luke", 9, 28, 36, synoptic_chirho),
            ],
            synoptic_chirho,
        ),
        // Triumphal Entry
        ParallelSetChirho::new_chirho(
            "Triumphal Entry",
            vec![
                ParallelPassageChirho::new_chirho("Matt", 21, 1, 11, synoptic_chirho),
                ParallelPassageChirho::new_chirho("Mark", 11, 1, 11, synoptic_chirho),
                ParallelPassageChirho::new_chirho("Luke", 19, 28, 44, synoptic_chirho),
                ParallelPassageChirho::new_chirho("John", 12, 12, 19, synoptic_chirho),
            ],
            synoptic_chirho,
        ),
        // Last Supper
        ParallelSetChirho::new_chirho(
            "Last Supper",
            vec![
                ParallelPassageChirho::new_chirho("Matt", 26, 17, 30, synoptic_chirho),
                ParallelPassageChirho::new_chirho("Mark", 14, 12, 26, synoptic_chirho),
                ParallelPassageChirho::new_chirho("Luke", 22, 7, 23, synoptic_chirho),
            ],
            synoptic_chirho,
        ),
        // Crucifixion
        ParallelSetChirho::new_chirho(
            "Crucifixion",
            vec![
                ParallelPassageChirho::new_chirho("Matt", 27, 32, 56, synoptic_chirho),
                ParallelPassageChirho::new_chirho("Mark", 15, 21, 41, synoptic_chirho),
                ParallelPassageChirho::new_chirho("Luke", 23, 26, 49, synoptic_chirho),
                ParallelPassageChirho::new_chirho("John", 19, 17, 37, synoptic_chirho),
            ],
            synoptic_chirho,
        ),
        // Resurrection
        ParallelSetChirho::new_chirho(
            "Resurrection",
            vec![
                ParallelPassageChirho::new_chirho("Matt", 28, 1, 10, synoptic_chirho),
                ParallelPassageChirho::new_chirho("Mark", 16, 1, 8, synoptic_chirho),
                ParallelPassageChirho::new_chirho("Luke", 24, 1, 12, synoptic_chirho),
                ParallelPassageChirho::new_chirho("John", 20, 1, 10, synoptic_chirho),
            ],
            synoptic_chirho,
        ),
        // Lord's Prayer
        ParallelSetChirho::new_chirho(
            "Lord's Prayer",
            vec![
                ParallelPassageChirho::new_chirho("Matt", 6, 9, 13, synoptic_chirho),
                ParallelPassageChirho::new_chirho("Luke", 11, 2, 4, synoptic_chirho),
            ],
            synoptic_chirho,
        ),
        // Beatitudes
        ParallelSetChirho::new_chirho(
            "Beatitudes",
            vec![
                ParallelPassageChirho::new_chirho("Matt", 5, 3, 12, synoptic_chirho),
                ParallelPassageChirho::new_chirho("Luke", 6, 20, 26, synoptic_chirho),
            ],
            synoptic_chirho,
        ),
    ]
});

/// Static OT history parallels.
static HISTORY_PARALLELS_CHIRHO: LazyLock<Vec<ParallelSetChirho>> = LazyLock::new(|| {
    let history_chirho = ParallelTypeChirho::HistoryChirho;

    vec![
        // David becomes king
        ParallelSetChirho::new_chirho(
            "David becomes king of Israel",
            vec![
                ParallelPassageChirho::new_chirho("2Sam", 5, 1, 5, history_chirho),
                ParallelPassageChirho::new_chirho("1Chr", 11, 1, 3, history_chirho),
            ],
            history_chirho,
        ),
        // David captures Jerusalem
        ParallelSetChirho::new_chirho(
            "David captures Jerusalem",
            vec![
                ParallelPassageChirho::new_chirho("2Sam", 5, 6, 10, history_chirho),
                ParallelPassageChirho::new_chirho("1Chr", 11, 4, 9, history_chirho),
            ],
            history_chirho,
        ),
        // Ark brought to Jerusalem
        ParallelSetChirho::new_chirho(
            "Ark brought to Jerusalem",
            vec![
                ParallelPassageChirho::new_chirho("2Sam", 6, 1, 19, history_chirho),
                ParallelPassageChirho::new_chirho("1Chr", 13, 1, 14, history_chirho),
            ],
            history_chirho,
        ),
        // Solomon's reign
        ParallelSetChirho::new_chirho(
            "Solomon's reign begins",
            vec![
                ParallelPassageChirho::new_chirho("1Kgs", 1, 28, 53, history_chirho),
                ParallelPassageChirho::new_chirho("1Chr", 29, 21, 25, history_chirho),
            ],
            history_chirho,
        ),
        // Temple dedication
        ParallelSetChirho::new_chirho(
            "Solomon's temple dedication",
            vec![
                ParallelPassageChirho::new_chirho("1Kgs", 8, 1, 66, history_chirho),
                ParallelPassageChirho::new_chirho("2Chr", 5, 1, 14, history_chirho),
            ],
            history_chirho,
        ),
        // Hezekiah's illness
        ParallelSetChirho::new_chirho(
            "Hezekiah's illness and recovery",
            vec![
                ParallelPassageChirho::new_chirho("2Kgs", 20, 1, 11, history_chirho),
                ParallelPassageChirho::new_chirho("2Chr", 32, 24, 26, history_chirho),
                ParallelPassageChirho::new_chirho("Isa", 38, 1, 8, history_chirho),
            ],
            history_chirho,
        ),
    ]
});

impl ParallelManagerChirho {
    /// Create a new parallel manager with built-in data.
    pub fn new_chirho() -> Self {
        let mut manager_chirho = Self {
            ref_to_sets_chirho: HashMap::new(),
            parallel_sets_chirho: Vec::new(),
        };

        // Load synoptic parallels
        for set_chirho in SYNOPTIC_PARALLELS_CHIRHO.iter() {
            manager_chirho.add_parallel_set_chirho(set_chirho.clone());
        }

        // Load history parallels
        for set_chirho in HISTORY_PARALLELS_CHIRHO.iter() {
            manager_chirho.add_parallel_set_chirho(set_chirho.clone());
        }

        manager_chirho
    }

    /// Add a parallel set.
    pub fn add_parallel_set_chirho(&mut self, set_chirho: ParallelSetChirho) {
        let set_idx_chirho = self.parallel_sets_chirho.len();

        // Index all passages in this set
        for passage_chirho in &set_chirho.passages_chirho {
            // Index by exact reference
            self.ref_to_sets_chirho
                .entry(passage_chirho.osis_ref_chirho.clone())
                .or_default()
                .push(set_idx_chirho);

            // Also index individual verses for range lookups
            for verse_chirho in passage_chirho.verse_start_chirho..=passage_chirho.verse_end_chirho {
                let single_ref_chirho = format!(
                    "{}.{}.{}",
                    passage_chirho.book_chirho, passage_chirho.chapter_chirho, verse_chirho
                );
                self.ref_to_sets_chirho
                    .entry(single_ref_chirho)
                    .or_default()
                    .push(set_idx_chirho);
            }
        }

        self.parallel_sets_chirho.push(set_chirho);
    }

    /// Get parallel passages for a given reference.
    pub fn get_parallels_chirho(&self, osis_ref_chirho: &str) -> Vec<&ParallelSetChirho> {
        let normalized_chirho = osis_ref_chirho.replace(' ', ".");

        if let Some(indices_chirho) = self.ref_to_sets_chirho.get(&normalized_chirho) {
            indices_chirho
                .iter()
                .filter_map(|&idx_chirho| self.parallel_sets_chirho.get(idx_chirho))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get all parallel sets of a specific type.
    pub fn get_by_type_chirho(&self, parallel_type_chirho: ParallelTypeChirho) -> Vec<&ParallelSetChirho> {
        self.parallel_sets_chirho
            .iter()
            .filter(|set_chirho| set_chirho.parallel_type_chirho == parallel_type_chirho)
            .collect()
    }

    /// Get total count of parallel sets.
    pub fn count_chirho(&self) -> usize {
        self.parallel_sets_chirho.len()
    }
}

impl Default for ParallelManagerChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_parallel_passage_creation_chirho() {
        let passage_chirho = ParallelPassageChirho::new_chirho(
            "Matt",
            3,
            13,
            17,
            ParallelTypeChirho::SynopticChirho,
        );

        assert_eq!(passage_chirho.osis_ref_chirho, "Matt.3.13-17");
        assert_eq!(passage_chirho.display_ref_chirho(), "Matt 3:13-17");
    }

    #[test]
    fn test_single_verse_parallel_chirho() {
        let passage_chirho = ParallelPassageChirho::single_chirho(
            "John",
            3,
            16,
            ParallelTypeChirho::QuotationChirho,
        );

        assert_eq!(passage_chirho.osis_ref_chirho, "John.3.16");
        assert_eq!(passage_chirho.display_ref_chirho(), "John 3:16");
    }

    #[test]
    fn test_manager_creation_chirho() {
        let manager_chirho = ParallelManagerChirho::new_chirho();

        // Should have both synoptic and history parallels
        assert!(manager_chirho.count_chirho() > 0);

        let synoptics_chirho = manager_chirho.get_by_type_chirho(ParallelTypeChirho::SynopticChirho);
        assert!(!synoptics_chirho.is_empty());

        let histories_chirho = manager_chirho.get_by_type_chirho(ParallelTypeChirho::HistoryChirho);
        assert!(!histories_chirho.is_empty());
    }

    #[test]
    fn test_baptism_parallel_lookup_chirho() {
        let manager_chirho = ParallelManagerChirho::new_chirho();

        // Look up Matthew's baptism account
        let parallels_chirho = manager_chirho.get_parallels_chirho("Matt.3.16");

        assert!(!parallels_chirho.is_empty());

        let baptism_set_chirho = parallels_chirho
            .iter()
            .find(|s_chirho| s_chirho.description_chirho == "Baptism of Jesus");
        assert!(baptism_set_chirho.is_some());

        // Should include Mark and Luke
        let set_chirho = baptism_set_chirho.unwrap();
        assert!(set_chirho.passages_chirho.iter().any(|p_chirho| p_chirho.book_chirho == "Mark"));
        assert!(set_chirho.passages_chirho.iter().any(|p_chirho| p_chirho.book_chirho == "Luke"));
    }

    #[test]
    fn test_feeding_5000_parallel_chirho() {
        let manager_chirho = ParallelManagerChirho::new_chirho();

        // Look up John's account of feeding 5000
        let parallels_chirho = manager_chirho.get_parallels_chirho("John.6.5");

        assert!(!parallels_chirho.is_empty());

        let feeding_set_chirho = parallels_chirho
            .iter()
            .find(|s_chirho| s_chirho.description_chirho == "Feeding of the 5000");
        assert!(feeding_set_chirho.is_some());

        // Should include all four gospels
        let set_chirho = feeding_set_chirho.unwrap();
        assert_eq!(set_chirho.passages_chirho.len(), 4);
    }

    #[test]
    fn test_history_parallel_chirho() {
        let manager_chirho = ParallelManagerChirho::new_chirho();

        // Look up David captures Jerusalem in 2 Samuel
        let parallels_chirho = manager_chirho.get_parallels_chirho("2Sam.5.6");

        assert!(!parallels_chirho.is_empty());

        // Should find Chronicles parallel
        let jerusalem_set_chirho = parallels_chirho
            .iter()
            .find(|s_chirho| s_chirho.description_chirho.contains("Jerusalem"));
        assert!(jerusalem_set_chirho.is_some());
    }

    #[test]
    fn test_no_parallels_chirho() {
        let manager_chirho = ParallelManagerChirho::new_chirho();

        // A verse with no known parallels
        let parallels_chirho = manager_chirho.get_parallels_chirho("Obad.1.1");

        assert!(parallels_chirho.is_empty());
    }
}
