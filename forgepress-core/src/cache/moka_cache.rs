// /forgepress-core/src/cache/moka_cache.rs
use moka::future::Cache;
use std::time::Duration;

/// High-performance thread-safe in-memory cache wrapper.
#[derive(Clone)]
pub struct PageCache {
    inner: Cache<String, String>,
}

impl PageCache {
    /// Constructs a new Cache with a set maximum capacity and an automatic 24-hour expiration.
    pub fn new(max_capacity: u64) -> Self {
        let inner = Cache::builder()
            .max_capacity(max_capacity)
            .time_to_idle(Duration::from_secs(86400)) // Expire entries untouched for 24 hours
            .build();

        Self { inner }
    }

    /// Retrieves an entry from the cache.
    pub async fn get(&self, key: &str) -> Option<String> {
        self.inner.get(key).await
    }

    /// Saves or updates an entry inside the cache.
    pub async fn insert(&self, key: String, value: String).await {
        self.inner.insert(key, value).await;
    }

    /// Invalidates a specific cached key.
    pub async fn invalidate(&self, key: &str).await {
        self.inner.invalidate(key).await;
    }

    /// Completely flushes the cache memory.
    pub async fn invalidate_all(&self).await {
        self.inner.invalidate_all().await;
    }
}