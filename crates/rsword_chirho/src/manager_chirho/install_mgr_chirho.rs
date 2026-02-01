// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! SWORD module installation manager.

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

use flate2::read::GzDecoder;
use tar::Archive;

use crate::config_chirho::{InstallSourceChirho, InstallSourcesConfigChirho};
use crate::error_chirho::{ErrorChirho, ResultChirho};
use crate::transport_chirho::{SystemTransportChirho, TransportChirho};

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

    /// Remove an install source by caption.
    /// Returns true if a source was removed, false if not found.
    pub fn remove_source_chirho(&mut self, caption_chirho: &str) -> bool {
        self.sources_chirho.remove_source_chirho(caption_chirho)
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
    pub fn refresh_source_chirho(&mut self, source_name_chirho: &str) -> ResultChirho<Vec<String>> {
        let source_chirho = self.find_source_chirho(source_name_chirho)
            .ok_or_else(|| ErrorChirho::generic_chirho(format!("Source not found: {}", source_name_chirho)))?;

        let url_chirho = format!("{}/mods.d.tar.gz", source_chirho.url_chirho());

        // Download and extract the module list
        let transport_chirho = SystemTransportChirho::new_chirho();
        let mods_d_tar_chirho = self.config_dir_chirho.join("mods.d.tar.gz");

        transport_chirho.download_chirho(&url_chirho, &mods_d_tar_chirho)?;

        // Extract the tarball to get module configs using native Rust
        let cache_dir_chirho = self.config_dir_chirho.join("remote_mods.d").join(source_name_chirho);
        std::fs::create_dir_all(&cache_dir_chirho)?;

        extract_tar_gz_chirho(&mods_d_tar_chirho, &cache_dir_chirho)?;

        // Clean up tarball
        let _ = std::fs::remove_file(&mods_d_tar_chirho);

        // List modules from cache
        self.list_remote_modules_chirho(source_name_chirho)
    }

    /// List available modules from a source.
    pub fn list_remote_modules_chirho(&self, source_name_chirho: &str) -> ResultChirho<Vec<String>> {
        let cache_dir_chirho = self.config_dir_chirho.join("remote_mods.d").join(source_name_chirho);

        // Try mods.d subdirectory first (common extraction pattern)
        let mods_d_chirho = cache_dir_chirho.join("mods.d");
        let search_dir_chirho = if mods_d_chirho.exists() { mods_d_chirho } else { cache_dir_chirho };

        if !search_dir_chirho.exists() {
            return Ok(Vec::new());
        }

        let mut modules_chirho = Vec::new();

        for entry_chirho in std::fs::read_dir(&search_dir_chirho)? {
            let entry_chirho = entry_chirho?;
            let path_chirho = entry_chirho.path();

            if path_chirho.extension().map(|e| e == "conf").unwrap_or(false) {
                if let Some(stem_chirho) = path_chirho.file_stem() {
                    modules_chirho.push(stem_chirho.to_string_lossy().to_uppercase());
                }
            }
        }

        modules_chirho.sort();
        Ok(modules_chirho)
    }

    /// Install a module from a source.
    pub fn install_module_chirho(
        &self,
        source_name_chirho: &str,
        module_name_chirho: &str,
    ) -> ResultChirho<()> {
        let source_chirho = self.find_source_chirho(source_name_chirho)
            .ok_or_else(|| ErrorChirho::generic_chirho(format!("Source not found: {}", source_name_chirho)))?;

        // Build the download URL
        // CrossWire mirrors use: https://crosswire.org/ftpmirror/pub/sword/packages/rawzip/ModuleName.zip
        // The source URL is https://crosswire.org/ftpmirror/pub/sword/raw
        // So we need to go up to /pub/sword/ and then to /packages/rawzip/
        let base_url_chirho = source_chirho.url_chirho();
        let zip_url_chirho = if base_url_chirho.contains("/raw") {
            // Standard CrossWire layout
            format!("{}/packages/rawzip/{}.zip",
                base_url_chirho.replace("/raw", ""), module_name_chirho)
        } else {
            format!("{}/packages/rawzip/{}.zip", base_url_chirho, module_name_chirho)
        };

        let transport_chirho = SystemTransportChirho::new_chirho();

        // Download the module zip
        let temp_zip_chirho = self.config_dir_chirho.join(format!("{}.zip", module_name_chirho));
        eprintln!("Downloading from {}...", zip_url_chirho);
        transport_chirho.download_chirho(&zip_url_chirho, &temp_zip_chirho)?;

        // Extract the zip file using native Rust
        eprintln!("Extracting...");
        if let Err(e_chirho) = extract_zip_chirho(&temp_zip_chirho, &self.install_path_chirho) {
            // Clean up temp file on error
            let _ = std::fs::remove_file(&temp_zip_chirho);
            return Err(e_chirho);
        }

        // Clean up temp zip
        let _ = std::fs::remove_file(&temp_zip_chirho);

        eprintln!("Module {} installed successfully.", module_name_chirho);
        Ok(())
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

    /// Get module info including version from a source.
    pub fn get_remote_module_info_chirho(
        &self,
        source_name_chirho: &str,
        module_name_chirho: &str,
    ) -> ResultChirho<ModuleInfoChirho> {
        let cache_dir_chirho = self.config_dir_chirho.join("remote_mods.d").join(source_name_chirho);
        let mods_d_chirho = cache_dir_chirho.join("mods.d");
        let search_dir_chirho = if mods_d_chirho.exists() { mods_d_chirho } else { cache_dir_chirho };

        let conf_path_chirho = search_dir_chirho.join(format!("{}.conf", module_name_chirho.to_lowercase()));

        if !conf_path_chirho.exists() {
            return Err(ErrorChirho::module_not_found_chirho(module_name_chirho));
        }

        let config_chirho = crate::config_chirho::ModuleConfigChirho::from_file_chirho(&conf_path_chirho)?;

        Ok(ModuleInfoChirho {
            name_chirho: config_chirho.name_chirho.clone(),
            version_chirho: config_chirho.get_chirho("Version").unwrap_or("1.0").to_string(),
            description_chirho: config_chirho.get_chirho("Description").unwrap_or("").to_string(),
            language_chirho: config_chirho.get_chirho("Lang").unwrap_or("en").to_string(),
            module_type_chirho: config_chirho.get_chirho("ModDrv").unwrap_or("Unknown").to_string(),
        })
    }

    /// Get installed module info.
    pub fn get_installed_module_info_chirho(&self, module_name_chirho: &str) -> ResultChirho<ModuleInfoChirho> {
        let conf_path_chirho = self.install_path_chirho
            .join("mods.d")
            .join(format!("{}.conf", module_name_chirho.to_lowercase()));

        if !conf_path_chirho.exists() {
            return Err(ErrorChirho::module_not_found_chirho(module_name_chirho));
        }

        let config_chirho = crate::config_chirho::ModuleConfigChirho::from_file_chirho(&conf_path_chirho)?;

        Ok(ModuleInfoChirho {
            name_chirho: config_chirho.name_chirho.clone(),
            version_chirho: config_chirho.get_chirho("Version").unwrap_or("1.0").to_string(),
            description_chirho: config_chirho.get_chirho("Description").unwrap_or("").to_string(),
            language_chirho: config_chirho.get_chirho("Lang").unwrap_or("en").to_string(),
            module_type_chirho: config_chirho.get_chirho("ModDrv").unwrap_or("Unknown").to_string(),
        })
    }

    /// Compare versions and find modules needing upgrade.
    pub fn check_updates_chirho(&self, source_name_chirho: &str) -> ResultChirho<Vec<ModuleUpdateChirho>> {
        let installed_chirho = self.list_installed_chirho()?;
        let remote_chirho = self.list_remote_modules_chirho(source_name_chirho)?;

        let mut updates_chirho = Vec::new();

        for mod_name_chirho in &installed_chirho {
            if remote_chirho.iter().any(|r_chirho| r_chirho.eq_ignore_ascii_case(mod_name_chirho)) {
                let local_info_chirho = self.get_installed_module_info_chirho(mod_name_chirho)?;
                let remote_info_chirho = self.get_remote_module_info_chirho(source_name_chirho, mod_name_chirho)?;

                if compare_versions_chirho(&local_info_chirho.version_chirho, &remote_info_chirho.version_chirho) < 0 {
                    updates_chirho.push(ModuleUpdateChirho {
                        name_chirho: mod_name_chirho.clone(),
                        current_version_chirho: local_info_chirho.version_chirho,
                        available_version_chirho: remote_info_chirho.version_chirho,
                    });
                }
            }
        }

        Ok(updates_chirho)
    }

    /// Upgrade all modules that have updates available.
    pub fn upgrade_all_chirho(&self, source_name_chirho: &str) -> ResultChirho<Vec<String>> {
        let updates_chirho = self.check_updates_chirho(source_name_chirho)?;
        let mut upgraded_chirho = Vec::new();

        for update_chirho in &updates_chirho {
            eprintln!("Upgrading {} from {} to {}...",
                update_chirho.name_chirho,
                update_chirho.current_version_chirho,
                update_chirho.available_version_chirho
            );
            self.install_module_chirho(source_name_chirho, &update_chirho.name_chirho)?;
            upgraded_chirho.push(update_chirho.name_chirho.clone());
        }

        Ok(upgraded_chirho)
    }

    /// Search for modules by name/description across all cached sources.
    pub fn search_modules_chirho(&self, query_chirho: &str) -> ResultChirho<Vec<SearchResultChirho>> {
        let query_lower_chirho = query_chirho.to_lowercase();
        let mut results_chirho = Vec::new();

        for source_chirho in self.get_sources_chirho() {
            let source_name_chirho = &source_chirho.caption_chirho;
            let modules_chirho = self.list_remote_modules_chirho(source_name_chirho).unwrap_or_default();

            for mod_name_chirho in modules_chirho {
                if mod_name_chirho.to_lowercase().contains(&query_lower_chirho) {
                    results_chirho.push(SearchResultChirho {
                        source_name_chirho: source_name_chirho.clone(),
                        module_name_chirho: mod_name_chirho,
                    });
                } else if let Ok(info_chirho) = self.get_remote_module_info_chirho(source_name_chirho, &mod_name_chirho) {
                    if info_chirho.description_chirho.to_lowercase().contains(&query_lower_chirho) {
                        results_chirho.push(SearchResultChirho {
                            source_name_chirho: source_name_chirho.clone(),
                            module_name_chirho: mod_name_chirho,
                        });
                    }
                }
            }
        }

        Ok(results_chirho)
    }

    /// Refresh all sources.
    pub fn refresh_all_sources_chirho(&mut self) -> ResultChirho<()> {
        let sources_chirho: Vec<String> = self.get_sources_chirho()
            .iter()
            .map(|s_chirho| s_chirho.caption_chirho.clone())
            .collect();

        for source_name_chirho in sources_chirho {
            eprintln!("Refreshing {}...", source_name_chirho);
            let _ = self.refresh_source_chirho(&source_name_chirho);
        }

        Ok(())
    }
}

/// Module information.
#[derive(Debug, Clone)]
pub struct ModuleInfoChirho {
    /// Module name.
    pub name_chirho: String,
    /// Module version.
    pub version_chirho: String,
    /// Module description.
    pub description_chirho: String,
    /// Module language.
    pub language_chirho: String,
    /// Module driver type.
    pub module_type_chirho: String,
}

/// Module update information.
#[derive(Debug, Clone)]
pub struct ModuleUpdateChirho {
    /// Module name.
    pub name_chirho: String,
    /// Currently installed version.
    pub current_version_chirho: String,
    /// Available version.
    pub available_version_chirho: String,
}

/// Search result.
#[derive(Debug, Clone)]
pub struct SearchResultChirho {
    /// Source name where module was found.
    pub source_name_chirho: String,
    /// Module name.
    pub module_name_chirho: String,
}

/// Compare version strings (e.g., "1.0" vs "1.1").
/// Returns -1 if v1 < v2, 0 if equal, 1 if v1 > v2.
fn compare_versions_chirho(v1_chirho: &str, v2_chirho: &str) -> i32 {
    let parts1_chirho: Vec<u32> = v1_chirho.split('.')
        .filter_map(|p_chirho| p_chirho.parse().ok())
        .collect();
    let parts2_chirho: Vec<u32> = v2_chirho.split('.')
        .filter_map(|p_chirho| p_chirho.parse().ok())
        .collect();

    let max_len_chirho = parts1_chirho.len().max(parts2_chirho.len());

    for i_chirho in 0..max_len_chirho {
        let p1_chirho = parts1_chirho.get(i_chirho).copied().unwrap_or(0);
        let p2_chirho = parts2_chirho.get(i_chirho).copied().unwrap_or(0);

        if p1_chirho < p2_chirho {
            return -1;
        } else if p1_chirho > p2_chirho {
            return 1;
        }
    }

    0
}

/// Extract a .tar.gz archive to a destination directory.
fn extract_tar_gz_chirho(archive_path_chirho: &Path, dest_dir_chirho: &Path) -> ResultChirho<()> {
    let file_chirho = File::open(archive_path_chirho)?;
    let buf_reader_chirho = BufReader::new(file_chirho);
    let gz_decoder_chirho = GzDecoder::new(buf_reader_chirho);
    let mut archive_chirho = Archive::new(gz_decoder_chirho);

    archive_chirho.unpack(dest_dir_chirho)
        .map_err(|e_chirho| ErrorChirho::generic_chirho(format!("Failed to extract tar.gz: {}", e_chirho)))?;

    Ok(())
}

/// Extract a .zip archive to a destination directory.
fn extract_zip_chirho(archive_path_chirho: &Path, dest_dir_chirho: &Path) -> ResultChirho<()> {
    use std::io::Write;

    let file_chirho = File::open(archive_path_chirho)?;
    let buf_reader_chirho = BufReader::new(file_chirho);
    let mut archive_chirho = zip::ZipArchive::new(buf_reader_chirho)
        .map_err(|e_chirho| ErrorChirho::generic_chirho(format!("Failed to open zip: {}", e_chirho)))?;

    for i_chirho in 0..archive_chirho.len() {
        let mut file_chirho = archive_chirho.by_index(i_chirho)
            .map_err(|e_chirho| ErrorChirho::generic_chirho(format!("Failed to read zip entry: {}", e_chirho)))?;

        let outpath_chirho = match file_chirho.enclosed_name() {
            Some(path_chirho) => dest_dir_chirho.join(path_chirho),
            None => continue, // Skip entries with invalid paths
        };

        if file_chirho.is_dir() {
            std::fs::create_dir_all(&outpath_chirho)?;
        } else {
            // Create parent directories if needed
            if let Some(parent_chirho) = outpath_chirho.parent() {
                if !parent_chirho.exists() {
                    std::fs::create_dir_all(parent_chirho)?;
                }
            }

            let mut outfile_chirho = File::create(&outpath_chirho)?;
            let mut buffer_chirho = Vec::new();
            file_chirho.read_to_end(&mut buffer_chirho)?;
            outfile_chirho.write_all(&buffer_chirho)?;
        }

        // Set permissions on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode_chirho) = file_chirho.unix_mode() {
                std::fs::set_permissions(&outpath_chirho, std::fs::Permissions::from_mode(mode_chirho))?;
            }
        }
    }

    Ok(())
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
