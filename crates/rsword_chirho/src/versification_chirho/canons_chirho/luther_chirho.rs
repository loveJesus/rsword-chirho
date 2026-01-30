// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Luther versification system data.
//!
//! The Luther versification is used by German Lutheran Bibles and follows
//! Martin Luther's original German Bible translation ordering.
//! Key differences from KJV:
//! - Hebrews placed before James
//! - Different verse counts in some Psalms

use crate::versification_chirho::{BookInfoChirho, VersificationChirho};

/// Create the Luther versification system.
pub fn create_luther_chirho() -> VersificationChirho {
    VersificationChirho::new_chirho("Luther", create_ot_books_chirho(), create_nt_books_chirho())
}

fn create_ot_books_chirho() -> Vec<BookInfoChirho> {
    // Luther OT is similar to KJV but with some verse count differences
    vec![
        BookInfoChirho::new_chirho("1. Mose", "Gen", "Gen", vec![
            31, 25, 24, 26, 32, 22, 24, 22, 29, 32,
            32, 20, 18, 24, 21, 16, 27, 33, 38, 18,
            34, 24, 20, 67, 34, 35, 46, 22, 35, 43,
            55, 32, 20, 31, 29, 43, 36, 30, 23, 23,
            57, 38, 34, 34, 28, 34, 31, 22, 33, 26,
        ]),
        BookInfoChirho::new_chirho("2. Mose", "Exod", "Exod", vec![
            22, 25, 22, 31, 23, 30, 25, 32, 35, 29,
            10, 51, 22, 31, 27, 36, 16, 27, 25, 26,
            36, 31, 33, 18, 40, 37, 21, 43, 46, 38,
            18, 35, 23, 35, 35, 38, 29, 31, 43, 38,
        ]),
        BookInfoChirho::new_chirho("3. Mose", "Lev", "Lev", vec![
            17, 16, 17, 35, 19, 30, 38, 36, 24, 20,
            47, 8, 59, 57, 33, 34, 16, 30, 37, 27,
            24, 33, 44, 23, 55, 46, 34,
        ]),
        BookInfoChirho::new_chirho("4. Mose", "Num", "Num", vec![
            54, 34, 51, 49, 31, 27, 89, 26, 23, 36,
            35, 16, 33, 45, 41, 50, 13, 32, 22, 29,
            35, 41, 30, 25, 18, 65, 23, 31, 40, 16,
            54, 42, 56, 29, 34, 13,
        ]),
        BookInfoChirho::new_chirho("5. Mose", "Deut", "Deut", vec![
            46, 37, 29, 49, 33, 25, 26, 20, 29, 22,
            32, 32, 18, 29, 23, 22, 20, 22, 21, 20,
            23, 30, 25, 22, 19, 19, 26, 68, 29, 20,
            30, 52, 29, 12,
        ]),
        BookInfoChirho::new_chirho("Josua", "Josh", "Josh", vec![
            18, 24, 17, 24, 15, 27, 26, 35, 27, 43,
            23, 24, 33, 15, 63, 10, 18, 28, 51, 9,
            45, 34, 16, 33,
        ]),
        BookInfoChirho::new_chirho("Richter", "Judg", "Judg", vec![
            36, 23, 31, 24, 31, 40, 25, 35, 57, 18,
            40, 15, 25, 20, 20, 31, 13, 31, 30, 48,
            25,
        ]),
        BookInfoChirho::new_chirho("Ruth", "Ruth", "Ruth", vec![
            22, 23, 18, 22,
        ]),
        BookInfoChirho::new_chirho("1. Samuel", "1Sam", "1Sam", vec![
            28, 36, 21, 22, 12, 21, 17, 22, 27, 27,
            15, 25, 23, 52, 35, 23, 58, 30, 24, 42,
            15, 23, 29, 22, 44, 25, 12, 25, 11, 31,
            13,
        ]),
        BookInfoChirho::new_chirho("2. Samuel", "2Sam", "2Sam", vec![
            27, 32, 39, 12, 25, 23, 29, 18, 13, 19,
            27, 31, 39, 33, 37, 23, 29, 33, 43, 26,
            22, 51, 39, 25,
        ]),
        BookInfoChirho::new_chirho("1. Könige", "1Kgs", "1Kgs", vec![
            53, 46, 28, 34, 18, 38, 51, 66, 28, 29,
            43, 33, 34, 31, 34, 34, 24, 46, 21, 43,
            29, 53,
        ]),
        BookInfoChirho::new_chirho("2. Könige", "2Kgs", "2Kgs", vec![
            18, 25, 27, 44, 27, 33, 20, 29, 37, 36,
            21, 21, 25, 29, 38, 20, 41, 37, 37, 21,
            26, 20, 37, 20, 30,
        ]),
        BookInfoChirho::new_chirho("1. Chronik", "1Chr", "1Chr", vec![
            54, 55, 24, 43, 26, 81, 40, 40, 44, 14,
            47, 40, 14, 17, 29, 43, 27, 17, 19, 8,
            30, 19, 32, 31, 31, 32, 34, 21, 30,
        ]),
        BookInfoChirho::new_chirho("2. Chronik", "2Chr", "2Chr", vec![
            17, 18, 17, 22, 14, 42, 22, 18, 31, 19,
            23, 16, 22, 15, 19, 14, 19, 34, 11, 37,
            20, 12, 21, 27, 28, 23, 9, 27, 36, 27,
            21, 33, 25, 33, 27, 23,
        ]),
        BookInfoChirho::new_chirho("Esra", "Ezra", "Ezra", vec![
            11, 70, 13, 24, 17, 22, 28, 36, 15, 44,
        ]),
        BookInfoChirho::new_chirho("Nehemia", "Neh", "Neh", vec![
            11, 20, 32, 23, 19, 19, 73, 18, 38, 39,
            36, 47, 31,
        ]),
        BookInfoChirho::new_chirho("Esther", "Esth", "Esth", vec![
            22, 23, 15, 17, 14, 14, 10, 17, 32, 3,
        ]),
        BookInfoChirho::new_chirho("Hiob", "Job", "Job", vec![
            22, 13, 26, 21, 27, 30, 21, 22, 35, 22,
            20, 25, 28, 22, 35, 22, 16, 21, 29, 29,
            34, 30, 17, 25, 6, 14, 23, 28, 25, 31,
            40, 22, 33, 37, 16, 33, 24, 41, 30, 24,
            34, 17,
        ]),
        BookInfoChirho::new_chirho("Psalmen", "Ps", "Ps", vec![
            // Luther Psalms have different verse counts due to superscription handling
            6, 12, 9, 9, 13, 11, 18, 10, 21, 18,
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
            10, 10, 9, 26, 9, 19, 2, 29, 176, 7,
            8, 9, 4, 8, 5, 6, 5, 6, 8, 8,
            3, 18, 3, 3, 21, 26, 9, 8, 24, 14,
            10, 7, 12, 15, 21, 10, 20, 14, 9, 6,
        ]),
        BookInfoChirho::new_chirho("Sprüche", "Prov", "Prov", vec![
            33, 22, 35, 27, 23, 35, 27, 36, 18, 32,
            31, 28, 25, 35, 33, 33, 28, 24, 29, 30,
            31, 29, 35, 34, 28, 28, 27, 28, 27, 33,
            31,
        ]),
        BookInfoChirho::new_chirho("Prediger", "Eccl", "Eccl", vec![
            18, 26, 22, 16, 20, 12, 29, 17, 18, 20,
            10, 14,
        ]),
        BookInfoChirho::new_chirho("Hohelied", "Song", "Song", vec![
            17, 17, 11, 16, 16, 13, 13, 14,
        ]),
        BookInfoChirho::new_chirho("Jesaja", "Isa", "Isa", vec![
            31, 22, 26, 6, 30, 13, 25, 22, 21, 34,
            16, 6, 22, 32, 9, 14, 14, 7, 25, 6,
            17, 25, 18, 23, 12, 21, 13, 29, 24, 33,
            9, 20, 24, 17, 10, 22, 38, 22, 8, 31,
            29, 25, 28, 28, 25, 13, 15, 22, 26, 11,
            23, 15, 12, 17, 13, 12, 21, 14, 21, 22,
            11, 12, 19, 12, 25, 24,
        ]),
        BookInfoChirho::new_chirho("Jeremia", "Jer", "Jer", vec![
            19, 37, 25, 31, 31, 30, 34, 22, 26, 25,
            23, 17, 27, 22, 21, 21, 27, 23, 15, 18,
            14, 30, 40, 10, 38, 24, 22, 17, 32, 24,
            40, 44, 26, 22, 19, 32, 21, 28, 18, 16,
            18, 22, 13, 30, 5, 28, 7, 47, 39, 46,
            64, 34,
        ]),
        BookInfoChirho::new_chirho("Klagelieder", "Lam", "Lam", vec![
            22, 22, 66, 22, 22,
        ]),
        BookInfoChirho::new_chirho("Hesekiel", "Ezek", "Ezek", vec![
            28, 10, 27, 17, 17, 14, 27, 18, 11, 22,
            25, 28, 23, 23, 8, 63, 24, 32, 14, 49,
            32, 31, 49, 27, 17, 21, 36, 26, 21, 26,
            18, 32, 33, 31, 15, 38, 28, 23, 29, 49,
            26, 20, 27, 31, 25, 24, 23, 35,
        ]),
        BookInfoChirho::new_chirho("Daniel", "Dan", "Dan", vec![
            21, 49, 30, 37, 31, 28, 28, 27, 27, 21,
            45, 13,
        ]),
        BookInfoChirho::new_chirho("Hosea", "Hos", "Hos", vec![
            11, 23, 5, 19, 15, 11, 16, 14, 17, 15,
            12, 14, 16, 9,
        ]),
        BookInfoChirho::new_chirho("Joel", "Joel", "Joel", vec![
            20, 32, 21,
        ]),
        BookInfoChirho::new_chirho("Amos", "Amos", "Amos", vec![
            15, 16, 15, 13, 27, 14, 17, 14, 15,
        ]),
        BookInfoChirho::new_chirho("Obadja", "Obad", "Obad", vec![
            21,
        ]),
        BookInfoChirho::new_chirho("Jona", "Jonah", "Jonah", vec![
            17, 10, 10, 11,
        ]),
        BookInfoChirho::new_chirho("Micha", "Mic", "Mic", vec![
            16, 13, 12, 13, 15, 16, 20,
        ]),
        BookInfoChirho::new_chirho("Nahum", "Nah", "Nah", vec![
            15, 13, 19,
        ]),
        BookInfoChirho::new_chirho("Habakuk", "Hab", "Hab", vec![
            17, 20, 19,
        ]),
        BookInfoChirho::new_chirho("Zefanja", "Zeph", "Zeph", vec![
            18, 15, 20,
        ]),
        BookInfoChirho::new_chirho("Haggai", "Hag", "Hag", vec![
            15, 23,
        ]),
        BookInfoChirho::new_chirho("Sacharja", "Zech", "Zech", vec![
            21, 13, 10, 14, 11, 15, 14, 23, 17, 12,
            17, 14, 9, 21,
        ]),
        BookInfoChirho::new_chirho("Maleachi", "Mal", "Mal", vec![
            14, 17, 18, 6,
        ]),
    ]
}

