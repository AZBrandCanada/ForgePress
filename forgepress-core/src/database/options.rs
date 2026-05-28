// /forgepress-core/src/database/options.rs
use sqlx::AnyPool;
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::error::AppError;

/// Sets or updates a global configuration option.
pub async fn set_option<T: Serialize>(pool: &AnyPool, key: &str, value: &T) -> Result<(), AppError> {
    let serialized_value = serde_json::to_string(value)
        .map_err(|e| AppError::Internal(format!("Failed to serialize option: {}", e)))?;

    sqlx::query(
        "INSERT INTO options (option_key, option_value) VALUES ($1, $2) \
         ON CONFLICT (option_key) DO UPDATE SET option_value = EXCLUDED.option_value"
    )
    .bind(key)
    .bind(serialized_value)
    .execute(pool)
    .await?;

    Ok(())
}

/// Retrieves a configuration option, deserializing it back to its native Rust type.
pub async fn get_option<T: DeserializeOwned>(pool: &AnyPool, key: &str) -> Result<Option<T>, AppError> {
    let row: Option<(String,)> = sqlx::query_as("SELECT option_value FROM options WHERE option_key = $1")
        .bind(key)
        .fetch_optional(pool)
        .await?;

    match row {
        Some((val_str,)) => {
            let deserialized: T = serde_json::from_str(&val_str)
                .map_err(|e| AppError::Internal(format!("Failed to deserialize option payload: {}", e)))?;
            Ok(Some(deserialized))
        }
        None => Ok(None),
    }
}