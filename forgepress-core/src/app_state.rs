// /forgepress-core/src/app_state.rs (Updated)
use std::sync::Arc;
use sqlx::AnyPool;
use minijinja::Environment;

use crate::config::AppConfig;
use crate::cache::{CacheInvalidator, PageCache};
use crate::plugin_engine::PluginManager;
use crate::i18n::I18nLoader; // Updated import

/// The global shared state of the ForgePress application.
#[derive(Clone)]
pub struct AppState {
    pub db: AnyPool,
    pub config: Arc<AppConfig>,
    pub template_env: Arc<Environment<'static>>,
    pub cache: PageCache,
    pub invalidator: CacheInvalidator,
    pub plugins: PluginManager,
    pub i18n: I18nLoader, // Added field
}

impl AppState {
    pub fn new(
        db: AnyPool,
        config: AppConfig,
        template_env: Environment<'static>,
    ) -> Self {
        let cache = PageCache::new(10_000);
        let invalidator = CacheInvalidator::new();
        let plugins = PluginManager::new();
        let i18n = I18nLoader::new("en"); // Instantiate translations (English as default fallback)

        Self {
            db,
            config: Arc::new(config),
            template_env: Arc::new(template_env),
            cache,
            invalidator,
            plugins,
            i18n,
        }
    }
}