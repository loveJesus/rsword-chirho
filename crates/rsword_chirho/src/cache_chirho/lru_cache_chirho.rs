// For God so loved the world that he gave his only begotten Son,
// that whoever believes in him should not perish but have eternal life.
// John 3:16

//! LRU (Least Recently Used) cache implementation.
//!
//! A simple but efficient LRU cache using a HashMap for O(1) lookups
//! and a doubly-linked list for O(1) evictions.

use std::collections::HashMap;
use std::hash::Hash;

/// LRU cache node.
#[derive(Debug)]
struct NodeChirho<K, V> {
    key_chirho: K,
    value_chirho: V,
    prev_chirho: Option<usize>,
    next_chirho: Option<usize>,
}

/// LRU cache statistics.
#[derive(Debug, Clone, Default)]
pub struct CacheStatsChirho {
    /// Number of cache hits.
    pub hits_chirho: u64,
    /// Number of cache misses.
    pub misses_chirho: u64,
    /// Number of evictions.
    pub evictions_chirho: u64,
}

impl CacheStatsChirho {
    /// Get hit ratio (0.0 to 1.0).
    pub fn hit_ratio_chirho(&self) -> f64 {
        let total_chirho = self.hits_chirho + self.misses_chirho;
        if total_chirho == 0 {
            0.0
        } else {
            self.hits_chirho as f64 / total_chirho as f64
        }
    }
}

/// LRU (Least Recently Used) cache.
///
/// Uses a combination of a HashMap for O(1) key lookups and a
/// doubly-linked list for O(1) LRU ordering.
#[derive(Debug)]
pub struct LruCacheChirho<K, V> {
    /// Maximum capacity.
    capacity_chirho: usize,
    /// Key to node index mapping.
    map_chirho: HashMap<K, usize>,
    /// Node storage (acts as arena).
    nodes_chirho: Vec<Option<NodeChirho<K, V>>>,
    /// Head of LRU list (most recently used).
    head_chirho: Option<usize>,
    /// Tail of LRU list (least recently used).
    tail_chirho: Option<usize>,
    /// Free list for reusing slots.
    free_list_chirho: Vec<usize>,
}

impl<K: Hash + Eq + Clone, V> LruCacheChirho<K, V> {
    /// Create a new LRU cache with the given capacity.
    pub fn new_chirho(capacity_chirho: usize) -> Self {
        Self {
            capacity_chirho: capacity_chirho.max(1),
            map_chirho: HashMap::with_capacity(capacity_chirho),
            nodes_chirho: Vec::with_capacity(capacity_chirho),
            head_chirho: None,
            tail_chirho: None,
            free_list_chirho: Vec::new(),
        }
    }

    /// Get current size.
    pub fn len_chirho(&self) -> usize {
        self.map_chirho.len()
    }

    /// Check if empty.
    pub fn is_empty_chirho(&self) -> bool {
        self.map_chirho.is_empty()
    }

    /// Get maximum capacity.
    pub fn capacity_chirho(&self) -> usize {
        self.capacity_chirho
    }

    /// Get a value from the cache, moving it to most recently used.
    pub fn get_chirho(&mut self, key_chirho: &K) -> Option<&V> {
        if let Some(&idx_chirho) = self.map_chirho.get(key_chirho) {
            self.move_to_head_chirho(idx_chirho);
            self.nodes_chirho[idx_chirho].as_ref().map(|n| &n.value_chirho)
        } else {
            None
        }
    }

    /// Get a mutable value from the cache, moving it to most recently used.
    pub fn get_mut_chirho(&mut self, key_chirho: &K) -> Option<&mut V> {
        if let Some(&idx_chirho) = self.map_chirho.get(key_chirho) {
            self.move_to_head_chirho(idx_chirho);
            self.nodes_chirho[idx_chirho].as_mut().map(|n| &mut n.value_chirho)
        } else {
            None
        }
    }

    /// Put a value in the cache. Returns the evicted value if any.
    pub fn put_chirho(&mut self, key_chirho: K, value_chirho: V) -> Option<V> {
        // If key exists, update value and move to head
        if let Some(&idx_chirho) = self.map_chirho.get(&key_chirho) {
            let old_value_chirho = {
                let node_chirho = self.nodes_chirho[idx_chirho].as_mut().unwrap();
                std::mem::replace(&mut node_chirho.value_chirho, value_chirho)
            };
            self.move_to_head_chirho(idx_chirho);
            return Some(old_value_chirho);
        }

        // Evict if at capacity
        let evicted_chirho = if self.map_chirho.len() >= self.capacity_chirho {
            self.evict_lru_chirho()
        } else {
            None
        };

        // Add new node
        let new_idx_chirho = self.allocate_node_chirho(key_chirho.clone(), value_chirho);
        self.map_chirho.insert(key_chirho, new_idx_chirho);
        self.add_to_head_chirho(new_idx_chirho);

        evicted_chirho
    }

