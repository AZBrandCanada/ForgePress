// /forgepress-core/src/main.rs
use std::net::SocketAddr;
use sqlx::any::{install_default_drivers, AnyPoolOptions};
use tokio::net::TcpListener;
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Import CORS and HTTP types
use tower_http::cors::{Any, CorsLayer};
use axum::http::Method;

mod app_state;
mod config;
mod error;
mod auth;
mod database;
mod domain;
mod cache;
mod media;
mod plugin_engine;
mod template_engine;
mod jobs;
mod i18n;
mod routing;

use app_state::AppState;
use config::AppConfig;
use routing::app_router;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // 1. Initialize structural tracing logs
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

    // 3. Setup database connection pool dynamically
    install_default_drivers();
    
    info!("Connecting to database...");
    let db_pool = AnyPoolOptions::new()
        .max_connections(20)
        .connect(&config.database_url)
        .await?;
    
    let is_sqlite = config.database_url.starts_with("sqlite:") || config.database_url.starts_with("sqlite://");

    if is_sqlite {
        sqlx::query("PRAGMA foreign_keys = ON;")
            .execute(&db_pool)
            .await?;
        info!("Enforced SQLite foreign key constraints.");
    }

    // 4. Run database migrations
    info!("Executing schema migrations...");
    if is_sqlite {
        sqlx::migrate!("../migrations/sqlite")
            .run(&db_pool)
            .await?;
        info!("SQLite schema migrations executed successfully.");
    } else {
        sqlx::migrate!("../migrations/postgres")
            .run(&db_pool)
            .await?;
        info!("PostgreSQL schema migrations executed successfully.");
    }

    // 5. Initialize MiniJinja Template Environment
    info!("Initializing MiniJinja environment...");
    let jinja_env = template_engine::create_environment()?;
    
    // 6. Instantiate global AppState
    let state = AppState::new(db_pool, config, jinja_env);

    // 7. Load dynamic dictionaries into RAM
    info!("Starting language dictionaries discovery...");
    state.i18n.discover_and_load("content/languages").await?;

    // 8. Load plugins into memory
    info!("Starting plugin discovery...");
    state.plugins.discover_and_load("content/plugins").await?;

    // 9. Start background scheduler
    jobs::scheduler::start_scheduler(state.clone());

    // 10. Configure CORS Middleware for Local Development
    // Permits requests from the Vite frontend (port 5173) to cross over securely
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allows any origin, including localhost:5173
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([axum::http::header::CONTENT_TYPE, axum::http::header::AUTHORIZATION]);

    // 11. Build the router and apply the CORS layer
    let app = app_router(state.clone()).layer(cors); 

    // 12. Bind port and launch TCP server (Axum v0.7 syntax)
    let addr = SocketAddr::from(([0, 0, 0, 0], state.config.port));
    let listener = TcpListener::bind(addr).await?;
    info!("ForgePress Core listening securely on: http://{}", addr);
    
    axum::serve(listener, app).await?;

    Ok(())
}