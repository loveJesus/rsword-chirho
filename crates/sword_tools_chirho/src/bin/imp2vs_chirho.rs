// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! imp2vs_chirho - Create Bible module from IMP format.
//!
//! Imports a text file in IMP format and creates a SWORD Bible module.
//!
//! Usage:
//!   imp2vs_chirho ./modules/texts/rawtext/mymod myfile.imp
//!   imp2vs_chirho -v KJV ./modules/texts/rawtext/mymod myfile.imp

use clap::Parser;
use std::fs;
use std::path::PathBuf;

use rsword_chirho::import_chirho::parse_imp_chirho;
use rsword_chirho::storage_chirho::RawVerseChirho;
use rsword_chirho::versification_chirho::{kjv_chirho, TestamentChirho, VersificationChirho};

/// Create Bible module from IMP format.
#[derive(Parser, Debug)]
#[command(name = "imp2vs_chirho")]
#[command(version)]
#[command(about = "Create Bible module from IMP format")]
struct ArgsChirho {
    /// Output module path
    output_path_chirho: PathBuf,

    /// Input IMP file
    imp_file_chirho: PathBuf,

    /// Versification system
    #[arg(short = 'v', long, default_value = "KJV")]
    versification_chirho: String,

    /// Append to existing module
    #[arg(short = 'a', long)]
    append_chirho: bool,

    /// Verbose output
    #[arg(long)]
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
    // Read the IMP file
    let content_chirho = fs::read_to_string(&args_chirho.imp_file_chirho)?;
    let entries_chirho = parse_imp_chirho(&content_chirho);

    if entries_chirho.is_empty() {
        return Err("No entries found in IMP file".into());
    }

    println!("Importing {} entries from {:?}", entries_chirho.len(), args_chirho.imp_file_chirho);
    println!("Creating module at: {:?}", args_chirho.output_path_chirho);

    // Get versification
    let v11n_chirho = kjv_chirho();

    // Create or open storage
    let mut storage_chirho = if args_chirho.append_chirho && args_chirho.output_path_chirho.exists() {
        RawVerseChirho::open_chirho(&args_chirho.output_path_chirho)?
    } else {
        RawVerseChirho::create_chirho(&args_chirho.output_path_chirho)?
    };

    let mut count_chirho = 0u32;
    let mut error_count_chirho = 0u32;

    for entry_chirho in &entries_chirho {
        // Parse the key to get testament, book, chapter, verse
        match parse_verse_key_chirho(&entry_chirho.key_chirho, v11n_chirho) {
            Some((testament_chirho, idx_off_chirho)) => {
                let testament_num_chirho = match testament_chirho {
                    TestamentChirho::OldChirho => 1,
                    TestamentChirho::NewChirho => 2,
                };

                storage_chirho.write_verse_chirho(
                    testament_num_chirho,
                    idx_off_chirho,
                    &entry_chirho.content_chirho,
                )?;

                count_chirho += 1;

                if args_chirho.verbose_chirho && count_chirho % 100 == 0 {
                    println!("  Imported {} verses...", count_chirho);
                }
            }
            None => {
                if args_chirho.verbose_chirho {
                    eprintln!("Warning: Could not parse key: {}", entry_chirho.key_chirho);
                }
                error_count_chirho += 1;
            }
        }
    }

    println!("Done. Imported {} verses.", count_chirho);
    if error_count_chirho > 0 {
        println!("Skipped {} entries with unparseable keys.", error_count_chirho);
    }

    // Create a basic config file
    let conf_path_chirho = args_chirho.output_path_chirho.parent()
        .unwrap_or(&args_chirho.output_path_chirho)
        .join("mods.d");
    fs::create_dir_all(&conf_path_chirho)?;

    let module_name_chirho = args_chirho.output_path_chirho
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("module");

    let conf_content_chirho = format!(
        r#"[{}]
DataPath=./modules/texts/rawtext/{}/
ModDrv=RawText
SourceType=Plain
Encoding=UTF-8
Lang=en
Description=Imported Bible module
About=Created by imp2vs_chirho from rsword-chirho
"#,
        module_name_chirho,
        module_name_chirho
    );

    let conf_file_path_chirho = conf_path_chirho.join(format!("{}.conf", module_name_chirho.to_lowercase()));
    fs::write(&conf_file_path_chirho, conf_content_chirho)?;
    println!("Config file created: {:?}", conf_file_path_chirho);

    Ok(())
}

/// Parse a verse key like "Gen 1:1" or "Genesis 1:1" into (testament, index_offset).
fn parse_verse_key_chirho(key_chirho: &str, v11n_chirho: &VersificationChirho) -> Option<(TestamentChirho, u32)> {
    // Split into book and chapter:verse
    let parts_chirho: Vec<&str> = key_chirho.rsplitn(2, ' ').collect();
    if parts_chirho.len() != 2 {
        return None;
    }

    let cv_part_chirho = parts_chirho[0]; // "1:1"
    let book_part_chirho = parts_chirho[1]; // "Gen" or "Genesis"

    // Parse chapter:verse
    let cv_parts_chirho: Vec<&str> = cv_part_chirho.split(':').collect();
    if cv_parts_chirho.len() != 2 {
        return None;
    }

    let chapter_chirho: u8 = cv_parts_chirho[0].parse().ok()?;
    let verse_chirho: u8 = cv_parts_chirho[1].parse().ok()?;

    // Look up the book
    let (testament_chirho, book_idx_chirho) = v11n_chirho.lookup_book_chirho(book_part_chirho)?;

    // Calculate the index offset
    let idx_off_chirho = v11n_chirho.calculate_index_chirho(
        testament_chirho,
        book_idx_chirho,
        chapter_chirho,
        verse_chirho,
    )?;

    Some((testament_chirho, idx_off_chirho))
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_parse_verse_key_chirho() {
        let v11n_chirho = kjv_chirho();

        // Test Genesis 1:1
        let result_chirho = parse_verse_key_chirho("Gen 1:1", v11n_chirho);
        assert!(result_chirho.is_some());
        let (testament_chirho, _) = result_chirho.unwrap();
        assert_eq!(testament_chirho, TestamentChirho::OldChirho);

        // Test John 3:16
        let result_chirho = parse_verse_key_chirho("John 3:16", v11n_chirho);
        assert!(result_chirho.is_some());
        let (testament_chirho, _) = result_chirho.unwrap();
        assert_eq!(testament_chirho, TestamentChirho::NewChirho);
    }
}
