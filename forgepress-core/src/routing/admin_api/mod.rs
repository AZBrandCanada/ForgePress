// /forgepress-core/src/routing/admin_api/mod.rs
use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::app_state::AppState;

pub mod auth;
pub mod pages;
pub mod media;

/// Constructs and returns the complete administrative API router path tree.
pub fn router(state: AppState) -> Router<AppState> {
    let auth_routes = Router::new()
        .route("/login", post(auth::login));

    let protected_routes = Router::new()
        // Page Builder Endpoints
        .route("/pages", post(pages::create_page))
        // Fixed: Moved the slug endpoint into its own sub-route namespace to prevent conflicts with :id
        .route("/pages/by-slug/:slug", get(pages::get_page))
        .route("/pages/:id", put(pages::save_page).delete(pages::remove_page))
        // Media Upload Endpoints
        .route("/media/upload", post(media::upload))
        // Apply JWT validation middleware across all protected routes
        .route_layer(axum::middleware::from_fn_with_state(
            state,
            crate::auth::require_auth,
        ));

    Router::new()
        .nest("/auth", auth_routes)
        .nest("/admin", protected_routes)
}