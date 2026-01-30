// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! mod2osis_chirho - Export SWORD module to OSIS XML.
//!
//! Exports a SWORD Bible module to OSIS (Open Scripture Information Standard) XML format.
//!
//! Usage:
//!   mod2osis_chirho KJV > kjv.osis
//!   mod2osis_chirho -v KJV > kjv_verbose.osis

use clap::Parser;

use rsword_chirho::manager_chirho::SwMgrChirho;
use rsword_chirho::versification_chirho::{kjv_chirho, TestamentChirho};

/// Export SWORD module to OSIS XML.
#[derive(Parser, Debug)]
#[command(name = "mod2osis_chirho")]
#[command(version)]
#[command(about = "Export SWORD module to OSIS XML")]
struct ArgsChirho {
    /// Module name to export
    module_name_chirho: String,

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

    // Print OSIS XML header
    println!(r#"<?xml version="1.0" encoding="UTF-8"?>"#);
    println!(r#"<osis xmlns="http://www.bibletechnologies.net/2003/OSIS/namespace""#);
    println!(r#"      xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance""#);
    println!(r#"      xsi:schemaLocation="http://www.bibletechnologies.net/2003/OSIS/namespace http://www.bibletechnologies.net/osisCore.2.1.1.xsd">"#);
    println!(r#"  <osisText osisIDWork="{}" osisRefWork="Bible" xml:lang="en">"#, args_chirho.module_name_chirho);
    println!(r#"    <header>"#);
    println!(r#"      <work osisWork="{}">"#, args_chirho.module_name_chirho);
    println!(r#"        <title>{}</title>"#, args_chirho.module_name_chirho);
    println!(r#"        <identifier type="OSIS">{}</identifier>"#, args_chirho.module_name_chirho);
    println!(r#"        <refSystem>Bible.KJV</refSystem>"#);
    println!(r#"      </work>"#);
    println!(r#"    </header>"#);

    let mut count_chirho = 0u32;

    // Export Old Testament
    println!(r#"    <div type="bookGroup">"#);
    for book_idx_chirho in 0..v11n_chirho.book_count_chirho(TestamentChirho::OldChirho) {
        if let Some(book_chirho) = v11n_chirho.get_book_chirho(TestamentChirho::OldChirho, book_idx_chirho) {
            println!(r#"      <div type="book" osisID="{}">"#, book_chirho.osis_chirho);

            for chapter_chirho in 1..=book_chirho.chapter_count_chirho {
                println!(r#"        <chapter osisID="{}.{}">"#, book_chirho.osis_chirho, chapter_chirho);

                if let Some(max_verse_chirho) = book_chirho.max_verse_chirho(chapter_chirho) {
                    for verse_chirho in 1..=max_verse_chirho {
                        let key_chirho = format!("{} {}:{}", book_chirho.osis_chirho, chapter_chirho, verse_chirho);
                        let osis_id_chirho = format!("{}.{}.{}", book_chirho.osis_chirho, chapter_chirho, verse_chirho);

                        if let Ok(text_chirho) = module_chirho.read_entry_chirho(&key_chirho) {
                            if !text_chirho.is_empty() {
                                let escaped_chirho = escape_xml_chirho(&text_chirho);
                                println!(r#"          <verse osisID="{}">{}</verse>"#, osis_id_chirho, escaped_chirho);
                                count_chirho += 1;

                                if args_chirho.verbose_chirho && count_chirho.is_multiple_of(1000) {
                                    eprintln!("  Exported {} verses...", count_chirho);
                                }
                            }
                        }
                    }
                }

                println!(r#"        </chapter>"#);
            }

            println!(r#"      </div>"#);
        }
    }
    println!(r#"    </div>"#);

    // Export New Testament
    println!(r#"    <div type="bookGroup">"#);
    for book_idx_chirho in 0..v11n_chirho.book_count_chirho(TestamentChirho::NewChirho) {
        if let Some(book_chirho) = v11n_chirho.get_book_chirho(TestamentChirho::NewChirho, book_idx_chirho) {
            println!(r#"      <div type="book" osisID="{}">"#, book_chirho.osis_chirho);

            for chapter_chirho in 1..=book_chirho.chapter_count_chirho {
                println!(r#"        <chapter osisID="{}.{}">"#, book_chirho.osis_chirho, chapter_chirho);

                if let Some(max_verse_chirho) = book_chirho.max_verse_chirho(chapter_chirho) {
                    for verse_chirho in 1..=max_verse_chirho {
                        let key_chirho = format!("{} {}:{}", book_chirho.osis_chirho, chapter_chirho, verse_chirho);
                        let osis_id_chirho = format!("{}.{}.{}", book_chirho.osis_chirho, chapter_chirho, verse_chirho);

                        if let Ok(text_chirho) = module_chirho.read_entry_chirho(&key_chirho) {
                            if !text_chirho.is_empty() {
                                let escaped_chirho = escape_xml_chirho(&text_chirho);
                                println!(r#"          <verse osisID="{}">{}</verse>"#, osis_id_chirho, escaped_chirho);
                                count_chirho += 1;

                                if args_chirho.verbose_chirho && count_chirho.is_multiple_of(1000) {
                                    eprintln!("  Exported {} verses...", count_chirho);
                                }
                            }
                        }
                    }
                }

                println!(r#"        </chapter>"#);
            }

            println!(r#"      </div>"#);
        }
    }
    println!(r#"    </div>"#);

    // Print OSIS XML footer
    println!(r#"  </osisText>"#);
    println!(r#"</osis>"#);

    if args_chirho.verbose_chirho {
        eprintln!("Done. Exported {} verses.", count_chirho);
    }

    Ok(())
}

/// Escape special XML characters.
fn escape_xml_chirho(text_chirho: &str) -> String {
    text_chirho
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
