// /forgepress-core/src/app_state.rs
use std::sync::Arc;
use sqlx::AnyPool;
use minijinja::Environment;
use moka::future::Cache;
use crate::config::AppConfig; // Updated import

/// The global shared state of the ForgePress application.
#[derive(Clone)]
pub struct AppState {
    pub db: AnyPool,
    pub config: Arc<AppConfig>,
    pub template_env: Arc<Environment<'static>>,
    pub cache: Cache<String, String>,
}

impl AppState {
    pub fn new(
        db: AnyPool,
        config: AppConfig,
        template_env: Environment<'static>,
    ) -> Self {
        let cache = Cache::builder()
            .max_capacity(10_000)
            .build();

        Self {
            db,
            config: Arc::new(config),
            template_env: Arc::new(template_env),
            cache,
        }
    }
}