// /forgepress-core/src/cache/invalidator.rs
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, debug};
use crate::cache::moka_cache::PageCache;

/// Tracks relationships between data sources and rendered page caches.
#[derive(Clone)]
pub struct CacheInvalidator {
    /// Maps dependency keys (e.g., "post:15") to sets of dependent page slugs (e.g., ["index", "about-us"])
    dependencies: Arc<RwLock<HashMap<String, HashSet<String>>>>,
}

impl CacheInvalidator {
    pub fn new() -> Self {
        Self {
            dependencies: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Registers dependencies between a page slug and its dynamic data nodes.
    /// Call this inside the renderer whenever a page compiles successfully.
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

    /// Triggers selective cache invalidation based on an updated dependency key.
    /// Purges all dependent page HTMLs from RAM instantly.
    pub async fn invalidate(&self, cache: &PageCache, dep_key: &str) {
        let mut deps = self.dependencies.write().await;

        if let Some(dependent_slugs) = deps.remove(dep_key) {
            for slug in &dependent_slugs {
                cache.invalidate(slug).await;
                info!("Targeted Cache Purge: Invalidated '/{}' due to modification on '{}'", slug, dep_key);
            }
        }
    }

    /// Completely flushes all tracked relationship mappings.
    pub async fn clear_graph(&self).await {
        let mut deps = self.dependencies.write().await;
        deps.clear();
        info!("Cleared entire cache dependency graph mapping.");
    }
}