// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! diatheke_chirho - Bible text lookup and search tool.
//!
//! Drop-in replacement for the SWORD diatheke command.
//!
//! Usage:
//!   diatheke_chirho -b KJV -k "John 3:16"
//!   diatheke_chirho -b KJV -s phrase -k "love"
//!   diatheke_chirho -b KJV -f html -k "Gen 1"

use clap::{Parser, ValueEnum};
use rsword_chirho::keys_chirho::VerseKeyChirho;
use rsword_chirho::manager_chirho::SwMgrChirho;

/// Bible text lookup and search tool.
#[derive(Parser, Debug)]
#[command(name = "diatheke_chirho")]
#[command(author = "rsword-chirho contributors")]
#[command(version)]
#[command(about = "Bible text lookup and search tool", long_about = None)]
struct ArgsChirho {
    /// Module to use
    #[arg(short = 'b', long)]
    module_chirho: Option<String>,

    /// Key/reference to look up
    #[arg(short = 'k', long)]
    key_chirho: Option<String>,

    /// Search type (regex, phrase, multiword, lucene)
    #[arg(short = 's', long)]
    search_chirho: Option<SearchTypeChirho>,

    /// Output format (plain, html, htmlhref, rtf, osis, latex, xhtml)
    #[arg(short = 'f', long, default_value = "plain")]
    format_chirho: OutputFormatChirho,

    /// Text encoding (utf8, latin1)
    #[arg(short = 'e', long, default_value = "utf8")]
    encoding_chirho: String,

    /// Option filters (n=notes, f=footnotes, m=morph, s=strongs, etc.)
    #[arg(short = 'o', long)]
    options_chirho: Option<String>,

    /// Maximum results for search
    #[arg(short = 'm', long)]
    max_results_chirho: Option<usize>,

    /// Range/scope for search
    #[arg(short = 'r', long)]
    range_chirho: Option<String>,

    /// Locale for output
    #[arg(short = 'l', long)]
    locale_chirho: Option<String>,

    /// Variant reading option
    #[arg(short = 'v', long)]
    variant_chirho: Option<String>,

    /// List available modules
    #[arg(long)]
    list_modules_chirho: bool,

    /// System query (modulelist, localelist, etc.)
    #[arg(long)]
    system_query_chirho: Option<String>,
}

#[derive(Clone, Debug, ValueEnum)]
enum SearchTypeChirho {
    Regex,
    Phrase,
    Multiword,
    Lucene,
}

#[derive(Clone, Debug, ValueEnum, Default)]
enum OutputFormatChirho {
    #[default]
    Plain,
    Html,
    Htmlhref,
    Rtf,
    Osis,
    Latex,
    Xhtml,
}

fn main() {
    let args_chirho = ArgsChirho::parse();

    if let Err(e) = run_chirho(args_chirho) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run_chirho(args_chirho: ArgsChirho) -> Result<(), Box<dyn std::error::Error>> {
    // Handle list modules
    if args_chirho.list_modules_chirho {
        return list_modules_chirho();
    }

    // Handle system queries
    if let Some(query_chirho) = &args_chirho.system_query_chirho {
        return handle_system_query_chirho(query_chirho);
    }

    // Regular lookup
    let module_name_chirho = args_chirho.module_chirho
        .ok_or("Module name required (-b)")?;

    let key_text_chirho = args_chirho.key_chirho
        .ok_or("Key/reference required (-k)")?;

    // Check if it's a search
    if let Some(search_type_chirho) = args_chirho.search_chirho {
        search_module_chirho(
            &module_name_chirho,
            &key_text_chirho,
            search_type_chirho,
            args_chirho.max_results_chirho,
            args_chirho.range_chirho.as_deref(),
        )
    } else {
        lookup_reference_chirho(
            &module_name_chirho,
            &key_text_chirho,
            &args_chirho.format_chirho,
            args_chirho.options_chirho.as_deref(),
        )
    }
}

fn list_modules_chirho() -> Result<(), Box<dyn std::error::Error>> {
    let mgr_chirho = SwMgrChirho::with_system_paths_chirho()?;

    println!("Installed modules:");
    for name_chirho in mgr_chirho.get_module_names_chirho() {
        if let Some(config_chirho) = mgr_chirho.get_module_chirho(name_chirho) {
            let desc_chirho = config_chirho.description_chirho().unwrap_or("No description");
            let lang_chirho = config_chirho.language_chirho().unwrap_or("??");
            println!("  {} [{}] - {}", name_chirho, lang_chirho, desc_chirho);
        }
    }

    Ok(())
}

fn handle_system_query_chirho(query_chirho: &str) -> Result<(), Box<dyn std::error::Error>> {
    match query_chirho {
        "modulelist" => {
            let mgr_chirho = SwMgrChirho::with_system_paths_chirho()?;
            for name_chirho in mgr_chirho.get_module_names_chirho() {
                println!("{}", name_chirho);
            }
        }
        "localelist" => {
            // TODO: Implement locale list
            println!("en");
        }
        _ => {
            eprintln!("Unknown system query: {}", query_chirho);
        }
    }
    Ok(())
}

fn lookup_reference_chirho(
    module_name_chirho: &str,
    key_text_chirho: &str,
    format_chirho: &OutputFormatChirho,
    options_chirho: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mgr_chirho = SwMgrChirho::with_system_paths_chirho()?;

    let config_chirho = mgr_chirho.get_module_chirho(module_name_chirho)
        .ok_or_else(|| format!("Module not found: {}", module_name_chirho))?;

    // Parse the verse reference
    let key_chirho = VerseKeyChirho::from_str_chirho(key_text_chirho)?;

    // TODO: Actually read from module storage
    // For now, print the parsed reference
    println!("{}: {}", module_name_chirho, key_chirho);
    println!("(Module reading not yet implemented - showing parsed reference)");

    Ok(())
}

fn search_module_chirho(
    module_name_chirho: &str,
    pattern_chirho: &str,
    search_type_chirho: SearchTypeChirho,
    max_results_chirho: Option<usize>,
    range_chirho: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Searching {} for '{}' ({:?})", module_name_chirho, pattern_chirho, search_type_chirho);
    println!("(Search not yet implemented)");

    Ok(())
}
