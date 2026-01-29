// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! SWORD module manager.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::config_chirho::ModuleConfigChirho;
use crate::error_chirho::{ErrorChirho, ResultChirho};

/// SWORD module manager.
///
/// Discovers and manages installed SWORD modules from various paths.
pub struct SwMgrChirho {
    /// List of module paths to search.
    mod_paths_chirho: Vec<PathBuf>,
    /// Discovered module configurations.
    modules_chirho: HashMap<String, ModuleConfigChirho>,
}

impl SwMgrChirho {
    /// Create a new module manager.
    pub fn new_chirho() -> Self {
        Self {
            mod_paths_chirho: Vec::new(),
            modules_chirho: HashMap::new(),
        }
    }

    /// Create a module manager with default system paths.
    pub fn with_system_paths_chirho() -> ResultChirho<Self> {
        let mut mgr_chirho = Self::new_chirho();
        mgr_chirho.add_system_paths_chirho();
        mgr_chirho.load_modules_chirho()?;
        Ok(mgr_chirho)
    }

    /// Add default system module paths.
    pub fn add_system_paths_chirho(&mut self) {
        // Common SWORD module locations
        let paths_chirho = vec![
            // Unix-like systems
            PathBuf::from("/usr/share/sword"),
            PathBuf::from("/usr/local/share/sword"),
            // User-specific
            dirs::home_dir()
                .map(|h| h.join(".sword"))
                .unwrap_or_default(),
            // macOS
            PathBuf::from("/Applications/SWORD"),
            // Current directory
            PathBuf::from("./modules"),
        ];

        for path_chirho in paths_chirho {
            if path_chirho.exists() {
                self.mod_paths_chirho.push(path_chirho);
            }
        }
    }

    /// Add a module path.
    pub fn add_path_chirho<P: AsRef<Path>>(&mut self, path_chirho: P) {
        self.mod_paths_chirho.push(path_chirho.as_ref().to_path_buf());
    }

    /// Load all modules from configured paths.
    pub fn load_modules_chirho(&mut self) -> ResultChirho<()> {
        self.modules_chirho.clear();

        for path_chirho in &self.mod_paths_chirho.clone() {
            self.load_modules_from_path_chirho(path_chirho)?;
        }

        Ok(())
    }

    /// Load modules from a specific path.
    fn load_modules_from_path_chirho(&mut self, path_chirho: &Path) -> ResultChirho<()> {
        let mods_d_chirho = path_chirho.join("mods.d");

        if mods_d_chirho.exists() {
            // Load from mods.d directory
            for entry_result_chirho in WalkDir::new(&mods_d_chirho)
                .max_depth(1)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let conf_path_chirho = entry_result_chirho.path();
                if conf_path_chirho.extension().map(|e| e == "conf").unwrap_or(false) {
                    if let Ok(config_chirho) = ModuleConfigChirho::from_file_chirho(conf_path_chirho) {
                        self.modules_chirho.insert(config_chirho.name_chirho.clone(), config_chirho);
                    }
                }
            }
        }

        Ok(())
    }

    /// Get a module by name.
    pub fn get_module_chirho(&self, name_chirho: &str) -> Option<&ModuleConfigChirho> {
        self.modules_chirho.get(name_chirho)
    }

    /// Get all module names.
    pub fn get_module_names_chirho(&self) -> Vec<&str> {
        self.modules_chirho.keys().map(|s| s.as_str()).collect()
    }

    /// Get all modules.
    pub fn get_modules_chirho(&self) -> &HashMap<String, ModuleConfigChirho> {
        &self.modules_chirho
    }

    /// Get modules by type.
    pub fn get_modules_by_type_chirho(&self, mod_type_chirho: &str) -> Vec<&ModuleConfigChirho> {
        self.modules_chirho
            .values()
            .filter(|m| {
                m.get_chirho("ModDrv")
                    .map(|d| d.contains(mod_type_chirho))
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Get Bible modules.
    pub fn get_bibles_chirho(&self) -> Vec<&ModuleConfigChirho> {
        self.modules_chirho
            .values()
            .filter(|m| m.is_bible_chirho())
            .collect()
    }

    /// Get commentary modules.
    pub fn get_commentaries_chirho(&self) -> Vec<&ModuleConfigChirho> {
        self.modules_chirho
            .values()
            .filter(|m| m.is_commentary_chirho())
            .collect()
    }

    /// Get lexicon modules.
    pub fn get_lexicons_chirho(&self) -> Vec<&ModuleConfigChirho> {
        self.modules_chirho
            .values()
            .filter(|m| m.is_lexicon_chirho())
            .collect()
    }

    /// Get general book modules.
    pub fn get_genbooks_chirho(&self) -> Vec<&ModuleConfigChirho> {
        self.modules_chirho
            .values()
            .filter(|m| m.is_genbook_chirho())
            .collect()
    }

    /// Get module count.
    pub fn module_count_chirho(&self) -> usize {
        self.modules_chirho.len()
    }

    /// Check if a module exists.
    pub fn has_module_chirho(&self, name_chirho: &str) -> bool {
        self.modules_chirho.contains_key(name_chirho)
    }

    /// Get the data path for a module.
    pub fn get_module_data_path_chirho(&self, name_chirho: &str) -> Option<PathBuf> {
        let config_chirho = self.modules_chirho.get(name_chirho)?;
        let data_path_chirho = config_chirho.data_path_chirho()?;

        // Find which base path contains this module
        for base_path_chirho in &self.mod_paths_chirho {
            let full_path_chirho = base_path_chirho.join(data_path_chirho);
            if full_path_chirho.exists() {
                return Some(full_path_chirho);
            }
        }

        None
    }
}

impl Default for SwMgrChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

// Stub for dirs crate functionality
mod dirs {
    use std::path::PathBuf;

    pub fn home_dir() -> Option<PathBuf> {
        std::env::var_os("HOME").map(PathBuf::from)
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_new_manager_chirho() {
        let mgr_chirho = SwMgrChirho::new_chirho();
        assert_eq!(mgr_chirho.module_count_chirho(), 0);
    }

    #[test]
    fn test_add_path_chirho() {
        let mut mgr_chirho = SwMgrChirho::new_chirho();
        mgr_chirho.add_path_chirho("/some/path");
        assert!(!mgr_chirho.mod_paths_chirho.is_empty());
    }
}
