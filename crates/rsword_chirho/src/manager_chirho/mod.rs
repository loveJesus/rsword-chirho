// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Module management functionality.
//!
//! This module provides facilities for discovering, loading, and installing SWORD modules.
//!
//! ## Module Manager
//!
//! [`SwMgrChirho`] discovers and provides access to installed modules:
//!
//! ```rust,ignore
//! use rsword_chirho::SwMgrChirho;
//!
//! // Load from system paths
//! let mgr_chirho = SwMgrChirho::with_system_paths_chirho().unwrap();
//!
//! // List all modules
//! for name_chirho in mgr_chirho.get_module_names_chirho() {
//!     println!("Found: {}", name_chirho);
//! }
//!
//! // Get modules by type
//! let bibles_chirho = mgr_chirho.get_bibles_chirho();
//! let commentaries_chirho = mgr_chirho.get_commentaries_chirho();
//! let lexicons_chirho = mgr_chirho.get_lexicons_chirho();
//! ```
//!
//! ## Install Manager
//!
//! [`InstallMgrChirho`] handles downloading and installing modules from remote sources:
//!
//! ```rust,ignore
//! use rsword_chirho::manager_chirho::InstallMgrChirho;
//!
//! // new_chirho takes the config/install directory (where modules are stored).
//! let mut install_mgr_chirho = InstallMgrChirho::new_chirho("/path/to/.sword").unwrap();
//!
//! // Initialize: creates dirs and registers the default CrossWire source.
//! install_mgr_chirho.init_chirho().unwrap();
//!
//! // Download + cache the remote module list. This is the call that actually fetches
//! // the catalog (init_chirho only registers the source). Returns `Vec<String>`.
//! let modules_chirho = install_mgr_chirho.refresh_source_chirho("CrossWire").unwrap();
//! for name_chirho in &modules_chirho {
//!     println!("{name_chirho}");
//! }
//!
//! // Subsequent reads can use the cache without re-downloading:
//! // let modules_chirho = install_mgr_chirho.list_remote_modules_chirho("CrossWire").unwrap();
//!
//! // Install a module (KJV is freely available from CrossWire).
//! install_mgr_chirho.install_module_chirho("CrossWire", "KJV").unwrap();
//! ```
//!
//! ## Module Factory
//!
//! [`load_module_chirho`] creates the appropriate driver for a module configuration:
//!
//! ```rust,ignore
//! use rsword_chirho::manager_chirho::{load_module_chirho, OutputFormatChirho};
//! use rsword_chirho::config_chirho::ModuleConfigChirho;
//! use std::path::Path;
//!
//! // Load config file
//! let config_chirho = ModuleConfigChirho::from_file_chirho(Path::new("kjv.conf")).unwrap();
//!
//! // Load the module with HTML output
//! let module_chirho = load_module_chirho(
//!     &config_chirho,
//!     Path::new("/usr/share/sword"),
//!     OutputFormatChirho::HtmlChirho,
//! ).unwrap();
//! ```

pub mod sw_mgr_chirho;
pub mod module_factory_chirho;

// Install manager requires native platform (filesystem + network)
#[cfg(feature = "native")]
pub mod install_mgr_chirho;

pub use sw_mgr_chirho::SwMgrChirho;

#[cfg(feature = "native")]
pub use install_mgr_chirho::{
    InstallMgrChirho, ModuleInfoChirho, ModuleUpdateChirho, SearchResultChirho,
};

pub use module_factory_chirho::{
    LoadedModuleChirho, ModuleDriverTypeChirho, OutputFormatChirho,
    CreateModuleOptionsChirho, GenBookWrapperChirho,
    load_module_chirho, create_module_chirho,
};
