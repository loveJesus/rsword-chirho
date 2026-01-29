// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! mod2imp_chirho - Export SWORD module to IMP format.

use clap::Parser;

/// Export SWORD module to IMP format.
#[derive(Parser, Debug)]
#[command(name = "mod2imp_chirho")]
#[command(version)]
#[command(about = "Export SWORD module to IMP format")]
struct ArgsChirho {
    /// Module name to export
    module_name_chirho: String,

    /// Include intro verses
    #[arg(short = 'i', long)]
    intros_chirho: bool,
}

fn main() {
    let args_chirho = ArgsChirho::parse();

    println!("Exporting module: {}", args_chirho.module_name_chirho);
    println!("(Module export not yet implemented)");
}
