// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! imp2vs_chirho - Create Bible module from IMP format.

use clap::Parser;
use std::path::PathBuf;

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
}

fn main() {
    let args_chirho = ArgsChirho::parse();

    println!("Creating Bible module at: {:?}", args_chirho.output_path_chirho);
    println!("From IMP file: {:?}", args_chirho.imp_file_chirho);
    println!("(IMP import not yet implemented)");
}
