// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! vpl2mod_chirho - Create module from verse-per-line format.
//!
//! Imports a text file where each line contains a verse, and creates
//! a SWORD Bible module. Lines can be prefixed with verse references
//! or follow sequential order.
//!
//! Usage:
//!   vpl2mod_chirho ./modules/texts/rawtext/mymod verses.txt
//!   vpl2mod_chirho -v Catholic ./modules/texts/rawtext/mymod verses.txt

use clap::Parser;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use rsword_chirho::storage_chirho::RawVerseChirho;
use rsword_chirho::versification_chirho::{kjv_chirho, TestamentChirho, VersificationChirho};

/// Create module from verse-per-line format.
#[derive(Parser, Debug)]
#[command(name = "vpl2mod_chirho")]
#[command(version)]
#[command(about = "Create module from verse-per-line format")]
struct ArgsChirho {
    /// Output module path
    output_path_chirho: PathBuf,

    /// Input VPL file
    vpl_file_chirho: PathBuf,

    /// Versification system
    #[arg(short = 'v', long, default_value = "KJV")]
    versification_chirho: String,

    /// Lines have verse references (format: "Gen 1:1 In the beginning...")
    #[arg(short = 'r', long)]
    referenced_chirho: bool,

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
    println!("Creating module at: {:?}", args_chirho.output_path_chirho);
    println!("From VPL file: {:?}", args_chirho.vpl_file_chirho);
    println!("Versification: {}", args_chirho.versification_chirho);

    // Get versification
    let v11n_chirho = kjv_chirho();

    // Create storage
    let mut storage_chirho = RawVerseChirho::create_chirho(&args_chirho.output_path_chirho)?;

    // Read the VPL file
    let file_chirho = fs::File::open(&args_chirho.vpl_file_chirho)?;
    let reader_chirho = BufReader::new(file_chirho);

    let mut count_chirho = 0u32;
    let mut error_count_chirho = 0u32;

    if args_chirho.referenced_chirho {
        // Each line has a reference prefix
        for line_result_chirho in reader_chirho.lines() {
            let line_chirho = line_result_chirho?;
            let line_chirho = line_chirho.trim();

            if line_chirho.is_empty() || line_chirho.starts_with('#') {
                continue;
            }

            // Parse reference and text
            if let Some((key_chirho, text_chirho)) = parse_referenced_line_chirho(line_chirho) {
                if let Some((testament_chirho, idx_off_chirho)) = parse_verse_key_chirho(&key_chirho, v11n_chirho) {
                    let testament_num_chirho = match testament_chirho {
                        TestamentChirho::OldChirho => 1,
                        TestamentChirho::NewChirho => 2,
                    };

                    storage_chirho.write_verse_chirho(testament_num_chirho, idx_off_chirho, &text_chirho)?;
                    count_chirho += 1;

                    if args_chirho.verbose_chirho && count_chirho.is_multiple_of(100) {
                        println!("  Imported {} verses...", count_chirho);
                    }
                } else {
                    if args_chirho.verbose_chirho {
                        eprintln!("Warning: Could not parse key: {}", key_chirho);
                    }
                    error_count_chirho += 1;
                }
            } else {
                if args_chirho.verbose_chirho {
                    eprintln!("Warning: Could not parse line: {}", line_chirho);
                }
                error_count_chirho += 1;
            }
        }
    } else {
        // Sequential mode - iterate through all verses in order
        let mut verse_iter_chirho = VerseIteratorChirho::new_chirho(v11n_chirho);

        for line_result_chirho in reader_chirho.lines() {
            let line_chirho = line_result_chirho?;
            let text_chirho = line_chirho.trim();

            if text_chirho.is_empty() {
                continue;
            }

            if let Some((testament_chirho, idx_off_chirho)) = verse_iter_chirho.next_chirho() {
                let testament_num_chirho = match testament_chirho {
                    TestamentChirho::OldChirho => 1,
                    TestamentChirho::NewChirho => 2,
                };

                storage_chirho.write_verse_chirho(testament_num_chirho, idx_off_chirho, text_chirho)?;
                count_chirho += 1;

                if args_chirho.verbose_chirho && count_chirho.is_multiple_of(100) {
                    println!("  Imported {} verses...", count_chirho);
                }
            } else {
                eprintln!("Warning: More lines than verses in versification");
                break;
            }
        }
    }

