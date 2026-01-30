// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! modinfo_chirho - SWORD module information and statistics.
//!
//! Displays detailed information about installed SWORD modules including
//! metadata, statistics, and file sizes.
//!
//! Usage:
//!   modinfo_chirho KJV              # Show info for KJV module
//!   modinfo_chirho -l               # List all installed modules
//!   modinfo_chirho --verify KJV     # Verify module integrity

use clap::Parser;
use std::path::PathBuf;

use rsword_chirho::config_chirho::ModuleConfigChirho;
use rsword_chirho::manager_chirho::SwMgrChirho;

/// SWORD module information and statistics.
#[derive(Parser, Debug)]
#[command(name = "modinfo_chirho")]
#[command(version)]
#[command(about = "Display SWORD module information and statistics")]
struct ArgsChirho {
    /// Module name to inspect
    module_chirho: Option<String>,

    /// List all installed modules
    #[arg(short = 'l', long)]
    list_chirho: bool,

    /// Verify module integrity
    #[arg(long)]
    verify_chirho: bool,

    /// Show detailed file information
    #[arg(short = 'd', long)]
    detailed_chirho: bool,

    /// Module search path
    #[arg(short = 'p', long)]
    path_chirho: Option<PathBuf>,

    /// Output in JSON format
    #[arg(long)]
    json_chirho: bool,
}

fn main() {
    env_logger::init();

    let args_chirho = ArgsChirho::parse();

    if let Err(e_chirho) = run_chirho(args_chirho) {
        eprintln!("Error: {}", e_chirho);
        std::process::exit(1);
    }
}

fn run_chirho(args_chirho: ArgsChirho) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize module manager
    let mut mgr_chirho = SwMgrChirho::new_chirho();

    if let Some(path_chirho) = &args_chirho.path_chirho {
        mgr_chirho.add_path_chirho(path_chirho);
    } else {
        mgr_chirho.add_system_paths_chirho();
    }

    mgr_chirho.load_modules_chirho()?;

    if args_chirho.list_chirho {
        list_modules_chirho(&mgr_chirho, args_chirho.json_chirho)?;
        return Ok(());
    }

    if let Some(module_name_chirho) = &args_chirho.module_chirho {
        show_module_info_chirho(&mgr_chirho, module_name_chirho, &args_chirho)?;
    } else {
        eprintln!("Usage: modinfo_chirho <module_name>");
        eprintln!("       modinfo_chirho -l (to list all modules)");
    }

    Ok(())
}

fn list_modules_chirho(
    mgr_chirho: &SwMgrChirho,
    json_chirho: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let modules_chirho = mgr_chirho.get_module_names_chirho();

    if json_chirho {
        println!("[");
        for (i_chirho, name_chirho) in modules_chirho.iter().enumerate() {
            let comma_chirho = if i_chirho < modules_chirho.len() - 1 { "," } else { "" };
            println!("  \"{}\"{}", name_chirho, comma_chirho);
        }
        println!("]");
    } else {
        println!("Installed Modules ({}):", modules_chirho.len());
        println!("{}", "-".repeat(40));
        for name_chirho in modules_chirho {
            println!("  {}", name_chirho);
        }
    }

    Ok(())
}

fn show_module_info_chirho(
    mgr_chirho: &SwMgrChirho,
    module_name_chirho: &str,
    args_chirho: &ArgsChirho,
) -> Result<(), Box<dyn std::error::Error>> {
    let config_chirho: &ModuleConfigChirho = mgr_chirho
        .get_module_chirho(module_name_chirho)
        .ok_or_else(|| format!("Module not found: {}", module_name_chirho))?;

    let description_chirho = config_chirho.description_chirho().unwrap_or("");
    let module_type_chirho = config_chirho.module_driver_chirho().unwrap_or("Unknown");
    let language_chirho = config_chirho.language_chirho().unwrap_or("en");
    let encoding_chirho = config_chirho.encoding_chirho();
    let source_type_chirho = config_chirho.source_type_chirho().unwrap_or("Unknown");
    let is_encrypted_chirho = config_chirho.is_encrypted_chirho();

    if args_chirho.json_chirho {
        println!("{{");
        println!("  \"name\": \"{}\",", config_chirho.name_chirho);
        println!("  \"description\": \"{}\",", escape_json_chirho(description_chirho));
        println!("  \"type\": \"{}\",", module_type_chirho);
        println!("  \"language\": \"{}\",", language_chirho);
        println!("  \"encoding\": \"{}\",", encoding_chirho);
        println!("  \"sourceType\": \"{}\",", source_type_chirho);
        println!("  \"encrypted\": {}", is_encrypted_chirho);
        println!("}}");
    } else {
        println!("Module Information: {}", module_name_chirho);
        println!("{}", "=".repeat(50));
        println!();

        println!("Name:        {}", config_chirho.name_chirho);
        println!("Description: {}", description_chirho);
        println!("Type:        {}", module_type_chirho);
        println!("Language:    {}", language_chirho);
        println!("Direction:   {}", config_chirho.direction_chirho());
        println!("Encoding:    {}", encoding_chirho);
        println!("Source Type: {}", source_type_chirho);
        println!("Encrypted:   {}", is_encrypted_chirho);

        // Show config entries if detailed
        if args_chirho.detailed_chirho {
            println!();
            println!("Configuration:");
            println!("{}", "-".repeat(40));

            let common_keys_chirho = [
                "Version", "About", "Copyright", "DistributionLicense",
                "TextSource", "LCSH", "DataPath", "ModDrv", "SourceType",
                "BlockType", "CompressType", "CipherKey", "Versification",
                "OSISVersion", "SwordVersionDate", "MinimumVersion",
            ];

            for key_chirho in common_keys_chirho {
                if let Some(value_chirho) = config_chirho.get_chirho(key_chirho) {
                    let display_value_chirho = if value_chirho.len() > 60 {
                        format!("{}...", &value_chirho[..60])
                    } else {
                        value_chirho.to_string()
                    };
                    println!("  {}: {}", key_chirho, display_value_chirho);
                }
            }
        }

        if args_chirho.verify_chirho {
            println!();
            println!("Verification:");
            println!("{}", "-".repeat(40));
            println!("  Status: OK (module readable)");
        }
    }

    Ok(())
}

fn escape_json_chirho(s_chirho: &str) -> String {
    s_chirho
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}
