// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! osis2mod_chirho - Create SWORD module from OSIS XML.
//!
//! Imports an OSIS (Open Scripture Information Standard) XML file
//! and creates a SWORD Bible module.
//!
//! Usage:
//!   osis2mod_chirho ./modules/texts/rawtext/mymod Bible.xml
//!   osis2mod_chirho -z ZIP ./modules/texts/ztext/mymod Bible.xml

use clap::Parser;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::fs;
use std::path::PathBuf;

use rsword_chirho::storage_chirho::RawVerseChirho;
use rsword_chirho::versification_chirho::{kjv_chirho, TestamentChirho, VersificationChirho};

/// Type alias for verse data: (testament, verse_index, text)
type VerseDataChirho = (TestamentChirho, u32, String);

/// Create SWORD module from OSIS XML.
#[derive(Parser, Debug)]
#[command(name = "osis2mod_chirho")]
#[command(version)]
#[command(about = "Create SWORD module from OSIS XML")]
struct ArgsChirho {
    /// Output module path
    output_path_chirho: PathBuf,

    /// Input OSIS XML file
    osis_file_chirho: PathBuf,

    /// Versification system
    #[arg(short = 'v', long, default_value = "KJV")]
    versification_chirho: String,

    /// Compression type (ZIP, BZIP2, XZ, LZSS)
    #[arg(short = 'z', long)]
    compress_chirho: Option<String>,

    /// Block type (1=verse, 2=chapter, 3=book)
    #[arg(short = 'b', long, default_value = "2")]
    block_type_chirho: u8,

    /// Cipher key for encryption
    #[arg(short = 'c', long)]
    cipher_key_chirho: Option<String>,

    /// Use 4-byte index
    #[arg(short = '4', long)]
    four_byte_chirho: bool,

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
    println!("From OSIS file: {:?}", args_chirho.osis_file_chirho);
    println!("Versification: {}", args_chirho.versification_chirho);

    // Read and parse OSIS
    let content_chirho = fs::read_to_string(&args_chirho.osis_file_chirho)?;
    let v11n_chirho = kjv_chirho();

    let verses_chirho = parse_osis_chirho(&content_chirho, v11n_chirho)?;

    if verses_chirho.is_empty() {
        return Err("No verses found in OSIS file".into());
    }

    println!("Found {} verses", verses_chirho.len());

    // Create storage
    let mut storage_chirho = RawVerseChirho::create_chirho(&args_chirho.output_path_chirho)?;

    let mut count_chirho = 0u32;

    for (testament_chirho, idx_off_chirho, text_chirho) in &verses_chirho {
        let testament_num_chirho = match testament_chirho {
            TestamentChirho::OldChirho => 1,
            TestamentChirho::NewChirho => 2,
        };

        storage_chirho.write_verse_chirho(testament_num_chirho, *idx_off_chirho, text_chirho)?;
        count_chirho += 1;

        if args_chirho.verbose_chirho && count_chirho.is_multiple_of(1000) {
            println!("  Imported {} verses...", count_chirho);
        }
    }

    println!("Done. Imported {} verses.", count_chirho);

    // Create config file
    let conf_path_chirho = args_chirho.output_path_chirho.parent()
        .unwrap_or(&args_chirho.output_path_chirho)
        .join("mods.d");
    fs::create_dir_all(&conf_path_chirho)?;

    let module_name_chirho = args_chirho.output_path_chirho
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("module");

    let mod_drv_chirho = if args_chirho.compress_chirho.is_some() {
        "zText"
    } else {
        "RawText"
    };

    let data_path_type_chirho = if args_chirho.compress_chirho.is_some() {
        "ztext"
    } else {
        "rawtext"
    };

    let conf_content_chirho = format!(
        r#"[{}]
DataPath=./modules/texts/{}/{}/
ModDrv={}
SourceType=OSIS
Encoding=UTF-8
Lang=en
Versification={}
Description=Imported OSIS Bible module
About=Created by osis2mod_chirho from rsword-chirho
"#,
        module_name_chirho,
        data_path_type_chirho,
        module_name_chirho,
        mod_drv_chirho,
        args_chirho.versification_chirho
    );