    /// Remove a key from the cache.
    pub fn remove_chirho(&mut self, key_chirho: &K) -> Option<V> {
        if let Some(idx_chirho) = self.map_chirho.remove(key_chirho) {
            let node_chirho = self.remove_node_chirho(idx_chirho);
            self.free_list_chirho.push(idx_chirho);
            node_chirho.map(|n| n.value_chirho)
        } else {
            None
        }
    }

    /// Clear all entries.
    pub fn clear_chirho(&mut self) {
        self.map_chirho.clear();
        self.nodes_chirho.clear();
        self.free_list_chirho.clear();
        self.head_chirho = None;
        self.tail_chirho = None;
    }

    /// Check if cache contains a key.
    pub fn contains_chirho(&self, key_chirho: &K) -> bool {
        self.map_chirho.contains_key(key_chirho)
    }

    /// Allocate a new node or reuse from free list.
    fn allocate_node_chirho(&mut self, key_chirho: K, value_chirho: V) -> usize {
        let node_chirho = NodeChirho {
            key_chirho,
            value_chirho,
            prev_chirho: None,
            next_chirho: None,
        };

        if let Some(idx_chirho) = self.free_list_chirho.pop() {
            self.nodes_chirho[idx_chirho] = Some(node_chirho);
            idx_chirho
        } else {
            let idx_chirho = self.nodes_chirho.len();
            self.nodes_chirho.push(Some(node_chirho));
            idx_chirho
        }
    }

    /// Add node to head of list.
    fn add_to_head_chirho(&mut self, idx_chirho: usize) {
        if let Some(node_chirho) = self.nodes_chirho[idx_chirho].as_mut() {
            node_chirho.prev_chirho = None;
            node_chirho.next_chirho = self.head_chirho;
        }

        if let Some(old_head_chirho) = self.head_chirho {
            if let Some(node_chirho) = self.nodes_chirho[old_head_chirho].as_mut() {
                node_chirho.prev_chirho = Some(idx_chirho);
            }
        }

        self.head_chirho = Some(idx_chirho);

        if self.tail_chirho.is_none() {
            self.tail_chirho = Some(idx_chirho);
        }
    }

    /// Remove node from its current position.
    fn remove_node_chirho(&mut self, idx_chirho: usize) -> Option<NodeChirho<K, V>> {
        let node_chirho = self.nodes_chirho[idx_chirho].take()?;

        // Update previous node's next pointer
        if let Some(prev_idx_chirho) = node_chirho.prev_chirho {
            if let Some(prev_node_chirho) = self.nodes_chirho[prev_idx_chirho].as_mut() {
                prev_node_chirho.next_chirho = node_chirho.next_chirho;
            }
        } else {
            // This was the head
            self.head_chirho = node_chirho.next_chirho;
        }

        // Update next node's prev pointer
        if let Some(next_idx_chirho) = node_chirho.next_chirho {
            if let Some(next_node_chirho) = self.nodes_chirho[next_idx_chirho].as_mut() {
                next_node_chirho.prev_chirho = node_chirho.prev_chirho;
            }
        } else {
            // This was the tail
            self.tail_chirho = node_chirho.prev_chirho;
        }

        Some(node_chirho)
    }

    /// Move existing node to head.
    fn move_to_head_chirho(&mut self, idx_chirho: usize) {
        if self.head_chirho == Some(idx_chirho) {
            return; // Already at head
        }

        // Get prev/next before removing
        let (prev_chirho, next_chirho) = {
            let node_chirho = self.nodes_chirho[idx_chirho].as_ref().unwrap();
            (node_chirho.prev_chirho, node_chirho.next_chirho)
        };

        // Update previous node
        if let Some(prev_idx_chirho) = prev_chirho {
            if let Some(prev_node_chirho) = self.nodes_chirho[prev_idx_chirho].as_mut() {
                prev_node_chirho.next_chirho = next_chirho;
            }
        }

        // Update next node
        if let Some(next_idx_chirho) = next_chirho {
            if let Some(next_node_chirho) = self.nodes_chirho[next_idx_chirho].as_mut() {
                next_node_chirho.prev_chirho = prev_chirho;
            }
        } else {
            // Was the tail
            self.tail_chirho = prev_chirho;
        }

        // Update current node
        if let Some(node_chirho) = self.nodes_chirho[idx_chirho].as_mut() {
            node_chirho.prev_chirho = None;
            node_chirho.next_chirho = self.head_chirho;
        }

        // Update old head
        if let Some(old_head_chirho) = self.head_chirho {
            if let Some(head_node_chirho) = self.nodes_chirho[old_head_chirho].as_mut() {
                head_node_chirho.prev_chirho = Some(idx_chirho);
            }
        }

        self.head_chirho = Some(idx_chirho);
    }

