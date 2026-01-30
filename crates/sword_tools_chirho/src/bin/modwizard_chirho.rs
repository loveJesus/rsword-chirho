// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Interactive SWORD module creation wizard.
//!
//! Guides users through creating new SWORD modules with prompts for
//! all required and optional metadata.
//!
//! # Usage
//!
//! ```bash
//! # Interactive mode
//! modwizard_chirho
//!
//! # Quick mode with type specified
//! modwizard_chirho --type-chirho bible --name-chirho "MyBible" --input-chirho bible.osis
//!
//! # Create lexicon
//! modwizard_chirho --type-chirho lexicon --name-chirho "MyDict" --input-chirho dict.tei
//! ```

use std::io::{self, Write};
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

/// SWORD module creation wizard.
#[derive(Parser, Debug)]
#[command(name = "modwizard_chirho")]
#[command(about = "Interactive wizard for creating SWORD modules")]
#[command(version)]
struct ArgsChirho {
    /// Module type (bible, commentary, lexicon, genbook).
    #[arg(short = 't', long = "type-chirho")]
    module_type_chirho: Option<String>,

    /// Module name (short identifier, e.g., "KJV").
    #[arg(short = 'n', long = "name-chirho")]
    name_chirho: Option<String>,

    /// Module description.
    #[arg(short = 'd', long = "description-chirho")]
    description_chirho: Option<String>,

    /// Input file (OSIS, TEI, or IMP).
    #[arg(short = 'i', long = "input-chirho")]
    input_chirho: Option<PathBuf>,

    /// Output directory.
    #[arg(short = 'o', long = "output-chirho")]
    output_chirho: Option<PathBuf>,

    /// Language code (e.g., "en", "grc", "heb").
    #[arg(short = 'l', long = "lang-chirho")]
    language_chirho: Option<String>,

    /// Versification system.
    #[arg(long = "v11n-chirho")]
    versification_chirho: Option<String>,

    /// Source markup type (OSIS, ThML, GBF, TEI).
    #[arg(long = "source-type-chirho")]
    source_type_chirho: Option<String>,

    /// Use compression (ZIP, BZIP2, XZ).
    #[arg(short = 'c', long = "compress-chirho")]
    compress_chirho: Option<String>,

    /// Block type for compression (verse, chapter, book).
    #[arg(long = "block-type-chirho")]
    block_type_chirho: Option<String>,

    /// Non-interactive mode.
    #[arg(long = "non-interactive-chirho")]
    non_interactive_chirho: bool,
}

/// Module creation configuration.
#[derive(Debug, Clone)]
struct ModuleConfigOptionsChirho {
    /// Module type.
    module_type_chirho: String,
    /// Module name.
    name_chirho: String,
    /// Description.
    description_chirho: String,
    /// Input file path.
    input_path_chirho: PathBuf,
    /// Output directory.
    output_path_chirho: PathBuf,
    /// Language code.
    language_chirho: String,
    /// Versification system.
    versification_chirho: String,
    /// Source markup type.
    source_type_chirho: String,
    /// Compression type.
    compress_chirho: Option<String>,
    /// Block type.
    block_type_chirho: String,
}

/// Prompt for input with a default value.
fn prompt_chirho(prompt_chirho: &str, default_chirho: Option<&str>) -> Result<String> {
    let default_str_chirho = default_chirho.unwrap_or("");
    if default_str_chirho.is_empty() {
        print!("{}: ", prompt_chirho);
    } else {
        print!("{} [{}]: ", prompt_chirho, default_str_chirho);
    }
    io::stdout().flush()?;

    let mut input_chirho = String::new();
    io::stdin().read_line(&mut input_chirho)?;
    let input_chirho = input_chirho.trim();

    if input_chirho.is_empty() {
        Ok(default_str_chirho.to_string())
    } else {
        Ok(input_chirho.to_string())
    }
}

