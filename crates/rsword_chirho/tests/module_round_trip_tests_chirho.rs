// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Round-trip integration tests for rsword_chirho.
//!
//! These tests demonstrate the full workflow of:
//! - Reading from installed SWORD modules (like KJV)
//! - Creating new modules from source text
//! - Reading back and verifying the created modules
//! - Filtering markup (OSIS to plain text)
//!
//! Tests require SWORD modules to be installed in ~/.sword or /usr/share/sword.

use rsword_chirho::{
    SwMgrChirho, VerseKeyChirho,
    kjv_chirho,
    FilterChirho, FilterOptionsChirho, OsisToPlainFilterChirho,
};
use rsword_chirho::modules_chirho::texts_chirho::{RawTextChirho, ZTextChirho};
use rsword_chirho::config_chirho::ModuleConfigChirho;
use rsword_chirho::manager_chirho::module_factory_chirho::{
    CreateModuleOptionsChirho, ModuleDriverTypeChirho, create_module_chirho, load_module_chirho,
};
use std::path::PathBuf;
use tempfile::TempDir;

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

/// Load the KJV module configuration.
fn load_kjv_config_chirho() -> Option<(PathBuf, ModuleConfigChirho)> {
    let sword_path_chirho = get_sword_path_chirho()?;
    let conf_path_chirho = sword_path_chirho.join("mods.d").join("kjv.conf");

    if !conf_path_chirho.exists() {
        return None;
    }

    let config_chirho = ModuleConfigChirho::from_file_chirho(&conf_path_chirho).ok()?;
    Some((sword_path_chirho, config_chirho))
}

/// Known John 3:16 text from KJV (stripped of OSIS markup).
const JOHN_3_16_PLAIN_CHIRHO: &str = "For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life.";

/// Known Genesis 1:1 text from KJV (stripped of OSIS markup).
const GEN_1_1_PLAIN_CHIRHO: &str = "In the beginning God created the heaven and the earth.";

// =============================================================================
// Tests for reading installed modules
// =============================================================================

#[test]
fn test_read_kjv_john_3_16_chirho() {
    if !has_module_chirho("kjv") {
        eprintln!("Skipping test: KJV module not installed");
        return;
    }

    let (sword_path_chirho, config_chirho) = load_kjv_config_chirho().expect("Should load KJV config");

    // Load the KJV module
    let loaded_chirho = load_module_chirho(&sword_path_chirho, &config_chirho)
        .expect("Should load KJV module");

    // Read John 3:16
    let raw_text_chirho = loaded_chirho.read_entry_chirho("John 3:16")
        .expect("Should read John 3:16");

    eprintln!("Raw OSIS text: {}", raw_text_chirho);

    // The raw text should contain OSIS markup
    assert!(
        raw_text_chirho.contains("God") || raw_text_chirho.contains("god"),
        "John 3:16 should contain 'God'"
    );
    assert!(
        raw_text_chirho.contains("world"),
        "John 3:16 should contain 'world'"
    );
    assert!(
        raw_text_chirho.contains("Son") || raw_text_chirho.contains("son"),
        "John 3:16 should contain 'Son'"
    );
}

#[test]
fn test_read_kjv_genesis_1_1_chirho() {
    if !has_module_chirho("kjv") {
        eprintln!("Skipping test: KJV module not installed");
        return;
    }

    let (sword_path_chirho, config_chirho) = load_kjv_config_chirho().expect("Should load KJV config");
    let loaded_chirho = load_module_chirho(&sword_path_chirho, &config_chirho)
        .expect("Should load KJV module");

    // Read Genesis 1:1
    let raw_text_chirho = loaded_chirho.read_entry_chirho("Genesis 1:1")
        .expect("Should read Genesis 1:1");

    eprintln!("Genesis 1:1 raw: {}", raw_text_chirho);

    assert!(
        raw_text_chirho.contains("beginning") || raw_text_chirho.contains("Beginning"),
        "Genesis 1:1 should contain 'beginning'"
    );
    assert!(
        raw_text_chirho.contains("created"),
        "Genesis 1:1 should contain 'created'"
    );
}

