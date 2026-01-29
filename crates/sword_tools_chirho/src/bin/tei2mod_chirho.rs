// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! tei2mod_chirho - Create SWORD lexicon module from TEI XML.

use clap::Parser;
use std::path::PathBuf;

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
}

fn main() {
    let args_chirho = ArgsChirho::parse();

    println!("Creating lexicon at: {:?}", args_chirho.output_path_chirho);
    println!("From TEI file: {:?}", args_chirho.tei_file_chirho);
    println!("(TEI import not yet implemented)");
}
