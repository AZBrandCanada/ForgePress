// /forgepress-core/src/app_state.rs
use std::sync::Arc;
use sqlx::AnyPool;
use minijinja::Environment;
use moka::future::Cache;

/// Core application configuration loaded at boot time.
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub database_url: String,
    pub secret_key: String,
    pub content_dir: String,
    pub upload_dir: String,
    pub max_upload_size: usize,
}

/// The global shared state of the ForgePress application.
/// Cloned internally by Axum for every incoming request thread.
#[derive(Clone)]
pub struct AppState {
    /// SQLx AnyPool handles connection routing to either SQLite or PostgreSQL dynamically.
    pub db: AnyPool,
    
    /// Global application-wide configurations (Port, Secret keys, Directories).
    pub config: Arc<AppConfig>,
    
    /// MiniJinja template environment for on-the-fly rendering.
    pub template_env: Arc<Environment<'static>>,
    
    /// High-performance thread-safe in-memory cache for fragments and rendered pages.
    pub cache: Cache<String, String>,
}

impl AppState {
    pub fn new(
        db: AnyPool,
        config: AppConfig,
        template_env: Environment<'static>,
    ) -> Self {
        // Initialize an in-memory cache limited to 10,000 entries
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