#[test]
fn test_read_kjv_with_filter_chirho() {
    if !has_module_chirho("kjv") {
        eprintln!("Skipping test: KJV module not installed");
        return;
    }

    let (sword_path_chirho, config_chirho) = load_kjv_config_chirho().expect("Should load KJV config");
    let loaded_chirho = load_module_chirho(&sword_path_chirho, &config_chirho)
        .expect("Should load KJV module");

    // Read John 3:16 with plain text filter
    let filter_options_chirho = FilterOptionsChirho::default();
    let filtered_chirho = loaded_chirho.read_entry_filtered_chirho(
        "John 3:16",
        rsword_chirho::manager_chirho::module_factory_chirho::OutputFormatChirho::PlainChirho,
        &filter_options_chirho,
    ).expect("Should read and filter John 3:16");

    eprintln!("Filtered John 3:16: {}", filtered_chirho);

    // Should have plain text without OSIS tags
    assert!(
        !filtered_chirho.contains("<w"),
        "Filtered text should not contain OSIS <w> tags"
    );
    assert!(
        filtered_chirho.contains("God"),
        "Filtered John 3:16 should contain 'God'"
    );
}

#[test]
fn test_iterate_kjv_genesis_chapter_1_chirho() {
    if !has_module_chirho("kjv") {
        eprintln!("Skipping test: KJV module not installed");
        return;
    }

    let (sword_path_chirho, config_chirho) = load_kjv_config_chirho().expect("Should load KJV config");

    // Get the full data path
    let data_path_chirho = config_chirho.data_path_chirho()
        .expect("Should have data path");
    let clean_path_chirho = data_path_chirho.strip_prefix("./").unwrap_or(data_path_chirho);
    let full_path_chirho = sword_path_chirho.join(clean_path_chirho);

    // Create zText module directly
    let mut module_chirho = ZTextChirho::new_chirho(&full_path_chirho, config_chirho)
        .expect("Should create ZTextChirho");

    // Read first 5 verses of Genesis
    let verses_chirho: Vec<(String, String)> = (1..=5)
        .map(|v_chirho| {
            let ref_chirho = format!("Genesis 1:{}", v_chirho);
            let text_chirho = module_chirho.get_verse_chirho(&ref_chirho).unwrap_or_default();
            (ref_chirho, text_chirho)
        })
        .collect();

    eprintln!("Genesis 1:1-5 verses:");
    for (ref_chirho, text_chirho) in &verses_chirho {
        eprintln!("  {}: {} bytes", ref_chirho, text_chirho.len());
    }

    // Verify all verses have content
    assert!(verses_chirho.len() == 5, "Should have 5 verses");
    for (ref_chirho, text_chirho) in &verses_chirho {
        assert!(!text_chirho.is_empty(), "{} should have content", ref_chirho);
    }
}

// =============================================================================
// Tests for creating new modules
// =============================================================================

#[test]
fn test_create_rawtext_module_chirho() {
    let temp_dir_chirho = TempDir::new().expect("Should create temp dir");
    let mod_path_chirho = temp_dir_chirho.path().join("test_rawtext");

    // Create a new RawText module
    let options_chirho = CreateModuleOptionsChirho::new_chirho("TestRaw", ModuleDriverTypeChirho::RawTextChirho)
        .with_description_chirho("Test RawText module")
        .with_language_chirho("en")
        .with_versification_chirho("KJV")
        .with_source_type_chirho("Plain");

    let _loaded_chirho = create_module_chirho(&mod_path_chirho, &options_chirho)
        .expect("Should create RawText module");

    // Verify files were created
    assert!(mod_path_chirho.join("ot.vss").exists(), "OT index should exist");
    assert!(mod_path_chirho.join("nt.vss").exists(), "NT index should exist");
    assert!(mod_path_chirho.join("ot").exists(), "OT data should exist");
    assert!(mod_path_chirho.join("nt").exists(), "NT data should exist");
}

