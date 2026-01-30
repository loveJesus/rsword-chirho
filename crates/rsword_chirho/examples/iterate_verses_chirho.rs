// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Example: Iterate through Bible verses.
//!
//! This example demonstrates how to:
//! - Load a Bible module directly
//! - Navigate through verses using VerseKeyChirho
//! - Iterate through a chapter or book
//! - Process verse text with filters
//!
//! Usage:
//!   cargo run --example iterate_verses_chirho [module] [book] [chapter]
//!   cargo run --example iterate_verses_chirho KJV John 3

use rsword_chirho::{
    kjv_chirho, FilterOptionsChirho,
};
use rsword_chirho::manager_chirho::module_factory_chirho::{load_module_chirho, OutputFormatChirho};
use rsword_chirho::config_chirho::ModuleConfigChirho;
use std::path::PathBuf;

fn main() {
    let args_chirho: Vec<String> = std::env::args().collect();

    let module_name_chirho = args_chirho.get(1).map(|s| s.as_str()).unwrap_or("KJV");
    let book_name_chirho = args_chirho.get(2).map(|s| s.as_str()).unwrap_or("John");
    let chapter_num_chirho: u8 = args_chirho
        .get(3)
        .and_then(|s| s.parse().ok())
        .unwrap_or(3);

    println!("rsword_chirho - Iterate Through Verses\n");
    println!("Module: {}", module_name_chirho);
    println!("Reading: {} {}\n", book_name_chirho, chapter_num_chirho);

    // Get SWORD data path
    let sword_path_chirho = match get_sword_path_chirho() {
        Some(path_chirho) => path_chirho,
        None => {
            eprintln!("No SWORD modules found.");
            return;
        }
    };

    // Load module configuration
    let conf_path_chirho = sword_path_chirho
        .join("mods.d")
        .join(format!("{}.conf", module_name_chirho.to_lowercase()));

    if !conf_path_chirho.exists() {
        eprintln!("Module '{}' not found at: {}", module_name_chirho, conf_path_chirho.display());
        return;
    }

    let config_chirho = match ModuleConfigChirho::from_file_chirho(&conf_path_chirho) {
        Ok(c_chirho) => c_chirho,
        Err(e_chirho) => {
            eprintln!("Error reading module config: {:?}", e_chirho);
            return;
        }
    };

    // Load the module
    let loaded_chirho = match load_module_chirho(&sword_path_chirho, &config_chirho) {
        Ok(m_chirho) => m_chirho,
        Err(e_chirho) => {
            eprintln!("Error loading module: {:?}", e_chirho);
            return;
        }
    };

    // Iterate through the chapter
    iterate_chapter_chirho(&loaded_chirho, book_name_chirho, chapter_num_chirho);
}

fn get_sword_path_chirho() -> Option<PathBuf> {
    if let Ok(home_chirho) = std::env::var("HOME") {
        let sword_home_chirho = PathBuf::from(home_chirho).join(".sword");
        if sword_home_chirho.exists() && sword_home_chirho.join("mods.d").exists() {
            return Some(sword_home_chirho);
        }
    }

    let system_path_chirho = PathBuf::from("/usr/share/sword");
    if system_path_chirho.exists() && system_path_chirho.join("mods.d").exists() {
        return Some(system_path_chirho);
    }

    None
}

fn iterate_chapter_chirho(
    loaded_chirho: &rsword_chirho::manager_chirho::module_factory_chirho::LoadedModuleChirho,
    book_name_chirho: &str,
    chapter_chirho: u8,
) {
    let filter_options_chirho = FilterOptionsChirho::default();
    let v11n_chirho = kjv_chirho();

    // Get chapter info
    let book_info_chirho = v11n_chirho.ot_books_chirho.iter()
        .chain(v11n_chirho.nt_books_chirho.iter())
        .find(|b_chirho| {
            b_chirho.name_chirho.eq_ignore_ascii_case(book_name_chirho) ||
            b_chirho.abbrev_chirho.eq_ignore_ascii_case(book_name_chirho) ||
            b_chirho.osis_chirho.eq_ignore_ascii_case(book_name_chirho)
        });

    let book_full_name_chirho = match book_info_chirho {
        Some(info_chirho) => info_chirho.name_chirho.clone(),
        None => {
            eprintln!("Book '{}' not found in versification.", book_name_chirho);
            eprintln!("\nAvailable books:");
            for book_chirho in &v11n_chirho.ot_books_chirho {
                eprintln!("  OT: {} ({})", book_chirho.name_chirho, book_chirho.abbrev_chirho);
            }
            for book_chirho in &v11n_chirho.nt_books_chirho {
                eprintln!("  NT: {} ({})", book_chirho.name_chirho, book_chirho.abbrev_chirho);
            }
            return;
        }
    };

    println!("{} {}", book_full_name_chirho, chapter_chirho);
    println!("{}\n", "=".repeat(40));

    // Iterate through verses (assume max 176 verses like Psalm 119)
    let mut verse_num_chirho = 1;
    let mut last_successful_chirho = 0;

    while verse_num_chirho <= 176 {
        let ref_chirho = format!("{} {}:{}", book_full_name_chirho, chapter_chirho, verse_num_chirho);

        let text_chirho = match loaded_chirho.read_entry_filtered_chirho(
            &ref_chirho,
            OutputFormatChirho::PlainChirho,
            &filter_options_chirho,
        ) {
            Ok(t_chirho) => t_chirho,
            Err(_) => {
                // If we've had successful verses and now get an error, we're at the end
                if last_successful_chirho > 0 && verse_num_chirho > last_successful_chirho + 1 {
                    break;
                }
                verse_num_chirho += 1;
                continue;
            }
        };

        if text_chirho.is_empty() {
            if last_successful_chirho > 0 && verse_num_chirho > last_successful_chirho + 2 {
                break;
            }
            verse_num_chirho += 1;
            continue;
        }

        last_successful_chirho = verse_num_chirho;

        // Normalize whitespace and print
        let normalized_chirho: String = text_chirho
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");

        println!("{}:{} {}", chapter_chirho, verse_num_chirho, normalized_chirho);

        verse_num_chirho += 1;
    }

    println!("\n---");
    println!("Read {} verses from {} {}.", last_successful_chirho, book_full_name_chirho, chapter_chirho);
}
