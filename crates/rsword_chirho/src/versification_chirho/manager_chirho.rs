// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Versification manager for handling different Bible versification systems.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::error_chirho::{ErrorChirho, ResultChirho};
use crate::versification_chirho::{
    catholic_chirho, ethiopian_chirho, kjv_chirho, leningrad_chirho, luther_chirho, lxx_chirho,
    nrsv_chirho, synodal_chirho, vulgate_chirho, VersificationChirho,
};

/// Manager for versification systems.
///
/// This provides access to different versification systems and caches them
/// for efficient reuse.
pub struct VersificationManagerChirho {
    /// Cached versification systems.
    cache_chirho: RwLock<HashMap<String, Arc<VersificationChirho>>>,
}

impl VersificationManagerChirho {
    /// Create a new versification manager.
    pub fn new_chirho() -> Self {
        Self {
            cache_chirho: RwLock::new(HashMap::new()),
        }
    }

    /// Get a versification system by name.
    ///
    /// Returns an Arc to the versification for shared access.
    pub fn get_chirho(&self, name_chirho: &str) -> ResultChirho<Arc<VersificationChirho>> {
        // Check cache first
        {
            let cache_chirho = self.cache_chirho.read().unwrap();
            if let Some(v11n_chirho) = cache_chirho.get(name_chirho) {
                return Ok(Arc::clone(v11n_chirho));
            }
        }

        // Create and cache the versification
        let v11n_chirho = self.create_versification_chirho(name_chirho)?;
        let arc_chirho = Arc::new(v11n_chirho);

        {
            let mut cache_chirho = self.cache_chirho.write().unwrap();
            cache_chirho.insert(name_chirho.to_string(), Arc::clone(&arc_chirho));
        }

        Ok(arc_chirho)
    }

    /// Get the default (KJV) versification.
    pub fn get_default_chirho(&self) -> Arc<VersificationChirho> {
        self.get_chirho("KJV").expect("KJV versification should always exist")
    }

    /// Create a versification system by name.
    fn create_versification_chirho(&self, name_chirho: &str) -> ResultChirho<VersificationChirho> {
        match name_chirho.to_uppercase().as_str() {
            "KJV" => Ok(kjv_chirho().clone()),
            "CATHOLIC" => Ok(catholic_chirho().clone()),
            "LXX" | "SEPTUAGINT" => Ok(lxx_chirho().clone()),
            "SYNODAL" | "SYNODALPROT" => Ok(synodal_chirho().clone()),
            "LUTHER" | "GERMAN" => Ok(luther_chirho().clone()),
            "VULGATE" | "VULG" => Ok(vulgate_chirho().clone()),
            "NRSV" => Ok(nrsv_chirho().clone()),
            "LENINGRAD" | "MT" | "HEBREW" => Ok(leningrad_chirho().clone()),
            "ETHIOPIAN" | "ETHIOPIC" => Ok(ethiopian_chirho().clone()),
            _ => Err(ErrorChirho::InvalidVersificationChirho {
                v11n_chirho: name_chirho.to_string(),
            }),
        }
    }

    /// List available versification systems.
    pub fn list_available_chirho(&self) -> Vec<&'static str> {
        vec![
            "KJV",
            "Catholic",
            "LXX",
            "Synodal",
            "Luther",
            "Vulgate",
            "NRSV",
            "Leningrad",
            "Ethiopian",
        ]
    }

    /// Check if a versification system is available.
    pub fn is_available_chirho(&self, name_chirho: &str) -> bool {
        matches!(
            name_chirho.to_uppercase().as_str(),
            "KJV" | "CATHOLIC" | "LXX" | "SEPTUAGINT" | "SYNODAL" | "SYNODALPROT" |
            "LUTHER" | "GERMAN" | "VULGATE" | "VULG" | "NRSV" |
            "LENINGRAD" | "MT" | "HEBREW" | "ETHIOPIAN" | "ETHIOPIC"
        )
    }
}

impl Default for VersificationManagerChirho {
    fn default() -> Self {
        Self::new_chirho()
    }
}

/// Global versification manager instance.
static GLOBAL_V11N_MGR_CHIRHO: std::sync::LazyLock<VersificationManagerChirho> =
    std::sync::LazyLock::new(VersificationManagerChirho::new_chirho);

/// Get the global versification manager.
pub fn global_v11n_mgr_chirho() -> &'static VersificationManagerChirho {
    &GLOBAL_V11N_MGR_CHIRHO
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_get_kjv_chirho() {
        let mgr_chirho = VersificationManagerChirho::new_chirho();
        let kjv_chirho = mgr_chirho.get_chirho("KJV").unwrap();
        assert_eq!(kjv_chirho.name_chirho, "KJV");
    }

    #[test]
    fn test_get_default_chirho() {
        let mgr_chirho = VersificationManagerChirho::new_chirho();
        let default_chirho = mgr_chirho.get_default_chirho();
        assert_eq!(default_chirho.name_chirho, "KJV");
    }

    #[test]
    fn test_invalid_versification_chirho() {
        let mgr_chirho = VersificationManagerChirho::new_chirho();
        let result_chirho = mgr_chirho.get_chirho("NonExistent");
        assert!(result_chirho.is_err());
    }

    #[test]
    fn test_caching_chirho() {
        let mgr_chirho = VersificationManagerChirho::new_chirho();

        let kjv1_chirho = mgr_chirho.get_chirho("KJV").unwrap();
        let kjv2_chirho = mgr_chirho.get_chirho("KJV").unwrap();

        // Should be the same Arc instance
        assert!(Arc::ptr_eq(&kjv1_chirho, &kjv2_chirho));
    }

    #[test]
    fn test_global_manager_chirho() {
        let mgr_chirho = global_v11n_mgr_chirho();
        let kjv_chirho = mgr_chirho.get_chirho("KJV").unwrap();
        assert_eq!(kjv_chirho.name_chirho, "KJV");
    }
}
