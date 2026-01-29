// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! vpl2mod_chirho - Create module from verse-per-line format.

use clap::Parser;
use std::path::PathBuf;

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
}

fn main() {
    let args_chirho = ArgsChirho::parse();

    println!("Creating module at: {:?}", args_chirho.output_path_chirho);
    println!("From VPL file: {:?}", args_chirho.vpl_file_chirho);
    println!("Versification: {}", args_chirho.versification_chirho);
    println!("(VPL import not yet implemented)");
}
