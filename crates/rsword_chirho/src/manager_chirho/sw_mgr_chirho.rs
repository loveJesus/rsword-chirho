// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! SWORD module manager.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[cfg(feature = "parallel")]
use std::sync::{Arc, Mutex};

use walkdir::WalkDir;

use crate::config_chirho::ModuleConfigChirho;
use crate::error_chirho::{ErrorChirho, ResultChirho};
use super::module_factory_chirho::{LoadedModuleChirho, load_module_chirho};

/// Configuration for the module manager.
#[derive(Debug, Clone)]
pub struct SwMgrConfigChirho {
    /// Enable parallel loading of modules.
    pub parallel_chirho: bool,
    /// Number of threads to use for parallel loading (0 = auto).
    pub num_threads_chirho: usize,
}

impl Default for SwMgrConfigChirho {
    fn default() -> Self {
        Self {
            parallel_chirho: cfg!(feature = "parallel"),
            num_threads_chirho: 0, // Auto-detect
        }
    }
}

/// SWORD module manager.
///
/// Discovers and manages installed SWORD modules from various paths.
///
/// # Parallel Loading
///
/// When the `parallel` feature is enabled, module configuration files
/// can be loaded in parallel using rayon for improved performance on
/// multi-core systems.
///
/// ```rust,ignore
/// use rsword_chirho::manager_chirho::{SwMgrChirho, SwMgrConfigChirho};
///
/// // Create manager with parallel loading enabled
/// let config_chirho = SwMgrConfigChirho {
///     parallel_chirho: true,
///     num_threads_chirho: 4, // Use 4 threads
/// };
/// let mgr_chirho = SwMgrChirho::with_config_chirho(config_chirho);
/// ```
pub struct SwMgrChirho {
    /// List of module paths to search.
    mod_paths_chirho: Vec<PathBuf>,
    /// Discovered module configurations.
    modules_chirho: HashMap<String, ModuleConfigChirho>,
    /// Manager configuration.
    config_chirho: SwMgrConfigChirho,
}

impl SwMgrChirho {
    /// Create a new module manager.
    pub fn new_chirho() -> Self {
        Self {
            mod_paths_chirho: Vec::new(),
            modules_chirho: HashMap::new(),
            config_chirho: SwMgrConfigChirho::default(),
        }
    }

    /// Create a new module manager with specific configuration.
    pub fn with_config_chirho(config_chirho: SwMgrConfigChirho) -> Self {
        Self {
            mod_paths_chirho: Vec::new(),
            modules_chirho: HashMap::new(),
            config_chirho,
        }
    }

    /// Create a module manager with default system paths.
    pub fn with_system_paths_chirho() -> ResultChirho<Self> {
        let mut mgr_chirho = Self::new_chirho();
        mgr_chirho.add_system_paths_chirho();
        mgr_chirho.load_modules_chirho()?;
        Ok(mgr_chirho)
    }

