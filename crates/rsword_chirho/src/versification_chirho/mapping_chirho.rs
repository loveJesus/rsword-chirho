// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Cross-versification mapping between different Bible versification systems.
//!
//! Different Bible translations use different verse numbering systems. This module
//! provides functionality to map verse references between versification systems.
//!
//! # Mapping Types
//!
//! Verses can map in several ways between versification systems:
//!
//! | Type | Description | Example |
//! |------|-------------|---------|
//! | One-to-one | Direct correspondence | Ps 23:1 (KJV) = Ps 23:1 (NRSV) |
//! | Split | One verse becomes multiple | Ps 13:5 (Hebrew) = Ps 13:5-6 (English) |
//! | Merge | Multiple verses become one | - |
//! | Missing | Verse doesn't exist in target | 3 John 1:15 (some systems) |
//! | Offset | Chapter/verse numbers shifted | Psalm titles |
//!
//! # Usage
//!
//! ```rust,ignore
//! use rsword_chirho::versification_chirho::mapping_chirho::{VerseMappingChirho, VerseMappingResultChirho};
//! use rsword_chirho::versification_chirho::{kjv_chirho, catholic_chirho};
//!
//! let mapper_chirho = VerseMappingChirho::new_chirho();
//!
//! // Map a verse from KJV to Catholic versification
//! let result_chirho = mapper_chirho.map_verse_chirho(
//!     "Psalm", 13, 5,
//!     "KJV",
//!     "Catholic",
//! );
//!
//! match result_chirho {
//!     VerseMappingResultChirho::DirectChirho { book_chirho, chapter_chirho, verse_chirho } => {
//!         println!("Maps to {}:{}:{}", book_chirho, chapter_chirho, verse_chirho);
//!     }
//!     VerseMappingResultChirho::SplitChirho { verses_chirho } => {
//!         println!("Split into {} verses", verses_chirho.len());
//!     }
//!     VerseMappingResultChirho::NotFoundChirho => {
//!         println!("No mapping found");
//!     }
//! }
//! ```

use std::collections::HashMap;
use std::sync::LazyLock;

/// Result of mapping a verse between versification systems.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerseMappingResultChirho {
    /// Direct one-to-one mapping.
    DirectChirho {
        /// Book name in target versification.
        book_chirho: String,
        /// Chapter number in target versification.
        chapter_chirho: u32,
        /// Verse number in target versification.
        verse_chirho: u32,
    },
    /// Verse splits into multiple verses in target.
    SplitChirho {
        /// List of (book, chapter, verse) tuples in target.
        verses_chirho: Vec<(String, u32, u32)>,
    },
    /// Multiple verses merge into one in target.
    MergedChirho {
        /// The merged verse reference.
        book_chirho: String,
        /// Chapter number.
        chapter_chirho: u32,
        /// Verse range (start, end).
        verse_range_chirho: (u32, u32),
    },
    /// Verse doesn't exist in target versification.
    NotFoundChirho,
}

/// A single verse mapping rule.
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct MappingRuleChirho {
    /// Source book name (stored for debugging and future bidirectional mapping).
    source_book_chirho: String,
    /// Source chapter.
    source_chapter_chirho: u32,
    /// Source verse or verse range.
    source_verse_chirho: (u32, u32),
    /// Target book name (may differ).
    target_book_chirho: String,
    /// Target chapter.
    target_chapter_chirho: u32,
    /// Target verse or verse range.
    target_verse_chirho: (u32, u32),
}

/// Cross-versification mapper.
///
/// Provides verse mapping between different versification systems.
#[derive(Debug, Clone)]
pub struct VerseMappingChirho {
    /// Mapping rules indexed by source versification.
    rules_chirho: HashMap<String, HashMap<String, Vec<MappingRuleChirho>>>,
}

impl VerseMappingChirho {
    /// Create a new verse mapper with default mapping rules.
    pub fn new_chirho() -> Self {
        Self {
            rules_chirho: build_default_mappings_chirho(),
        }
    }