/// Prompt for a selection from a list.
fn prompt_select_chirho(prompt_chirho: &str, options_chirho: &[&str], default_idx_chirho: usize) -> Result<String> {
    println!("{}:", prompt_chirho);
    for (i_chirho, opt_chirho) in options_chirho.iter().enumerate() {
        let marker_chirho = if i_chirho == default_idx_chirho { "*" } else { " " };
        println!("  {} {}. {}", marker_chirho, i_chirho + 1, opt_chirho);
    }
    print!("Select [{}]: ", default_idx_chirho + 1);
    io::stdout().flush()?;

    let mut input_chirho = String::new();
    io::stdin().read_line(&mut input_chirho)?;
    let input_chirho = input_chirho.trim();

    if input_chirho.is_empty() {
        return Ok(options_chirho[default_idx_chirho].to_string());
    }

    if let Ok(idx_chirho) = input_chirho.parse::<usize>() {
        if idx_chirho > 0 && idx_chirho <= options_chirho.len() {
            return Ok(options_chirho[idx_chirho - 1].to_string());
        }
    }

    // Try matching by name
    for opt_chirho in options_chirho {
        if opt_chirho.eq_ignore_ascii_case(input_chirho) {
            return Ok(opt_chirho.to_string());
        }
    }

    // Default
    Ok(options_chirho[default_idx_chirho].to_string())
}

/// Prompt for yes/no.
fn prompt_yn_chirho(prompt_chirho: &str, default_chirho: bool) -> Result<bool> {
    let default_str_chirho = if default_chirho { "Y/n" } else { "y/N" };
    print!("{} [{}]: ", prompt_chirho, default_str_chirho);
    io::stdout().flush()?;

    let mut input_chirho = String::new();
    io::stdin().read_line(&mut input_chirho)?;
    let input_chirho = input_chirho.trim().to_lowercase();

    if input_chirho.is_empty() {
        return Ok(default_chirho);
    }

    Ok(input_chirho.starts_with('y'))
}

