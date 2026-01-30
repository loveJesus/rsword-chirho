// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! SWORD module implementations.
//!
//! This module provides drivers for reading and writing SWORD module formats.
//! Each driver implements the [`SwModuleChirho`] trait for uniform access.
//!
//! ## Module Types
//!
//! | Category | Uncompressed | Compressed |
//! |----------|--------------|------------|
//! | Bible texts | [`RawTextChirho`], [`RawText4Chirho`] | [`ZTextChirho`], [`ZText4Chirho`] |
//! | Commentaries | [`RawComChirho`], [`RawCom4Chirho`] | [`ZComChirho`], [`ZCom4Chirho`] |
//! | Lexicons | [`RawLdChirho`], [`RawLd4Chirho`] | [`ZLdChirho`], [`ZLd4Chirho`] |
//! | General books | [`RawGenBookChirho`] | [`ZGenBookChirho`] |
//! | Daily devotional | [`DailyDevotionalChirho`] | - |
//! | Images/Maps | [`ImageModuleChirho`] | - |
//!
//! ## Storage Formats
//!
//! - **Raw** modules use uncompressed storage with 6-byte or 8-byte index entries
//! - **Z** (compressed) modules use block compression (verse, chapter, or book blocks)
//! - **4** variants use 32-bit offsets (for modules > 64KB)
//!
//! ## Usage
//!
//! Modules are typically loaded via [`SwMgrChirho`](crate::manager_chirho::SwMgrChirho):
//!
//! ```rust,ignore
//! use rsword_chirho::{SwMgrChirho, VerseKeyChirho};
//!
//! let mgr_chirho = SwMgrChirho::with_system_paths_chirho().unwrap();
//!
//! // Get a module driver
//! if let Some(mut module_chirho) = mgr_chirho.get_module_driver_chirho("KJV").unwrap() {
//!     // Set position and read
//!     let key_chirho = VerseKeyChirho::from_str_chirho("Psalm 23:1").unwrap();
//!     module_chirho.set_key_chirho(&key_chirho).unwrap();
//!
//!     // Get raw text (with original markup)
//!     let raw_chirho = module_chirho.get_raw_entry_chirho().unwrap();
//!
//!     // Get rendered text (filters applied)
//!     let rendered_chirho = module_chirho.render_text_chirho().unwrap();
//! }
//! ```
//!
//! ## Encrypted Modules
//!
//! Some modules require a cipher key:
//!
//! ```rust,ignore
//! use rsword_chirho::{SwMgrChirho, SwModuleChirho};
//!
//! let mgr_chirho = SwMgrChirho::with_system_paths_chirho().unwrap();
//!
//! if let Some(mut module_chirho) = mgr_chirho.get_module_driver_chirho("EncryptedMod").unwrap() {
//!     if module_chirho.is_encrypted_chirho() {
//!         module_chirho.set_cipher_key_chirho("unlock-key").unwrap();
//!     }
//! }
//! ```

pub mod sw_module_chirho;
pub mod texts_chirho;
pub mod comments_chirho;
pub mod lexdict_chirho;
pub mod genbook_chirho;
pub mod devotional_chirho;
pub mod glossary_chirho;
pub mod images_chirho;

pub use sw_module_chirho::{ModuleBaseChirho, SwModuleChirho};
pub use texts_chirho::{
    RawTextChirho, RawTextIteratorChirho, RawText4Chirho,
    ZTextChirho, ZTextIteratorChirho, ZText4Chirho, ZText4IteratorChirho,
};
pub use comments_chirho::{
    RawComChirho, RawComIteratorChirho, RawCom4Chirho, RawCom4IteratorChirho,
    ZComChirho, ZComIteratorChirho, ZCom4Chirho, ZCom4IteratorChirho,
};
pub use lexdict_chirho::{
    RawLdChirho, RawLdIteratorChirho, RawLd4Chirho, RawLd4IteratorChirho,
    ZLdChirho, ZLdIteratorChirho, ZLd4Chirho, ZLd4IteratorChirho,
};
pub use genbook_chirho::{RawGenBookChirho, RawGenBookIteratorChirho, GenBookEntryChirho, ZGenBookChirho, ZGenBookIteratorChirho};
pub use devotional_chirho::DailyDevotionalChirho;
pub use comments_chirho::HrefComChirho;
pub use glossary_chirho::GlossaryChirho;
pub use images_chirho::{
    ImageModuleChirho, ImageEntryChirho, ImageFormatChirho,
    HotspotChirho, HotspotShapeChirho, MapOverlayChirho,
};
