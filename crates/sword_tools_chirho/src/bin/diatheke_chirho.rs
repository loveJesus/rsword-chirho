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
use rsword_chirho::filters_chirho::FilterOptionsChirho;
use rsword_chirho::keys_chirho::VerseKeyChirho;
use rsword_chirho::manager_chirho::{SwMgrChirho, OutputFormatChirho as ModOutputFormatChirho};
use rsword_chirho::search_chirho::{RegexSearchChirho, SearchOptionsChirho, SearchTypeChirho as LibSearchTypeChirho};

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

    /// Option filters (n=notes, f=footnotes, m=morph, s=strongs, h=headings, r=redletter, x=xrefs)
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

    if let Err(e_chirho) = run_chirho(args_chirho) {
        eprintln!("Error: {}", e_chirho);
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

    // Parse filter options
    let filter_options_chirho = parse_filter_options_chirho(args_chirho.options_chirho.as_deref());

    // Convert output format
    let output_format_chirho = match args_chirho.format_chirho {
        OutputFormatChirho::Plain => ModOutputFormatChirho::PlainChirho,
        OutputFormatChirho::Html | OutputFormatChirho::Htmlhref | OutputFormatChirho::Xhtml => {
            ModOutputFormatChirho::HtmlChirho
        }
        _ => ModOutputFormatChirho::PlainChirho,
    };

    // Check if it's a search
    if let Some(search_type_chirho) = args_chirho.search_chirho {
        search_module_chirho(
            &module_name_chirho,
            &key_text_chirho,
            search_type_chirho,
            args_chirho.max_results_chirho,
            args_chirho.range_chirho.as_deref(),
            &filter_options_chirho,
            output_format_chirho,
        )
    } else {
        lookup_reference_chirho(
            &module_name_chirho,
            &key_text_chirho,
            &filter_options_chirho,
            output_format_chirho,
        )
    }
}

/// Parse option string into FilterOptionsChirho.
fn parse_filter_options_chirho(options_chirho: Option<&str>) -> FilterOptionsChirho {
    let mut filter_opts_chirho = FilterOptionsChirho::default();

    if let Some(opts_chirho) = options_chirho {
        for ch_chirho in opts_chirho.chars() {
            match ch_chirho {
                'n' | 'f' => filter_opts_chirho.footnotes_chirho = true,
                'm' => filter_opts_chirho.morph_chirho = true,
                's' => filter_opts_chirho.strongs_chirho = true,
                'h' => filter_opts_chirho.headings_chirho = true,
                'r' => filter_opts_chirho.red_letter_chirho = true,
                'x' => filter_opts_chirho.xrefs_chirho = true,
                'l' => filter_opts_chirho.lemmas_chirho = true,
                'a' => {
                    // All options
                    filter_opts_chirho = FilterOptionsChirho::all_chirho();
                }
                _ => {}
            }
        }
    }

    filter_opts_chirho
}

fn list_modules_chirho() -> Result<(), Box<dyn std::error::Error>> {
    let mgr_chirho = SwMgrChirho::with_system_paths_chirho()?;

    println!("Installed modules:");
    for name_chirho in mgr_chirho.get_module_names_chirho() {
        if let Some(config_chirho) = mgr_chirho.get_module_chirho(name_chirho) {
            let desc_chirho = config_chirho.description_chirho().unwrap_or("No description");
            let lang_chirho = config_chirho.language_chirho().unwrap_or("??");
            let driver_chirho = config_chirho.module_driver_chirho().unwrap_or("Unknown");
            println!("  {} [{}] ({}) - {}", name_chirho, lang_chirho, driver_chirho, desc_chirho);
        }
    }

    if mgr_chirho.module_count_chirho() == 0 {
        println!("  (No modules installed)");
        println!();
        println!("Install modules with: installmgr_chirho -ri CrossWire <module_name>");
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
            println!("en");
            println!("de");
            println!("es");
            println!("fr");
            println!("he");
            println!("el");
            println!("la");
        }
        "version" => {
            println!("diatheke_chirho {}", rsword_chirho::VERSION_CHIRHO);
        }
        _ => {
            eprintln!("Unknown system query: {}", query_chirho);
            eprintln!("Available queries: modulelist, localelist, version");
        }
    }
    Ok(())
}

