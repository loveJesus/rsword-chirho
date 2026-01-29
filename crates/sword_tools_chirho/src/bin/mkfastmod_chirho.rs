// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! mkfastmod_chirho - Create search index for a SWORD module.

use clap::Parser;

/// Create search index for a SWORD module.
#[derive(Parser, Debug)]
#[command(name = "mkfastmod_chirho")]
#[command(version)]
#[command(about = "Create search index for a SWORD module")]
struct ArgsChirho {
    /// Module name to index
    module_name_chirho: String,
}

fn main() {
    let args_chirho = ArgsChirho::parse();

    println!("Creating search index for: {}", args_chirho.module_name_chirho);
    println!("(Search index creation not yet implemented)");
}
