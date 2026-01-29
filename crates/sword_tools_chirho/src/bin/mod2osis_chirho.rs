// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! mod2osis_chirho - Export SWORD module to OSIS XML.

use clap::Parser;

/// Export SWORD module to OSIS XML.
#[derive(Parser, Debug)]
#[command(name = "mod2osis_chirho")]
#[command(version)]
#[command(about = "Export SWORD module to OSIS XML")]
struct ArgsChirho {
    /// Module name to export
    module_name_chirho: String,
}

fn main() {
    let args_chirho = ArgsChirho::parse();

    println!("Exporting module to OSIS: {}", args_chirho.module_name_chirho);
    println!("(OSIS export not yet implemented)");
}
