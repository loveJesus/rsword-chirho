// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Example: Creating a SWORD module.
//!
//! This example shows how to create a new SWORD module and write verses to it.

use rsword_chirho::config_chirho::ModuleConfigChirho;
use rsword_chirho::storage_chirho::RawVerseChirho;
use rsword_chirho::modules_chirho::RawTextChirho;
use rsword_chirho::modules_chirho::SwModuleChirho;
use std::path::Path;

fn main() -> rsword_chirho::error_chirho::ResultChirho<()> {
    let output_dir_chirho = std::env::args().nth(1)
        .unwrap_or_else(|| "/tmp/mymodule".to_string());

    println!("Creating module at: {}", output_dir_chirho);

    // Create the directory
    std::fs::create_dir_all(&output_dir_chirho)?;

    // Create the storage layer
    let mut storage_chirho = RawVerseChirho::create_chirho(Path::new(&output_dir_chirho))?;

    // Write some verses (Genesis 1:1-3)
    // Index 2 is Genesis 1:1 (after testament and book intros)
    storage_chirho.write_verse_chirho(1, 2, "In the beginning God created the heaven and the earth.")?;
    storage_chirho.write_verse_chirho(1, 3, "And the earth was without form, and void; and darkness was upon the face of the deep.")?;
    storage_chirho.write_verse_chirho(1, 4, "And God said, Let there be light: and there was light.")?;

    println!("Wrote 3 verses to storage");

    // Create module configuration
    let mut config_chirho = ModuleConfigChirho::new_chirho("MyBible".to_string());
    config_chirho.set_chirho("Description", "My Custom Bible Module");
    config_chirho.set_chirho("Lang", "en");
    config_chirho.set_chirho("ModDrv", "RawText");
    config_chirho.set_chirho("Versification", "KJV");
    config_chirho.set_chirho("SourceType", "OSIS");

    // Save configuration file
    let conf_path_chirho = format!("{}/mybible.conf", output_dir_chirho);
    std::fs::write(&conf_path_chirho, format!(
        "[MyBible]\nDescription={}\nLang={}\nModDrv={}\nDataPath=.\nVersification={}\n",
        "My Custom Bible Module", "en", "RawText", "KJV"
    ))?;
    println!("Wrote config file: {}", conf_path_chirho);

    // Verify by reading back
    println!("\n--- Verifying module ---");
    let mut module_chirho = RawTextChirho::new_chirho(&output_dir_chirho, config_chirho)?;

    println!("Module: {}", module_chirho.name_chirho());
    println!("Type: {}", module_chirho.module_type_chirho());

    // Read the verses back
    let text_chirho = module_chirho.get_verse_chirho("Genesis 1:1")?;
    println!("\nGenesis 1:1: {}", text_chirho);

    println!("\n=== Module created successfully! ===");
    Ok(())
}
