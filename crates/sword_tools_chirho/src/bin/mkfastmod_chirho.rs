// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! mkfastmod_chirho - Create search index for a SWORD module.
//!
//! Creates a tantivy-based full-text search index for fast searching.
//!
//! Usage:
//!   mkfastmod_chirho KJV
//!   mkfastmod_chirho -d to delete existing index

use clap::Parser;

use rsword_chirho::manager_chirho::SwMgrChirho;
use rsword_chirho::search_chirho::{SearchEngineChirho, TantivySearchChirho};
use rsword_chirho::versification_chirho::{kjv_chirho, TestamentChirho};
use rsword_chirho::filters_chirho::{FilterChirho, StripFilterChirho};

/// Create search index for a SWORD module.
#[derive(Parser, Debug)]
#[command(name = "mkfastmod_chirho")]
#[command(version)]
#[command(about = "Create search index for a SWORD module")]
struct ArgsChirho {
    /// Module name to index
    module_name_chirho: String,

    /// Delete existing index
    #[arg(short = 'd', long)]
    delete_chirho: bool,

    /// Verbose output
    #[arg(short = 'v', long)]
    verbose_chirho: bool,
}

fn main() {
    env_logger::init();

    let args_chirho = ArgsChirho::parse();

    if let Err(e_chirho) = run_chirho(args_chirho) {
        eprintln!("Error: {}", e_chirho);
        std::process::exit(1);
    }
}

fn run_chirho(args_chirho: ArgsChirho) -> Result<(), Box<dyn std::error::Error>> {
    let mgr_chirho = SwMgrChirho::with_system_paths_chirho()?;

    // Get module data path
    let data_path_chirho = mgr_chirho.get_module_data_path_chirho(&args_chirho.module_name_chirho)
        .ok_or_else(|| format!("Module not found: {}", args_chirho.module_name_chirho))?;

    // Get the parent directory for index storage
    let index_base_chirho = data_path_chirho.parent()
        .unwrap_or(&data_path_chirho);

    let mut search_chirho = TantivySearchChirho::new_chirho(index_base_chirho)?;

    // Handle delete
    if args_chirho.delete_chirho {
        if search_chirho.has_index_chirho() {
            println!("Deleting existing index...");
            rsword_chirho::search_chirho::SearchEngineChirho::delete_index_chirho(&mut search_chirho)?;
            println!("Index deleted.");
        } else {
            println!("No index exists for {}", args_chirho.module_name_chirho);
        }
        return Ok(());
    }

    // Check if index already exists
    if search_chirho.has_index_chirho() {
        println!("Index already exists for {}. Use -d to delete first.", args_chirho.module_name_chirho);
        return Ok(());
    }

    // Load the module
    let module_chirho = mgr_chirho.load_module_chirho(&args_chirho.module_name_chirho)?;

    println!("Creating search index for {}...", args_chirho.module_name_chirho);

    // Create index writer
    let mut writer_chirho = search_chirho.create_writer_chirho()?;

    // Get versification
    let v11n_chirho = kjv_chirho();
    let strip_filter_chirho = StripFilterChirho;

    let mut count_chirho = 0u32;

    // Index Old Testament
    for book_idx_chirho in 0..v11n_chirho.book_count_chirho(TestamentChirho::OldChirho) {
        if let Some(book_chirho) = v11n_chirho.get_book_chirho(TestamentChirho::OldChirho, book_idx_chirho) {
            for chapter_chirho in 1..=book_chirho.chapter_count_chirho {
                if let Some(max_verse_chirho) = book_chirho.max_verse_chirho(chapter_chirho) {
                    for verse_chirho in 1..=max_verse_chirho {
                        let key_chirho = format!("{} {}:{}", book_chirho.osis_chirho, chapter_chirho, verse_chirho);

                        if let Ok(text_chirho) = module_chirho.read_entry_chirho(&key_chirho) {
                            if !text_chirho.is_empty() {
                                // Strip markup for indexing
                                let plain_chirho = strip_filter_chirho.process_chirho(&text_chirho)
                                    .unwrap_or(text_chirho);

                                search_chirho.add_document_chirho(&mut writer_chirho, &key_chirho, &plain_chirho)?;
                                count_chirho += 1;

                                if args_chirho.verbose_chirho && count_chirho % 1000 == 0 {
                                    println!("  Indexed {} entries...", count_chirho);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Index New Testament
    for book_idx_chirho in 0..v11n_chirho.book_count_chirho(TestamentChirho::NewChirho) {
        if let Some(book_chirho) = v11n_chirho.get_book_chirho(TestamentChirho::NewChirho, book_idx_chirho) {
            for chapter_chirho in 1..=book_chirho.chapter_count_chirho {
                if let Some(max_verse_chirho) = book_chirho.max_verse_chirho(chapter_chirho) {
                    for verse_chirho in 1..=max_verse_chirho {
                        let key_chirho = format!("{} {}:{}", book_chirho.osis_chirho, chapter_chirho, verse_chirho);

                        if let Ok(text_chirho) = module_chirho.read_entry_chirho(&key_chirho) {
                            if !text_chirho.is_empty() {
                                // Strip markup for indexing
                                let plain_chirho = strip_filter_chirho.process_chirho(&text_chirho)
                                    .unwrap_or(text_chirho);

                                search_chirho.add_document_chirho(&mut writer_chirho, &key_chirho, &plain_chirho)?;
                                count_chirho += 1;

                                if args_chirho.verbose_chirho && count_chirho % 1000 == 0 {
                                    println!("  Indexed {} entries...", count_chirho);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Commit the index
    search_chirho.commit_chirho(&mut writer_chirho)?;

    println!("Done. Indexed {} entries.", count_chirho);

    Ok(())
}
