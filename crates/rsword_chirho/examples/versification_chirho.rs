// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Example: Explore versification systems.
//!
//! This example demonstrates how to:
//! - Load different versification systems (KJV, Catholic, LXX, etc.)
//! - List books and their chapter counts
//! - Parse and normalize verse references
//! - Calculate verse indices
//!
//! Usage:
//!   cargo run --example versification_chirho

use rsword_chirho::{
    VerseKeyChirho, TestamentChirho, SwKeyChirho,
    kjv_chirho, catholic_chirho, lxx_chirho,
};

fn main() {
    println!("rsword_chirho - Versification Systems\n");

    // Demonstrate different versification systems
    demonstrate_kjv_chirho();
    demonstrate_catholic_chirho();
    demonstrate_lxx_chirho();
    demonstrate_verse_parsing_chirho();
    demonstrate_verse_navigation_chirho();
}

fn demonstrate_kjv_chirho() {
    println!("KJV Versification (Protestant 66-book canon)");
    println!("{}\n", "=".repeat(50));

    let v11n_chirho = kjv_chirho();

    println!("Old Testament: {} books", v11n_chirho.ot_books_chirho.len());
    println!("  First book: {} ({} chapters)",
             v11n_chirho.ot_books_chirho[0].name_chirho,
             v11n_chirho.ot_books_chirho[0].chapter_count_chirho);
    println!("  Last book: {} ({} chapters)",
             v11n_chirho.ot_books_chirho.last().unwrap().name_chirho,
             v11n_chirho.ot_books_chirho.last().unwrap().chapter_count_chirho);

    println!("\nNew Testament: {} books", v11n_chirho.nt_books_chirho.len());
    println!("  First book: {} ({} chapters)",
             v11n_chirho.nt_books_chirho[0].name_chirho,
             v11n_chirho.nt_books_chirho[0].chapter_count_chirho);
    println!("  Last book: {} ({} chapters)",
             v11n_chirho.nt_books_chirho.last().unwrap().name_chirho,
             v11n_chirho.nt_books_chirho.last().unwrap().chapter_count_chirho);

    // Count total verses
    let mut total_verses_chirho = 0;
    for book_chirho in &v11n_chirho.ot_books_chirho {
        for &verses_chirho in &book_chirho.verse_max_chirho {
            total_verses_chirho += verses_chirho as usize;
        }
    }
    for book_chirho in &v11n_chirho.nt_books_chirho {
        for &verses_chirho in &book_chirho.verse_max_chirho {
            total_verses_chirho += verses_chirho as usize;
        }
    }
    println!("\nTotal verses: ~{}", total_verses_chirho);
    println!();
}

fn demonstrate_catholic_chirho() {
    println!("Catholic Versification (includes deuterocanonical books)");
    println!("{}\n", "=".repeat(50));

    let v11n_chirho = catholic_chirho();

    println!("Old Testament: {} books", v11n_chirho.ot_books_chirho.len());
    println!("New Testament: {} books\n", v11n_chirho.nt_books_chirho.len());

    // List deuterocanonical books
    let deutero_books_chirho = ["Tobit", "Judith", "Wisdom", "Sirach", "Baruch", "1 Maccabees", "2 Maccabees"];

    println!("Deuterocanonical books:");
    for name_chirho in &deutero_books_chirho {
        if let Some((testament_chirho, idx_chirho)) = v11n_chirho.lookup_book_chirho(name_chirho) {
            if let Some(book_chirho) = v11n_chirho.get_book_chirho(testament_chirho, idx_chirho) {
                println!("  {} - {} chapters", book_chirho.name_chirho, book_chirho.chapter_count_chirho);
            }
        }
    }
    println!();
}