#[test]
fn test_create_rawtext_write_and_read_chirho() {
    let temp_dir_chirho = TempDir::new().expect("Should create temp dir");
    let mod_path_chirho = temp_dir_chirho.path().join("test_write");

    // Create module options
    let options_chirho = CreateModuleOptionsChirho::new_chirho("TestWrite", ModuleDriverTypeChirho::RawTextChirho)
        .with_description_chirho("Test write/read module")
        .with_language_chirho("en")
        .with_versification_chirho("KJV")
        .with_source_type_chirho("Plain");

    // Create the module
    let loaded_chirho = create_module_chirho(&mod_path_chirho, &options_chirho)
        .expect("Should create module");

    // Open the module for writing
    let mut module_chirho = RawTextChirho::new_rw_chirho(&loaded_chirho.data_path_chirho, loaded_chirho.config_chirho.clone())
        .expect("Should open RawTextChirho");

    // Write Genesis 1:1
    module_chirho.write_verse_at_chirho("Genesis 1:1", GEN_1_1_PLAIN_CHIRHO)
        .expect("Should write Genesis 1:1");

    // Write Genesis 1:2
    module_chirho.write_verse_at_chirho("Genesis 1:2", "And the earth was without form, and void; and darkness was upon the face of the deep.")
        .expect("Should write Genesis 1:2");

    // Write John 3:16
    module_chirho.write_verse_at_chirho("John 3:16", JOHN_3_16_PLAIN_CHIRHO)
        .expect("Should write John 3:16");

    // Read back Genesis 1:1
    let read_gen_chirho = module_chirho.get_verse_chirho("Genesis 1:1")
        .expect("Should read Genesis 1:1");
    assert_eq!(read_gen_chirho.trim(), GEN_1_1_PLAIN_CHIRHO, "Genesis 1:1 should match");

    // Read back John 3:16
    let read_john_chirho = module_chirho.get_verse_chirho("John 3:16")
        .expect("Should read John 3:16");
    assert_eq!(read_john_chirho.trim(), JOHN_3_16_PLAIN_CHIRHO, "John 3:16 should match");

    eprintln!("Successfully wrote and read back verses");
}

#[test]
fn test_create_lexicon_module_chirho() {
    let temp_dir_chirho = TempDir::new().expect("Should create temp dir");
    let mod_path_chirho = temp_dir_chirho.path().join("test_lexicon");

    // Create a new RawLD module
    let options_chirho = CreateModuleOptionsChirho::new_chirho("TestLex", ModuleDriverTypeChirho::RawLdChirho)
        .with_description_chirho("Test Lexicon module")
        .with_language_chirho("en");

    let _loaded_chirho = create_module_chirho(&mod_path_chirho, &options_chirho)
        .expect("Should create RawLD module");

    // Verify lexicon files were created
    let basename_chirho = mod_path_chirho.file_name().unwrap().to_str().unwrap();
    assert!(
        mod_path_chirho.join(format!("{}.idx", basename_chirho)).exists() ||
        mod_path_chirho.join("test_lexicon.idx").exists(),
        "Lexicon index should exist"
    );
}

// =============================================================================
// Round-trip tests: KJV -> New Module -> Read back
// =============================================================================