/// Interactive configuration wizard.
fn interactive_wizard_chirho(args_chirho: &ArgsChirho) -> Result<ModuleConfigOptionsChirho> {
    println!("=================================================");
    println!("       SWORD Module Creation Wizard");
    println!("=================================================");
    println!();

    // Module type
    let module_types_chirho = ["Bible", "Commentary", "Lexicon", "GenBook"];
    let module_type_chirho = if let Some(ref t_chirho) = args_chirho.module_type_chirho {
        t_chirho.clone()
    } else {
        prompt_select_chirho("Module type", &module_types_chirho, 0)?
    };

    println!();

    // Module name
    let name_chirho = if let Some(ref n_chirho) = args_chirho.name_chirho {
        n_chirho.clone()
    } else {
        let name_chirho = prompt_chirho("Module name (short identifier, e.g., KJV)", None)?;
        if name_chirho.is_empty() {
            anyhow::bail!("Module name is required");
        }
        name_chirho
    };

    // Description
    let description_chirho = if let Some(ref d_chirho) = args_chirho.description_chirho {
        d_chirho.clone()
    } else {
        prompt_chirho("Description", Some(&format!("{} module", name_chirho)))?
    };

    // Language
    let language_chirho = if let Some(ref l_chirho) = args_chirho.language_chirho {
        l_chirho.clone()
    } else {
        let common_langs_chirho = ["en", "grc", "heb", "la", "de", "es", "fr", "other"];
        prompt_select_chirho("Language", &common_langs_chirho, 0)?
    };

    println!();

    // Input file
    let input_path_chirho = if let Some(ref p_chirho) = args_chirho.input_chirho {
        p_chirho.clone()
    } else {
        let path_str_chirho = prompt_chirho("Input file path (OSIS, TEI, or IMP)", None)?;
        if path_str_chirho.is_empty() {
            anyhow::bail!("Input file is required");
        }
        PathBuf::from(path_str_chirho)
    };

    if !input_path_chirho.exists() {
        eprintln!("Warning: Input file '{}' does not exist", input_path_chirho.display());
    }

    // Detect source type from file extension
    let detected_source_chirho = match input_path_chirho.extension().and_then(|e_chirho| e_chirho.to_str()) {
        Some("xml") | Some("osis") => "OSIS",
        Some("tei") => "TEI",
        Some("imp") => "Plain",
        Some("thml") => "ThML",
        Some("gbf") => "GBF",
        _ => "OSIS",
    };

    // Source type
    let source_type_chirho = if let Some(ref s_chirho) = args_chirho.source_type_chirho {
        s_chirho.clone()
    } else {
        let source_types_chirho = ["OSIS", "ThML", "GBF", "TEI", "Plain"];
        let default_idx_chirho = source_types_chirho.iter().position(|s_chirho| *s_chirho == detected_source_chirho).unwrap_or(0);
        prompt_select_chirho("Source markup type", &source_types_chirho, default_idx_chirho)?
    };

    // Output directory
    let output_path_chirho = if let Some(ref p_chirho) = args_chirho.output_chirho {
        p_chirho.clone()
    } else {
        let default_output_chirho = format!("./modules/{}", name_chirho.to_lowercase());
        PathBuf::from(prompt_chirho("Output directory", Some(&default_output_chirho))?)
    };

    println!();

    // Versification (for Bible/Commentary)
    let versification_chirho = if module_type_chirho.eq_ignore_ascii_case("bible")
        || module_type_chirho.eq_ignore_ascii_case("commentary")
    {
        if let Some(ref v_chirho) = args_chirho.versification_chirho {
            v_chirho.clone()
        } else {
            let v11n_systems_chirho = ["KJV", "Catholic", "LXX", "Synodal", "Luther", "Vulgate", "NRSV", "Leningrad"];
            prompt_select_chirho("Versification system", &v11n_systems_chirho, 0)?
        }
    } else {
        "KJV".to_string()
    };

    println!();

    // Compression
    let compress_chirho = if let Some(ref c_chirho) = args_chirho.compress_chirho {
        Some(c_chirho.clone())
    } else if prompt_yn_chirho("Use compression", true)? {
        let comp_types_chirho = ["ZIP", "BZIP2", "XZ"];
        Some(prompt_select_chirho("Compression type", &comp_types_chirho, 0)?)
    } else {
        None
    };

    // Block type
    let block_type_chirho = if compress_chirho.is_some() {
        if let Some(ref b_chirho) = args_chirho.block_type_chirho {
            b_chirho.clone()
        } else {
            let block_types_chirho = ["verse", "chapter", "book"];
            prompt_select_chirho("Block type (how to group compressed data)", &block_types_chirho, 1)?
        }
    } else {
        "verse".to_string()
    };

    Ok(ModuleConfigOptionsChirho {
        module_type_chirho,
        name_chirho,
        description_chirho,
        input_path_chirho,
        output_path_chirho,
        language_chirho,
        versification_chirho,
        source_type_chirho,
        compress_chirho,
        block_type_chirho,
    })
}

/// Display configuration summary.
fn show_summary_chirho(config_chirho: &ModuleConfigOptionsChirho) {
    println!();
    println!("=================================================");
    println!("            Configuration Summary");
    println!("=================================================");
    println!("Module Type:     {}", config_chirho.module_type_chirho);
    println!("Name:            {}", config_chirho.name_chirho);
    println!("Description:     {}", config_chirho.description_chirho);
    println!("Language:        {}", config_chirho.language_chirho);
    println!("Input File:      {}", config_chirho.input_path_chirho.display());
    println!("Output Dir:      {}", config_chirho.output_path_chirho.display());
    println!("Versification:   {}", config_chirho.versification_chirho);
    println!("Source Markup:   {}", config_chirho.source_type_chirho);
    if let Some(ref comp_chirho) = config_chirho.compress_chirho {
        println!("Compression:     {} ({})", comp_chirho, config_chirho.block_type_chirho);
    } else {
        println!("Compression:     None (raw module)");
    }
    println!("=================================================");
    println!();
}

