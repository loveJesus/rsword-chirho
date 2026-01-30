// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Integration tests for rsword_chirho using real SWORD modules.
//!
//! These tests require SWORD modules to be installed in the default location
//! (~/.sword or /usr/share/sword). Tests will be skipped if modules are not available.

use rsword_chirho::{
    SwMgrChirho, SwKeyChirho, VerseKeyChirho, TestamentChirho,
    kjv_chirho, catholic_chirho, lxx_chirho, synodal_chirho,
};
use std::path::PathBuf;

/// Get the SWORD data path if modules are installed.
fn get_sword_path_chirho() -> Option<PathBuf> {
    // Check home directory first
    let home_chirho = std::env::var("HOME").ok()?;
    let sword_home_chirho = PathBuf::from(home_chirho).join(".sword");
    if sword_home_chirho.exists() && sword_home_chirho.join("mods.d").exists() {
        return Some(sword_home_chirho);
    }

    // Check system path
    let system_path_chirho = PathBuf::from("/usr/share/sword");
    if system_path_chirho.exists() && system_path_chirho.join("mods.d").exists() {
        return Some(system_path_chirho);
    }

    None
}

/// Check if a specific module is available.
fn has_module_chirho(name_chirho: &str) -> bool {
    if let Some(path_chirho) = get_sword_path_chirho() {
        let conf_path_chirho = path_chirho.join("mods.d").join(format!("{}.conf", name_chirho.to_lowercase()));
        conf_path_chirho.exists()
    } else {
        false
    }
}

#[test]
fn test_module_manager_creation_chirho() {
    if get_sword_path_chirho().is_none() {
        eprintln!("Skipping test: No SWORD path found");
        return;
    }

    let path_chirho = get_sword_path_chirho().expect("SWORD path should exist");
    let mut mgr_chirho = SwMgrChirho::new_chirho();

    // Add the SWORD path
    mgr_chirho.add_path_chirho(&path_chirho);

    // Load modules
    let result_chirho = mgr_chirho.load_modules_chirho();
    assert!(result_chirho.is_ok(), "Should load modules successfully: {:?}", result_chirho.err());

    // Check that some modules were loaded
    let count_chirho = mgr_chirho.module_count_chirho();
    assert!(count_chirho > 0, "Should have at least one module loaded");
}

#[test]
fn test_kjv_module_available_chirho() {
    if !has_module_chirho("kjv") {
        eprintln!("Skipping test: KJV module not installed");
        return;
    }

    let path_chirho = get_sword_path_chirho().expect("SWORD path should exist");
    let mut mgr_chirho = SwMgrChirho::new_chirho();
    mgr_chirho.add_path_chirho(&path_chirho);
    mgr_chirho.load_modules_chirho().expect("Should load modules");

    // Check that KJV is available
    let has_kjv_chirho = mgr_chirho.has_module_chirho("KJV");
    assert!(has_kjv_chirho, "KJV module should be listed");
}

#[test]
fn test_get_module_names_chirho() {
    if get_sword_path_chirho().is_none() {
        eprintln!("Skipping test: No SWORD path found");
        return;
    }

    let path_chirho = get_sword_path_chirho().expect("SWORD path should exist");
    let mut mgr_chirho = SwMgrChirho::new_chirho();
    mgr_chirho.add_path_chirho(&path_chirho);
    mgr_chirho.load_modules_chirho().expect("Should load modules");

    // Get module names
    let names_chirho = mgr_chirho.get_module_names_chirho();
    assert!(!names_chirho.is_empty(), "Should have module names");

    // Print available modules for debugging
    for name_chirho in &names_chirho {
        eprintln!("Found module: {}", name_chirho);
    }
}

#[test]
fn test_load_kjv_module_chirho() {
    if !has_module_chirho("kjv") {
        eprintln!("Skipping test: KJV module not installed");
        return;
    }

    let path_chirho = get_sword_path_chirho().expect("SWORD path should exist");
    let mut mgr_chirho = SwMgrChirho::new_chirho();
    mgr_chirho.add_path_chirho(&path_chirho);
    mgr_chirho.load_modules_chirho().expect("Should load modules");

    // Try to load the KJV module
    let result_chirho = mgr_chirho.load_module_chirho("KJV");
    assert!(result_chirho.is_ok(), "Should load KJV module: {:?}", result_chirho.err());

    let loaded_chirho = result_chirho.unwrap();
    eprintln!("Loaded KJV module successfully: {}", loaded_chirho.name_chirho);
}

