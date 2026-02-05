// Test Josephus in module list
use rsword_chirho::SwMgrChirho;
use std::path::PathBuf;

#[test]
fn test_josephus_in_list_chirho() {
    let mut mgr_chirho = SwMgrChirho::new_chirho();
    mgr_chirho.add_path_chirho(&PathBuf::from("/Users/hallelujah/.sword"));
    mgr_chirho.load_modules_chirho().expect("load failed");
    
    let names_chirho = mgr_chirho.get_module_names_chirho();
    println!("Found {} modules:", names_chirho.len());
    for name_chirho in &names_chirho {
        println!("  - {}", name_chirho);
    }
    
    assert!(names_chirho.iter().any(|n| *n == "Josephus"), 
            "Josephus not found in module list");
}