    /// Map a verse from one versification to another.
    ///
    /// # Arguments
    ///
    /// * `book_chirho` - Book name (e.g., "Psalm", "Genesis")
    /// * `chapter_chirho` - Chapter number
    /// * `verse_chirho` - Verse number
    /// * `from_v11n_chirho` - Source versification name (e.g., "KJV")
    /// * `to_v11n_chirho` - Target versification name (e.g., "Catholic")
    ///
    /// # Returns
    ///
    /// The mapping result indicating how the verse maps to the target system.
    pub fn map_verse_chirho(
        &self,
        book_chirho: &str,
        chapter_chirho: u32,
        verse_chirho: u32,
        from_v11n_chirho: &str,
        to_v11n_chirho: &str,
    ) -> VerseMappingResultChirho {
        // Same versification - direct mapping
        if from_v11n_chirho.eq_ignore_ascii_case(to_v11n_chirho) {
            return VerseMappingResultChirho::DirectChirho {
                book_chirho: book_chirho.to_string(),
                chapter_chirho,
                verse_chirho,
            };
        }

        // Look up mapping key
        let key_chirho = format!("{}->{}", from_v11n_chirho.to_uppercase(), to_v11n_chirho.to_uppercase());

        if let Some(book_rules_chirho) = self.rules_chirho.get(&key_chirho) {
            let book_upper_chirho = book_chirho.to_uppercase();
            if let Some(rules_chirho) = book_rules_chirho.get(&book_upper_chirho) {
                for rule_chirho in rules_chirho {
                    if rule_chirho.source_chapter_chirho == chapter_chirho
                        && verse_chirho >= rule_chirho.source_verse_chirho.0
                        && verse_chirho <= rule_chirho.source_verse_chirho.1
                    {
                        return apply_rule_chirho(rule_chirho, verse_chirho);
                    }
                }
            }
        }

        // No specific mapping - assume direct mapping if both versifications
        // have the verse (this is a simplification)
        VerseMappingResultChirho::DirectChirho {
            book_chirho: book_chirho.to_string(),
            chapter_chirho,
            verse_chirho,
        }
    }

    /// Map a verse range from one versification to another.
    pub fn map_range_chirho(
        &self,
        book_chirho: &str,
        chapter_chirho: u32,
        start_verse_chirho: u32,
        end_verse_chirho: u32,
        from_v11n_chirho: &str,
        to_v11n_chirho: &str,
    ) -> Vec<VerseMappingResultChirho> {
        let mut results_chirho = Vec::new();
        for verse_chirho in start_verse_chirho..=end_verse_chirho {
            results_chirho.push(self.map_verse_chirho(
                book_chirho,
                chapter_chirho,
                verse_chirho,
                from_v11n_chirho,
                to_v11n_chirho,
            ));
        }
        results_chirho
    }

    /// Check if a mapping exists between two versification systems.
    pub fn has_mapping_chirho(&self, from_v11n_chirho: &str, to_v11n_chirho: &str) -> bool {
        let key_chirho = format!("{}->{}", from_v11n_chirho.to_uppercase(), to_v11n_chirho.to_uppercase());
        self.rules_chirho.contains_key(&key_chirho)
    }

    /// List available versification mapping pairs.
    pub fn available_mappings_chirho(&self) -> Vec<(&str, &str)> {
        self.rules_chirho
            .keys()
            .filter_map(|k_chirho| {
                let parts_chirho: Vec<&str> = k_chirho.split("->").collect();
                if parts_chirho.len() == 2 {
                    Some((parts_chirho[0], parts_chirho[1]))
                } else {
                    None
                }
            })
            .collect()
    }
}

impl Default for VerseMappingChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