/// Create the module.
fn create_module_chirho(config_chirho: &ModuleConfigOptionsChirho) -> Result<()> {
    use std::process::Command;

    // Determine which tool to use based on input file and module type
    let tool_chirho = match (config_chirho.module_type_chirho.to_lowercase().as_str(), config_chirho.source_type_chirho.as_str()) {
        ("bible", "OSIS") | ("commentary", "OSIS") => "osis2mod_chirho",
        ("lexicon", "TEI") => "tei2mod_chirho",
        ("bible", "Plain") | ("commentary", "Plain") => "imp2vs_chirho",
        ("lexicon", "Plain") => "imp2ld_chirho",
        ("genbook", _) => "imp2gbs_chirho",
        _ => "osis2mod_chirho",
    };

    // Create output directory
    std::fs::create_dir_all(&config_chirho.output_path_chirho)?;

    // Build command
    let mut cmd_chirho = Command::new(tool_chirho);
    cmd_chirho.arg(&config_chirho.output_path_chirho);
    cmd_chirho.arg(&config_chirho.input_path_chirho);

    // Add versification for Bible/Commentary
    if config_chirho.module_type_chirho.eq_ignore_ascii_case("bible")
        || config_chirho.module_type_chirho.eq_ignore_ascii_case("commentary")
    {
        cmd_chirho.arg("-v");
        cmd_chirho.arg(&config_chirho.versification_chirho);
    }

    // Add compression options
    if let Some(ref comp_chirho) = config_chirho.compress_chirho {
        cmd_chirho.arg("-z");

        let block_num_chirho = match config_chirho.block_type_chirho.as_str() {
            "verse" => "2",
            "chapter" => "3",
            "book" => "4",
            _ => "3",
        };
        cmd_chirho.arg("-b");
        cmd_chirho.arg(block_num_chirho);

        let comp_flag_chirho = match comp_chirho.to_uppercase().as_str() {
            "BZIP2" => "b",
            "XZ" => "x",
            _ => "z",
        };
        cmd_chirho.arg("-c");
        cmd_chirho.arg(comp_flag_chirho);
    }

    println!("Running: {:?}", cmd_chirho);
    println!();

    // Try to run the tool
    match cmd_chirho.status() {
        Ok(status_chirho) => {
            if status_chirho.success() {
                println!("Module data created successfully!");

                // Create .conf file
                create_conf_file_chirho(config_chirho)?;

                println!();
                println!("Module created successfully!");
                println!();
                println!("Next steps:");
                println!("1. Copy {} to your SWORD module directory", config_chirho.output_path_chirho.display());
                println!("2. Copy {}.conf to mods.d/", config_chirho.name_chirho.to_lowercase());
                println!("3. Test with: diatheke_chirho -b {} -k 'Gen 1:1'", config_chirho.name_chirho);
            } else {
                anyhow::bail!("Module creation failed with exit code: {:?}", status_chirho.code());
            }
        }
        Err(e_chirho) => {
            eprintln!("Warning: Could not run '{}': {}", tool_chirho, e_chirho);
            eprintln!();
            eprintln!("The tool might not be installed or not in PATH.");
            eprintln!("You can run manually:");
            eprintln!("  {} {} {}", tool_chirho, config_chirho.output_path_chirho.display(), config_chirho.input_path_chirho.display());

            // Still create the .conf file
            create_conf_file_chirho(config_chirho)?;
        }
    }

    Ok(())
}