#[test]
fn test_verse_key_parsing_chirho() {
    // Test various verse reference formats
    let test_refs_chirho = vec![
        ("John 3:16", TestamentChirho::NewChirho, 3, 16),
        ("Gen 1:1", TestamentChirho::OldChirho, 1, 1),
        ("Rev 22:21", TestamentChirho::NewChirho, 22, 21),
        ("Ps 119:105", TestamentChirho::OldChirho, 119, 105),
        ("Matt 28:19", TestamentChirho::NewChirho, 28, 19),
    ];

    let v11n_chirho = kjv_chirho();

    for (ref_chirho, expected_testament_chirho, expected_chapter_chirho, expected_verse_chirho) in test_refs_chirho {
        let mut key_chirho = VerseKeyChirho::new_chirho();
        key_chirho.set_versification_chirho(v11n_chirho);
        let result_chirho = key_chirho.parse_chirho(ref_chirho);
        assert!(result_chirho.is_ok(), "Should parse '{}': {:?}", ref_chirho, result_chirho.err());

        assert_eq!(
            key_chirho.testament_enum_chirho(), expected_testament_chirho,
            "Testament for '{}' should match",
            ref_chirho
        );
        assert_eq!(
            key_chirho.get_chapter_chirho(), expected_chapter_chirho,
            "Chapter for '{}' should be {}",
            ref_chirho, expected_chapter_chirho
        );
        assert_eq!(
            key_chirho.get_verse_chirho(), expected_verse_chirho,
            "Verse for '{}' should be {}",
            ref_chirho, expected_verse_chirho
        );
    }
}

#[test]
fn test_verse_key_navigation_chirho() {
    let v11n_chirho = kjv_chirho();

    // Start at John 3:16
    let mut key_chirho = VerseKeyChirho::new_chirho();
    key_chirho.set_versification_chirho(v11n_chirho);
    key_chirho.parse_chirho("John 3:16").expect("Should parse John 3:16");

    // Increment to John 3:17
    key_chirho.increment_chirho(1);
    assert_eq!(key_chirho.get_verse_chirho(), 17, "Should be John 3:17");
    assert_eq!(key_chirho.get_chapter_chirho(), 3, "Should still be chapter 3");

    // Decrement back to John 3:16
    key_chirho.decrement_chirho(1);
    assert_eq!(key_chirho.get_verse_chirho(), 16, "Should be back to John 3:16");

    // Test chapter boundary
    let mut key2_chirho = VerseKeyChirho::new_chirho();
    key2_chirho.set_versification_chirho(v11n_chirho);
    key2_chirho.parse_chirho("John 3:36").expect("Should parse John 3:36");
    key2_chirho.increment_chirho(1);
    assert_eq!(key2_chirho.get_chapter_chirho(), 4, "Should be John 4");
    assert_eq!(key2_chirho.get_verse_chirho(), 1, "Should be verse 1");
}

#[test]
fn test_versification_book_counts_chirho() {
    let v11n_chirho = kjv_chirho();

    // KJV has 39 OT books and 27 NT books
    assert_eq!(v11n_chirho.ot_books_chirho.len(), 39, "KJV should have 39 OT books");
    assert_eq!(v11n_chirho.nt_books_chirho.len(), 27, "KJV should have 27 NT books");

    // First OT book is Genesis
    assert_eq!(v11n_chirho.ot_books_chirho[0].name_chirho, "Genesis");

    // Last OT book is Malachi
    assert_eq!(v11n_chirho.ot_books_chirho[38].name_chirho, "Malachi");

    // First NT book is Matthew
    assert_eq!(v11n_chirho.nt_books_chirho[0].name_chirho, "Matthew");

    // Last NT book is Revelation
    assert_eq!(v11n_chirho.nt_books_chirho[26].name_chirho, "Revelation of John");
}

#[test]
fn test_book_chapter_counts_chirho() {
    let v11n_chirho = kjv_chirho();

    // Genesis has 50 chapters
    assert_eq!(v11n_chirho.ot_books_chirho[0].chapter_count_chirho, 50);

    // Psalms has 150 chapters
    assert_eq!(v11n_chirho.ot_books_chirho[18].chapter_count_chirho, 150);

    // Matthew has 28 chapters
    assert_eq!(v11n_chirho.nt_books_chirho[0].chapter_count_chirho, 28);

    // Revelation has 22 chapters
    assert_eq!(v11n_chirho.nt_books_chirho[26].chapter_count_chirho, 22);
}

