// /forgepress-core/src/routing/public/mod.rs
use axum::{routing::get, Router};
use crate::app_state::AppState;

pub mod permalinks;
pub mod renderer;

/// Constructs and returns the public front-end router.
pub fn router() -> Router<AppState> {
    Router::new()
        // Explicitly handles the root homepage "/" which "/*path" fails to match
        .route("/", get(permalinks::handle_homepage_request))
        // Catch-all wildcard matches all other public paths not prefixed with /api
        .route("/*path", get(permalinks::handle_public_request))
}