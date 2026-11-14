// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! imp2gbs_chirho - Create general book module from IMP format.
//!
//! Not available in this release. Its storage was a lexicon, not the RawGenBook tree that SWORD and rsword
//! read as a general book; measured 2026-11-14 against SWORD 1.9.0, whose reader hung on it. It exits
//! non-zero and writes nothing until a RawGenBook writer lands.
//!
//! Usage:
//!   imp2gbs_chirho ./modules/genbook/rawgenbook/mybook mybook.imp

use clap::Parser;
use std::path::PathBuf;

/// Create general book module from IMP format.
#[derive(Parser, Debug)]
#[command(name = "imp2gbs_chirho")]
#[command(version)]
#[command(about = "Create general book module from IMP format (not available in this release)")]
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

    if let Err(e_chirho) = run_chirho(&args_chirho) {
        eprintln!("Error: {}", e_chirho);
        std::process::exit(1);
    }
}

/// Refuse before touching the filesystem: no data files and no conf.
fn run_chirho(args_chirho: &ArgsChirho) -> Result<(), Box<dyn std::error::Error>> {
    if args_chirho.verbose_chirho {
        eprintln!("imp2gbs_chirho: refusing before reading {}", args_chirho.imp_file_chirho.display());
    }
    Err(format!(
        "imp2gbs_chirho cannot write a general book in this release: its output was not a RawGenBook that SWORD or rsword \
         can read. Nothing was written to {}. Use SWORD's imp2gbs for now.",
        args_chirho.output_path_chirho.display()
    )
    .into())
}
