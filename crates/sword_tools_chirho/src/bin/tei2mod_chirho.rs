// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! tei2mod_chirho - Create SWORD lexicon module from TEI XML.
//!
//! Imports a TEI (Text Encoding Initiative) XML file and creates
//! a SWORD lexicon/dictionary module.
//!
//! Usage:
//!   tei2mod_chirho ./modules/lexdict/rawld/strongs strongs.xml

use clap::Parser;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::fs;
use std::path::PathBuf;

use rsword_chirho::storage_chirho::RawStrChirho;

/// Create SWORD lexicon module from TEI XML.
#[derive(Parser, Debug)]
#[command(name = "tei2mod_chirho")]
#[command(version)]
#[command(about = "Create SWORD lexicon module from TEI XML")]
struct ArgsChirho {
    /// Output module path
    output_path_chirho: PathBuf,

    /// Input TEI XML file
    tei_file_chirho: PathBuf,

    /// Compression type
    #[arg(short = 'z', long)]
    compress_chirho: Option<String>,

    /// Use 4-byte index
    #[arg(short = '4', long)]
    four_byte_chirho: bool,

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
    println!("Creating lexicon at: {:?}", args_chirho.output_path_chirho);
    println!("From TEI file: {:?}", args_chirho.tei_file_chirho);

    // Read and parse the TEI file
    let content_chirho = fs::read_to_string(&args_chirho.tei_file_chirho)?;
    let entries_chirho = parse_tei_chirho(&content_chirho)?;

    if entries_chirho.is_empty() {
        return Err("No entries found in TEI file".into());
    }

    println!("Found {} entries", entries_chirho.len());

    // Create storage
    let basename_chirho = args_chirho.output_path_chirho
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("dict");

    let mut storage_chirho = RawStrChirho::create_chirho(&args_chirho.output_path_chirho, basename_chirho)?;

    // Sort entries by key
    let mut entries_sorted_chirho = entries_chirho;
    entries_sorted_chirho.sort_by(|a_chirho, b_chirho| a_chirho.0.cmp(&b_chirho.0));

    let mut count_chirho = 0u32;

    for (key_chirho, content_chirho) in &entries_sorted_chirho {
        storage_chirho.write_entry_chirho(key_chirho, content_chirho)?;
        count_chirho += 1;

        if args_chirho.verbose_chirho && count_chirho.is_multiple_of(100) {
            println!("  Imported {} entries...", count_chirho);
        }
    }

    println!("Done. Imported {} entries.", count_chirho);

    // Create a basic config file
    let conf_path_chirho = args_chirho.output_path_chirho.parent()
        .unwrap_or(&args_chirho.output_path_chirho)
        .join("mods.d");
    fs::create_dir_all(&conf_path_chirho)?;

    let module_name_chirho = basename_chirho;

    let conf_content_chirho = format!(
        r#"[{}]
DataPath=./modules/lexdict/rawld/{}/
ModDrv=RawLD
SourceType=TEI
Encoding=UTF-8
Lang=en
Description=Imported TEI lexicon
About=Created by tei2mod_chirho from rsword-chirho
"#,
        module_name_chirho,
        module_name_chirho
    );

    let conf_file_path_chirho = conf_path_chirho.join(format!("{}.conf", module_name_chirho.to_lowercase()));
    fs::write(&conf_file_path_chirho, conf_content_chirho)?;
    println!("Config file created: {:?}", conf_file_path_chirho);

    Ok(())
}

/// Parse TEI XML and extract entries.
fn parse_tei_chirho(content_chirho: &str) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let mut entries_chirho: Vec<(String, String)> = Vec::new();
    let mut reader_chirho = Reader::from_str(content_chirho);
    reader_chirho.config_mut().trim_text(true);

    let mut current_key_chirho: Option<String> = None;
    let mut current_content_chirho = String::new();
    let mut in_entry_chirho = false;
    let mut in_orth_chirho = false;
    let mut depth_chirho = 0u32;

    let mut buf_chirho = Vec::new();

    loop {
        match reader_chirho.read_event_into(&mut buf_chirho)? {
            Event::Start(e_chirho) => {
                let name_chirho = e_chirho.name();
                let name_str_chirho = std::str::from_utf8(name_chirho.as_ref()).unwrap_or("");

                match name_str_chirho {
                    "entry" => {
                        in_entry_chirho = true;
                        depth_chirho = 0;
                        current_content_chirho.clear();

                        // Try to get n attribute as key
                        for attr_chirho in e_chirho.attributes().flatten() {
                            if attr_chirho.key.as_ref() == b"n" {
                                current_key_chirho = Some(
                                    String::from_utf8_lossy(&attr_chirho.value).to_string()
                                );
                            }
                        }
                    }
                    "orth" if in_entry_chirho => {
                        in_orth_chirho = true;
                    }
                    _ if in_entry_chirho => {
                        depth_chirho += 1;
                        // Preserve inner XML
                        current_content_chirho.push('<');
                        current_content_chirho.push_str(name_str_chirho);
                        current_content_chirho.push('>');
                    }
                    _ => {}
                }
            }
            Event::End(e_chirho) => {
                let name_chirho = e_chirho.name();
                let name_str_chirho = std::str::from_utf8(name_chirho.as_ref()).unwrap_or("");

                match name_str_chirho {
                    "entry" => {
                        if let Some(key_chirho) = current_key_chirho.take() {
                            entries_chirho.push((key_chirho, current_content_chirho.trim().to_string()));
                        }
                        in_entry_chirho = false;
                        current_content_chirho.clear();
                    }
                    "orth" => {
                        in_orth_chirho = false;
                    }
                    _ if in_entry_chirho && depth_chirho > 0 => {
                        depth_chirho -= 1;
                        current_content_chirho.push_str("</");
                        current_content_chirho.push_str(name_str_chirho);
                        current_content_chirho.push('>');
                    }
                    _ => {}
                }
            }
            Event::Text(e_chirho) => {
                if in_orth_chirho && current_key_chirho.is_none() {
                    // Use orth text as key if no n attribute
                    current_key_chirho = Some(e_chirho.unescape()?.to_string());
                } else if in_entry_chirho {
                    current_content_chirho.push_str(&e_chirho.unescape()?);
                }
            }
            Event::Eof => break,
            _ => {}
        }
        buf_chirho.clear();
    }

    Ok(entries_chirho)
}
