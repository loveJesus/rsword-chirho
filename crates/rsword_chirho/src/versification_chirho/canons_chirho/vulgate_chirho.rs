// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Vulgate versification system data.
//!
//! The Latin Vulgate versification, used by traditional Catholic Bibles.
//! Based on Jerome's Latin translation (4th-5th century).
//! Notable differences from KJV:
//! - Includes deuterocanonical/apocryphal books
//! - Different Psalm numbering (Ps 10-146 are -1 from Hebrew/KJV)
//! - Different verse divisions in many books

use crate::versification_chirho::{BookInfoChirho, VersificationChirho};

/// Create the Vulgate versification system.
pub fn create_vulgate_chirho() -> VersificationChirho {
    VersificationChirho::new_chirho("Vulgate", create_ot_books_chirho(), create_nt_books_chirho())
}

fn create_ot_books_chirho() -> Vec<BookInfoChirho> {
    vec![
        BookInfoChirho::new_chirho("Genesis", "Gen", "Gen", vec![
            31, 25, 24, 26, 32, 22, 24, 22, 29, 32,
            32, 20, 18, 24, 21, 16, 27, 33, 38, 18,
            34, 24, 20, 67, 34, 35, 46, 22, 35, 43,
            55, 32, 20, 31, 29, 43, 36, 30, 23, 23,
            57, 38, 34, 34, 28, 34, 31, 22, 33, 26,
        ]),
        BookInfoChirho::new_chirho("Exodus", "Exod", "Exod", vec![
            22, 25, 22, 31, 23, 30, 29, 28, 35, 29,
            10, 51, 22, 31, 27, 36, 16, 27, 25, 26,
            37, 30, 33, 18, 40, 37, 21, 43, 46, 38,
            18, 35, 23, 35, 35, 38, 29, 31, 43, 36,
        ]),
        BookInfoChirho::new_chirho("Leviticus", "Lev", "Lev", vec![
            17, 16, 17, 35, 26, 23, 38, 36, 24, 20,
            47, 8, 59, 57, 33, 34, 16, 30, 37, 27,
            24, 33, 44, 23, 55, 46, 34,
        ]),
        BookInfoChirho::new_chirho("Numeri", "Num", "Num", vec![
            54, 34, 51, 49, 31, 27, 89, 26, 23, 36,
            35, 16, 33, 45, 41, 35, 28, 32, 22, 29,
            35, 41, 30, 25, 19, 65, 23, 31, 39, 17,
            54, 42, 56, 29, 34, 13,
        ]),
        BookInfoChirho::new_chirho("Deuteronomium", "Deut", "Deut", vec![
            46, 37, 29, 49, 33, 25, 26, 20, 29, 22,
            32, 31, 19, 29, 23, 22, 20, 22, 21, 20,
            23, 29, 26, 22, 19, 19, 26, 69, 28, 20,
            30, 52, 29, 12,
        ]),
        BookInfoChirho::new_chirho("Iosue", "Josh", "Josh", vec![
            18, 24, 17, 24, 15, 27, 26, 35, 27, 43,
            23, 24, 33, 15, 63, 10, 18, 28, 51, 9,
            45, 34, 16, 33,
        ]),
        BookInfoChirho::new_chirho("Iudicum", "Judg", "Judg", vec![
            36, 23, 31, 24, 31, 40, 25, 35, 57, 18,
            40, 15, 25, 20, 20, 31, 13, 31, 30, 48,
            25,
        ]),
        BookInfoChirho::new_chirho("Ruth", "Ruth", "Ruth", vec![
            22, 23, 18, 22,
        ]),
        BookInfoChirho::new_chirho("I Samuelis", "1Sam", "1Sam", vec![
            28, 36, 21, 22, 12, 21, 17, 22, 27, 27,
            15, 25, 23, 52, 35, 23, 58, 30, 24, 42,
            16, 23, 28, 23, 44, 25, 12, 25, 11, 31,
            13,
        ]),
        BookInfoChirho::new_chirho("II Samuelis", "2Sam", "2Sam", vec![
            27, 32, 39, 12, 25, 23, 29, 18, 13, 19,
            27, 31, 39, 33, 37, 23, 29, 32, 44, 26,
            22, 51, 39, 25,
        ]),
        BookInfoChirho::new_chirho("I Regum", "1Kgs", "1Kgs", vec![
            53, 46, 28, 20, 32, 38, 51, 66, 28, 29,
            43, 33, 34, 31, 34, 34, 24, 46, 21, 43,
            29, 54,
        ]),
        BookInfoChirho::new_chirho("II Regum", "2Kgs", "2Kgs", vec![
            18, 25, 27, 44, 27, 33, 20, 29, 37, 36,
            20, 22, 25, 29, 38, 20, 41, 37, 37, 21,
            26, 20, 37, 20, 30,
        ]),
        BookInfoChirho::new_chirho("I Paralipomenon", "1Chr", "1Chr", vec![
            54, 55, 24, 43, 41, 66, 40, 40, 44, 14,
            47, 41, 14, 17, 29, 43, 27, 17, 19, 8,
            30, 19, 32, 31, 31, 32, 34, 21, 30,
        ]),
        BookInfoChirho::new_chirho("II Paralipomenon", "2Chr", "2Chr", vec![
            18, 17, 17, 22, 14, 42, 22, 18, 31, 19,
            23, 16, 23, 14, 19, 14, 19, 34, 11, 37,
            20, 12, 21, 27, 28, 23, 9, 27, 36, 27,
            21, 33, 25, 33, 27, 23,
        ]),
        BookInfoChirho::new_chirho("Esdras", "Ezra", "Ezra", vec![
            11, 70, 13, 24, 17, 22, 28, 36, 15, 44,
        ]),
        BookInfoChirho::new_chirho("Nehemias", "Neh", "Neh", vec![
            11, 20, 38, 17, 19, 19, 72, 18, 37, 40,
            36, 47, 31,
        ]),
        // Deuterocanonical: Tobit
        BookInfoChirho::new_chirho("Tobias", "Tob", "Tob", vec![
            25, 23, 25, 23, 28, 22, 20, 24, 12, 13,
            21, 22, 23, 17,
        ]),
        // Deuterocanonical: Judith
        BookInfoChirho::new_chirho("Iudith", "Jdt", "Jdt", vec![
            12, 18, 15, 17, 29, 21, 25, 34, 19, 20,
            21, 20, 31, 18, 15, 31,
        ]),
        BookInfoChirho::new_chirho("Esther", "Esth", "Esth", vec![
            22, 23, 15, 17, 14, 14, 10, 17, 32, 13,
            12, 6, 18, 19, 19, 24,
        ]),
        BookInfoChirho::new_chirho("Iob", "Job", "Job", vec![
            22, 13, 26, 21, 27, 30, 21, 22, 35, 22,
            20, 25, 28, 22, 35, 23, 16, 21, 29, 29,
            34, 30, 17, 25, 6, 14, 23, 28, 25, 31,
            40, 22, 33, 37, 16, 33, 24, 41, 35, 28,
            25, 16,
        ]),
        // Vulgate Psalms follow LXX numbering
        BookInfoChirho::new_chirho("Psalmi", "Ps", "Ps", vec![
            6, 13, 9, 9, 13, 11, 18, 10, 39, 8,
            9, 6, 7, 5, 11, 15, 51, 15, 10, 14,
            32, 6, 10, 22, 12, 14, 9, 11, 13, 25,
            11, 22, 23, 28, 13, 40, 23, 14, 18, 14,
            12, 6, 27, 18, 12, 10, 15, 21, 23, 21,
            11, 7, 9, 24, 14, 12, 12, 18, 14, 9,
            13, 12, 11, 14, 20, 8, 36, 37, 7, 24,
            20, 28, 23, 11, 13, 21, 72, 13, 20, 17,
            8, 19, 13, 14, 17, 7, 19, 53, 17, 16,
            16, 5, 23, 11, 13, 12, 9, 9, 5, 8,
            29, 22, 35, 45, 48, 43, 14, 31, 7, 10,
            10, 9, 26, 18, 19, 2, 29, 176, 7, 8,
            9, 4, 8, 5, 6, 6, 5, 8, 8, 3,
            18, 3, 3, 21, 26, 9, 8, 24, 14, 10,
            8, 12, 15, 21, 10, 11, 20, 14, 9, 7,
        ]),
        BookInfoChirho::new_chirho("Proverbia", "Prov", "Prov", vec![
            33, 22, 35, 27, 23, 35, 27, 36, 18, 32,
            31, 28, 25, 35, 33, 33, 28, 24, 29, 30,
            31, 29, 35, 34, 28, 28, 27, 28, 27, 33,
            31,
        ]),
        BookInfoChirho::new_chirho("Ecclesiastes", "Eccl", "Eccl", vec![
            18, 26, 22, 17, 19, 12, 29, 17, 18, 20,
            10, 14,
        ]),
        BookInfoChirho::new_chirho("Canticum", "Song", "Song", vec![
            16, 17, 11, 16, 17, 12, 13, 14,
        ]),
        // Deuterocanonical: Wisdom
        BookInfoChirho::new_chirho("Sapientia", "Wis", "Wis", vec![
            16, 25, 19, 20, 24, 27, 30, 21, 19, 21,
            27, 27, 19, 31, 19, 29, 20, 25, 22,
        ]),
        // Deuterocanonical: Sirach/Ecclesiasticus
        BookInfoChirho::new_chirho("Ecclesiasticus", "Sir", "Sir", vec![
            40, 23, 34, 36, 18, 37, 40, 22, 25, 34,
            36, 19, 32, 27, 22, 31, 31, 33, 28, 33,
            31, 33, 38, 47, 36, 28, 33, 30, 35, 27,
            42, 28, 33, 31, 26, 28, 34, 39, 41, 32,
            28, 26, 37, 27, 31, 23, 31, 28, 19, 31,
            38,
        ]),
        BookInfoChirho::new_chirho("Isaias", "Isa", "Isa", vec![
            31, 22, 26, 6, 30, 13, 25, 23, 20, 34,
            16, 6, 22, 32, 9, 14, 14, 7, 25, 6,
            17, 25, 18, 23, 12, 21, 13, 29, 24, 33,
            9, 20, 24, 17, 10, 22, 38, 22, 8, 31,
            29, 25, 28, 28, 25, 13, 15, 22, 26, 11,
            23, 15, 12, 17, 13, 12, 21, 14, 21, 22,
            11, 12, 19, 11, 25, 24,
        ]),
        BookInfoChirho::new_chirho("Ieremias", "Jer", "Jer", vec![
            19, 37, 25, 31, 31, 30, 34, 23, 25, 25,
            23, 17, 27, 22, 21, 21, 27, 23, 15, 18,
            14, 30, 40, 10, 38, 24, 22, 17, 32, 24,
            40, 44, 26, 22, 19, 32, 21, 28, 18, 16,
            18, 22, 13, 30, 5, 28, 7, 47, 39, 46,
            64, 34,
        ]),
        BookInfoChirho::new_chirho("Lamentationes", "Lam", "Lam", vec![
            22, 22, 66, 22, 22,
        ]),
        // Deuterocanonical: Baruch (includes Letter of Jeremiah as ch. 6)
        BookInfoChirho::new_chirho("Baruch", "Bar", "Bar", vec![
            22, 35, 38, 37, 9, 72,
        ]),
        BookInfoChirho::new_chirho("Ezechiel", "Ezek", "Ezek", vec![
            28, 10, 27, 17, 17, 14, 27, 18, 11, 22,
            25, 28, 23, 23, 8, 63, 24, 32, 14, 44,
            37, 31, 49, 27, 17, 21, 36, 26, 21, 26,
            18, 32, 33, 31, 15, 38, 28, 23, 29, 49,
            26, 20, 27, 31, 25, 24, 23, 35,
        ]),
        BookInfoChirho::new_chirho("Daniel", "Dan", "Dan", vec![
            21, 49, 100, 34, 31, 28, 28, 27, 27, 21,
            45, 13, 65, 42,
        ]),
        BookInfoChirho::new_chirho("Osee", "Hos", "Hos", vec![
            9, 25, 5, 19, 15, 11, 16, 14, 17, 15,
            11, 15, 15, 10,
        ]),
        BookInfoChirho::new_chirho("Ioel", "Joel", "Joel", vec![
            20, 27, 5, 21,
        ]),
        BookInfoChirho::new_chirho("Amos", "Amos", "Amos", vec![
            15, 16, 15, 13, 27, 15, 17, 14, 15,
        ]),
        BookInfoChirho::new_chirho("Abdias", "Obad", "Obad", vec![
            21,
        ]),
        BookInfoChirho::new_chirho("Ionas", "Jonah", "Jonah", vec![
            16, 11, 10, 11,
        ]),
        BookInfoChirho::new_chirho("Michaeas", "Mic", "Mic", vec![
            16, 13, 12, 14, 14, 16, 20,
        ]),
        BookInfoChirho::new_chirho("Nahum", "Nah", "Nah", vec![
            14, 14, 19,
        ]),
        BookInfoChirho::new_chirho("Habacuc", "Hab", "Hab", vec![
            17, 20, 19,
        ]),
        BookInfoChirho::new_chirho("Sophonias", "Zeph", "Zeph", vec![
            18, 15, 20,
        ]),
        BookInfoChirho::new_chirho("Aggaeus", "Hag", "Hag", vec![
            15, 24,
        ]),
        BookInfoChirho::new_chirho("Zacharias", "Zech", "Zech", vec![
            17, 17, 10, 14, 11, 15, 14, 23, 17, 12,
            17, 14, 9, 21,
        ]),
        BookInfoChirho::new_chirho("Malachias", "Mal", "Mal", vec![
            14, 17, 24,
        ]),
        // Deuterocanonical: 1 Maccabees
        BookInfoChirho::new_chirho("I Machabaeorum", "1Macc", "1Macc", vec![
            67, 70, 60, 61, 68, 63, 50, 32, 73, 89,
            74, 54, 54, 49, 41, 24,
        ]),
        // Deuterocanonical: 2 Maccabees
        BookInfoChirho::new_chirho("II Machabaeorum", "2Macc", "2Macc", vec![
            36, 32, 40, 50, 27, 31, 42, 36, 29, 38,
            38, 46, 26, 46, 39,
        ]),
    ]
}

