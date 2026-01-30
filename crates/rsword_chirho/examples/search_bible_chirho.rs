// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Example: Search a Bible module.
//!
//! This example demonstrates how to:
//! - Load a Bible module
//! - Perform text searches using regex
//! - Display search results with context
//!
//! Usage:
//!   cargo run --example search_bible_chirho [module] [search_term]
//!   cargo run --example search_bible_chirho KJV love
//!   cargo run --example search_bible_chirho KJV "God.*world"

use rsword_chirho::{SwMgrChirho, FilterOptionsChirho};
use rsword_chirho::manager_chirho::module_factory_chirho::OutputFormatChirho;
use std::path::PathBuf;

fn main() {
    let args_chirho: Vec<String> = std::env::args().collect();

    let (module_name_chirho, search_term_chirho) = if args_chirho.len() >= 3 {
        (args_chirho[1].clone(), args_chirho[2].clone())
    } else {
        ("KJV".to_string(), "love".to_string())
    };

    println!("rsword_chirho - Bible Search\n");
    println!("Searching for '{}' in {}\n", search_term_chirho, module_name_chirho);

    // Get SWORD data path
    let sword_path_chirho = match get_sword_path_chirho() {
        Some(path_chirho) => path_chirho,
        None => {
            eprintln!("No SWORD modules found.");
            eprintln!("Install modules to ~/.sword or /usr/share/sword");
            return;
        }
    };

    // Initialize module manager
    let mut mgr_chirho = SwMgrChirho::new_chirho();
    mgr_chirho.add_path_chirho(&sword_path_chirho);

    if let Err(e_chirho) = mgr_chirho.load_modules_chirho() {
        eprintln!("Error loading modules: {:?}", e_chirho);
        return;
    }

    // Load the specified module
    let loaded_chirho = match mgr_chirho.load_module_chirho(&module_name_chirho) {
        Ok(m_chirho) => m_chirho,
        Err(e_chirho) => {
            eprintln!("Error loading module '{}': {:?}", module_name_chirho, e_chirho);
            eprintln!("\nAvailable modules:");
            for name_chirho in mgr_chirho.get_module_names_chirho() {
                eprintln!("  - {}", name_chirho);
            }
            return;
        }
    };

    // Search the module
    search_module_chirho(&loaded_chirho, &search_term_chirho);
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

fn search_module_chirho(
    loaded_chirho: &rsword_chirho::manager_chirho::module_factory_chirho::LoadedModuleChirho,
    search_term_chirho: &str,
) {
    // Create a regex pattern (case-insensitive)
    let pattern_chirho = match regex::RegexBuilder::new(search_term_chirho)
        .case_insensitive(true)
        .build()
    {
        Ok(p_chirho) => p_chirho,
        Err(e_chirho) => {
            eprintln!("Invalid search pattern: {:?}", e_chirho);
            return;
        }
    };

    let filter_options_chirho = FilterOptionsChirho::default();

    // Define books to search
    let books_to_search_chirho: Vec<(&str, u8, u8)> = vec![
        // Search just a few key books for this example
        ("Genesis", 1, 50),
        ("Psalms", 1, 150),
        ("Proverbs", 1, 31),
        ("John", 1, 21),
        ("Romans", 1, 16),
        ("1 John", 1, 5),
    ];

    let mut results_count_chirho = 0;
    let max_results_chirho = 20;

    println!("Results (showing first {} matches):\n", max_results_chirho);

    'outer: for (book_name_chirho, _start_ch_chirho, end_ch_chirho) in &books_to_search_chirho {
        for chapter_chirho in 1..=*end_ch_chirho {
            // Get max verses for this chapter (simplified: assume 50 max)
            let max_verses_chirho = 50;

            for verse_chirho in 1..=max_verses_chirho {
                let ref_chirho = format!("{} {}:{}", book_name_chirho, chapter_chirho, verse_chirho);

                // Try to read the verse
                let text_chirho = match loaded_chirho.read_entry_filtered_chirho(
                    &ref_chirho,
                    OutputFormatChirho::PlainChirho,
                    &filter_options_chirho,
                ) {
                    Ok(t_chirho) => t_chirho,
                    Err(_) => continue, // Skip invalid verses
                };

                if text_chirho.is_empty() {
                    continue;
                }

                // Check if the verse matches
                if pattern_chirho.is_match(&text_chirho) {
                    results_count_chirho += 1;

                    // Highlight the match (simple approach)
                    let display_text_chirho = truncate_text_chirho(&text_chirho, 100);

                    println!("{}:", ref_chirho);
                    println!("  {}\n", display_text_chirho);

                    if results_count_chirho >= max_results_chirho {
                        break 'outer;
                    }
                }
            }
        }
    }

    println!("---");
    if results_count_chirho >= max_results_chirho {
        println!("Showing first {} results. Use a more specific search to narrow results.", max_results_chirho);
    } else {
        println!("Found {} matching verses.", results_count_chirho);
    }
}

fn truncate_text_chirho(text_chirho: &str, max_len_chirho: usize) -> String {
    // Normalize whitespace
    let normalized_chirho: String = text_chirho
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    if normalized_chirho.len() <= max_len_chirho {
        normalized_chirho
    } else {
        format!("{}...", &normalized_chirho[..max_len_chirho])
    }
}
