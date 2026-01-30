// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Locale data structures and built-in locale definitions.

use std::collections::HashMap;

/// A localized book name with abbreviations.
#[derive(Debug, Clone)]
pub struct LocaleBookChirho {
    /// OSIS ID of the book (e.g., "Gen", "Matt").
    pub osis_chirho: String,
    /// Full localized name (e.g., "Genesis", "Génesis").
    pub name_chirho: String,
    /// Short abbreviation (e.g., "Gen", "Gn").
    pub abbrev_chirho: String,
    /// Alternative abbreviations.
    pub alt_abbrevs_chirho: Vec<String>,
}

impl LocaleBookChirho {
    /// Create a new locale book entry.
    pub fn new_chirho(osis_chirho: &str, name_chirho: &str, abbrev_chirho: &str) -> Self {
        Self {
            osis_chirho: osis_chirho.to_string(),
            name_chirho: name_chirho.to_string(),
            abbrev_chirho: abbrev_chirho.to_string(),
            alt_abbrevs_chirho: Vec::new(),
        }
    }

    /// Create with alternative abbreviations.
    pub fn with_alts_chirho(osis_chirho: &str, name_chirho: &str, abbrev_chirho: &str, alts_chirho: &[&str]) -> Self {
        Self {
            osis_chirho: osis_chirho.to_string(),
            name_chirho: name_chirho.to_string(),
            abbrev_chirho: abbrev_chirho.to_string(),
            alt_abbrevs_chirho: alts_chirho.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// Check if a given name or abbreviation matches this book.
    pub fn matches_chirho(&self, name_chirho: &str) -> bool {
        let lower_chirho = name_chirho.to_lowercase();
        self.name_chirho.to_lowercase() == lower_chirho
            || self.abbrev_chirho.to_lowercase() == lower_chirho
            || self.osis_chirho.to_lowercase() == lower_chirho
            || self.alt_abbrevs_chirho.iter().any(|a| a.to_lowercase() == lower_chirho)
    }
}

/// A locale definition with book names and other strings.
#[derive(Debug, Clone)]
pub struct LocaleChirho {
    /// Locale code (e.g., "en", "de", "es").
    pub code_chirho: String,
    /// Locale description (e.g., "English", "German").
    pub description_chirho: String,
    /// Book definitions indexed by OSIS ID.
    pub books_chirho: HashMap<String, LocaleBookChirho>,
    /// Book order (list of OSIS IDs).
    pub book_order_chirho: Vec<String>,
}

impl LocaleChirho {
    /// Create a new empty locale.
    pub fn new_chirho(code_chirho: &str, description_chirho: &str) -> Self {
        Self {
            code_chirho: code_chirho.to_string(),
            description_chirho: description_chirho.to_string(),
            books_chirho: HashMap::new(),
            book_order_chirho: Vec::new(),
        }
    }

    /// Add a book to the locale.
    pub fn add_book_chirho(&mut self, book_chirho: LocaleBookChirho) {
        self.book_order_chirho.push(book_chirho.osis_chirho.clone());
        self.books_chirho.insert(book_chirho.osis_chirho.clone(), book_chirho);
    }

    /// Get a book by OSIS ID.
    pub fn get_book_chirho(&self, osis_chirho: &str) -> Option<&LocaleBookChirho> {
        self.books_chirho.get(osis_chirho)
    }

    /// Get localized book name from OSIS ID.
    pub fn get_book_name_chirho(&self, osis_chirho: &str) -> Option<&str> {
        self.books_chirho.get(osis_chirho).map(|b| b.name_chirho.as_str())
    }

    /// Get book abbreviation from OSIS ID.
    pub fn get_book_abbrev_chirho(&self, osis_chirho: &str) -> Option<&str> {
        self.books_chirho.get(osis_chirho).map(|b| b.abbrev_chirho.as_str())
    }

    /// Look up a book by any name or abbreviation.
    pub fn lookup_book_chirho(&self, name_chirho: &str) -> Option<&LocaleBookChirho> {
        self.books_chirho.values().find(|b| b.matches_chirho(name_chirho))
    }

    /// Format a verse reference in this locale.
    pub fn format_reference_chirho(&self, osis_chirho: &str, chapter_chirho: u16, verse_chirho: u16) -> String {
        let book_name_chirho = self.get_book_name_chirho(osis_chirho)
            .unwrap_or(osis_chirho);
        format!("{} {}:{}", book_name_chirho, chapter_chirho, verse_chirho)
    }

    /// Format a verse reference with abbreviation.
    pub fn format_reference_short_chirho(&self, osis_chirho: &str, chapter_chirho: u16, verse_chirho: u16) -> String {
        let abbrev_chirho = self.get_book_abbrev_chirho(osis_chirho)
            .unwrap_or(osis_chirho);
        format!("{} {}:{}", abbrev_chirho, chapter_chirho, verse_chirho)
    }
}

/// Create the built-in English locale.
pub fn create_english_locale_chirho() -> LocaleChirho {
    let mut locale_chirho = LocaleChirho::new_chirho("en", "English");

    // Old Testament
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Gen", "Genesis", "Gen", &["Gn"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Exod", "Exodus", "Exod", &["Ex"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Lev", "Leviticus", "Lev", &["Lv"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Num", "Numbers", "Num", &["Nm"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Deut", "Deuteronomy", "Deut", &["Dt"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Josh", "Joshua", "Josh", &["Jos"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Judg", "Judges", "Judg", &["Jdg"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Ruth", "Ruth", "Ruth", &["Ru"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("1Sam", "1 Samuel", "1Sam", &["1Sm", "1 Sm"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("2Sam", "2 Samuel", "2Sam", &["2Sm", "2 Sm"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("1Kgs", "1 Kings", "1Kgs", &["1Ki", "1 Ki"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("2Kgs", "2 Kings", "2Kgs", &["2Ki", "2 Ki"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("1Chr", "1 Chronicles", "1Chr", &["1Ch", "1 Ch"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("2Chr", "2 Chronicles", "2Chr", &["2Ch", "2 Ch"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Ezra", "Ezra", "Ezra", &["Ezr"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Neh", "Nehemiah", "Neh", &["Ne"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Esth", "Esther", "Esth", &["Est"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Job", "Job", "Job", &["Jb"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Ps", "Psalms", "Ps", &["Pss", "Psalm"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Prov", "Proverbs", "Prov", &["Pr"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Eccl", "Ecclesiastes", "Eccl", &["Ec", "Qoh"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Song", "Song of Solomon", "Song", &["SS", "Cant", "SoS"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Isa", "Isaiah", "Isa", &["Is"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Jer", "Jeremiah", "Jer", &["Je"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Lam", "Lamentations", "Lam", &["La"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Ezek", "Ezekiel", "Ezek", &["Eze"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Dan", "Daniel", "Dan", &["Da"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Hos", "Hosea", "Hos", &["Ho"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Joel", "Joel", "Joel", &["Jl"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Amos", "Amos", "Amos", &["Am"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Obad", "Obadiah", "Obad", &["Ob"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Jonah", "Jonah", "Jonah", &["Jon"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Mic", "Micah", "Mic", &["Mi"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Nah", "Nahum", "Nah", &["Na"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Hab", "Habakkuk", "Hab", &["Hb"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Zeph", "Zephaniah", "Zeph", &["Zep"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Hag", "Haggai", "Hag", &["Hg"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Zech", "Zechariah", "Zech", &["Zec"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Mal", "Malachi", "Mal", &["Ml"]));

    // New Testament
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Matt", "Matthew", "Matt", &["Mt"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Mark", "Mark", "Mark", &["Mk"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Luke", "Luke", "Luke", &["Lk"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("John", "John", "John", &["Jn"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Acts", "Acts", "Acts", &["Ac"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Rom", "Romans", "Rom", &["Ro"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("1Cor", "1 Corinthians", "1Cor", &["1Co", "1 Co"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("2Cor", "2 Corinthians", "2Cor", &["2Co", "2 Co"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Gal", "Galatians", "Gal", &["Ga"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Eph", "Ephesians", "Eph", &["Ep"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Phil", "Philippians", "Phil", &["Php"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Col", "Colossians", "Col", &["Co"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("1Thess", "1 Thessalonians", "1Thess", &["1Th", "1 Th"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("2Thess", "2 Thessalonians", "2Thess", &["2Th", "2 Th"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("1Tim", "1 Timothy", "1Tim", &["1Ti", "1 Ti"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("2Tim", "2 Timothy", "2Tim", &["2Ti", "2 Ti"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Titus", "Titus", "Titus", &["Tit"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Phlm", "Philemon", "Phlm", &["Phm"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Heb", "Hebrews", "Heb", &["He"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Jas", "James", "Jas", &["Jm"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("1Pet", "1 Peter", "1Pet", &["1Pe", "1 Pe"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("2Pet", "2 Peter", "2Pet", &["2Pe", "2 Pe"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("1John", "1 John", "1John", &["1Jn", "1 Jn"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("2John", "2 John", "2John", &["2Jn", "2 Jn"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("3John", "3 John", "3John", &["3Jn", "3 Jn"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Jude", "Jude", "Jude", &["Jd"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Rev", "Revelation", "Rev", &["Re", "Apoc"]));

    locale_chirho
}

/// Create the Spanish locale.
pub fn create_spanish_locale_chirho() -> LocaleChirho {
    let mut locale_chirho = LocaleChirho::new_chirho("es", "Spanish");

    // Old Testament (selected books)
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Gen", "Génesis", "Gn", &["Gen"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Exod", "Éxodo", "Ex", &["Exod"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Lev", "Levítico", "Lv", &["Lev"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Num", "Números", "Nm", &["Num"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Deut", "Deuteronomio", "Dt", &["Deut"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Ps", "Salmos", "Sal", &["Ps"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Isa", "Isaías", "Is", &["Isa"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Jer", "Jeremías", "Jr", &["Jer"]));

    // New Testament (selected books)
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Matt", "Mateo", "Mt", &["Mat"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Mark", "Marcos", "Mc", &["Mar"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Luke", "Lucas", "Lc", &["Luc"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("John", "Juan", "Jn", &["Jua"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Acts", "Hechos", "Hch", &["Hec"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Rom", "Romanos", "Ro", &["Rom"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Rev", "Apocalipsis", "Ap", &["Apoc"]));

    locale_chirho
}

/// Create the German locale.
pub fn create_german_locale_chirho() -> LocaleChirho {
    let mut locale_chirho = LocaleChirho::new_chirho("de", "German");

    // Old Testament (selected books)
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Gen", "1. Mose", "1Mo", &["Gen", "1.Mose"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Exod", "2. Mose", "2Mo", &["Ex", "2.Mose"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Lev", "3. Mose", "3Mo", &["Lev", "3.Mose"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Num", "4. Mose", "4Mo", &["Num", "4.Mose"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Deut", "5. Mose", "5Mo", &["Dtn", "5.Mose"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Ps", "Psalmen", "Ps", &["Pss"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Isa", "Jesaja", "Jes", &["Isa"]));

    // New Testament (selected books)
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Matt", "Matthäus", "Mt", &["Mat"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Mark", "Markus", "Mk", &["Mar"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Luke", "Lukas", "Lk", &["Luk"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("John", "Johannes", "Joh", &["Jn"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Acts", "Apostelgeschichte", "Apg", &["Acts"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Rom", "Römer", "Röm", &["Rom"]));
    locale_chirho.add_book_chirho(LocaleBookChirho::with_alts_chirho("Rev", "Offenbarung", "Offb", &["Off", "Apoc"]));

    locale_chirho
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_locale_book_matches_chirho() {
        let book_chirho = LocaleBookChirho::with_alts_chirho("Gen", "Genesis", "Gen", &["Gn"]);
        assert!(book_chirho.matches_chirho("Genesis"));
        assert!(book_chirho.matches_chirho("Gen"));
        assert!(book_chirho.matches_chirho("Gn"));
        assert!(book_chirho.matches_chirho("genesis"));
        assert!(!book_chirho.matches_chirho("Exodus"));
    }

    #[test]
    fn test_english_locale_books_chirho() {
        let locale_chirho = create_english_locale_chirho();
        assert_eq!(locale_chirho.get_book_name_chirho("Gen"), Some("Genesis"));
        assert_eq!(locale_chirho.get_book_name_chirho("John"), Some("John"));
        assert_eq!(locale_chirho.get_book_name_chirho("Rev"), Some("Revelation"));
    }

    #[test]
    fn test_spanish_locale_chirho() {
        let locale_chirho = create_spanish_locale_chirho();
        assert_eq!(locale_chirho.get_book_name_chirho("Gen"), Some("Génesis"));
        assert_eq!(locale_chirho.get_book_name_chirho("John"), Some("Juan"));
    }

    #[test]
    fn test_german_locale_chirho() {
        let locale_chirho = create_german_locale_chirho();
        assert_eq!(locale_chirho.get_book_name_chirho("Gen"), Some("1. Mose"));
        assert_eq!(locale_chirho.get_book_name_chirho("John"), Some("Johannes"));
    }

    #[test]
    fn test_format_reference_chirho() {
        let locale_chirho = create_english_locale_chirho();
        assert_eq!(locale_chirho.format_reference_chirho("John", 3, 16), "John 3:16");

        let spanish_chirho = create_spanish_locale_chirho();
        assert_eq!(spanish_chirho.format_reference_chirho("John", 3, 16), "Juan 3:16");
    }

    #[test]
    fn test_lookup_book_chirho() {
        let locale_chirho = create_english_locale_chirho();
        let book_chirho = locale_chirho.lookup_book_chirho("Gn").unwrap();
        assert_eq!(book_chirho.osis_chirho, "Gen");
    }
}