    /// Evict the least recently used entry.
    fn evict_lru_chirho(&mut self) -> Option<V> {
        let tail_idx_chirho = self.tail_chirho?;
        let key_chirho = self.nodes_chirho[tail_idx_chirho].as_ref()?.key_chirho.clone();
        self.remove_chirho(&key_chirho)
    }
}

#[cfg(test)]
mod tests_chirho {
    use super::*;

    #[test]
    fn test_basic_operations_chirho() {
        let mut cache_chirho = LruCacheChirho::new_chirho(3);

        cache_chirho.put_chirho("a", 1);
        cache_chirho.put_chirho("b", 2);
        cache_chirho.put_chirho("c", 3);

        assert_eq!(cache_chirho.get_chirho(&"a"), Some(&1));
        assert_eq!(cache_chirho.get_chirho(&"b"), Some(&2));
        assert_eq!(cache_chirho.get_chirho(&"c"), Some(&3));
    }

    #[test]
    fn test_eviction_chirho() {
        let mut cache_chirho = LruCacheChirho::new_chirho(2);

        cache_chirho.put_chirho("a", 1);
        cache_chirho.put_chirho("b", 2);
        cache_chirho.put_chirho("c", 3); // Should evict "a"

        assert_eq!(cache_chirho.get_chirho(&"a"), None);
        assert_eq!(cache_chirho.get_chirho(&"b"), Some(&2));
        assert_eq!(cache_chirho.get_chirho(&"c"), Some(&3));
    }

    #[test]
    fn test_lru_order_chirho() {
        let mut cache_chirho = LruCacheChirho::new_chirho(2);

        cache_chirho.put_chirho("a", 1);
        cache_chirho.put_chirho("b", 2);

        // Access "a" to make it more recently used
        cache_chirho.get_chirho(&"a");

        // "b" should now be LRU and get evicted
        cache_chirho.put_chirho("c", 3);

        assert_eq!(cache_chirho.get_chirho(&"a"), Some(&1));
        assert_eq!(cache_chirho.get_chirho(&"b"), None);
        assert_eq!(cache_chirho.get_chirho(&"c"), Some(&3));
    }

    #[test]
    fn test_update_existing_chirho() {
        let mut cache_chirho = LruCacheChirho::new_chirho(2);

        cache_chirho.put_chirho("a", 1);
        let old_chirho = cache_chirho.put_chirho("a", 10);

        assert_eq!(old_chirho, Some(1));
        assert_eq!(cache_chirho.get_chirho(&"a"), Some(&10));
    }

    #[test]
    fn test_remove_chirho() {
        let mut cache_chirho = LruCacheChirho::new_chirho(2);

        cache_chirho.put_chirho("a", 1);
        cache_chirho.put_chirho("b", 2);

        let removed_chirho = cache_chirho.remove_chirho(&"a");
        assert_eq!(removed_chirho, Some(1));
        assert_eq!(cache_chirho.get_chirho(&"a"), None);
        assert_eq!(cache_chirho.len_chirho(), 1);
    }

    #[test]
    fn test_clear_chirho() {
        let mut cache_chirho = LruCacheChirho::new_chirho(3);

        cache_chirho.put_chirho("a", 1);
        cache_chirho.put_chirho("b", 2);
        cache_chirho.clear_chirho();

        assert!(cache_chirho.is_empty_chirho());
        assert_eq!(cache_chirho.get_chirho(&"a"), None);
    }

    #[test]
    fn test_capacity_chirho() {
        let cache_chirho: LruCacheChirho<i32, i32> = LruCacheChirho::new_chirho(5);
        assert_eq!(cache_chirho.capacity_chirho(), 5);
    }

    #[test]
    fn test_stats_hit_ratio_chirho() {
        let stats_chirho = CacheStatsChirho {
            hits_chirho: 75,
            misses_chirho: 25,
            evictions_chirho: 10,
        };

        assert!((stats_chirho.hit_ratio_chirho() - 0.75).abs() < 0.001);
    }

    #[test]
    fn test_get_mut_chirho() {
        let mut cache_chirho = LruCacheChirho::new_chirho(2);
        cache_chirho.put_chirho("a", vec![1, 2, 3]);

        if let Some(v_chirho) = cache_chirho.get_mut_chirho(&"a") {
            v_chirho.push(4);
        }

        assert_eq!(cache_chirho.get_chirho(&"a"), Some(&vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_contains_chirho() {
        let mut cache_chirho = LruCacheChirho::new_chirho(2);
        cache_chirho.put_chirho("a", 1);

        assert!(cache_chirho.contains_chirho(&"a"));
        assert!(!cache_chirho.contains_chirho(&"b"));
    }
}
