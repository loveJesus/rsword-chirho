// Test Josephus config parsing
use rsword_chirho::config_chirho::ModuleConfigChirho;
use std::path::Path;

#[test]
fn test_josephus_config_parse_chirho() {
    let path_chirho = Path::new("/Users/hallelujah/.sword/mods.d/josephus.conf");
    
    match ModuleConfigChirho::from_file_chirho(path_chirho) {
        Ok(config_chirho) => {
            println!("Successfully parsed Josephus config:");
            println!("  Name: {}", config_chirho.name_chirho);
            println!("  Driver: {:?}", config_chirho.module_driver_chirho());
            println!("  DataPath: {:?}", config_chirho.data_path_chirho());
            println!("  Is GenBook: {}", config_chirho.is_genbook_chirho());
        }
        Err(e_chirho) => {
            panic!("Failed to parse: {:?}", e_chirho);
        }
    }
}
