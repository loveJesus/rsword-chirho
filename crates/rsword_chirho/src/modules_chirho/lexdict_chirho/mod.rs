// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Lexicon and dictionary module implementations.
//!
//! Provides RawLD, RawLD4, zLD, and zLD4 module drivers.

mod raw_ld_chirho;
mod raw_ld4_chirho;
mod z_ld_chirho;
mod z_ld4_chirho;

pub use raw_ld_chirho::{RawLdChirho, RawLdIteratorChirho};
pub use raw_ld4_chirho::{RawLd4Chirho, RawLd4IteratorChirho};
pub use z_ld_chirho::{ZLdChirho, ZLdIteratorChirho};
pub use z_ld4_chirho::{ZLd4Chirho, ZLd4IteratorChirho};

use crate::error_chirho::ResultChirho;

/// Zero-pad a Strong's-style key to the 5-digit form stored by Strong's lexicon
/// modules (`G25`, `g25`, `25`, `0025` → `00025`). Returns `None` for keys that
/// are not a bare, optionally `G`/`H`-prefixed number, so word-keyed dictionaries
/// (e.g. "Aaron") are left untouched.
pub(crate) fn strongs_padded_key_chirho(key_chirho: &str) -> Option<String> {
    let trimmed_chirho = key_chirho.trim();
    let digits_chirho = trimmed_chirho
        .strip_prefix(['G', 'H', 'g', 'h'])
        .unwrap_or(trimmed_chirho);
    if digits_chirho.is_empty() || !digits_chirho.bytes().all(|b_chirho| b_chirho.is_ascii_digit()) {
        return None;
    }
    let stripped_chirho = digits_chirho.trim_start_matches('0');
    let number_chirho = if stripped_chirho.is_empty() { "0" } else { stripped_chirho };
    Some(format!("{:0>5}", number_chirho))
}

/// Look up a lexicon key, falling back to the zero-padded Strong's form when the
/// exact key is absent. `lookup_chirho` performs the underlying storage lookup,
/// so this stays storage-agnostic (shared by the Raw and compressed drivers).
pub(crate) fn lookup_with_strongs_fallback_chirho<F>(
    key_chirho: &str,
    mut lookup_chirho: F,
) -> ResultChirho<Option<String>>
where
    F: FnMut(&str) -> ResultChirho<Option<String>>,
{
    if let Some(value_chirho) = lookup_chirho(key_chirho)? {
        return Ok(Some(value_chirho));
    }
    if let Some(padded_chirho) = strongs_padded_key_chirho(key_chirho) {
        if padded_chirho != key_chirho {
            return lookup_chirho(&padded_chirho);
        }
    }
    Ok(None)
}

#[cfg(test)]
mod key_tests_chirho {
    use super::strongs_padded_key_chirho;

    #[test]
    fn test_strongs_padded_key_chirho() {
        assert_eq!(strongs_padded_key_chirho("G25").as_deref(), Some("00025"));
        assert_eq!(strongs_padded_key_chirho("g25").as_deref(), Some("00025"));
        assert_eq!(strongs_padded_key_chirho("H430").as_deref(), Some("00430"));
        assert_eq!(strongs_padded_key_chirho("25").as_deref(), Some("00025"));
        assert_eq!(strongs_padded_key_chirho("00025").as_deref(), Some("00025"));
        assert_eq!(strongs_padded_key_chirho(" 25 ").as_deref(), Some("00025"));
        // Numbers longer than 5 digits are preserved, not truncated.
        assert_eq!(strongs_padded_key_chirho("123456").as_deref(), Some("123456"));
        // Word-keyed dictionary entries are left alone.
        assert_eq!(strongs_padded_key_chirho("Aaron"), None);
        assert_eq!(strongs_padded_key_chirho("G25a"), None);
        assert_eq!(strongs_padded_key_chirho(""), None);
    }
}
