// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! Caching layer for module content.
//!
//! Provides LRU caching to improve performance when accessing
//! frequently-read verses and entries.

mod lru_cache_chirho;

pub use lru_cache_chirho::{LruCacheChirho, CacheStatsChirho};

use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

/// Cache key for verse lookups.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct VerseCacheKeyChirho {
    /// Module name.
    pub module_chirho: String,
    /// Book number.
    pub book_chirho: u8,
    /// Chapter number.
    pub chapter_chirho: u16,
    /// Verse number.
    pub verse_chirho: u16,
    /// Filter options hash (for different renderings).
    pub options_hash_chirho: u64,
}

impl VerseCacheKeyChirho {
    /// Create a new verse cache key.
    pub fn new_chirho(
        module_chirho: &str,
        book_chirho: u8,
        chapter_chirho: u16,
        verse_chirho: u16,
        options_hash_chirho: u64,
    ) -> Self {
        Self {
            module_chirho: module_chirho.to_string(),
            book_chirho,
            chapter_chirho,
            verse_chirho,
            options_hash_chirho,
        }
    }
}

/// Thread-safe cache statistics.
#[derive(Debug, Default)]
pub struct AtomicCacheStatsChirho {
    /// Number of cache hits.
    hits_chirho: AtomicU64,
    /// Number of cache misses.
    misses_chirho: AtomicU64,
    /// Number of evictions.
    evictions_chirho: AtomicU64,
}

impl AtomicCacheStatsChirho {
    /// Create new stats.
    pub fn new_chirho() -> Self {
        Self::default()
    }

    /// Record a cache hit.
    pub fn record_hit_chirho(&self) {
        self.hits_chirho.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a cache miss.
    pub fn record_miss_chirho(&self) {
        self.misses_chirho.fetch_add(1, Ordering::Relaxed);
    }

    /// Record an eviction.
    pub fn record_eviction_chirho(&self) {
        self.evictions_chirho.fetch_add(1, Ordering::Relaxed);
    }

    /// Get current stats snapshot.
    pub fn snapshot_chirho(&self) -> CacheStatsChirho {
        CacheStatsChirho {
            hits_chirho: self.hits_chirho.load(Ordering::Relaxed),
            misses_chirho: self.misses_chirho.load(Ordering::Relaxed),
            evictions_chirho: self.evictions_chirho.load(Ordering::Relaxed),
        }
    }

    /// Reset all stats.
    pub fn reset_chirho(&self) {
        self.hits_chirho.store(0, Ordering::Relaxed);
        self.misses_chirho.store(0, Ordering::Relaxed);
        self.evictions_chirho.store(0, Ordering::Relaxed);
    }
}

/// Cached module wrapper that caches verse lookups.
pub struct CachedModuleChirho<M> {
    /// The underlying module.
    module_chirho: M,
    /// LRU cache for verse content.
    cache_chirho: Arc<std::sync::RwLock<LruCacheChirho<VerseCacheKeyChirho, String>>>,
    /// Cache statistics.
    stats_chirho: Arc<AtomicCacheStatsChirho>,
}

impl<M> CachedModuleChirho<M> {
    /// Create a new cached module wrapper.
    pub fn new_chirho(module_chirho: M, cache_size_chirho: usize) -> Self {
        Self {
            module_chirho,
            cache_chirho: Arc::new(std::sync::RwLock::new(
                LruCacheChirho::new_chirho(cache_size_chirho)
            )),
            stats_chirho: Arc::new(AtomicCacheStatsChirho::new_chirho()),
        }
    }

    /// Get the underlying module.
    pub fn module_chirho(&self) -> &M {
        &self.module_chirho
    }

    /// Get mutable reference to underlying module.
    pub fn module_mut_chirho(&mut self) -> &mut M {
        &mut self.module_chirho
    }

    /// Get cache statistics.
    pub fn stats_chirho(&self) -> CacheStatsChirho {
        self.stats_chirho.snapshot_chirho()
    }

