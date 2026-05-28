// /forgepress-core/src/cache/invalidator.rs
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug};
use crate::cache::moka_cache::PageCache;

#[derive(Clone)]
pub struct CacheInvalidator {
    dependencies: Arc<RwLock<HashMap<String, HashSet<String>>>>,
}

impl CacheInvalidator {
    pub fn new() -> Self {
        Self {
            dependencies: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register(&self, slug: String, dep_keys: Vec<String>) {
        if dep_keys.is_empty() {
            return;
        }

        let mut deps = self.dependencies.write().await;
        for key in dep_keys {
            deps.entry(key)
                .or_insert_with(HashSet::new)
                .insert(slug.clone());
        }
        debug!("Registered cache dependencies for page slug: '/{}'", slug);
    }

    pub async fn invalidate(&self, cache: &PageCache, dep_key: &str) {
        let mut deps = self.dependencies.write().await;

        if let Some(dependent_slugs) = deps.remove(dep_key) {
            for slug in &dependent_slugs {
                cache.invalidate(slug).await;
                info!("Targeted Cache Purge: Invalidated '/{}' due to modification on '{}'", slug, dep_key);
            }
        }
    }

    // Fixed: Removed the syntax-error ".await" from this synchronous call
    pub async fn clear_graph(&self) {
        let mut deps = self.dependencies.write().await;
        deps.clear();
        info!("Cleared entire cache dependency graph mapping.");
    }
}