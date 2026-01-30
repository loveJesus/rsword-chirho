// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Leningrad (MT/Hebrew Bible) versification system data.
//!
//! The Leningrad versification is based on the Leningrad Codex, the oldest
//! complete manuscript of the Hebrew Bible in the Masoretic Text tradition.
//! It follows the traditional Hebrew book order (Torah, Nevi'im, Ketuvim)
//! and the Hebrew verse numbering.
//!
//! Key differences from KJV:
//! - Different book order (Tanakh order)
//! - Psalm superscriptions counted as verse 1
//! - Some chapter/verse divisions differ

use crate::versification_chirho::{BookInfoChirho, VersificationChirho};

/// Create the Leningrad versification system.
pub fn create_leningrad_chirho() -> VersificationChirho {
    VersificationChirho::new_chirho("Leningrad", create_ot_books_chirho(), create_nt_books_chirho())
}

fn create_ot_books_chirho() -> Vec<BookInfoChirho> {
    // Leningrad Codex follows traditional Hebrew Bible order:
    // Torah (Pentateuch), Nevi'im (Prophets), Ketuvim (Writings)
    // Uses Hebrew verse numbering where superscriptions count as verses
    vec![
        // TORAH (Pentateuch)
        BookInfoChirho::new_chirho("Genesis", "Gen", "Gen", vec![
            31, 25, 24, 26, 32, 22, 24, 22, 29, 32,
            32, 20, 18, 24, 21, 16, 27, 33, 38, 18,
            34, 24, 20, 67, 34, 35, 46, 22, 35, 43,
            55, 32, 20, 31, 29, 43, 36, 30, 23, 23,
            57, 38, 34, 34, 28, 34, 31, 22, 33, 26,
        ]),
        BookInfoChirho::new_chirho("Exodus", "Exod", "Exod", vec![
            22, 25, 22, 31, 23, 30, 25, 32, 35, 29,
            10, 51, 22, 31, 27, 36, 16, 27, 25, 26,
            36, 31, 33, 18, 40, 37, 21, 43, 46, 38,
            18, 35, 23, 35, 35, 38, 29, 31, 43, 38,
        ]),
        BookInfoChirho::new_chirho("Leviticus", "Lev", "Lev", vec![
            17, 16, 17, 35, 19, 30, 38, 36, 24, 20,
            47, 8, 59, 57, 33, 34, 16, 30, 37, 27,
            24, 33, 44, 23, 55, 46, 34,
        ]),
        BookInfoChirho::new_chirho("Numbers", "Num", "Num", vec![
            54, 34, 51, 49, 31, 27, 89, 26, 23, 36,
            35, 16, 33, 45, 41, 50, 13, 32, 22, 29,
            35, 41, 30, 25, 18, 65, 23, 31, 40, 16,
            54, 42, 56, 29, 34, 13,
        ]),
        BookInfoChirho::new_chirho("Deuteronomy", "Deut", "Deut", vec![
            46, 37, 29, 49, 33, 25, 26, 20, 29, 22,
            32, 32, 18, 29, 23, 22, 20, 22, 21, 20,
            23, 30, 25, 22, 19, 19, 26, 68, 29, 20,
            30, 52, 29, 12,
        ]),
        // NEVI'IM (Prophets) - Former Prophets
        BookInfoChirho::new_chirho("Joshua", "Josh", "Josh", vec![
            18, 24, 17, 24, 15, 27, 26, 35, 27, 43,
            23, 24, 33, 15, 63, 10, 18, 28, 51, 9,
            45, 34, 16, 33,
        ]),
        BookInfoChirho::new_chirho("Judges", "Judg", "Judg", vec![
            36, 23, 31, 24, 31, 40, 25, 35, 57, 18,
            40, 15, 25, 20, 20, 31, 13, 31, 30, 48,
            25,
        ]),
        BookInfoChirho::new_chirho("1 Samuel", "1Sam", "1Sam", vec![
            28, 36, 21, 22, 12, 21, 17, 22, 27, 27,
            15, 25, 23, 52, 35, 23, 58, 30, 24, 42,
            15, 23, 29, 22, 44, 25, 12, 25, 11, 31,
            13,
        ]),
        BookInfoChirho::new_chirho("2 Samuel", "2Sam", "2Sam", vec![
            27, 32, 39, 12, 25, 23, 29, 18, 13, 19,
            27, 31, 39, 33, 37, 23, 29, 33, 43, 26,
            22, 51, 39, 25,
        ]),
        BookInfoChirho::new_chirho("1 Kings", "1Kgs", "1Kgs", vec![
            53, 46, 28, 34, 18, 38, 51, 66, 28, 29,
            43, 33, 34, 31, 34, 34, 24, 46, 21, 43,
            29, 53,
        ]),
        BookInfoChirho::new_chirho("2 Kings", "2Kgs", "2Kgs", vec![
            18, 25, 27, 44, 27, 33, 20, 29, 37, 36,
            21, 21, 25, 29, 38, 20, 41, 37, 37, 21,
            26, 20, 37, 20, 30,
        ]),
        // NEVI'IM (Prophets) - Latter Prophets
        BookInfoChirho::new_chirho("Isaiah", "Isa", "Isa", vec![
            31, 22, 26, 6, 30, 13, 25, 22, 21, 34,
            16, 6, 22, 32, 9, 14, 14, 7, 25, 6,
            17, 25, 18, 23, 12, 21, 13, 29, 24, 33,
            9, 20, 24, 17, 10, 22, 38, 22, 8, 31,
            29, 25, 28, 28, 25, 13, 15, 22, 26, 11,
            23, 15, 12, 17, 13, 12, 21, 14, 21, 22,
            11, 12, 19, 12, 25, 24,
        ]),
        BookInfoChirho::new_chirho("Jeremiah", "Jer", "Jer", vec![
            19, 37, 25, 31, 31, 30, 34, 22, 26, 25,
            23, 17, 27, 22, 21, 21, 27, 23, 15, 18,
            14, 30, 40, 10, 38, 24, 22, 17, 32, 24,
            40, 44, 26, 22, 19, 32, 21, 28, 18, 16,
            18, 22, 13, 30, 5, 28, 7, 47, 39, 46,
            64, 34,
        ]),
        BookInfoChirho::new_chirho("Ezekiel", "Ezek", "Ezek", vec![
            28, 10, 27, 17, 17, 14, 27, 18, 11, 22,
            25, 28, 23, 23, 8, 63, 24, 32, 14, 49,
            32, 31, 49, 27, 17, 21, 36, 26, 21, 26,
            18, 32, 33, 31, 15, 38, 28, 23, 29, 49,
            26, 20, 27, 31, 25, 24, 23, 35,
        ]),
        // The Twelve (Minor Prophets)
        BookInfoChirho::new_chirho("Hosea", "Hos", "Hos", vec![
            11, 23, 5, 19, 15, 11, 16, 14, 17, 15,
            12, 14, 16, 9,
        ]),
        BookInfoChirho::new_chirho("Joel", "Joel", "Joel", vec![
            20, 32, 5, 21,
        ]),
        BookInfoChirho::new_chirho("Amos", "Amos", "Amos", vec![
            15, 16, 15, 13, 27, 14, 17, 14, 15,
        ]),
        BookInfoChirho::new_chirho("Obadiah", "Obad", "Obad", vec![
            21,
        ]),
        BookInfoChirho::new_chirho("Jonah", "Jonah", "Jonah", vec![
            17, 10, 10, 11,
        ]),
        BookInfoChirho::new_chirho("Micah", "Mic", "Mic", vec![
            16, 13, 12, 13, 15, 16, 20,
        ]),
        BookInfoChirho::new_chirho("Nahum", "Nah", "Nah", vec![
            15, 13, 19,
        ]),
        BookInfoChirho::new_chirho("Habakkuk", "Hab", "Hab", vec![
            17, 20, 19,
        ]),
        BookInfoChirho::new_chirho("Zephaniah", "Zeph", "Zeph", vec![
            18, 15, 20,
        ]),
        BookInfoChirho::new_chirho("Haggai", "Hag", "Hag", vec![
            15, 23,
        ]),
        BookInfoChirho::new_chirho("Zechariah", "Zech", "Zech", vec![
            21, 13, 10, 14, 11, 15, 14, 23, 17, 12,
            17, 14, 9, 21,
        ]),
        BookInfoChirho::new_chirho("Malachi", "Mal", "Mal", vec![
            14, 17, 18, 6,
        ]),
        // KETUVIM (Writings) - Hebrew order
        // Psalms with Hebrew verse numbering (superscriptions as verse 1)
        BookInfoChirho::new_chirho("Psalms", "Ps", "Ps", vec![
            6, 13, 9, 9, 13, 11, 18, 10, 21, 18,
            7, 9, 6, 7, 5, 11, 15, 51, 15, 10,
            14, 32, 6, 10, 22, 12, 14, 9, 11, 13,
            25, 11, 22, 23, 28, 13, 40, 23, 14, 18,
            14, 12, 5, 27, 18, 12, 10, 15, 21, 23,
            21, 11, 7, 9, 24, 14, 12, 12, 18, 14,
            9, 13, 12, 11, 14, 20, 8, 36, 37, 6,
            24, 20, 28, 23, 11, 13, 21, 72, 13, 20,
            17, 8, 19, 13, 14, 17, 7, 19, 53, 17,
            16, 16, 5, 23, 11, 13, 12, 9, 9, 5,
            8, 29, 22, 35, 45, 48, 43, 14, 31, 7,
            10, 10, 9, 9, 18, 19, 2, 29, 176, 7,
            8, 9, 4, 8, 5, 6, 5, 6, 8, 8,
            3, 18, 3, 3, 21, 26, 9, 8, 24, 14,
            10, 8, 12, 15, 21, 10, 20, 14, 9, 6,
        ]),
        BookInfoChirho::new_chirho("Proverbs", "Prov", "Prov", vec![
            33, 22, 35, 27, 23, 35, 27, 36, 18, 32,
            31, 28, 25, 35, 33, 33, 28, 24, 29, 30,
            31, 29, 35, 34, 28, 28, 27, 28, 27, 33,
            31,
        ]),
        BookInfoChirho::new_chirho("Job", "Job", "Job", vec![
            22, 13, 26, 21, 27, 30, 21, 22, 35, 22,
            20, 25, 28, 22, 35, 22, 16, 21, 29, 29,
            34, 30, 17, 25, 6, 14, 23, 28, 25, 31,
            40, 22, 33, 37, 16, 33, 24, 41, 30, 24,
            34, 17,
        ]),
        // Five Megillot (Five Scrolls)
        BookInfoChirho::new_chirho("Song of Songs", "Song", "Song", vec![
            17, 17, 11, 16, 16, 13, 13, 14,
        ]),
        BookInfoChirho::new_chirho("Ruth", "Ruth", "Ruth", vec![
            22, 23, 18, 22,
        ]),
        BookInfoChirho::new_chirho("Lamentations", "Lam", "Lam", vec![
            22, 22, 66, 22, 22,
        ]),
        BookInfoChirho::new_chirho("Ecclesiastes", "Eccl", "Eccl", vec![
            18, 26, 22, 16, 20, 12, 29, 17, 18, 20,
            10, 14,
        ]),
        BookInfoChirho::new_chirho("Esther", "Esth", "Esth", vec![
            22, 23, 15, 17, 14, 14, 10, 17, 32, 3,
        ]),
        // Historical books
        BookInfoChirho::new_chirho("Daniel", "Dan", "Dan", vec![
            21, 49, 33, 34, 30, 29, 28, 27, 27, 21,
            45, 13,
        ]),
        BookInfoChirho::new_chirho("Ezra", "Ezra", "Ezra", vec![
            11, 70, 13, 24, 17, 22, 28, 36, 15, 44,
        ]),
        BookInfoChirho::new_chirho("Nehemiah", "Neh", "Neh", vec![
            11, 20, 37, 17, 19, 19, 72, 18, 38, 39,
            36, 47, 31,
        ]),
        BookInfoChirho::new_chirho("1 Chronicles", "1Chr", "1Chr", vec![
            54, 55, 24, 43, 26, 81, 40, 40, 44, 14,
            47, 40, 14, 17, 29, 43, 27, 17, 19, 8,
            30, 19, 32, 31, 31, 32, 34, 21, 30,
        ]),
        BookInfoChirho::new_chirho("2 Chronicles", "2Chr", "2Chr", vec![
            17, 18, 17, 22, 14, 42, 22, 18, 31, 19,
            23, 16, 22, 15, 19, 14, 19, 34, 11, 37,
            20, 12, 21, 27, 28, 23, 9, 27, 36, 27,
            21, 33, 25, 33, 27, 23,
        ]),
    ]
}

