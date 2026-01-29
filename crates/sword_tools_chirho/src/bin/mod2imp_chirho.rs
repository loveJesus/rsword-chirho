// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! mod2imp_chirho - Export SWORD module to IMP format.
//!
//! Exports a SWORD module to the IMP (SWORD Import) text format.
//!
//! Usage:
//!   mod2imp_chirho KJV > kjv.imp
//!   mod2imp_chirho -r "Gen-Exod" KJV > genesis_exodus.imp

use clap::Parser;

use rsword_chirho::manager_chirho::SwMgrChirho;
use rsword_chirho::versification_chirho::{kjv_chirho, TestamentChirho};

/// Export SWORD module to IMP format.
#[derive(Parser, Debug)]
#[command(name = "mod2imp_chirho")]
#[command(version)]
#[command(about = "Export SWORD module to IMP format")]
struct ArgsChirho {
    /// Module name to export
    module_name_chirho: String,

    /// Include intro verses
    #[arg(short = 'i', long)]
    intros_chirho: bool,

    /// Range to export (e.g., "Gen-Rev", "Matt-John")
    #[arg(short = 'r', long)]
    range_chirho: Option<String>,

    /// Output raw text (no filtering)
    #[arg(long)]
    raw_chirho: bool,

    /// Verbose output to stderr
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

    // Load the module
    let module_chirho = mgr_chirho.load_module_chirho(&args_chirho.module_name_chirho)?;

    if args_chirho.verbose_chirho {
        eprintln!("Exporting module: {}", args_chirho.module_name_chirho);
    }

    // Get versification
    let v11n_chirho = kjv_chirho();

    let mut count_chirho = 0u32;

    // Export Old Testament
    for book_idx_chirho in 0..v11n_chirho.book_count_chirho(TestamentChirho::OldChirho) {
        if let Some(book_chirho) = v11n_chirho.get_book_chirho(TestamentChirho::OldChirho, book_idx_chirho) {
            // Export book intro if requested
            if args_chirho.intros_chirho {
                let intro_key_chirho = format!("{} 0:0", book_chirho.osis_chirho);
                if let Ok(text_chirho) = module_chirho.read_entry_chirho(&intro_key_chirho) {
                    if !text_chirho.is_empty() {
                        println!("$$${}", intro_key_chirho);
                        println!("{}", text_chirho);
                        count_chirho += 1;
                    }
                }
            }

            for chapter_chirho in 1..=book_chirho.chapter_count_chirho {
                // Export chapter intro if requested
                if args_chirho.intros_chirho {
                    let chapter_intro_key_chirho = format!("{} {}:0", book_chirho.osis_chirho, chapter_chirho);
                    if let Ok(text_chirho) = module_chirho.read_entry_chirho(&chapter_intro_key_chirho) {
                        if !text_chirho.is_empty() {
                            println!("$$${}", chapter_intro_key_chirho);
                            println!("{}", text_chirho);
                            count_chirho += 1;
                        }
                    }
                }

                if let Some(max_verse_chirho) = book_chirho.max_verse_chirho(chapter_chirho) {
                    for verse_chirho in 1..=max_verse_chirho {
                        let key_chirho = format!("{} {}:{}", book_chirho.osis_chirho, chapter_chirho, verse_chirho);

                        if let Ok(text_chirho) = module_chirho.read_entry_chirho(&key_chirho) {
                            if !text_chirho.is_empty() {
                                println!("$$${}", key_chirho);
                                println!("{}", text_chirho);
                                count_chirho += 1;

                                if args_chirho.verbose_chirho && count_chirho % 1000 == 0 {
                                    eprintln!("  Exported {} entries...", count_chirho);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Export New Testament
    for book_idx_chirho in 0..v11n_chirho.book_count_chirho(TestamentChirho::NewChirho) {
        if let Some(book_chirho) = v11n_chirho.get_book_chirho(TestamentChirho::NewChirho, book_idx_chirho) {
            // Export book intro if requested
            if args_chirho.intros_chirho {
                let intro_key_chirho = format!("{} 0:0", book_chirho.osis_chirho);
                if let Ok(text_chirho) = module_chirho.read_entry_chirho(&intro_key_chirho) {
                    if !text_chirho.is_empty() {
                        println!("$$${}", intro_key_chirho);
                        println!("{}", text_chirho);
                        count_chirho += 1;
                    }
                }
            }

            for chapter_chirho in 1..=book_chirho.chapter_count_chirho {
                // Export chapter intro if requested
                if args_chirho.intros_chirho {
                    let chapter_intro_key_chirho = format!("{} {}:0", book_chirho.osis_chirho, chapter_chirho);
                    if let Ok(text_chirho) = module_chirho.read_entry_chirho(&chapter_intro_key_chirho) {
                        if !text_chirho.is_empty() {
                            println!("$$${}", chapter_intro_key_chirho);
                            println!("{}", text_chirho);
                            count_chirho += 1;
                        }
                    }
                }

                if let Some(max_verse_chirho) = book_chirho.max_verse_chirho(chapter_chirho) {
                    for verse_chirho in 1..=max_verse_chirho {
                        let key_chirho = format!("{} {}:{}", book_chirho.osis_chirho, chapter_chirho, verse_chirho);

                        if let Ok(text_chirho) = module_chirho.read_entry_chirho(&key_chirho) {
                            if !text_chirho.is_empty() {
                                println!("$$${}", key_chirho);
                                println!("{}", text_chirho);
                                count_chirho += 1;

                                if args_chirho.verbose_chirho && count_chirho % 1000 == 0 {
                                    eprintln!("  Exported {} entries...", count_chirho);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if args_chirho.verbose_chirho {
        eprintln!("Done. Exported {} entries.", count_chirho);
    }

    Ok(())
}