fn create_nt_books_chirho() -> Vec<BookInfoChirho> {
    // Vulgate NT is essentially the same as KJV with Latin names
    vec![
        BookInfoChirho::new_chirho("Matthaeus", "Matt", "Matt", vec![
            25, 23, 17, 25, 48, 34, 29, 34, 38, 42,
            30, 50, 58, 36, 39, 28, 27, 35, 30, 34,
            46, 46, 39, 51, 46, 75, 66, 20,
        ]),
        BookInfoChirho::new_chirho("Marcus", "Mark", "Mark", vec![
            45, 28, 35, 41, 43, 56, 37, 38, 50, 52,
            33, 44, 37, 72, 47, 20,
        ]),
        BookInfoChirho::new_chirho("Lucas", "Luke", "Luke", vec![
            80, 52, 38, 44, 39, 49, 50, 56, 62, 42,
            54, 59, 35, 35, 32, 31, 37, 43, 48, 47,
            38, 71, 56, 53,
        ]),
        BookInfoChirho::new_chirho("Ioannes", "John", "John", vec![
            51, 25, 36, 54, 47, 71, 53, 59, 41, 42,
            57, 50, 38, 31, 27, 33, 26, 40, 42, 31,
            25,
        ]),
        BookInfoChirho::new_chirho("Actus", "Acts", "Acts", vec![
            26, 47, 26, 37, 42, 15, 60, 40, 43, 48,
            30, 25, 52, 28, 41, 40, 34, 28, 40, 38,
            40, 30, 35, 27, 27, 32, 44, 31,
        ]),
        BookInfoChirho::new_chirho("ad Romanos", "Rom", "Rom", vec![
            32, 29, 31, 25, 21, 23, 25, 39, 33, 21,
            36, 21, 14, 23, 33, 27,
        ]),
        BookInfoChirho::new_chirho("I ad Corinthios", "1Cor", "1Cor", vec![
            31, 16, 23, 21, 13, 20, 40, 13, 27, 33,
            34, 31, 13, 40, 58, 24,
        ]),
        BookInfoChirho::new_chirho("II ad Corinthios", "2Cor", "2Cor", vec![
            24, 17, 18, 18, 21, 18, 16, 24, 15, 18,
            33, 21, 13,
        ]),
        BookInfoChirho::new_chirho("ad Galatas", "Gal", "Gal", vec![
            24, 21, 29, 31, 26, 18,
        ]),
        BookInfoChirho::new_chirho("ad Ephesios", "Eph", "Eph", vec![
            23, 22, 21, 32, 33, 24,
        ]),
        BookInfoChirho::new_chirho("ad Philippenses", "Phil", "Phil", vec![
            30, 30, 21, 23,
        ]),
        BookInfoChirho::new_chirho("ad Colossenses", "Col", "Col", vec![
            29, 23, 25, 18,
        ]),
        BookInfoChirho::new_chirho("I ad Thessalonicenses", "1Thess", "1Thess", vec![
            10, 20, 13, 18, 28,
        ]),
        BookInfoChirho::new_chirho("II ad Thessalonicenses", "2Thess", "2Thess", vec![
            12, 17, 18,
        ]),
        BookInfoChirho::new_chirho("I ad Timotheum", "1Tim", "1Tim", vec![
            20, 15, 16, 16, 25, 21,
        ]),
        BookInfoChirho::new_chirho("II ad Timotheum", "2Tim", "2Tim", vec![
            18, 26, 17, 22,
        ]),
        BookInfoChirho::new_chirho("ad Titum", "Titus", "Titus", vec![
            16, 15, 15,
        ]),
        BookInfoChirho::new_chirho("ad Philemonem", "Phlm", "Phlm", vec![
            25,
        ]),
        BookInfoChirho::new_chirho("ad Hebraeos", "Heb", "Heb", vec![
            14, 18, 19, 16, 14, 20, 28, 13, 28, 39,
            40, 29, 25,
        ]),
        BookInfoChirho::new_chirho("Iacobi", "Jas", "Jas", vec![
            27, 26, 18, 17, 20,
        ]),
        BookInfoChirho::new_chirho("I Petri", "1Pet", "1Pet", vec![
            25, 25, 22, 19, 14,
        ]),
        BookInfoChirho::new_chirho("II Petri", "2Pet", "2Pet", vec![
            21, 22, 18,
        ]),
        BookInfoChirho::new_chirho("I Ioannis", "1John", "1John", vec![
            10, 29, 24, 21, 21,
        ]),
        BookInfoChirho::new_chirho("II Ioannis", "2John", "2John", vec![
            13,
        ]),
        BookInfoChirho::new_chirho("III Ioannis", "3John", "3John", vec![
            15,
        ]),
        BookInfoChirho::new_chirho("Iudae", "Jude", "Jude", vec![
            25,
        ]),
        BookInfoChirho::new_chirho("Apocalypsis", "Rev", "Rev", vec![
            20, 29, 22, 11, 14, 17, 17, 13, 21, 11,
            19, 18, 18, 20, 8, 21, 18, 24, 21, 15,
            27, 21,
        ]),
    ]
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_vulgate_has_deuterocanonical_chirho() {
        let ot_chirho = create_ot_books_chirho();
        // Should have more than 39 books (KJV count)
        assert!(ot_chirho.len() > 39);

        // Check for Tobit
        let tobit_chirho = ot_chirho.iter().find(|b| b.osis_chirho == "Tob");
        assert!(tobit_chirho.is_some());

        // Check for Wisdom
        let wisdom_chirho = ot_chirho.iter().find(|b| b.osis_chirho == "Wis");
        assert!(wisdom_chirho.is_some());

        // Check for Sirach
        let sirach_chirho = ot_chirho.iter().find(|b| b.osis_chirho == "Sir");
        assert!(sirach_chirho.is_some());

        // Check for Maccabees
        let macc1_chirho = ot_chirho.iter().find(|b| b.osis_chirho == "1Macc");
        assert!(macc1_chirho.is_some());
    }

    #[test]
    fn test_vulgate_latin_names_chirho() {
        let vulgate_chirho = create_vulgate_chirho();
        let lookup_result_chirho = vulgate_chirho.lookup_book_chirho("Gen");
        assert!(lookup_result_chirho.is_some());
        let (testament_chirho, book_idx_chirho) = lookup_result_chirho.unwrap();
        let genesis_chirho = vulgate_chirho.get_book_chirho(testament_chirho, book_idx_chirho);
        assert!(genesis_chirho.is_some());
        // Latin name stays "Genesis" for Genesis
        assert_eq!(genesis_chirho.unwrap().name_chirho, "Genesis");

        let lookup_matt_chirho = vulgate_chirho.lookup_book_chirho("Matt");
        assert!(lookup_matt_chirho.is_some());
        let (testament_nt_chirho, matt_idx_chirho) = lookup_matt_chirho.unwrap();
        let matthew_chirho = vulgate_chirho.get_book_chirho(testament_nt_chirho, matt_idx_chirho);
        assert!(matthew_chirho.is_some());
        assert_eq!(matthew_chirho.unwrap().name_chirho, "Matthaeus");
    }
}