fn create_nt_books_chirho() -> Vec<BookInfoChirho> {
    // Leningrad is Hebrew Bible only - no NT
    // Return empty for compatibility
    vec![]
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_leningrad_ot_books_chirho() {
        let ot_chirho = create_ot_books_chirho();
        // Hebrew Bible has 39 books
        assert_eq!(ot_chirho.len(), 39);
        assert_eq!(ot_chirho[0].name_chirho, "Genesis");
    }

    #[test]
    fn test_leningrad_no_nt_chirho() {
        let nt_chirho = create_nt_books_chirho();
        // Leningrad is Hebrew Bible only
        assert_eq!(nt_chirho.len(), 0);
    }

    #[test]
    fn test_leningrad_creation_chirho() {
        let leningrad_chirho = create_leningrad_chirho();
        assert_eq!(leningrad_chirho.name_chirho, "Leningrad");
        // Hebrew Bible only
        assert_eq!(leningrad_chirho.ot_books_chirho.len(), 39);
        assert_eq!(leningrad_chirho.nt_books_chirho.len(), 0);
    }

    #[test]
    fn test_leningrad_hebrew_order_chirho() {
        let ot_chirho = create_ot_books_chirho();
        // Verify Hebrew order - Torah first
        assert_eq!(ot_chirho[0].name_chirho, "Genesis");
        assert_eq!(ot_chirho[4].name_chirho, "Deuteronomy");
        // Prophets next
        assert_eq!(ot_chirho[5].name_chirho, "Joshua");
        // Isaiah after Kings
        assert_eq!(ot_chirho[11].name_chirho, "Isaiah");
    }
}