fn demonstrate_lxx_chirho() {
    println!("LXX Versification (Septuagint)");
    println!("{}\n", "=".repeat(50));

    let v11n_chirho = lxx_chirho();

    println!("Old Testament: {} books", v11n_chirho.ot_books_chirho.len());
    println!("New Testament: {} books\n", v11n_chirho.nt_books_chirho.len());

    // LXX-specific books
    let lxx_books_chirho = ["1 Esdras", "3 Maccabees", "4 Maccabees", "Odes", "Psalms of Solomon"];

    println!("LXX-specific books:");
    for name_chirho in &lxx_books_chirho {
        if let Some((testament_chirho, idx_chirho)) = v11n_chirho.lookup_book_chirho(name_chirho) {
            if let Some(book_chirho) = v11n_chirho.get_book_chirho(testament_chirho, idx_chirho) {
                println!("  {} - {} chapters", book_chirho.name_chirho, book_chirho.chapter_count_chirho);
            }
        }
    }

    // LXX has Psalm 151
    if let Some((testament_chirho, idx_chirho)) = v11n_chirho.lookup_book_chirho("Psalms") {
        if let Some(psalms_chirho) = v11n_chirho.get_book_chirho(testament_chirho, idx_chirho) {
            println!("\nPsalms: {} chapters (includes Psalm 151)", psalms_chirho.chapter_count_chirho);
        }
    }
    println!();
}

fn demonstrate_verse_parsing_chirho() {
    println!("Verse Reference Parsing");
    println!("{}\n", "=".repeat(50));

    let v11n_chirho = kjv_chirho();

    let test_refs_chirho = [
        "John 3:16",
        "Gen 1:1",
        "1 Cor 13:4",
        "III John 1:14",
        "Rev 22:21",
        "Ps 119:105",
        "Matt 28:19-20",
    ];

    for ref_str_chirho in &test_refs_chirho {
        let mut key_chirho = VerseKeyChirho::new_chirho();
        key_chirho.set_versification_chirho(v11n_chirho);

        match key_chirho.parse_chirho(ref_str_chirho) {
            Ok(_) => {
                let testament_chirho = if key_chirho.get_testament_chirho() == 1 { "OT" } else { "NT" };
                println!(
                    "  '{}' -> {} {}:{} ({})",
                    ref_str_chirho,
                    key_chirho.get_book_name_chirho(),
                    key_chirho.get_chapter_chirho(),
                    key_chirho.get_verse_chirho(),
                    testament_chirho
                );
            }
            Err(e_chirho) => {
                println!("  '{}' -> Error: {:?}", ref_str_chirho, e_chirho);
            }
        }
    }
    println!();
}

fn demonstrate_verse_navigation_chirho() {
    println!("Verse Navigation");
    println!("{}\n", "=".repeat(50));

    let v11n_chirho = kjv_chirho();

    // Navigate through John 3
    let mut key_chirho = VerseKeyChirho::new_chirho();
    key_chirho.set_versification_chirho(v11n_chirho);
    key_chirho.parse_chirho("John 3:14").expect("Should parse");

    println!("Starting at: John {}:{}", key_chirho.get_chapter_chirho(), key_chirho.get_verse_chirho());

    // Move forward
    for i_chirho in 1..=5 {
        key_chirho.increment_chirho(1);
        println!(
            "  +{}: John {}:{}",
            i_chirho,
            key_chirho.get_chapter_chirho(),
            key_chirho.get_verse_chirho()
        );
    }

    println!("\nCrossing chapter boundary:");
    key_chirho.parse_chirho("John 3:36").expect("Should parse");
    println!("At: John {}:{}", key_chirho.get_chapter_chirho(), key_chirho.get_verse_chirho());

    key_chirho.increment_chirho(1);
    println!("After +1: John {}:{}", key_chirho.get_chapter_chirho(), key_chirho.get_verse_chirho());

    println!("\nIndex calculation:");
    key_chirho.parse_chirho("Gen 1:1").expect("Should parse");
    let index_chirho = v11n_chirho.calculate_index_chirho(
        TestamentChirho::OldChirho,
        0, // Genesis is book 0
        1, // Chapter 1
        1, // Verse 1
    );
    println!("  Genesis 1:1 index = {:?}", index_chirho);

    key_chirho.parse_chirho("John 3:16").expect("Should parse");
    let nt_index_chirho = v11n_chirho.calculate_index_chirho(
        TestamentChirho::NewChirho,
        3, // John is book 3 in NT
        3, // Chapter 3
        16, // Verse 16
    );
    println!("  John 3:16 index = {:?}", nt_index_chirho);
}