/// Create the module .conf file.
fn create_conf_file_chirho(config_chirho: &ModuleConfigOptionsChirho) -> Result<()> {
    let conf_path_chirho = config_chirho.output_path_chirho.join(format!("{}.conf", config_chirho.name_chirho.to_lowercase()));

    let mod_drv_chirho = match (config_chirho.module_type_chirho.to_lowercase().as_str(), config_chirho.compress_chirho.is_some()) {
        ("bible", false) => "RawText",
        ("bible", true) => "zText",
        ("commentary", false) => "RawCom",
        ("commentary", true) => "zCom",
        ("lexicon", false) => "RawLD",
        ("lexicon", true) => "zLD",
        ("genbook", _) => "RawGenBook",
        _ => "RawText",
    };

    let data_path_chirho = format!("./modules/{}/{}", config_chirho.module_type_chirho.to_lowercase(), config_chirho.name_chirho.to_lowercase());

    let mut conf_content_chirho = format!(
        "[{}]\n\
         DataPath={}\n\
         ModDrv={}\n\
         Description={}\n\
         Lang={}\n\
         SourceType={}\n\
         Encoding=UTF-8\n",
        config_chirho.name_chirho,
        data_path_chirho,
        mod_drv_chirho,
        config_chirho.description_chirho,
        config_chirho.language_chirho,
        config_chirho.source_type_chirho,
    );

    if config_chirho.module_type_chirho.eq_ignore_ascii_case("bible")
        || config_chirho.module_type_chirho.eq_ignore_ascii_case("commentary")
    {
        conf_content_chirho.push_str(&format!("Versification={}\n", config_chirho.versification_chirho));
    }

    if let Some(ref comp_chirho) = config_chirho.compress_chirho {
        conf_content_chirho.push_str(&format!("CompressType={}\n", comp_chirho));
        let block_type_str_chirho = match config_chirho.block_type_chirho.as_str() {
            "verse" => "VERSEBLOCKS",
            "chapter" => "CHAPTERBLOCKS",
            "book" => "BOOKBLOCKS",
            _ => "CHAPTERBLOCKS",
        };
        conf_content_chirho.push_str(&format!("BlockType={}\n", block_type_str_chirho));
    }

    std::fs::write(&conf_path_chirho, conf_content_chirho)?;
    println!("Created config file: {}", conf_path_chirho.display());

    Ok(())
}

fn main() -> Result<()> {
    let args_chirho = ArgsChirho::parse();

    let config_chirho = if args_chirho.non_interactive_chirho {
        // Non-interactive mode requires all options
        let module_type_chirho = args_chirho.module_type_chirho.clone()
            .ok_or_else(|| anyhow::anyhow!("--type required in non-interactive mode"))?;
        let name_chirho = args_chirho.name_chirho.clone()
            .ok_or_else(|| anyhow::anyhow!("--name required in non-interactive mode"))?;
        let input_path_chirho = args_chirho.input_chirho.clone()
            .ok_or_else(|| anyhow::anyhow!("--input required in non-interactive mode"))?;

        ModuleConfigOptionsChirho {
            module_type_chirho,
            name_chirho: name_chirho.clone(),
            description_chirho: args_chirho.description_chirho.clone().unwrap_or_else(|| format!("{} module", name_chirho)),
            input_path_chirho: input_path_chirho.clone(),
            output_path_chirho: args_chirho.output_chirho.clone().unwrap_or_else(|| PathBuf::from(format!("./modules/{}", name_chirho.to_lowercase()))),
            language_chirho: args_chirho.language_chirho.clone().unwrap_or_else(|| "en".to_string()),
            versification_chirho: args_chirho.versification_chirho.clone().unwrap_or_else(|| "KJV".to_string()),
            source_type_chirho: args_chirho.source_type_chirho.clone().unwrap_or_else(|| {
                match input_path_chirho.extension().and_then(|e_chirho| e_chirho.to_str()) {
                    Some("tei") => "TEI",
                    Some("thml") => "ThML",
                    Some("gbf") => "GBF",
                    Some("imp") => "Plain",
                    _ => "OSIS",
                }.to_string()
            }),
            compress_chirho: args_chirho.compress_chirho.clone(),
            block_type_chirho: args_chirho.block_type_chirho.clone().unwrap_or_else(|| "chapter".to_string()),
        }
    } else {
        interactive_wizard_chirho(&args_chirho)?
    };

    show_summary_chirho(&config_chirho);

    if !args_chirho.non_interactive_chirho {
        if !prompt_yn_chirho("Proceed with module creation", true)? {
            println!("Aborted.");
            return Ok(());
        }
    }

    create_module_chirho(&config_chirho)?;

    Ok(())
}