    /// Reset cache statistics.
    pub fn reset_stats_chirho(&self) {
        self.stats_chirho.reset_chirho();
    }

    /// Clear the cache.
    pub fn clear_cache_chirho(&self) {
        let mut cache_chirho = self.cache_chirho.write().unwrap();
        cache_chirho.clear_chirho();
    }

    /// Get cached value or None.
    pub fn get_cached_chirho(&self, key_chirho: &VerseCacheKeyChirho) -> Option<String> {
        let mut cache_chirho = self.cache_chirho.write().unwrap();
        if let Some(value_chirho) = cache_chirho.get_chirho(key_chirho) {
            self.stats_chirho.record_hit_chirho();
            Some(value_chirho.clone())
        } else {
            self.stats_chirho.record_miss_chirho();
            None
        }
    }

    /// Put value in cache.
    pub fn put_cached_chirho(&self, key_chirho: VerseCacheKeyChirho, value_chirho: String) {
        let mut cache_chirho = self.cache_chirho.write().unwrap();
        if cache_chirho.put_chirho(key_chirho, value_chirho).is_some() {
            self.stats_chirho.record_eviction_chirho();
        }
    }
}

impl<M: Clone> Clone for CachedModuleChirho<M> {
    fn clone(&self) -> Self {
        Self {
            module_chirho: self.module_chirho.clone(),
            cache_chirho: self.cache_chirho.clone(),
            stats_chirho: self.stats_chirho.clone(),
        }
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_verse_cache_key_chirho() {
        let key1_chirho = VerseCacheKeyChirho::new_chirho("KJV", 1, 1, 1, 0);
        let key2_chirho = VerseCacheKeyChirho::new_chirho("KJV", 1, 1, 1, 0);
        let key3_chirho = VerseCacheKeyChirho::new_chirho("KJV", 1, 1, 2, 0);

        assert_eq!(key1_chirho, key2_chirho);
        assert_ne!(key1_chirho, key3_chirho);
    }

    #[test]
    fn test_atomic_stats_chirho() {
        let stats_chirho = AtomicCacheStatsChirho::new_chirho();

        stats_chirho.record_hit_chirho();
        stats_chirho.record_hit_chirho();
        stats_chirho.record_miss_chirho();

        let snapshot_chirho = stats_chirho.snapshot_chirho();
        assert_eq!(snapshot_chirho.hits_chirho, 2);
        assert_eq!(snapshot_chirho.misses_chirho, 1);
    }

    #[test]
    fn test_cached_module_basic_chirho() {
        let module_chirho = "test_module";
        let cached_chirho = CachedModuleChirho::new_chirho(module_chirho, 100);

        let key_chirho = VerseCacheKeyChirho::new_chirho("KJV", 1, 1, 1, 0);
        assert!(cached_chirho.get_cached_chirho(&key_chirho).is_none());

        cached_chirho.put_cached_chirho(key_chirho.clone(), "Genesis 1:1 text".to_string());

        let result_chirho = cached_chirho.get_cached_chirho(&key_chirho);
        assert_eq!(result_chirho, Some("Genesis 1:1 text".to_string()));
    }

    #[test]
    fn test_cached_module_stats_chirho() {
        let cached_chirho = CachedModuleChirho::new_chirho("module", 10);
        let key_chirho = VerseCacheKeyChirho::new_chirho("KJV", 1, 1, 1, 0);

        // Miss
        cached_chirho.get_cached_chirho(&key_chirho);
        let stats_chirho = cached_chirho.stats_chirho();
        assert_eq!(stats_chirho.misses_chirho, 1);
        assert_eq!(stats_chirho.hits_chirho, 0);

        // Put and hit
        cached_chirho.put_cached_chirho(key_chirho.clone(), "text".to_string());
        cached_chirho.get_cached_chirho(&key_chirho);

        let stats_chirho = cached_chirho.stats_chirho();
        assert_eq!(stats_chirho.hits_chirho, 1);
    }
}