    println!("Done. Imported {} verses.", count_chirho);
    if error_count_chirho > 0 {
        println!("Skipped {} lines.", error_count_chirho);
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
About=Created by vpl2mod_chirho from rsword-chirho
"#,
        module_name_chirho,
        module_name_chirho
    );

    let conf_file_path_chirho = conf_path_chirho.join(format!("{}.conf", module_name_chirho.to_lowercase()));
    fs::write(&conf_file_path_chirho, conf_content_chirho)?;
    println!("Config file created: {:?}", conf_file_path_chirho);

    Ok(())
}

/// Parse a line with a reference prefix.
fn parse_referenced_line_chirho(line_chirho: &str) -> Option<(String, String)> {
    // Try to find a verse reference pattern at the start
    // Format: "Gen 1:1 In the beginning..."
    let parts_chirho: Vec<&str> = line_chirho.splitn(2, |c: char| c.is_whitespace() && line_chirho[..line_chirho.find(c).unwrap()].contains(':'))
        .collect();

    if parts_chirho.len() == 2 {
        return Some((parts_chirho[0].trim().to_string(), parts_chirho[1].trim().to_string()));
    }

    // Alternative: look for tab separator
    if let Some(tab_pos_chirho) = line_chirho.find('\t') {
        return Some((
            line_chirho[..tab_pos_chirho].trim().to_string(),
            line_chirho[tab_pos_chirho + 1..].trim().to_string(),
        ));
    }

    None
}

/// Parse a verse key into testament and index offset.
fn parse_verse_key_chirho(key_chirho: &str, v11n_chirho: &VersificationChirho) -> Option<(TestamentChirho, u32)> {
    // Split into book and chapter:verse
    let parts_chirho: Vec<&str> = key_chirho.rsplitn(2, ' ').collect();
    if parts_chirho.len() != 2 {
        return None;
    }

    let cv_part_chirho = parts_chirho[0];
    let book_part_chirho = parts_chirho[1];

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

/// Iterator over all verses in a versification.
struct VerseIteratorChirho<'a> {
    v11n_chirho: &'a VersificationChirho,
    testament_chirho: TestamentChirho,
    book_idx_chirho: usize,
    chapter_chirho: u8,
    verse_chirho: u8,
}

impl<'a> VerseIteratorChirho<'a> {
    fn new_chirho(v11n_chirho: &'a VersificationChirho) -> Self {
        Self {
            v11n_chirho,
            testament_chirho: TestamentChirho::OldChirho,
            book_idx_chirho: 0,
            chapter_chirho: 1,
            verse_chirho: 1,
        }
    }

    fn next_chirho(&mut self) -> Option<(TestamentChirho, u32)> {
        loop {
            let book_chirho = self.v11n_chirho.get_book_chirho(self.testament_chirho, self.book_idx_chirho)?;

            if self.chapter_chirho <= book_chirho.chapter_count_chirho {
                if let Some(max_verse_chirho) = book_chirho.max_verse_chirho(self.chapter_chirho) {
                    if self.verse_chirho <= max_verse_chirho {
                        let idx_off_chirho = self.v11n_chirho.calculate_index_chirho(
                            self.testament_chirho,
                            self.book_idx_chirho,
                            self.chapter_chirho,
                            self.verse_chirho,
                        )?;

                        self.verse_chirho += 1;
                        return Some((self.testament_chirho, idx_off_chirho));
                    }
                }

                self.chapter_chirho += 1;
                self.verse_chirho = 1;
            } else {
                self.book_idx_chirho += 1;
                self.chapter_chirho = 1;
                self.verse_chirho = 1;

                if self.book_idx_chirho >= self.v11n_chirho.book_count_chirho(self.testament_chirho) {
                    if self.testament_chirho == TestamentChirho::OldChirho {
                        self.testament_chirho = TestamentChirho::NewChirho;
                        self.book_idx_chirho = 0;
                    } else {
                        return None;
                    }
                }
            }
        }
    }
}