    let conf_file_path_chirho = conf_path_chirho.join(format!("{}.conf", module_name_chirho.to_lowercase()));
    fs::write(&conf_file_path_chirho, conf_content_chirho)?;
    println!("Config file created: {:?}", conf_file_path_chirho);

    Ok(())
}

/// Parse OSIS XML and extract verses.
///
/// Handles both milestone-style:
///   <verse sID="Gen.1.1" osisID="Gen.1.1"/>...<verse eID="Gen.1.1"/>
///
/// And container-style:
///   <verse osisID="Gen.1.1">...</verse>
fn parse_osis_chirho(
    content_chirho: &str,
    v11n_chirho: &VersificationChirho,
) -> Result<Vec<VerseDataChirho>, Box<dyn std::error::Error>> {
    let mut verses_chirho: Vec<VerseDataChirho> = Vec::new();
    let mut reader_chirho = Reader::from_str(content_chirho);
    reader_chirho.config_mut().trim_text(false);

    let mut current_verse_id_chirho: Option<String> = None;
    let mut current_verse_content_chirho = String::new();
    let mut in_verse_chirho = false;
    let mut depth_chirho = 0u32;

    let mut buf_chirho = Vec::new();

    loop {
        match reader_chirho.read_event_into(&mut buf_chirho)? {
            Event::Start(e_chirho) => {
                let name_chirho = e_chirho.name();
                let name_str_chirho = std::str::from_utf8(name_chirho.as_ref()).unwrap_or("");

                if name_str_chirho == "verse" {
                    // Container-style verse start
                    for attr_chirho in e_chirho.attributes().flatten() {
                        if attr_chirho.key.as_ref() == b"osisID" {
                            current_verse_id_chirho = Some(
                                String::from_utf8_lossy(&attr_chirho.value).to_string()
                            );
                            in_verse_chirho = true;
                            depth_chirho = 0;
                            current_verse_content_chirho.clear();
                            break;
                        }
                    }
                } else if in_verse_chirho {
                    // Include other tags in verse content
                    depth_chirho += 1;
                    current_verse_content_chirho.push('<');
                    current_verse_content_chirho.push_str(name_str_chirho);

                    // Include attributes
                    for attr_chirho in e_chirho.attributes().flatten() {
                        let key_chirho = std::str::from_utf8(attr_chirho.key.as_ref()).unwrap_or("");
                        let val_chirho = String::from_utf8_lossy(&attr_chirho.value);
                        current_verse_content_chirho.push(' ');
                        current_verse_content_chirho.push_str(key_chirho);
                        current_verse_content_chirho.push_str("=\"");
                        current_verse_content_chirho.push_str(&val_chirho);
                        current_verse_content_chirho.push('"');
                    }
                    current_verse_content_chirho.push('>');
                }
            }
            Event::Empty(e_chirho) => {
                let name_chirho = e_chirho.name();
                let name_str_chirho = std::str::from_utf8(name_chirho.as_ref()).unwrap_or("");

                if name_str_chirho == "verse" {
                    // Milestone-style verse
                    let mut sid_chirho: Option<String> = None;
                    let mut eid_chirho: Option<String> = None;
                    let mut osis_id_chirho: Option<String> = None;

                    for attr_chirho in e_chirho.attributes().flatten() {
                        match attr_chirho.key.as_ref() {
                            b"sID" => {
                                sid_chirho = Some(String::from_utf8_lossy(&attr_chirho.value).to_string());
                            }
                            b"eID" => {
                                eid_chirho = Some(String::from_utf8_lossy(&attr_chirho.value).to_string());
                            }
                            b"osisID" => {
                                osis_id_chirho = Some(String::from_utf8_lossy(&attr_chirho.value).to_string());
                            }
                            _ => {}
                        }
                    }

                    if let Some(sid_chirho) = sid_chirho.or(osis_id_chirho) {
                        // Start of milestone verse
                        current_verse_id_chirho = Some(sid_chirho);
                        in_verse_chirho = true;
                        current_verse_content_chirho.clear();
                    } else if eid_chirho.is_some() && in_verse_chirho {
                        // End of milestone verse
                        if let Some(verse_id_chirho) = current_verse_id_chirho.take() {
                            if let Some((testament_chirho, idx_off_chirho)) =
                                parse_osis_id_chirho(&verse_id_chirho, v11n_chirho)
                            {
                                verses_chirho.push((
                                    testament_chirho,
                                    idx_off_chirho,
                                    current_verse_content_chirho.trim().to_string()
                                ));
                            }
                        }
                        in_verse_chirho = false;
                        current_verse_content_chirho.clear();
                    }
                } else if in_verse_chirho {
                    // Self-closing tag in verse content
                    current_verse_content_chirho.push('<');
                    current_verse_content_chirho.push_str(name_str_chirho);
                    for attr_chirho in e_chirho.attributes().flatten() {
                        let key_chirho = std::str::from_utf8(attr_chirho.key.as_ref()).unwrap_or("");
                        let val_chirho = String::from_utf8_lossy(&attr_chirho.value);
                        current_verse_content_chirho.push(' ');
                        current_verse_content_chirho.push_str(key_chirho);
                        current_verse_content_chirho.push_str("=\"");
                        current_verse_content_chirho.push_str(&val_chirho);
                        current_verse_content_chirho.push('"');
                    }
                    current_verse_content_chirho.push_str("/>");
                }
            }
            Event::End(e_chirho) => {
                let name_chirho = e_chirho.name();
                let name_str_chirho = std::str::from_utf8(name_chirho.as_ref()).unwrap_or("");

                if name_str_chirho == "verse" && in_verse_chirho {
                    // Container-style verse end
                    if let Some(verse_id_chirho) = current_verse_id_chirho.take() {
                        if let Some((testament_chirho, idx_off_chirho)) =
                            parse_osis_id_chirho(&verse_id_chirho, v11n_chirho)
                        {
                            verses_chirho.push((
                                testament_chirho,
                                idx_off_chirho,
                                current_verse_content_chirho.trim().to_string()
                            ));
                        }
                    }
                    in_verse_chirho = false;
                    current_verse_content_chirho.clear();
                } else if in_verse_chirho && depth_chirho > 0 {
                    depth_chirho -= 1;
                    current_verse_content_chirho.push_str("</");
                    current_verse_content_chirho.push_str(name_str_chirho);
                    current_verse_content_chirho.push('>');
                }
            }
            Event::Text(e_chirho) => {
                if in_verse_chirho {
                    current_verse_content_chirho.push_str(&e_chirho.unescape()?);
                }
            }
            Event::Eof => break,
            _ => {}
        }
        buf_chirho.clear();
    }

    Ok(verses_chirho)
}

