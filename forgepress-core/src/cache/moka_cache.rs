// /forgepress-core/src/cache/moka_cache.rs
use moka::future::Cache;
use std::time::Duration;

#[derive(Clone)]
pub struct PageCache {
    inner: Cache<String, String>,
}

impl PageCache {
    pub fn new(max_capacity: u64) -> Self {
        let inner = Cache::builder()
            .max_capacity(max_capacity)
            .time_to_idle(Duration::from_secs(86400))
            .build();

        Self { inner }
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        self.inner.get(key).await
    }

    pub async fn insert(&self, key: String, value: String) {
        self.inner.insert(key, value).await;
    }

    pub async fn invalidate(&self, key: &str) {
        self.inner.invalidate(key).await;
    }

    // Fixed: Removed the syntax-error ".await" from this synchronous call
    pub async fn invalidate_all(&self) {
        self.inner.invalidate_all();
    }
}