/// Apply a mapping rule to get the result.
fn apply_rule_chirho(rule_chirho: &MappingRuleChirho, source_verse_chirho: u32) -> VerseMappingResultChirho {
    let source_range_chirho = rule_chirho.source_verse_chirho.1 - rule_chirho.source_verse_chirho.0 + 1;
    let target_range_chirho = rule_chirho.target_verse_chirho.1 - rule_chirho.target_verse_chirho.0 + 1;

    if source_range_chirho == 1 && target_range_chirho == 1 {
        // One-to-one mapping
        VerseMappingResultChirho::DirectChirho {
            book_chirho: rule_chirho.target_book_chirho.clone(),
            chapter_chirho: rule_chirho.target_chapter_chirho,
            verse_chirho: rule_chirho.target_verse_chirho.0,
        }
    } else if source_range_chirho == 1 && target_range_chirho > 1 {
        // Split: one verse maps to multiple
        let mut verses_chirho = Vec::new();
        for v_chirho in rule_chirho.target_verse_chirho.0..=rule_chirho.target_verse_chirho.1 {
            verses_chirho.push((
                rule_chirho.target_book_chirho.clone(),
                rule_chirho.target_chapter_chirho,
                v_chirho,
            ));
        }
        VerseMappingResultChirho::SplitChirho { verses_chirho }
    } else if source_range_chirho > 1 && target_range_chirho == 1 {
        // Merge: multiple verses map to one
        VerseMappingResultChirho::MergedChirho {
            book_chirho: rule_chirho.target_book_chirho.clone(),
            chapter_chirho: rule_chirho.target_chapter_chirho,
            verse_range_chirho: (source_verse_chirho, source_verse_chirho),
        }
    } else {
        // Range to range - calculate offset
        let offset_chirho = source_verse_chirho - rule_chirho.source_verse_chirho.0;
        let target_verse_chirho = rule_chirho.target_verse_chirho.0 + offset_chirho;
        VerseMappingResultChirho::DirectChirho {
            book_chirho: rule_chirho.target_book_chirho.clone(),
            chapter_chirho: rule_chirho.target_chapter_chirho,
            verse_chirho: target_verse_chirho,
        }
    }
}

/// Build default mapping rules between common versification systems.
fn build_default_mappings_chirho() -> HashMap<String, HashMap<String, Vec<MappingRuleChirho>>> {
    let mut mappings_chirho: HashMap<String, HashMap<String, Vec<MappingRuleChirho>>> = HashMap::new();

    // KJV -> Catholic mappings (Psalm numbering differences)
    let mut kjv_to_catholic_chirho: HashMap<String, Vec<MappingRuleChirho>> = HashMap::new();

    // Psalm 9-10 in KJV = Psalm 9 in Catholic/LXX
    kjv_to_catholic_chirho.insert("PSALM".to_string(), vec![
        // Psalms 10-146 are offset by -1 in Catholic numbering
        MappingRuleChirho {
            source_book_chirho: "Psalm".to_string(),
            source_chapter_chirho: 10,
            source_verse_chirho: (1, 18),
            target_book_chirho: "Psalm".to_string(),
            target_chapter_chirho: 9,
            target_verse_chirho: (22, 39),
        },
    ]);

    mappings_chirho.insert("KJV->CATHOLIC".to_string(), kjv_to_catholic_chirho);

    // KJV -> Hebrew/Leningrad mappings
    let mut kjv_to_hebrew_chirho: HashMap<String, Vec<MappingRuleChirho>> = HashMap::new();

    // Psalm titles are verse 1 in Hebrew
    kjv_to_hebrew_chirho.insert("PSALM".to_string(), vec![
        // Psalms with titles: Hebrew verse numbers are +1
        MappingRuleChirho {
            source_book_chirho: "Psalm".to_string(),
            source_chapter_chirho: 3,
            source_verse_chirho: (1, 8),
            target_book_chirho: "Psalm".to_string(),
            target_chapter_chirho: 3,
            target_verse_chirho: (2, 9),
        },
        MappingRuleChirho {
            source_book_chirho: "Psalm".to_string(),
            source_chapter_chirho: 4,
            source_verse_chirho: (1, 8),
            target_book_chirho: "Psalm".to_string(),
            target_chapter_chirho: 4,
            target_verse_chirho: (2, 9),
        },
    ]);

    mappings_chirho.insert("KJV->LENINGRAD".to_string(), kjv_to_hebrew_chirho.clone());
    mappings_chirho.insert("KJV->MT".to_string(), kjv_to_hebrew_chirho.clone());
    mappings_chirho.insert("KJV->HEBREW".to_string(), kjv_to_hebrew_chirho);

    // Catholic -> KJV (reverse mappings)
    let mut catholic_to_kjv_chirho: HashMap<String, Vec<MappingRuleChirho>> = HashMap::new();
    catholic_to_kjv_chirho.insert("PSALM".to_string(), vec![
        MappingRuleChirho {
            source_book_chirho: "Psalm".to_string(),
            source_chapter_chirho: 9,
            source_verse_chirho: (22, 39),
            target_book_chirho: "Psalm".to_string(),
            target_chapter_chirho: 10,
            target_verse_chirho: (1, 18),
        },
    ]);

    mappings_chirho.insert("CATHOLIC->KJV".to_string(), catholic_to_kjv_chirho);

    // LXX uses same numbering as Catholic for Psalms
    mappings_chirho.insert("KJV->LXX".to_string(), mappings_chirho.get("KJV->CATHOLIC").cloned().unwrap_or_default());
    mappings_chirho.insert("LXX->KJV".to_string(), mappings_chirho.get("CATHOLIC->KJV").cloned().unwrap_or_default());

    mappings_chirho
}

