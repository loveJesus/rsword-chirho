// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Example: List all installed SWORD modules.
//!
//! This example demonstrates how to:
//! - Initialize the SwMgr module manager
//! - Discover and list installed modules
//! - Show module metadata (name, description, language, type)
//!
//! Usage:
//!   cargo run --example list_modules_chirho

use rsword_chirho::SwMgrChirho;
use std::path::PathBuf;

fn main() {
    println!("rsword_chirho - List Installed Modules\n");

    // Get SWORD data path
    let sword_path_chirho = get_sword_path_chirho();

    match sword_path_chirho {
        Some(path_chirho) => {
            println!("SWORD path: {}\n", path_chirho.display());
            list_modules_chirho(&path_chirho);
        }
        None => {
            eprintln!("No SWORD modules found.");
            eprintln!("Install modules to ~/.sword or /usr/share/sword");
        }
    }
}

fn get_sword_path_chirho() -> Option<PathBuf> {
    // Check home directory first
    if let Ok(home_chirho) = std::env::var("HOME") {
        let sword_home_chirho = PathBuf::from(home_chirho).join(".sword");
        if sword_home_chirho.exists() && sword_home_chirho.join("mods.d").exists() {
            return Some(sword_home_chirho);
        }
    }

    // Check system path
    let system_path_chirho = PathBuf::from("/usr/share/sword");
    if system_path_chirho.exists() && system_path_chirho.join("mods.d").exists() {
        return Some(system_path_chirho);
    }

    None
}

fn list_modules_chirho(sword_path_chirho: &PathBuf) {
    let mut mgr_chirho = SwMgrChirho::new_chirho();
    mgr_chirho.add_path_chirho(sword_path_chirho);

    if let Err(e_chirho) = mgr_chirho.load_modules_chirho() {
        eprintln!("Error loading modules: {:?}", e_chirho);
        return;
    }

    let names_chirho = mgr_chirho.get_module_names_chirho();

    if names_chirho.is_empty() {
        println!("No modules found.");
        return;
    }

    println!("Found {} modules:\n", names_chirho.len());
    println!("{:<15} {:<10} {:<40}", "Module", "Language", "Description");
    println!("{}", "-".repeat(65));

    for name_chirho in &names_chirho {
        if let Ok(loaded_chirho) = mgr_chirho.load_module_chirho(name_chirho) {
            let description_chirho = loaded_chirho.config_chirho
                .get_chirho("Description")
                .unwrap_or("No description");

            let lang_chirho = loaded_chirho.config_chirho
                .get_chirho("Lang")
                .unwrap_or("??");

            // Truncate description if too long
            let desc_display_chirho = if description_chirho.len() > 37 {
                format!("{}...", &description_chirho[..37])
            } else {
                description_chirho.to_string()
            };

            println!("{:<15} {:<10} {:<40}", name_chirho, lang_chirho, desc_display_chirho);
        }
    }
}
