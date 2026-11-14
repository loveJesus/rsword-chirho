// For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life. — John 3:16 (KJV)

//! Checks shared by the Bible writers (osis2mod, imp2vs and vpl2mod), included in each binary with `#[path]`.
//! A writer refuses what it cannot write truthfully instead of leaving a module that no reader opens correctly.

use std::path::Path;

/// These writers place verses by the KJV table, so another versification is refused before anything is written.
pub fn require_kjv_chirho(versification_chirho: &str) -> Result<(), String> {
    if versification_chirho.eq_ignore_ascii_case("KJV") {
        Ok(())
    } else {
        Err(format!("Versification {versification_chirho} is not available in this release: verses are placed by KJV. Nothing was written."))
    }
}

/// Importing no verses is a failure, reported before a conf would make the empty module visible.
pub fn require_imported_chirho(count_chirho: u32, source_chirho: &Path) -> Result<(), String> {
    if count_chirho > 0 {
        Ok(())
    } else {
        Err(format!("No verses imported from {}; no conf was written.", source_chirho.display()))
    }
}