#[test]
fn test_book_lookup_variations_chirho() {
    let v11n_chirho = kjv_chirho();

    // Test various book name formats
    let lookups_chirho = vec![
        // Full names
        ("Genesis", true),
        ("Exodus", true),
        ("Matthew", true),
        ("Revelation of John", true),
        // OSIS abbreviations
        ("Gen", true),
        ("Exod", true),
        ("Matt", true),
        ("Rev", true),
        // Numbered books
        ("1 Samuel", true),
        ("2 Kings", true),
        ("1 Corinthians", true),
        ("1John", true),
        // Invalid names
        ("Nonexistent", false),
        ("FakeBook", false),
    ];

    for (name_chirho, should_exist_chirho) in lookups_chirho {
        let result_chirho = v11n_chirho.lookup_book_chirho(name_chirho);
        assert_eq!(
            result_chirho.is_some(),
            should_exist_chirho,
            "Lookup for '{}' should {} exist",
            name_chirho,
            if should_exist_chirho { "" } else { "not" }
        );
    }
}

#[test]
fn test_multiple_versification_systems_chirho() {
    // Test KJV
    let kjv_v11n_chirho = kjv_chirho();
    assert_eq!(kjv_v11n_chirho.name_chirho, "KJV");
    assert_eq!(kjv_v11n_chirho.ot_books_chirho.len(), 39);

    // Test Catholic
    let catholic_v11n_chirho = catholic_chirho();
    assert_eq!(catholic_v11n_chirho.name_chirho, "Catholic");
    assert_eq!(catholic_v11n_chirho.ot_books_chirho.len(), 46); // Has deuterocanonical books

    // Test LXX
    let lxx_v11n_chirho = lxx_chirho();
    assert_eq!(lxx_v11n_chirho.name_chirho, "LXX");
    assert!(lxx_v11n_chirho.ot_books_chirho.len() > 46); // Has even more books

    // Test Synodal
    let synodal_v11n_chirho = synodal_chirho();
    assert_eq!(synodal_v11n_chirho.name_chirho, "Synodal");
    assert_eq!(synodal_v11n_chirho.ot_books_chirho.len(), 39);
}

#[test]
fn test_catholic_deuterocanonical_books_chirho() {
    let catholic_v11n_chirho = catholic_chirho();

    // Check that deuterocanonical books exist
    assert!(catholic_v11n_chirho.lookup_book_chirho("Tobit").is_some(), "Tobit should exist");
    assert!(catholic_v11n_chirho.lookup_book_chirho("Judith").is_some(), "Judith should exist");
    assert!(catholic_v11n_chirho.lookup_book_chirho("Wisdom").is_some(), "Wisdom should exist");
    assert!(catholic_v11n_chirho.lookup_book_chirho("Sirach").is_some(), "Sirach should exist");
    assert!(catholic_v11n_chirho.lookup_book_chirho("Baruch").is_some(), "Baruch should exist");
    assert!(catholic_v11n_chirho.lookup_book_chirho("1 Maccabees").is_some(), "1 Maccabees should exist");
    assert!(catholic_v11n_chirho.lookup_book_chirho("2 Maccabees").is_some(), "2 Maccabees should exist");
}

#[test]
fn test_lxx_unique_books_chirho() {
    let lxx_v11n_chirho = lxx_chirho();

    // Check that LXX-unique books exist
    assert!(lxx_v11n_chirho.lookup_book_chirho("1 Esdras").is_some(), "1 Esdras should exist");
    assert!(lxx_v11n_chirho.lookup_book_chirho("3 Maccabees").is_some(), "3 Maccabees should exist");
    assert!(lxx_v11n_chirho.lookup_book_chirho("4 Maccabees").is_some(), "4 Maccabees should exist");
    assert!(lxx_v11n_chirho.lookup_book_chirho("Odes").is_some(), "Odes should exist");
    assert!(lxx_v11n_chirho.lookup_book_chirho("Psalms of Solomon").is_some(), "Psalms of Solomon should exist");

    // LXX Psalms has 151 psalms
    let psalms_chirho = lxx_v11n_chirho.ot_books_chirho.iter().find(|b_chirho| b_chirho.name_chirho == "Psalms").unwrap();
    assert_eq!(psalms_chirho.chapter_count_chirho, 151, "LXX should have 151 Psalms");
}
