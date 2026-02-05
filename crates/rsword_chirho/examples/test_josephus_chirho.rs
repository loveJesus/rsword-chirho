// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

use rsword_chirho::SwMgrChirho;
use rsword_chirho::modules_chirho::RawGenBookChirho;
use std::path::PathBuf;

fn main() {
    // Initialize SwMgr
    let mut mgr_chirho = SwMgrChirho::new_chirho();
    mgr_chirho.add_path_chirho(&PathBuf::from("/Users/hallelujah/.sword"));
    mgr_chirho.load_modules_chirho().expect("Failed to load modules");

    println!("Modules loaded: {}", mgr_chirho.module_count_chirho());

    // Try to load Josephus directly with RawGenBook
    println!("\nDirect RawGenBook load test:");
    let dir_path_chirho = PathBuf::from("/Users/hallelujah/.sword/modules/genbook/rawgenbook/josephus");
    let basename_chirho = "josephus";

    println!("  Directory: {:?}", dir_path_chirho);
    println!("  Basename: {}", basename_chirho);
    println!("  Expected bdt: {:?}", dir_path_chirho.join(format!("{}.bdt", basename_chirho)));
    println!("  bdt exists: {}", dir_path_chirho.join(format!("{}.bdt", basename_chirho)).exists());

    match RawGenBookChirho::open_chirho(&dir_path_chirho, basename_chirho) {
        Ok(gb_chirho) => {
            println!("  Opened successfully!");
            println!("  entry_count: {}", gb_chirho.entry_count_chirho());
            let roots_chirho = gb_chirho.get_root_keys_chirho();
            println!("  Root keys ({}): ", roots_chirho.len());
            for (i_chirho, key_chirho) in roots_chirho.iter().enumerate().take(10) {
                println!("    {}: {}", i_chirho, key_chirho);
            }
        }
        Err(e_chirho) => {
            println!("  Failed: {:?}", e_chirho);
        }
    }

    // Now try via SwMgr
    println!("\nVia SwMgr:");
    match mgr_chirho.load_module_chirho("Josephus") {
        Ok(loaded_chirho) => {
            println!("  Driver type: {:?}", loaded_chirho.driver_type_chirho);
            println!("  Data path: {:?}", loaded_chirho.data_path_chirho);

            // Calculate what as_genbook_chirho should use
            let dir_chirho = loaded_chirho.data_path_chirho.parent();
            let base_chirho = loaded_chirho.data_path_chirho.file_name();
            println!("  Computed dir: {:?}", dir_chirho);
            println!("  Computed base: {:?}", base_chirho);

            match loaded_chirho.as_genbook_chirho() {
                Some(gb_chirho) => {
                    println!("  Got GenBook wrapper!");
                    let roots_chirho = gb_chirho.get_root_keys_chirho();
                    println!("  Root keys ({}): ", roots_chirho.len());
                    for (i_chirho, key_chirho) in roots_chirho.iter().enumerate().take(10) {
                        println!("    {}: {}", i_chirho, key_chirho);
                    }

                    // Test get_children
                    if let Some(first_root_chirho) = roots_chirho.first() {
                        println!("\n  Children of root '{}':", first_root_chirho);
                        let children_chirho = gb_chirho.get_children_chirho(first_root_chirho);
                        println!("  Found {} children:", children_chirho.len());
                        for (i_chirho, child_chirho) in children_chirho.iter().enumerate().take(10) {
                            println!("    {}: {}", i_chirho, child_chirho);
                        }

                        // Test second-level children
                        if let Some(first_child_chirho) = children_chirho.first() {
                            println!("\n  Children of '{}':", first_child_chirho);
                            let grandchildren_chirho = gb_chirho.get_children_chirho(first_child_chirho);
                            println!("  Found {} grandchildren:", grandchildren_chirho.len());
                            for (i_chirho, gc_chirho) in grandchildren_chirho.iter().enumerate().take(10) {
                                println!("    {}: {}", i_chirho, gc_chirho);
                            }
                        }
                    }
                }
                None => {
                    println!("  as_genbook_chirho returned None!");
                }
            }
        }
        Err(e_chirho) => {
            println!("  Failed to load: {:?}", e_chirho);
        }
    }
}
