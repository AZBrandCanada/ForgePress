// /forgepress-core/src/main.rs (Updated bootstrap)
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
mod routing; // Added module import

use app_state::AppState;
use config::AppConfig;
use routing::app_router; // Added Router import

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // 1. Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "info,forgepress_core=debug,tower_http=debug".into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting ForgePress Core Engine...");

    // 2. Load configurations
    let config = AppConfig::load();

    // Ensure upload directories exist prior to accepting media posts
    if let Err(e) = std::fs::create_dir_all(&config.upload_dir) {
        warn!("Could not create upload directory path: {}. Ensure permissions are correct.", e);
    }

    // 3. Setup database connection pool
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

    // 6. Build the dynamic global app router
    let app = app_router(state.clone()); // Load standard nested paths

    // 7. Bind port and launch TCP server (Axum v0.7 syntax)
    let addr = SocketAddr::from(([0, 0, 0, 0], state.config.port));
    let listener = TcpListener::bind(addr).await?;
    info!("ForgePress Core listening securely on: http://{}", addr);
    
    axum::serve(listener, app).await?;

    Ok(())
}