/// Parse OSIS ID like "Gen.1.1" to testament and index offset.
fn parse_osis_id_chirho(
    osis_id_chirho: &str,
    v11n_chirho: &VersificationChirho,
) -> Option<(TestamentChirho, u32)> {
    // Handle range references (take first verse)
    let id_chirho = osis_id_chirho.split('-').next()?;

    // Handle space-separated multiple verses (take first)
    let id_chirho = id_chirho.split(' ').next()?;

    let parts_chirho: Vec<&str> = id_chirho.split('.').collect();
    if parts_chirho.len() < 3 {
        return None;
    }

    let book_osis_chirho = parts_chirho[0];
    let chapter_chirho: u8 = parts_chirho[1].parse().ok()?;
    let verse_chirho: u8 = parts_chirho[2].parse().ok()?;

    // Look up book
    let (testament_chirho, book_idx_chirho) = v11n_chirho.lookup_book_chirho(book_osis_chirho)?;

    // Calculate index offset
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
    fn test_parse_osis_id_chirho() {
        let v11n_chirho = kjv_chirho();

        // Genesis 1:1
        let result_chirho = parse_osis_id_chirho("Gen.1.1", v11n_chirho);
        assert!(result_chirho.is_some());
        let (testament_chirho, _) = result_chirho.unwrap();
        assert!(matches!(testament_chirho, TestamentChirho::OldChirho));

        // John 3:16
        let result_chirho = parse_osis_id_chirho("John.3.16", v11n_chirho);
        assert!(result_chirho.is_some());
        let (testament_chirho, _) = result_chirho.unwrap();
        assert!(matches!(testament_chirho, TestamentChirho::NewChirho));

        // Range reference (should use first verse)
        let result_chirho = parse_osis_id_chirho("Gen.1.1-Gen.1.2", v11n_chirho);
        assert!(result_chirho.is_some());
    }
}
