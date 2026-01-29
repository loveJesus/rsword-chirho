// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! osis2mod_chirho - Create SWORD module from OSIS XML.

use clap::Parser;
use std::path::PathBuf;

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
}

fn main() {
    let args_chirho = ArgsChirho::parse();

    println!("Creating module at: {:?}", args_chirho.output_path_chirho);
    println!("From OSIS file: {:?}", args_chirho.osis_file_chirho);
    println!("Versification: {}", args_chirho.versification_chirho);
    println!("(OSIS import not yet implemented)");
}