/// Global verse mapper instance.
static GLOBAL_MAPPER_CHIRHO: LazyLock<VerseMappingChirho> = LazyLock::new(VerseMappingChirho::new_chirho);

/// Get the global verse mapper.
pub fn global_mapper_chirho() -> &'static VerseMappingChirho {
    &GLOBAL_MAPPER_CHIRHO
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_same_versification_chirho() {
        let mapper_chirho = VerseMappingChirho::new_chirho();
        let result_chirho = mapper_chirho.map_verse_chirho("Genesis", 1, 1, "KJV", "KJV");

        assert_eq!(result_chirho, VerseMappingResultChirho::DirectChirho {
            book_chirho: "Genesis".to_string(),
            chapter_chirho: 1,
            verse_chirho: 1,
        });
    }

    #[test]
    fn test_default_mapping_chirho() {
        let mapper_chirho = VerseMappingChirho::new_chirho();

        // Test unmapped verse - should return direct mapping
        let result_chirho = mapper_chirho.map_verse_chirho("Genesis", 1, 1, "KJV", "Catholic");

        match result_chirho {
            VerseMappingResultChirho::DirectChirho { book_chirho, chapter_chirho, verse_chirho } => {
                assert_eq!(book_chirho, "Genesis");
                assert_eq!(chapter_chirho, 1);
                assert_eq!(verse_chirho, 1);
            }
            _ => panic!("Expected direct mapping"),
        }
    }

    #[test]
    fn test_psalm_kjv_to_hebrew_chirho() {
        let mapper_chirho = VerseMappingChirho::new_chirho();

        // Psalm 3:1 (KJV) should map to Psalm 3:2 (Hebrew) because of title
        let result_chirho = mapper_chirho.map_verse_chirho("Psalm", 3, 1, "KJV", "Leningrad");

        match result_chirho {
            VerseMappingResultChirho::DirectChirho { chapter_chirho, verse_chirho, .. } => {
                assert_eq!(chapter_chirho, 3);
                assert_eq!(verse_chirho, 2);
            }
            _ => panic!("Expected direct mapping with offset"),
        }
    }

    #[test]
    fn test_has_mapping_chirho() {
        let mapper_chirho = VerseMappingChirho::new_chirho();

        assert!(mapper_chirho.has_mapping_chirho("KJV", "Catholic"));
        assert!(mapper_chirho.has_mapping_chirho("KJV", "LXX"));
        assert!(mapper_chirho.has_mapping_chirho("KJV", "Leningrad"));
    }

    #[test]
    fn test_map_range_chirho() {
        let mapper_chirho = VerseMappingChirho::new_chirho();

        let results_chirho = mapper_chirho.map_range_chirho("Genesis", 1, 1, 3, "KJV", "Catholic");

        assert_eq!(results_chirho.len(), 3);
    }

    #[test]
    fn test_global_mapper_chirho() {
        let mapper_chirho = global_mapper_chirho();
        assert!(mapper_chirho.has_mapping_chirho("KJV", "Catholic"));
    }
}