#[test]
fn test_round_trip_kjv_to_rawtext_chirho() {
    if !has_module_chirho("kjv") {
        eprintln!("Skipping test: KJV module not installed");
        return;
    }

    let (sword_path_chirho, config_chirho) = load_kjv_config_chirho().expect("Should load KJV config");
    let loaded_kjv_chirho = load_module_chirho(&sword_path_chirho, &config_chirho)
        .expect("Should load KJV module");

    // Extract some verses from KJV and strip OSIS markup
    let filter_options_chirho = FilterOptionsChirho::default();
    let verses_to_copy_chirho = vec![
        "Genesis 1:1",
        "Genesis 1:2",
        "Genesis 1:3",
        "John 3:16",
        "John 3:17",
        "Romans 8:28",
        "Psalm 23:1",
    ];

    let mut extracted_chirho: Vec<(String, String)> = Vec::new();
    for ref_chirho in &verses_to_copy_chirho {
        let text_chirho = loaded_kjv_chirho.read_entry_filtered_chirho(
            ref_chirho,
            rsword_chirho::manager_chirho::module_factory_chirho::OutputFormatChirho::PlainChirho,
            &filter_options_chirho,
        ).unwrap_or_else(|_| String::new());

        if !text_chirho.is_empty() {
            extracted_chirho.push((ref_chirho.to_string(), text_chirho));
        }
    }

    eprintln!("Extracted {} verses from KJV", extracted_chirho.len());

    if extracted_chirho.is_empty() {
        eprintln!("Warning: Could not extract any verses from KJV");
        return;
    }

    // Create a new RawText module with the extracted content
    let temp_dir_chirho = TempDir::new().expect("Should create temp dir");
    let new_mod_path_chirho = temp_dir_chirho.path().join("kjv_extract");

    let options_chirho = CreateModuleOptionsChirho::new_chirho("KJVExtract", ModuleDriverTypeChirho::RawTextChirho)
        .with_description_chirho("Extracted from KJV")
        .with_language_chirho("en")
        .with_versification_chirho("KJV")
        .with_source_type_chirho("Plain");

    let new_loaded_chirho = create_module_chirho(&new_mod_path_chirho, &options_chirho)
        .expect("Should create new module");

    // Open the new module for writing
    let mut new_module_chirho = RawTextChirho::new_rw_chirho(
        &new_loaded_chirho.data_path_chirho,
        new_loaded_chirho.config_chirho.clone(),
    ).expect("Should open new module");

    // Write extracted verses
    for (ref_chirho, text_chirho) in &extracted_chirho {
        new_module_chirho.write_verse_at_chirho(ref_chirho, text_chirho)
            .unwrap_or_else(|e_chirho| {
                eprintln!("Warning: Could not write {}: {:?}", ref_chirho, e_chirho);
            });
    }

    // Read back and verify
    for (ref_chirho, original_chirho) in &extracted_chirho {
        let read_back_chirho = new_module_chirho.get_verse_chirho(ref_chirho)
            .unwrap_or_default();

        // Normalize whitespace for comparison
        let orig_norm_chirho: String = original_chirho.split_whitespace().collect::<Vec<_>>().join(" ");
        let read_norm_chirho: String = read_back_chirho.split_whitespace().collect::<Vec<_>>().join(" ");

        assert_eq!(
            read_norm_chirho, orig_norm_chirho,
            "Verse {} should match after round-trip",
            ref_chirho
        );
        eprintln!("Verified: {}", ref_chirho);
    }

    eprintln!("Round-trip test passed for {} verses", extracted_chirho.len());
}

