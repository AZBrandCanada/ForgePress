// /forgepress-core/src/routing/mod.rs
use axum::Router;
use crate::app_state::AppState;

pub mod admin_api;
pub mod public;
pub mod webhooks;

// Changed return type from Router<AppState> to Router<()>
pub fn app_router(state: AppState) -> Router<()> {
    Router::new()
        .nest("/api", admin_api::router(state.clone()))
        .merge(webhooks::router())
        .merge(public::router())
        // Resolved the state globally so Axum v0.7 serve is satisfied
        .with_state(state)
}