fn lookup_reference_chirho(
    module_name_chirho: &str,
    key_text_chirho: &str,
    filter_options_chirho: &FilterOptionsChirho,
    output_format_chirho: ModOutputFormatChirho,
) -> Result<(), Box<dyn std::error::Error>> {
    let mgr_chirho = SwMgrChirho::with_system_paths_chirho()?;

    // Try to load the module
    match mgr_chirho.load_module_chirho(module_name_chirho) {
        Ok(module_chirho) => {
            // Parse the verse reference
            let key_chirho = VerseKeyChirho::from_str_chirho(key_text_chirho)?;

            // Read the entry with filtering
            let text_chirho = module_chirho.read_entry_filtered_chirho(
                &key_chirho.to_string(),
                output_format_chirho,
                filter_options_chirho,
            )?;

            if text_chirho.is_empty() {
                println!("{}: (No text found)", key_chirho);
            } else {
                println!("{}: {}", key_chirho, text_chirho);
            }
        }
        Err(_) => {
            // Module not found or not loadable - check if config exists
            if mgr_chirho.get_module_chirho(module_name_chirho).is_some() {
                // Config exists but module data not found
                let key_chirho = VerseKeyChirho::from_str_chirho(key_text_chirho)?;
                println!("{}: {}", module_name_chirho, key_chirho);
                println!("(Module data not found - module may not be fully installed)");
            } else {
                return Err(format!("Module not found: {}", module_name_chirho).into());
            }
        }
    }

    Ok(())
}

fn search_module_chirho(
    module_name_chirho: &str,
    pattern_chirho: &str,
    search_type_chirho: SearchTypeChirho,
    max_results_chirho: Option<usize>,
    _range_chirho: Option<&str>,
    _filter_options_chirho: &FilterOptionsChirho,
    _output_format_chirho: ModOutputFormatChirho,
) -> Result<(), Box<dyn std::error::Error>> {
    let mgr_chirho = SwMgrChirho::with_system_paths_chirho()?;

    // Convert search type
    let lib_search_type_chirho = match search_type_chirho {
        SearchTypeChirho::Regex => LibSearchTypeChirho::RegexChirho,
        SearchTypeChirho::Phrase => LibSearchTypeChirho::PhraseChirho,
        SearchTypeChirho::Multiword => LibSearchTypeChirho::MultiWordChirho,
        SearchTypeChirho::Lucene => LibSearchTypeChirho::ExternalChirho,
    };

    let _search_options_chirho = SearchOptionsChirho {
        search_type_chirho: Some(lib_search_type_chirho),
        case_insensitive_chirho: true,
        max_results_chirho,
        ..Default::default()
    };

    // Try to load the module
    match mgr_chirho.load_module_chirho(module_name_chirho) {
        Ok(_module_chirho) => {
            println!("Searching {} for '{}'...", module_name_chirho, pattern_chirho);
            println!();

            // For now, use a simple regex search
            // In the future, this would check for a tantivy index
            let _search_engine_chirho = RegexSearchChirho::new_chirho();

            // We would need to iterate all entries and add them to the search engine
            // For Bible modules, iterate all verses
            // This is a simplified version - a real implementation would use tantivy indexes

            println!("(Note: Full search requires a search index. Use mkfastmod_chirho to create one.)");
            println!();

            // Show what we're searching for
            println!("Search pattern: {}", pattern_chirho);
            println!("Search type: {:?}", search_type_chirho);
            println!("Max results: {:?}", max_results_chirho.unwrap_or(100));
        }
        Err(_) => {
            if mgr_chirho.get_module_chirho(module_name_chirho).is_some() {
                println!("Module {} found but data not accessible", module_name_chirho);
            } else {
                return Err(format!("Module not found: {}", module_name_chirho).into());
            }
        }
    }

    Ok(())
}
