// /forgepress-core/src/main.rs
use std::net::SocketAddr;
use axum::{routing::get, Router};
use sqlx::any::{install_default_drivers, AnyPoolOptions};
use tokio::net::TcpListener;
use tracing::{info, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app_state;
use app_state::{AppConfig, AppState};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // 1. Initialize structural tracing logs (RUST_LOG output configuration)
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "info,forgepress_core=debug,tower_http=debug".into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting ForgePress Core Engine...");

    // 2. Load configurations from system environment variables
    // Installs dotenv variables to local process if present
    let _ = dotenvy::dotenv();

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://content/forgepress.db".to_string());
    
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);

    let secret_key = std::env::var("SECRET_KEY")
        .unwrap_or_else(|_| "generate_a_random_64_character_string_for_production_use".to_string());

    let content_dir = std::env::var("CONTENT_DIR")
        .unwrap_or_else(|_| "./content".to_string());

    let upload_dir = std::env::var("UPLOAD_DIR")
        .unwrap_or_else(|_| "./content/uploads".to_string());

    let max_upload_size = std::env::var("MAX_UPLOAD_SIZE")
        .unwrap_or_else(|_| "10485760".to_string()) // 10MB
        .parse::<usize>()
        .unwrap_or(10_485_760);

    let config = AppConfig {
        port,
        database_url: database_url.clone(),
        secret_key,
        content_dir,
        upload_dir,
        max_upload_size,
    };

    // Ensure upload directories exist prior to accepting media posts
    if let Err(e) = std::fs::create_dir_all(&config.upload_dir) {
        warn!("Could not create upload directory path: {}. Ensure permissions are correct.", e);
    }

    // 3. Setup SQLx dynamic database drivers and pool connection
    install_default_drivers(); // Registers Postgres & SQLite drivers to SQLx at runtime
    
    info!("Connecting to database endpoint...");
    let db_pool = AnyPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await?;
    
    // Check if SQLite requires foreign key enforcement manually
    if database_url.starts_with("sqlite:") {
        sqlx::query("PRAGMA foreign_keys = ON;")
            .execute(&db_pool)
            .await?;
        info!("Enforced SQLite foreign key constraints.");
    }

    // Run active migrations automatically (Postgres & SQLite migrations)
    // SQLx automatically handles migration files based on the active driver
    info!("Executing schema migrations...");
    sqlx::migrate!("../migrations")
        .run(&db_pool)
        .await?;

    // 4. Initialize MiniJinja Template Environment
    let mut jinja_env = minijinja::Environment::new();
    jinja_env.set_loader(minijinja::path_loader("content/themes/default/templates"));
    
    // 5. Instantiate AppState
    let state = AppState::new(db_pool, config, jinja_env);

    // 6. Build Axum Routing Tree
    let app = Router::new()
        // Temporary test endpoint to verify system bootstrap
        .route("/health", get(|| async { "ForgePress Engine: Healthy" }))
        .with_state(state);

    // 7. Bind port and launch TCP server (Axum v0.7 syntax)
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = TcpListener::bind(addr).await?;
    info!("ForgePress Core listening securely on: http://{}", addr);
    
    axum::serve(listener, app).await?;

    Ok(())
}