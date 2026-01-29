// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! mod2zmod_chirho - Compress a raw SWORD module.

use clap::Parser;
use std::path::PathBuf;

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
}

fn main() {
    let args_chirho = ArgsChirho::parse();

    println!("Compressing module: {}", args_chirho.source_module_chirho);
    println!("Output path: {:?}", args_chirho.output_path_chirho);
    println!("Block type: {}", args_chirho.block_type_chirho);
    println!("Compression: {}", args_chirho.compress_chirho);
    println!("(Module compression not yet implemented)");
}
