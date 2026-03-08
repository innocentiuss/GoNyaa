use std::collections::HashMap;
use std::time::{Duration, Instant};

#[derive(Clone)]
struct CacheEntry<V> {
    value: V,
    inserted_at: Instant,
}

pub struct TtlCache<V> {
    ttl: Duration,
    values: HashMap<String, CacheEntry<V>>,
}

impl<V: Clone> TtlCache<V> {
    pub fn new(ttl_secs: u64) -> Self {
        Self {
            ttl: Duration::from_secs(ttl_secs),
            values: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: &str) -> Option<V> {
        let entry = self.values.get(key)?.clone();
        if entry.inserted_at.elapsed() <= self.ttl {
            return Some(entry.value);
        }

        self.values.remove(key);
        None
    }

    pub fn set(&mut self, key: String, value: V) {
        self.values.insert(
            key,
            CacheEntry {
                value,
                inserted_at: Instant::now(),
            },
        );
    }

    pub fn clear(&mut self) {
        self.values.clear();
    }
}
