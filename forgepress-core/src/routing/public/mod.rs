// /forgepress-core/src/routing/public/mod.rs
use axum::{routing::get, Router};
use crate::app_state::AppState;

pub mod permalinks;
pub mod renderer;

/// Constructs and returns the public front-end router.
pub fn router() -> Router<AppState> {
    Router::new()
        // Catch-all wildcard matches all public paths not prefixed with /api
        .route("/*path", get(permalinks::handle_public_request))
}