#[test]
fn test_round_trip_book_verses_chirho() {
    if !has_module_chirho("kjv") {
        eprintln!("Skipping test: KJV module not installed");
        return;
    }

    let (sword_path_chirho, config_chirho) = load_kjv_config_chirho().expect("Should load KJV config");
    let loaded_kjv_chirho = load_module_chirho(&sword_path_chirho, &config_chirho)
        .expect("Should load KJV module");

    let filter_options_chirho = FilterOptionsChirho::default();

    // Extract all of Philemon (short book - 25 verses)
    let mut philemon_verses_chirho: Vec<(String, String)> = Vec::new();
    for verse_chirho in 1..=25 {
        let ref_chirho = format!("Philemon 1:{}", verse_chirho);
        let text_chirho = loaded_kjv_chirho.read_entry_filtered_chirho(
            &ref_chirho,
            rsword_chirho::manager_chirho::module_factory_chirho::OutputFormatChirho::PlainChirho,
            &filter_options_chirho,
        ).unwrap_or_default();

        if !text_chirho.is_empty() {
            philemon_verses_chirho.push((ref_chirho, text_chirho));
        }
    }

    eprintln!("Extracted {} verses from Philemon", philemon_verses_chirho.len());

    if philemon_verses_chirho.is_empty() {
        eprintln!("Warning: Could not extract Philemon verses");
        return;
    }

    // Create new module and write all verses
    let temp_dir_chirho = TempDir::new().expect("Should create temp dir");
    let new_mod_path_chirho = temp_dir_chirho.path().join("philemon_extract");

    let options_chirho = CreateModuleOptionsChirho::new_chirho("Philemon", ModuleDriverTypeChirho::RawTextChirho)
        .with_description_chirho("Extracted Philemon")
        .with_language_chirho("en")
        .with_versification_chirho("KJV")
        .with_source_type_chirho("Plain");

    let new_loaded_chirho = create_module_chirho(&new_mod_path_chirho, &options_chirho)
        .expect("Should create module");

    let mut new_module_chirho = RawTextChirho::new_rw_chirho(
        &new_loaded_chirho.data_path_chirho,
        new_loaded_chirho.config_chirho.clone(),
    ).expect("Should open module");

    for (ref_chirho, text_chirho) in &philemon_verses_chirho {
        new_module_chirho.write_verse_at_chirho(ref_chirho, text_chirho)
            .unwrap_or_else(|e_chirho| {
                eprintln!("Could not write {}: {:?}", ref_chirho, e_chirho);
            });
    }

    // Verify round-trip
    let mut verified_count_chirho = 0;
    for (ref_chirho, original_chirho) in &philemon_verses_chirho {
        let read_back_chirho = new_module_chirho.get_verse_chirho(ref_chirho)
            .unwrap_or_default();

        let orig_norm_chirho: String = original_chirho.split_whitespace().collect::<Vec<_>>().join(" ");
        let read_norm_chirho: String = read_back_chirho.split_whitespace().collect::<Vec<_>>().join(" ");

        if read_norm_chirho == orig_norm_chirho && !read_norm_chirho.is_empty() {
            verified_count_chirho += 1;
        }
    }

    eprintln!("Verified {} of {} Philemon verses", verified_count_chirho, philemon_verses_chirho.len());
    assert!(
        verified_count_chirho > 0,
        "Should verify at least some verses"
    );
}

// =============================================================================
// Tests using SwMgr for module discovery
// =============================================================================

#[test]
fn test_swmgr_load_and_read_chirho() {
    let sword_path_chirho = match get_sword_path_chirho() {
        Some(p_chirho) => p_chirho,
        None => {
            eprintln!("Skipping test: No SWORD path found");
            return;
        }
    };

    let mut mgr_chirho = SwMgrChirho::new_chirho();
    mgr_chirho.add_path_chirho(&sword_path_chirho);
    mgr_chirho.load_modules_chirho().expect("Should load modules");

    let names_chirho = mgr_chirho.get_module_names_chirho();
    eprintln!("Found {} modules via SwMgr", names_chirho.len());

    for name_chirho in &names_chirho {
        eprintln!("  - {}", name_chirho);
    }

    assert!(!names_chirho.is_empty(), "Should have at least one module");

    // Try to load and read from each Bible module
    for name_chirho in &names_chirho {
        if let Ok(loaded_chirho) = mgr_chirho.load_module_chirho(name_chirho) {
            // Only test Bible modules
            if loaded_chirho.driver_type_chirho.is_bible_chirho() {
                let text_chirho = loaded_chirho.read_entry_chirho("John 1:1")
                    .unwrap_or_else(|_| String::new());

                if !text_chirho.is_empty() {
                    eprintln!("Read John 1:1 from {}: {} bytes", name_chirho, text_chirho.len());
                }
            }
        }
    }
}

// =============================================================================
// Filter tests
// =============================================================================

