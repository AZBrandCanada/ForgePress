// /forgepress-core/src/routing/mod.rs (Updated)
use axum::Router;
use crate::app_state::AppState;

pub mod admin_api;
pub mod public;
pub mod webhooks;

pub fn app_router(state: AppState) -> Router<AppState> {
    Router::new()
        // 1. Nest admin controllers under /api scope
        .nest("/api", admin_api::router(state.clone()))
        // 2. Nest webhook controllers directly
        .merge(webhooks::router())
        // 3. Fallback catch-all public rendering controllers (must be evaluated last)
        .merge(public::router())
}