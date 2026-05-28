// /forgepress-core/src/config.rs
use std::env;
use tracing::warn;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub database_url: String,
    pub secret_key: String,
    pub content_dir: String,
    pub upload_dir: String,
    pub max_upload_size: usize,
}

impl AppConfig {
    /// Safely parses environment variables and configures startup settings.
    /// Emits warnings for missing configurations and falls back to safe defaults.
    pub fn load() -> Self {
        // Attempts to load .env file into the process if it exists
        let _ = dotenvy::dotenv();

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| {
                warn!("DATABASE_URL not found in environment. Falling back to default SQLite database.");
                "sqlite://content/forgepress.db".to_string()
            });

        let port = env::var("PORT")
            .ok()
            .and_then(|p| p.parse::<u16>().ok())
            .unwrap_or(8080);

        let secret_key = env::var("SECRET_KEY")
            .unwrap_or_else(|_| {
                warn!("SECURITY WARNING: SECRET_KEY is not defined. Using an insecure fallback secret. Please set a strong SECRET_KEY in your production environment.");
                "insecure_fallback_development_secret_key_change_me".to_string()
            });

        let content_dir = env::var("CONTENT_DIR")
            .unwrap_or_else(|_| "./content".to_string());

        let upload_dir = env::var("UPLOAD_DIR")
            .unwrap_or_else(|_| "./content/uploads".to_string());

        let max_upload_size = env::var("MAX_UPLOAD_SIZE")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(10_485_760); // 10MB default

        Self {
            port,
            database_url,
            secret_key,
            content_dir,
            upload_dir,
            max_upload_size,
        }
    }
}