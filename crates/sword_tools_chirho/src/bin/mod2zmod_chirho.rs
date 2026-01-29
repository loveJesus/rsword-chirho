// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! mod2zmod_chirho - Compress a raw SWORD module.
//!
//! Converts a raw (uncompressed) SWORD module to a compressed (zText/zCom) format.
//!
//! Usage:
//!   mod2zmod_chirho RawMod ./modules/texts/ztext/CompMod 3
//!   mod2zmod_chirho -z BZIP2 RawMod ./modules/texts/ztext/CompMod 3

use clap::Parser;
use std::fs;
use std::path::PathBuf;

use rsword_chirho::manager_chirho::SwMgrChirho;
use rsword_chirho::versification_chirho::{kjv_chirho, TestamentChirho};
use rsword_chirho::{BlockTypeChirho, CompressionTypeChirho};

/// Compress a raw SWORD module.
#[derive(Parser, Debug)]
#[command(name = "mod2zmod_chirho")]
#[command(version)]
#[command(about = "Compress a raw SWORD module")]
struct ArgsChirho {
    /// Source module name
    source_module_chirho: String,

    /// Output path for compressed module
    output_path_chirho: PathBuf,

    /// Block type (2=verse, 3=chapter, 4=book)
    #[arg(default_value = "3")]
    block_type_chirho: u8,

    /// Compression type (ZIP, BZIP2, XZ)
    #[arg(short = 'z', long, default_value = "ZIP")]
    compress_chirho: String,

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

    // Load the source module
    let source_chirho = mgr_chirho.load_module_chirho(&args_chirho.source_module_chirho)?;

    println!("Compressing module: {}", args_chirho.source_module_chirho);
    println!("Output path: {:?}", args_chirho.output_path_chirho);

    let block_type_chirho = match args_chirho.block_type_chirho {
        2 => BlockTypeChirho::VerseBlocksChirho,
        3 => BlockTypeChirho::ChapterBlocksChirho,
        4 => BlockTypeChirho::BookBlocksChirho,
        _ => {
            return Err(format!("Invalid block type: {}. Use 2=verse, 3=chapter, 4=book", args_chirho.block_type_chirho).into());
        }
    };

    let compress_type_chirho = match args_chirho.compress_chirho.to_uppercase().as_str() {
        "ZIP" | "ZLIB" => CompressionTypeChirho::ZipChirho,
        "BZIP2" | "BZ2" => CompressionTypeChirho::Bzip2Chirho,
        "XZ" | "LZMA" => CompressionTypeChirho::XzChirho,
        "NONE" => CompressionTypeChirho::NoneChirho,
        _ => {
            return Err(format!("Unknown compression type: {}. Use ZIP, BZIP2, or XZ", args_chirho.compress_chirho).into());
        }
    };

    println!("Block type: {:?}", block_type_chirho);
    println!("Compression: {:?}", compress_type_chirho);

    // Create output directory
    fs::create_dir_all(&args_chirho.output_path_chirho)?;

    // Get versification
    let v11n_chirho = kjv_chirho();

    // Read all verses and collect them
    let mut verses_ot_chirho: Vec<(u32, String)> = Vec::new();
    let mut verses_nt_chirho: Vec<(u32, String)> = Vec::new();

    let mut count_chirho = 0u32;

    // Read Old Testament
    for book_idx_chirho in 0..v11n_chirho.book_count_chirho(TestamentChirho::OldChirho) {
        if let Some(book_chirho) = v11n_chirho.get_book_chirho(TestamentChirho::OldChirho, book_idx_chirho) {
            for chapter_chirho in 1..=book_chirho.chapter_count_chirho {
                if let Some(max_verse_chirho) = book_chirho.max_verse_chirho(chapter_chirho) {
                    for verse_chirho in 1..=max_verse_chirho {
                        let key_chirho = format!("{} {}:{}", book_chirho.osis_chirho, chapter_chirho, verse_chirho);

                        if let Ok(text_chirho) = source_chirho.read_entry_chirho(&key_chirho) {
                            if let Some(idx_off_chirho) = v11n_chirho.calculate_index_chirho(
                                TestamentChirho::OldChirho,
                                book_idx_chirho,
                                chapter_chirho,
                                verse_chirho,
                            ) {
                                verses_ot_chirho.push((idx_off_chirho, text_chirho));
                                count_chirho += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    // Read New Testament
    for book_idx_chirho in 0..v11n_chirho.book_count_chirho(TestamentChirho::NewChirho) {
        if let Some(book_chirho) = v11n_chirho.get_book_chirho(TestamentChirho::NewChirho, book_idx_chirho) {
            for chapter_chirho in 1..=book_chirho.chapter_count_chirho {
                if let Some(max_verse_chirho) = book_chirho.max_verse_chirho(chapter_chirho) {
                    for verse_chirho in 1..=max_verse_chirho {
                        let key_chirho = format!("{} {}:{}", book_chirho.osis_chirho, chapter_chirho, verse_chirho);

                        if let Ok(text_chirho) = source_chirho.read_entry_chirho(&key_chirho) {
                            if let Some(idx_off_chirho) = v11n_chirho.calculate_index_chirho(
                                TestamentChirho::NewChirho,
                                book_idx_chirho,
                                chapter_chirho,
                                verse_chirho,
                            ) {
                                verses_nt_chirho.push((idx_off_chirho, text_chirho));
                                count_chirho += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Read {} verses from source module.", count_chirho);

    // Note: Full zText compression would require implementing the block compression
    // and index file format. For now, we just demonstrate the structure.
    println!("(Full zText compression requires additional implementation)");
    println!("Verses collected: {} OT, {} NT", verses_ot_chirho.len(), verses_nt_chirho.len());

    // Create a basic config file
    let conf_path_chirho = args_chirho.output_path_chirho.parent()
        .unwrap_or(&args_chirho.output_path_chirho)
        .join("mods.d");
    fs::create_dir_all(&conf_path_chirho)?;

    let module_name_chirho = args_chirho.output_path_chirho
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("module");

    let compress_str_chirho = match compress_type_chirho {
        CompressionTypeChirho::ZipChirho => "ZIP",
        CompressionTypeChirho::Bzip2Chirho => "BZIP2",
        CompressionTypeChirho::XzChirho => "XZ",
        _ => "ZIP",
    };

    let conf_content_chirho = format!(
        r#"[{}]
DataPath=./modules/texts/ztext/{}/
ModDrv=zText
SourceType=Plain
Encoding=UTF-8
Lang=en
CompressType={}
BlockType={}
Description=Compressed Bible module
About=Created by mod2zmod_chirho from rsword-chirho
"#,
        module_name_chirho,
        module_name_chirho,
        compress_str_chirho,
        args_chirho.block_type_chirho
    );

    let conf_file_path_chirho = conf_path_chirho.join(format!("{}.conf", module_name_chirho.to_lowercase()));
    fs::write(&conf_file_path_chirho, conf_content_chirho)?;
    println!("Config file created: {:?}", conf_file_path_chirho);

    Ok(())
}