    /// Create a module manager with system paths and specific configuration.
    pub fn with_system_paths_and_config_chirho(config_chirho: SwMgrConfigChirho) -> ResultChirho<Self> {
        let mut mgr_chirho = Self::with_config_chirho(config_chirho);
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
                .map(|h_chirho| h_chirho.join(".sword"))
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

    /// Get the current configuration.
    pub fn get_config_chirho(&self) -> &SwMgrConfigChirho {
        &self.config_chirho
    }

    /// Set whether to use parallel loading.
    pub fn set_parallel_chirho(&mut self, parallel_chirho: bool) {
        self.config_chirho.parallel_chirho = parallel_chirho;
    }

    /// Load all modules from configured paths.
    pub fn load_modules_chirho(&mut self) -> ResultChirho<()> {
        self.modules_chirho.clear();

        #[cfg(feature = "parallel")]
        if self.config_chirho.parallel_chirho {
            return self.load_modules_parallel_chirho();
        }

        // Sequential loading
        for path_chirho in &self.mod_paths_chirho.clone() {
            self.load_modules_from_path_chirho(path_chirho)?;
        }

        Ok(())
    }

    /// Load modules in parallel using rayon.
    #[cfg(feature = "parallel")]
    fn load_modules_parallel_chirho(&mut self) -> ResultChirho<()> {
        use rayon::prelude::*;

        // Configure thread pool if specified
        if self.config_chirho.num_threads_chirho > 0 {
            rayon::ThreadPoolBuilder::new()
                .num_threads(self.config_chirho.num_threads_chirho)
                .build_global()
                .ok(); // Ignore if already initialized
        }

        // Collect all config file paths first
        let mut conf_paths_chirho: Vec<PathBuf> = Vec::new();
        for path_chirho in &self.mod_paths_chirho {
            let mods_d_chirho = path_chirho.join("mods.d");
            if mods_d_chirho.exists() {
                for entry_result_chirho in WalkDir::new(&mods_d_chirho)
                    .max_depth(1)
                    .into_iter()
                    .filter_map(|e_chirho| e_chirho.ok())
                {
                    let conf_path_chirho = entry_result_chirho.path().to_path_buf();
                    if conf_path_chirho.extension().map(|e_chirho| e_chirho == "conf").unwrap_or(false) {
                        conf_paths_chirho.push(conf_path_chirho);
                    }
                }
            }
        }

        // Load configs in parallel
        let modules_chirho: Arc<Mutex<HashMap<String, ModuleConfigChirho>>> =
            Arc::new(Mutex::new(HashMap::new()));

        conf_paths_chirho.par_iter().for_each(|conf_path_chirho| {
            if let Ok(config_chirho) = ModuleConfigChirho::from_file_chirho(conf_path_chirho) {
                if let Ok(mut map_chirho) = modules_chirho.lock() {
                    map_chirho.insert(config_chirho.name_chirho.clone(), config_chirho);
                }
            }
        });

        // Move results into self
        if let Ok(map_chirho) = Arc::try_unwrap(modules_chirho) {
            self.modules_chirho = map_chirho.into_inner().unwrap_or_default();
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
                .filter_map(|e_chirho| e_chirho.ok())
            {
                let conf_path_chirho = entry_result_chirho.path();
                if conf_path_chirho.extension().map(|e_chirho| e_chirho == "conf").unwrap_or(false) {
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
        self.modules_chirho.keys().map(|s_chirho| s_chirho.as_str()).collect()
    }

    /// Get all modules.
    pub fn get_modules_chirho(&self) -> &HashMap<String, ModuleConfigChirho> {
        &self.modules_chirho
    }

    /// Get modules by type.
    pub fn get_modules_by_type_chirho(&self, mod_type_chirho: &str) -> Vec<&ModuleConfigChirho> {
        self.modules_chirho
            .values()
            .filter(|m_chirho| {
                m_chirho.get_chirho("ModDrv")
                    .map(|d_chirho| d_chirho.contains(mod_type_chirho))
                    .unwrap_or(false)
            })
            .collect()
    }

    /// Get Bible modules.
    pub fn get_bibles_chirho(&self) -> Vec<&ModuleConfigChirho> {
        self.modules_chirho
            .values()
            .filter(|m_chirho| m_chirho.is_bible_chirho())
            .collect()
    }

    /// Get commentary modules.
    pub fn get_commentaries_chirho(&self) -> Vec<&ModuleConfigChirho> {
        self.modules_chirho
            .values()
            .filter(|m_chirho| m_chirho.is_commentary_chirho())
            .collect()
    }

    /// Get lexicon modules.
    pub fn get_lexicons_chirho(&self) -> Vec<&ModuleConfigChirho> {
        self.modules_chirho
            .values()
            .filter(|m_chirho| m_chirho.is_lexicon_chirho())
            .collect()
    }

    /// Get general book modules.
    pub fn get_genbooks_chirho(&self) -> Vec<&ModuleConfigChirho> {
        self.modules_chirho
            .values()
            .filter(|m_chirho| m_chirho.is_genbook_chirho())
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

        // Strip leading "./" from data path if present
        let clean_data_path_chirho = data_path_chirho.strip_prefix("./").unwrap_or(data_path_chirho);

        // Find which base path contains this module
        for base_path_chirho in &self.mod_paths_chirho {
            let full_path_chirho = base_path_chirho.join(clean_data_path_chirho);
            if full_path_chirho.exists() {
                return Some(full_path_chirho);
            }
        }

        None
    }

    /// Get the base path for a module.
    pub fn get_module_base_path_chirho(&self, name_chirho: &str) -> Option<PathBuf> {
        let config_chirho = self.modules_chirho.get(name_chirho)?;
        let data_path_chirho = config_chirho.data_path_chirho()?;

        // Strip leading "./" from data path if present
        let clean_data_path_chirho = data_path_chirho.strip_prefix("./").unwrap_or(data_path_chirho);

        // Find which base path contains this module
        for base_path_chirho in &self.mod_paths_chirho {
            let full_path_chirho = base_path_chirho.join(clean_data_path_chirho);
            if full_path_chirho.exists() {
                return Some(base_path_chirho.clone());
            }
        }

        None
    }

    /// Load a module for reading.
    pub fn load_module_chirho(&self, name_chirho: &str) -> ResultChirho<LoadedModuleChirho> {
        let config_chirho = self.modules_chirho.get(name_chirho)
            .ok_or_else(|| ErrorChirho::ModuleNotFoundChirho {
                module_name_chirho: name_chirho.to_string(),
            })?;

        let base_path_chirho = self.get_module_base_path_chirho(name_chirho)
            .ok_or_else(|| ErrorChirho::ModuleNotFoundChirho {
                module_name_chirho: name_chirho.to_string(),
            })?;

        load_module_chirho(&base_path_chirho, config_chirho)
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

    #[test]
    fn test_config_default_chirho() {
        let config_chirho = SwMgrConfigChirho::default();
        assert_eq!(config_chirho.num_threads_chirho, 0);
    }

    #[test]
    fn test_with_config_chirho() {
        let config_chirho = SwMgrConfigChirho {
            parallel_chirho: true,
            num_threads_chirho: 4,
        };
        let mgr_chirho = SwMgrChirho::with_config_chirho(config_chirho);
        assert!(mgr_chirho.config_chirho.parallel_chirho);
        assert_eq!(mgr_chirho.config_chirho.num_threads_chirho, 4);
    }

    #[test]
    fn test_set_parallel_chirho() {
        let mut mgr_chirho = SwMgrChirho::new_chirho();
        mgr_chirho.set_parallel_chirho(true);
        assert!(mgr_chirho.config_chirho.parallel_chirho);
    }
}
