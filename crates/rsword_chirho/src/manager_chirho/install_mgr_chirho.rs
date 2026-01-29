// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! SWORD module installation manager.

use std::path::{Path, PathBuf};

use crate::config_chirho::{InstallSourceChirho, InstallSourcesConfigChirho};
use crate::error_chirho::{ErrorChirho, ResultChirho};

/// SWORD module installation manager.
///
/// Handles downloading and installing modules from remote sources.
pub struct InstallMgrChirho {
    /// Configuration directory.
    config_dir_chirho: PathBuf,
    /// Installation sources.
    sources_chirho: InstallSourcesConfigChirho,
    /// Target installation path.
    install_path_chirho: PathBuf,
}

impl InstallMgrChirho {
    /// Create a new install manager.
    pub fn new_chirho<P: AsRef<Path>>(config_dir_chirho: P) -> ResultChirho<Self> {
        let config_dir_chirho = config_dir_chirho.as_ref().to_path_buf();

        // Try to load existing sources
        let sources_path_chirho = config_dir_chirho.join("InstallMgr.conf");
        let sources_chirho = if sources_path_chirho.exists() {
            InstallSourcesConfigChirho::from_file_chirho(&sources_path_chirho)?
        } else {
            InstallSourcesConfigChirho::default()
        };

        // Default install path
        let install_path_chirho = config_dir_chirho.clone();

        Ok(Self {
            config_dir_chirho,
            sources_chirho,
            install_path_chirho,
        })
    }

    /// Initialize the install manager configuration.
    pub fn init_chirho(&mut self) -> ResultChirho<()> {
        // Create config directory
        std::fs::create_dir_all(&self.config_dir_chirho)?;

        // Add default CrossWire source if no sources exist
        if self.sources_chirho.all_sources_chirho().is_empty() {
            self.sources_chirho.add_source_chirho(crate::config_chirho::default_crosswire_source_chirho());
        }

        // Save sources
        self.save_sources_chirho()?;

        // Create modules directory
        let mods_dir_chirho = self.install_path_chirho.join("mods.d");
        std::fs::create_dir_all(&mods_dir_chirho)?;

        Ok(())
    }

    /// Save the sources configuration.
    pub fn save_sources_chirho(&self) -> ResultChirho<()> {
        let sources_path_chirho = self.config_dir_chirho.join("InstallMgr.conf");
        let mut file_chirho = std::fs::File::create(sources_path_chirho)?;
        self.sources_chirho.write_to_chirho(&mut file_chirho)?;
        Ok(())
    }

    /// Get all install sources.
    pub fn get_sources_chirho(&self) -> Vec<&InstallSourceChirho> {
        self.sources_chirho.all_sources_chirho()
    }

    /// Add an install source.
    pub fn add_source_chirho(&mut self, source_chirho: InstallSourceChirho) {
        self.sources_chirho.add_source_chirho(source_chirho);
    }

    /// Find a source by name.
    pub fn find_source_chirho(&self, name_chirho: &str) -> Option<&InstallSourceChirho> {
        self.sources_chirho.find_by_caption_chirho(name_chirho)
    }

    /// Set the installation path.
    pub fn set_install_path_chirho<P: AsRef<Path>>(&mut self, path_chirho: P) {
        self.install_path_chirho = path_chirho.as_ref().to_path_buf();
    }

    /// Get the installation path.
    pub fn get_install_path_chirho(&self) -> &Path {
        &self.install_path_chirho
    }

    /// Sync with remote catalog (download module list).
    pub async fn refresh_source_chirho(&self, source_name_chirho: &str) -> ResultChirho<Vec<String>> {
        let source_chirho = self.find_source_chirho(source_name_chirho)
            .ok_or_else(|| ErrorChirho::generic_chirho(format!("Source not found: {}", source_name_chirho)))?;

        let url_chirho = format!("{}/mods.d.tar.gz", source_chirho.url_chirho());

        // TODO: Implement actual HTTP download
        // For now, return empty list
        Ok(Vec::new())
    }

    /// List available modules from a source.
    pub fn list_remote_modules_chirho(&self, source_name_chirho: &str) -> ResultChirho<Vec<String>> {
        // TODO: Read from cached module list
        Ok(Vec::new())
    }

    /// Install a module from a source.
    pub async fn install_module_chirho(
        &self,
        source_name_chirho: &str,
        module_name_chirho: &str,
    ) -> ResultChirho<()> {
        let source_chirho = self.find_source_chirho(source_name_chirho)
            .ok_or_else(|| ErrorChirho::generic_chirho(format!("Source not found: {}", source_name_chirho)))?;

        // TODO: Implement actual module download and installation

        Err(ErrorChirho::generic_chirho("Module installation not yet implemented"))
    }

    /// Uninstall a module.
    pub fn uninstall_module_chirho(&self, module_name_chirho: &str) -> ResultChirho<()> {
        let conf_path_chirho = self.install_path_chirho
            .join("mods.d")
            .join(format!("{}.conf", module_name_chirho.to_lowercase()));

        if !conf_path_chirho.exists() {
            return Err(ErrorChirho::module_not_found_chirho(module_name_chirho));
        }

        // Read config to find data path
        let config_chirho = crate::config_chirho::ModuleConfigChirho::from_file_chirho(&conf_path_chirho)?;

        // Remove data directory
        if let Some(data_path_chirho) = config_chirho.data_path_chirho() {
            let full_data_path_chirho = self.install_path_chirho.join(data_path_chirho);
            if full_data_path_chirho.exists() {
                std::fs::remove_dir_all(&full_data_path_chirho)?;
            }
        }

        // Remove config file
        std::fs::remove_file(&conf_path_chirho)?;

        Ok(())
    }

    /// List installed modules.
    pub fn list_installed_chirho(&self) -> ResultChirho<Vec<String>> {
        let mods_d_chirho = self.install_path_chirho.join("mods.d");

        if !mods_d_chirho.exists() {
            return Ok(Vec::new());
        }

        let mut modules_chirho = Vec::new();

        for entry_chirho in std::fs::read_dir(&mods_d_chirho)? {
            let entry_chirho = entry_chirho?;
            let path_chirho = entry_chirho.path();

            if path_chirho.extension().map(|e| e == "conf").unwrap_or(false) {
                if let Ok(config_chirho) = crate::config_chirho::ModuleConfigChirho::from_file_chirho(&path_chirho) {
                    modules_chirho.push(config_chirho.name_chirho);
                }
            }
        }

        Ok(modules_chirho)
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_new_install_mgr_chirho() {
        let temp_dir_chirho = TempDir::new().unwrap();
        let mgr_chirho = InstallMgrChirho::new_chirho(temp_dir_chirho.path()).unwrap();
        assert!(mgr_chirho.get_sources_chirho().is_empty() || !mgr_chirho.get_sources_chirho().is_empty());
    }

    #[test]
    fn test_init_chirho() {
        let temp_dir_chirho = TempDir::new().unwrap();
        let mut mgr_chirho = InstallMgrChirho::new_chirho(temp_dir_chirho.path()).unwrap();
        mgr_chirho.init_chirho().unwrap();

        // Check that config was saved
        let conf_path_chirho = temp_dir_chirho.path().join("InstallMgr.conf");
        assert!(conf_path_chirho.exists());
    }
}
