// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! imp2gbs_chirho - Create general book module from IMP format.
//!
//! Imports a text file in IMP format and creates a SWORD general book module.
//!
//! Usage:
//!   imp2gbs_chirho ./modules/genbook/rawgenbook/mybook mybook.imp

use clap::Parser;
use std::fs;
use std::path::PathBuf;

use rsword_chirho::import_chirho::parse_imp_chirho;
use rsword_chirho::storage_chirho::RawStrChirho;

/// Create general book module from IMP format.
#[derive(Parser, Debug)]
#[command(name = "imp2gbs_chirho")]
#[command(version)]
#[command(about = "Create general book module from IMP format")]
struct ArgsChirho {
    /// Output module path
    output_path_chirho: PathBuf,

    /// Input IMP file
    imp_file_chirho: PathBuf,

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
    // Read the IMP file
    let content_chirho = fs::read_to_string(&args_chirho.imp_file_chirho)?;
    let entries_chirho = parse_imp_chirho(&content_chirho);

    if entries_chirho.is_empty() {
        return Err("No entries found in IMP file".into());
    }

    println!("Importing {} entries from {:?}", entries_chirho.len(), args_chirho.imp_file_chirho);
    println!("Creating general book at: {:?}", args_chirho.output_path_chirho);

    // Create storage (general books use RawStr format, similar to lexicons)
    let basename_chirho = args_chirho.output_path_chirho
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("book");

    let mut storage_chirho = RawStrChirho::create_chirho(&args_chirho.output_path_chirho, basename_chirho)?;

    let mut count_chirho = 0u32;

    for entry_chirho in &entries_chirho {
        storage_chirho.write_entry_chirho(&entry_chirho.key_chirho, &entry_chirho.content_chirho)?;
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
DataPath=./modules/genbook/rawgenbook/{}/
ModDrv=RawGenBook
SourceType=Plain
Encoding=UTF-8
Lang=en
Description=Imported general book
About=Created by imp2gbs_chirho from rsword-chirho
"#,
        module_name_chirho,
        module_name_chirho
    );

    let conf_file_path_chirho = conf_path_chirho.join(format!("{}.conf", module_name_chirho.to_lowercase()));
    fs::write(&conf_file_path_chirho, conf_content_chirho)?;
    println!("Config file created: {:?}", conf_file_path_chirho);

    Ok(())
}
