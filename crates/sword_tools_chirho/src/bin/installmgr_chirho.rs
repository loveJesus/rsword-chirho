// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! installmgr_chirho - SWORD module installation manager.
//!
//! Drop-in replacement for the SWORD installmgr command.
//!
//! Usage:
//!   installmgr_chirho -init
//!   installmgr_chirho -s
//!   installmgr_chirho -r CrossWire
//!   installmgr_chirho -ri CrossWire KJV

use clap::Parser;
use std::path::PathBuf;

use rsword_chirho::manager_chirho::{InstallMgrChirho, SwMgrChirho};

/// SWORD module installation manager.
#[derive(Parser, Debug)]
#[command(name = "installmgr_chirho")]
#[command(author = "rsword-chirho contributors")]
#[command(version)]
#[command(about = "SWORD module installation manager", long_about = None)]
struct ArgsChirho {
    /// Initialize the install manager
    #[arg(long = "init")]
    init_chirho: bool,

    /// Sync with remote sources
    #[arg(long = "sc")]
    sync_config_chirho: bool,

    /// List remote sources
    #[arg(short = 's', long = "sources")]
    list_sources_chirho: bool,

    /// Add a new source
    #[arg(long = "sn")]
    add_source_chirho: Option<String>,

    /// Refresh a source (download module list)
    #[arg(short = 'r', long = "refresh")]
    refresh_source_chirho: Option<String>,

    /// List available modules from a source
    #[arg(long = "rl")]
    list_remote_chirho: Option<String>,

    /// List all remote modules (all sources)
    #[arg(long = "rlu")]
    list_all_remote_chirho: bool,

    /// Download a module description
    #[arg(long = "rd")]
    remote_desc_chirho: Option<String>,

    /// Download all module descriptions
    #[arg(long = "rdu")]
    all_remote_desc_chirho: bool,

    /// Get module description
    #[arg(long = "desc")]
    get_desc_chirho: Option<String>,

    /// Get remote module description
    #[arg(long = "rdesc")]
    get_remote_desc_chirho: Option<String>,

    /// Install a module: source module
    #[arg(long = "ri", num_args = 2)]
    install_chirho: Option<Vec<String>>,

    /// List installed modules
    #[arg(short = 'l', long = "list")]
    list_installed_chirho: bool,

    /// List all installed modules (detailed)
    #[arg(long = "lu")]
    list_installed_detailed_chirho: bool,

    /// Uninstall a module
    #[arg(short = 'u', long = "uninstall")]
    uninstall_chirho: Option<String>,

    /// Local module path for listing
    #[arg(long = "ll")]
    local_list_chirho: Option<PathBuf>,

    /// Local module path for install
    #[arg(long = "li")]
    local_install_chirho: Option<PathBuf>,

    /// Installation destination directory
    #[arg(short = 'd', long = "dest")]
    dest_dir_chirho: Option<PathBuf>,

    /// Exclude remote sources
    #[arg(long = "xr")]
    exclude_remote_chirho: bool,
}

fn main() {
    env_logger::init();

    let args_chirho = ArgsChirho::parse();

    if let Err(e) = run_chirho(args_chirho) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn get_config_dir_chirho() -> PathBuf {
    dirs::home_dir()
        .map(|h| h.join(".sword"))
        .unwrap_or_else(|| PathBuf::from(".sword"))
}

fn run_chirho(args_chirho: ArgsChirho) -> Result<(), Box<dyn std::error::Error>> {
    let config_dir_chirho = args_chirho.dest_dir_chirho.clone()
        .unwrap_or_else(get_config_dir_chirho);

    let mut mgr_chirho = InstallMgrChirho::new_chirho(&config_dir_chirho)?;

    // Handle init
    if args_chirho.init_chirho {
        println!("Initializing install manager...");
        mgr_chirho.init_chirho()?;
        println!("Done. Configuration saved to {:?}", config_dir_chirho);
        return Ok(());
    }

    // Handle list sources
    if args_chirho.list_sources_chirho {
        println!("Remote sources:");
        for source_chirho in mgr_chirho.get_sources_chirho() {
            println!("  [{}] {} - {}",
                source_chirho.source_type_chirho,
                source_chirho.caption_chirho,
                source_chirho.url_chirho()
            );
        }
        return Ok(());
    }

    // Handle list installed
    if args_chirho.list_installed_chirho || args_chirho.list_installed_detailed_chirho {
        let sw_mgr_chirho = SwMgrChirho::with_system_paths_chirho()?;
        println!("Installed modules:");
        for name_chirho in sw_mgr_chirho.get_module_names_chirho() {
            if args_chirho.list_installed_detailed_chirho {
                if let Some(config_chirho) = sw_mgr_chirho.get_module_chirho(name_chirho) {
                    let desc_chirho = config_chirho.description_chirho().unwrap_or("");
                    let driver_chirho = config_chirho.module_driver_chirho().unwrap_or("");
                    println!("  {} [{}] - {}", name_chirho, driver_chirho, desc_chirho);
                }
            } else {
                println!("  {}", name_chirho);
            }
        }
        return Ok(());
    }

    // Handle refresh source
    if let Some(source_name_chirho) = &args_chirho.refresh_source_chirho {
        println!("Refreshing source: {}", source_name_chirho);
        // TODO: Implement async refresh
        println!("(Refresh not yet implemented)");
        return Ok(());
    }

    // Handle list remote modules
    if let Some(source_name_chirho) = &args_chirho.list_remote_chirho {
        println!("Modules available from {}:", source_name_chirho);
        let modules_chirho = mgr_chirho.list_remote_modules_chirho(source_name_chirho)?;
        for module_chirho in modules_chirho {
            println!("  {}", module_chirho);
        }
        return Ok(());
    }

    // Handle install module
    if let Some(install_args_chirho) = &args_chirho.install_chirho {
        if install_args_chirho.len() >= 2 {
            let source_name_chirho = &install_args_chirho[0];
            let module_name_chirho = &install_args_chirho[1];
            println!("Installing {} from {}...", module_name_chirho, source_name_chirho);
            println!("(Installation not yet implemented)");
        }
        return Ok(());
    }

    // Handle uninstall module
    if let Some(module_name_chirho) = &args_chirho.uninstall_chirho {
        println!("Uninstalling {}...", module_name_chirho);
        mgr_chirho.uninstall_module_chirho(module_name_chirho)?;
        println!("Done.");
        return Ok(());
    }

    // Default: show help
    println!("Use --help for usage information");

    Ok(())
}

// Minimal dirs implementation
mod dirs {
    use std::path::PathBuf;

    pub fn home_dir() -> Option<PathBuf> {
        std::env::var_os("HOME").map(PathBuf::from)
    }
}
