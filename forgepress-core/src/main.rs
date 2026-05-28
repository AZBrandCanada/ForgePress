// /forgepress-core/src/main.rs
use std::net::SocketAddr;
use sqlx::any::{install_default_drivers, AnyPoolOptions};
use tokio::net::TcpListener;
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app_state;
mod config;
mod error;
mod auth;
mod database;
mod domain;
mod cache;
mod plugin_engine;
mod template_engine;
mod jobs; // Registered the background jobs module
mod routing;

use app_state::AppState;
use config::AppConfig;
use routing::app_router;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // 1. Initialize structural tracing logs (RUST_LOG configurations)
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "info,forgepress_core=debug,tower_http=debug".into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting ForgePress Core Engine...");

    // 2. Load environment configurations
    let config = AppConfig::load();

    // Ensure upload directories exist prior to accepting media posts
    if let Err(e) = std::fs::create_dir_all(&config.upload_dir) {
        warn!("Could not create upload directory path: {}. Ensure permissions are correct.", e);
    }

    // 3. Setup database connection pool dynamically (PostgreSQL or SQLite)
    install_default_drivers();
    
    info!("Connecting to database...");
    let db_pool = AnyPoolOptions::new()
        .max_connections(20)
        .connect(&config.database_url)
        .await?;
    
    if config.database_url.starts_with("sqlite:") {
        sqlx::query("PRAGMA foreign_keys = ON;")
            .execute(&db_pool)
            .await?;
        info!("Enforced SQLite foreign key constraints.");
    }

    // Run active schema migrations automatically
    info!("Executing schema migrations...");
    sqlx::migrate!("../migrations")
        .run(&db_pool)
        .await?;

    // 4. Initialize MiniJinja Template Environment
    let mut jinja_env = minijinja::Environment::new();
    jinja_env.set_loader(minijinja::path_loader("content/themes/default/templates"));
    
    // 5. Instantiate AppState
    let state = AppState::new(db_pool, config, jinja_env);

    // 6. Automatically scan, validate, and load dynamic dictionaries into RAM
    info!("Starting language dictionaries discovery...");
    state.i18n.discover_and_load("content/languages").await?; // <-- Added translation loader scan!

    // 7. Automatically scan, validate, and load plugins into memory
    info!("Starting plugin discovery...");
    state.plugins.discover_and_load("content/plugins").await?;

    // 8. Start the background task scheduler daemon
    jobs::scheduler::start_scheduler(state.clone());

    // 9. Build the dynamic global app router
    let app = app_router(state.clone()); 
    
    axum::serve(listener, app).await?;

    Ok(())
}