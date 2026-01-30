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
//! let mut install_mgr_chirho = InstallMgrChirho::new_chirho();
//!
//! // Initialize and sync with remote sources
//! install_mgr_chirho.init_chirho().unwrap();
//! install_mgr_chirho.sync_config_chirho().unwrap();
//!
//! // List available modules from a source
//! let modules_chirho = install_mgr_chirho.get_remote_modules_chirho("CrossWire").unwrap();
//! for module_chirho in modules_chirho {
//!     println!("{}: {}", module_chirho.name_chirho, module_chirho.description_chirho);
//! }
//!
//! // Install a module
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
pub mod install_mgr_chirho;
pub mod module_factory_chirho;

pub use sw_mgr_chirho::SwMgrChirho;
pub use install_mgr_chirho::{
    InstallMgrChirho, ModuleInfoChirho, ModuleUpdateChirho, SearchResultChirho,
};
pub use module_factory_chirho::{
    LoadedModuleChirho, ModuleDriverTypeChirho, OutputFormatChirho,
    CreateModuleOptionsChirho,
    load_module_chirho, create_module_chirho,
};
