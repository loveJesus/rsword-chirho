// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Example: Reading a verse from a SWORD module.
//!
//! This example shows how to load a SWORD module and read a verse.

use rsword_chirho::config_chirho::ModuleConfigChirho;
use rsword_chirho::modules_chirho::RawTextChirho;
use rsword_chirho::modules_chirho::SwModuleChirho;

fn main() -> rsword_chirho::error_chirho::ResultChirho<()> {
    // Path to your SWORD modules directory
    let module_path_chirho = std::env::args().nth(1)
        .unwrap_or_else(|| "/usr/share/sword/modules/texts/rawtext/kjv".to_string());

    // Create module configuration
    let mut config_chirho = ModuleConfigChirho::new_chirho("KJV".to_string());
    config_chirho.set_chirho("Description", "King James Version");
    config_chirho.set_chirho("Lang", "en");
    config_chirho.set_chirho("Versification", "KJV");

    // Open the module
    let mut module_chirho = RawTextChirho::new_chirho(&module_path_chirho, config_chirho)?;

    // Read John 3:16
    let verse_chirho = module_chirho.get_verse_chirho("John 3:16")?;

    println!("Module: {}", module_chirho.name_chirho());
    println!("Reference: John 3:16");
    println!("Text: {}", verse_chirho);

    // Read a range of verses
    println!("\n--- Reading Genesis 1:1-3 ---");
    for verse_ref_chirho in ["Genesis 1:1", "Genesis 1:2", "Genesis 1:3"] {
        let text_chirho = module_chirho.get_verse_chirho(verse_ref_chirho)?;
        println!("{}: {}", verse_ref_chirho, text_chirho);
    }

    Ok(())
}