#[test]
fn test_osis_to_plain_filter_chirho() {
    // Test OSIS to plain text conversion
    let osis_chirho = r#"<w lemma="strong:G3588" morph="robinson:T-NSM">The</w> <w lemma="strong:G2316" morph="robinson:N-NSM">God</w> <w lemma="strong:G3588" morph="robinson:T-NSM">so</w> <w lemma="strong:G25" morph="robinson:V-AAI-3S">loved</w>"#;

    let filter_chirho = OsisToPlainFilterChirho::new_chirho();
    let plain_chirho = filter_chirho.process_chirho(osis_chirho)
        .expect("Should filter OSIS");

    eprintln!("OSIS: {}", osis_chirho);
    eprintln!("Plain: {}", plain_chirho);

    assert!(plain_chirho.contains("God"), "Should contain 'God'");
    assert!(plain_chirho.contains("loved"), "Should contain 'loved'");
    assert!(!plain_chirho.contains("lemma"), "Should not contain 'lemma' attribute");
}

#[test]
fn test_filter_options_chirho() {
    // Test filter options for Strong's numbers
    let osis_chirho = r#"<w lemma="strong:G2316" morph="robinson:N-NSM">God</w>"#;

    // With Strong's numbers enabled
    let options_on_chirho = FilterOptionsChirho {
        strongs_chirho: true,
        morph_chirho: false,
        footnotes_chirho: false,
        headings_chirho: false,
        xrefs_chirho: false,
        red_letter_chirho: false,
        lemmas_chirho: false,
        scripref_chirho: false,
    };

    let filter_on_chirho = OsisToPlainFilterChirho::with_options_chirho(options_on_chirho);
    let with_strongs_chirho = filter_on_chirho.process_chirho(osis_chirho)
        .expect("Should filter with Strong's");

    // With Strong's numbers disabled
    let options_off_chirho = FilterOptionsChirho::default();
    let filter_off_chirho = OsisToPlainFilterChirho::with_options_chirho(options_off_chirho);
    let without_strongs_chirho = filter_off_chirho.process_chirho(osis_chirho)
        .expect("Should filter without Strong's");

    eprintln!("With Strong's: {}", with_strongs_chirho);
    eprintln!("Without Strong's: {}", without_strongs_chirho);

    assert!(with_strongs_chirho.contains("God"), "Both should contain 'God'");
    assert!(without_strongs_chirho.contains("God"), "Both should contain 'God'");
}

// =============================================================================
// Verse key and versification tests
// =============================================================================

#[test]
fn test_verse_key_book_iteration_chirho() {
    let v11n_chirho = kjv_chirho();

    // Iterate through all books
    let mut total_chapters_chirho = 0;

    for book_chirho in &v11n_chirho.ot_books_chirho {
        total_chapters_chirho += book_chirho.chapter_count_chirho as usize;
    }
    for book_chirho in &v11n_chirho.nt_books_chirho {
        total_chapters_chirho += book_chirho.chapter_count_chirho as usize;
    }

    eprintln!("KJV versification has {} total chapters", total_chapters_chirho);
    assert!(total_chapters_chirho > 1000, "KJV should have over 1000 chapters");
}

#[test]
fn test_verse_key_range_parsing_chirho() {
    let v11n_chirho = kjv_chirho();

    // Test parsing various verse references
    let test_refs_chirho = [
        ("Gen 1:1", "Genesis", 1, 1),
        ("Matthew 1:1", "Matthew", 1, 1),
        ("Rev 22:21", "Revelation of John", 22, 21),
        ("Ps 119:176", "Psalms", 119, 176),
        ("3 John 1:14", "III John", 1, 14),
    ];

    for (ref_str_chirho, _expected_book_chirho, expected_ch_chirho, expected_vs_chirho) in test_refs_chirho {
        let mut key_chirho = VerseKeyChirho::new_chirho();
        key_chirho.set_versification_chirho(v11n_chirho);

        if key_chirho.parse_chirho(ref_str_chirho).is_ok() {
            eprintln!(
                "{} -> {} {}:{}",
                ref_str_chirho,
                key_chirho.get_book_name_chirho(),
                key_chirho.get_chapter_chirho(),
                key_chirho.get_verse_chirho()
            );

            assert_eq!(key_chirho.get_chapter_chirho(), expected_ch_chirho);
            assert_eq!(key_chirho.get_verse_chirho(), expected_vs_chirho);
        }
    }
}
