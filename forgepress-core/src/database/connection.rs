// /forgepress-core/src/database/connection.rs
use sqlx::any::{install_default_drivers, AnyPoolOptions};
use sqlx::AnyPool;
use tracing::info;
use crate::error::AppError;

/// Establishes a database connection pool dynamically based on the connection string prefix.
pub async fn establish_pool(database_url: &str) -> Result<AnyPool, AppError> {
    install_default_drivers(); // Registers dynamic drivers at runtime

    info!("Initializing connection pool targeting: {}", database_url);

    let pool = AnyPoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await
        .map_err(|e| AppError::Database(e))?;

    // Enable SQLite specific runtime properties
    if database_url.starts_with("sqlite:") {
        sqlx::query("PRAGMA foreign_keys = ON;")
            .execute(&pool)
            .await
            .map_err(|e| AppError::Database(e))?;
        info!("Enforced SQLite foreign key constraints.");
    }

    Ok(pool)
}

/// Runs any pending schema migrations.
pub async fn run_migrations(pool: &AnyPool) -> Result<(), AppError> {
    info!("Checking database schema migrations...");
    sqlx::migrate!("../migrations")
        .run(pool)
        .await
        .map_err(|e| AppError::Database(e.into()))?;
    
    info!("Schema is up to date.");
    Ok(())
}