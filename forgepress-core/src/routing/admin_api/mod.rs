// /forgepress-core/src/routing/admin_api/mod.rs
use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::app_state::AppState;

pub mod auth;
pub mod pages;
pub mod media;

pub fn router(state: AppState) -> Router<AppState> {
    let auth_routes = Router::new()
        .route("/login", post(auth::login));

    let protected_routes = Router::new()
        // Page Builder Endpoints
        .route("/pages", get(pages::list_pages).post(pages::create_page))
        .route("/pages/by-slug/:slug", get(pages::get_page))
        .route("/pages/:id", put(pages::save_page).delete(pages::remove_page))
        
        // ADDED: Endpoint to handle safe-swapping homepages in database
        .route("/pages/:id/set-homepage", post(pages::set_homepage))
        
        .route("/media/upload", post(media::upload))
        .route_layer(axum::middleware::from_fn_with_state(
            state,
            crate::auth::require_auth,
        ));

    Router::new()
        .nest("/auth", auth_routes)
        .nest("/admin", protected_routes)
}