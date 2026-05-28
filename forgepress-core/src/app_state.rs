// /forgepress-core/src/app_state.rs (Updated)
use std::sync::Arc;
use sqlx::AnyPool;
use minijinja::Environment;

use crate::config::AppConfig;
use crate::cache::{CacheInvalidator, PageCache}; // Updated imports

/// The global shared state of the ForgePress application.
#[derive(Clone)]
pub struct AppState {
    /// Dynamic SQLite or PostgreSQL database connection pool
    pub db: AnyPool,
    
    /// System configurations
    pub config: Arc<AppConfig>,
    
    /// Theme template parsing engine
    pub template_env: Arc<Environment<'static>>,
    
    /// Custom Moka-based in-memory concurrent cache
    pub cache: PageCache,
    
    /// Thread-safe dependency invalidator graph
    pub invalidator: CacheInvalidator,
}

impl AppState {
    pub fn new(
        db: AnyPool,
        config: AppConfig,
        template_env: Environment<'static>,
    ) -> Self {
        // Instantiate our new customized PageCache (capped at 10k items)
        let cache = PageCache::new(10_000);
        let invalidator = CacheInvalidator::new();

        Self {
            db,
            config: Arc::new(config),
            template_env: Arc::new(template_env),
            cache,
            invalidator,
        }
    }
}