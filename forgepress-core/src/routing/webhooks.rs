// /forgepress-core/src/routing/webhooks.rs
use axum::{extract::State, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::info;

use crate::app_state::AppState;
use crate::error::AppError;

#[derive(Deserialize)]
pub struct PurgeCachePayload {
    /// Token signature to validate authenticity
    pub secret: String,
    /// Targeted slug to purge, or "all"
    pub target: String,
}

/// Secure public webhook to invalidate caching layers remotely (e.g., from external CI/CD or database mirrors).
pub async fn purge_cache(
    State(state): State<AppState>,
    Json(payload): Json<PurgeCachePayload>,
) -> Result<Json<Value>, AppError> {
    // 1. Validate signature against system configuration SECRET_KEY
    if payload.secret != state.config.secret_key {
        return Err(AppError::Auth("Access Denied: Invalid webhook signature.".to_string()));
    }

    // 2. Perform the cache invalidation
    if payload.target == "all" {
        state.cache.invalidate_all().await;
        info!("Remote webhook initiated a complete cache flush.");
        Ok(Json(json!({
            "status": "success",
            "message": "Entire application cache invalidated successfully."
        })))
    } else {
        state.cache.invalidate(&payload.target).await;
        info!("Remote webhook invalidated cache for path: '/{}'", payload.target);
        Ok(Json(json!({
            "status": "success",
            "message": format!("Cache invalidated for path: '/{}'", payload.target)
        })))
    }
}

pub fn router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/api/webhooks/cache-purge", axum::routing::post(purge_cache))
}