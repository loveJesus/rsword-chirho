// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! imp2ld_chirho - Create lexicon module from IMP format.

use clap::Parser;
use std::path::PathBuf;

/// Create lexicon module from IMP format.
#[derive(Parser, Debug)]
#[command(name = "imp2ld_chirho")]
#[command(version)]
#[command(about = "Create lexicon module from IMP format")]
struct ArgsChirho {
    /// Output module path
    output_path_chirho: PathBuf,

    /// Input IMP file
    imp_file_chirho: PathBuf,
}

fn main() {
    let args_chirho = ArgsChirho::parse();

    println!("Creating lexicon at: {:?}", args_chirho.output_path_chirho);
    println!("From IMP file: {:?}", args_chirho.imp_file_chirho);
    println!("(IMP lexicon import not yet implemented)");
}