fn create_nt_books_chirho() -> Vec<BookInfoChirho> {
    // Luther NT has same content as KJV but different German names
    // Note: Luther placed Hebrews before James (general epistles)
    vec![
        BookInfoChirho::new_chirho("Matthäus", "Matt", "Matt", vec![
            25, 23, 17, 25, 48, 34, 29, 34, 38, 42,
            30, 50, 58, 36, 39, 28, 27, 35, 30, 34,
            46, 46, 39, 51, 46, 75, 66, 20,
        ]),
        BookInfoChirho::new_chirho("Markus", "Mark", "Mark", vec![
            45, 28, 35, 41, 43, 56, 37, 38, 50, 52,
            33, 44, 37, 72, 47, 20,
        ]),
        BookInfoChirho::new_chirho("Lukas", "Luke", "Luke", vec![
            80, 52, 38, 44, 39, 49, 50, 56, 62, 42,
            54, 59, 35, 35, 32, 31, 37, 43, 48, 47,
            38, 71, 56, 53,
        ]),
        BookInfoChirho::new_chirho("Johannes", "John", "John", vec![
            51, 25, 36, 54, 47, 71, 53, 59, 41, 42,
            57, 50, 38, 31, 27, 33, 26, 40, 42, 31,
            25,
        ]),
        BookInfoChirho::new_chirho("Apostelgeschichte", "Acts", "Acts", vec![
            26, 47, 26, 37, 42, 15, 60, 40, 43, 48,
            30, 25, 52, 28, 41, 40, 34, 28, 41, 38,
            40, 30, 35, 27, 27, 32, 44, 31,
        ]),
        BookInfoChirho::new_chirho("Römer", "Rom", "Rom", vec![
            32, 29, 31, 25, 21, 23, 25, 39, 33, 21,
            36, 21, 14, 23, 33, 27,
        ]),
        BookInfoChirho::new_chirho("1. Korinther", "1Cor", "1Cor", vec![
            31, 16, 23, 21, 13, 20, 40, 13, 27, 33,
            34, 31, 13, 40, 58, 24,
        ]),
        BookInfoChirho::new_chirho("2. Korinther", "2Cor", "2Cor", vec![
            24, 17, 18, 18, 21, 18, 16, 24, 15, 18,
            33, 21, 14,
        ]),
        BookInfoChirho::new_chirho("Galater", "Gal", "Gal", vec![
            24, 21, 29, 31, 26, 18,
        ]),
        BookInfoChirho::new_chirho("Epheser", "Eph", "Eph", vec![
            23, 22, 21, 32, 33, 24,
        ]),
        BookInfoChirho::new_chirho("Philipper", "Phil", "Phil", vec![
            30, 30, 21, 23,
        ]),
        BookInfoChirho::new_chirho("Kolosser", "Col", "Col", vec![
            29, 23, 25, 18,
        ]),
        BookInfoChirho::new_chirho("1. Thessalonicher", "1Thess", "1Thess", vec![
            10, 20, 13, 18, 28,
        ]),
        BookInfoChirho::new_chirho("2. Thessalonicher", "2Thess", "2Thess", vec![
            12, 17, 18,
        ]),
        BookInfoChirho::new_chirho("1. Timotheus", "1Tim", "1Tim", vec![
            20, 15, 16, 16, 25, 21,
        ]),
        BookInfoChirho::new_chirho("2. Timotheus", "2Tim", "2Tim", vec![
            18, 26, 17, 22,
        ]),
        BookInfoChirho::new_chirho("Titus", "Titus", "Titus", vec![
            16, 15, 15,
        ]),
        BookInfoChirho::new_chirho("Philemon", "Phlm", "Phlm", vec![
            25,
        ]),
        BookInfoChirho::new_chirho("Hebräer", "Heb", "Heb", vec![
            14, 18, 19, 16, 14, 20, 28, 13, 28, 39,
            40, 29, 25,
        ]),
        BookInfoChirho::new_chirho("Jakobus", "Jas", "Jas", vec![
            27, 26, 18, 17, 20,
        ]),
        BookInfoChirho::new_chirho("1. Petrus", "1Pet", "1Pet", vec![
            25, 25, 22, 19, 14,
        ]),
        BookInfoChirho::new_chirho("2. Petrus", "2Pet", "2Pet", vec![
            21, 22, 18,
        ]),
        BookInfoChirho::new_chirho("1. Johannes", "1John", "1John", vec![
            10, 29, 24, 21, 21,
        ]),
        BookInfoChirho::new_chirho("2. Johannes", "2John", "2John", vec![
            13,
        ]),
        BookInfoChirho::new_chirho("3. Johannes", "3John", "3John", vec![
            14,
        ]),
        BookInfoChirho::new_chirho("Judas", "Jude", "Jude", vec![
            25,
        ]),
        BookInfoChirho::new_chirho("Offenbarung", "Rev", "Rev", vec![
            20, 29, 22, 11, 14, 17, 17, 13, 21, 11,
            19, 17, 18, 20, 8, 21, 18, 24, 21, 15,
            27, 21,
        ]),
    ]
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_luther_ot_books_chirho() {
        let ot_chirho = create_ot_books_chirho();
        assert_eq!(ot_chirho.len(), 39);
        assert_eq!(ot_chirho[0].name_chirho, "1. Mose"); // German Genesis
        assert_eq!(ot_chirho[0].osis_chirho, "Gen");
    }

    #[test]
    fn test_luther_nt_books_chirho() {
        let nt_chirho = create_nt_books_chirho();
        assert_eq!(nt_chirho.len(), 27);
        assert_eq!(nt_chirho[0].name_chirho, "Matthäus"); // German Matthew
    }

    #[test]
    fn test_luther_german_names_chirho() {
        let luther_chirho = create_luther_chirho();
        let lookup_result_chirho = luther_chirho.lookup_book_chirho("Gen");
        assert!(lookup_result_chirho.is_some());
        let (testament_chirho, book_idx_chirho) = lookup_result_chirho.unwrap();
        let genesis_chirho = luther_chirho.get_book_chirho(testament_chirho, book_idx_chirho);
        assert!(genesis_chirho.is_some());
        assert_eq!(genesis_chirho.unwrap().name_chirho, "1. Mose");
    }
}
