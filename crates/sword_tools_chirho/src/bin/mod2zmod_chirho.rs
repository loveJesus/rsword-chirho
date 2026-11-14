// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! mod2zmod_chirho - Compress a raw SWORD module.
//!
//! Not available in this release: zText block compression is not implemented, and the earlier version left a
//! zText conf with no data behind it (measured 2026-11-14). It exits non-zero and writes nothing.
//!
//! Usage:
//!   mod2zmod_chirho RawMod ./modules/texts/ztext/CompMod 3
//!   mod2zmod_chirho -z BZIP2 RawMod ./modules/texts/ztext/CompMod 3

use clap::Parser;
use std::path::PathBuf;

/// Compress a raw SWORD module.
#[derive(Parser, Debug)]
#[command(name = "mod2zmod_chirho")]
#[command(version)]
#[command(about = "Compress a raw SWORD module (not available in this release)")]
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

    if let Err(e_chirho) = run_chirho(&args_chirho) {
        eprintln!("Error: {}", e_chirho);
        std::process::exit(1);
    }
}

/// Refuse before touching the filesystem: no data files and no conf.
fn run_chirho(args_chirho: &ArgsChirho) -> Result<(), Box<dyn std::error::Error>> {
    if args_chirho.verbose_chirho {
        eprintln!("mod2zmod_chirho: block type {} with {} compression was requested", args_chirho.block_type_chirho, args_chirho.compress_chirho);
    }
    Err(format!(
        "mod2zmod_chirho cannot compress {} in this release: zText block compression is not implemented. \
         Nothing was written to {}. Use SWORD's mod2zmod for now.",
        args_chirho.source_module_chirho,
        args_chirho.output_path_chirho.display()
    )
    .into())
}
