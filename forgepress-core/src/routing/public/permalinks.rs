// /forgepress-core/src/routing/public/permalinks.rs
use axum::{
    extract::{Path, State},
    response::Html,
};
use tracing::debug;

use crate::app_state::AppState;
use crate::database::pages::get_page_by_slug;
use crate::error::AppError;
use crate::routing::public::renderer::render_page;

/// Shared internal handler logic to safely process the normalized page path.
async fn handle_public_request_with_path(
    state: AppState,
    mut path: String,
) -> Result<Html<String>, AppError> {
    // If the path is empty, default to "index"
    if path.is_empty() || path == "/" {
        path = "index".to_string();
    }

    // Strip any leading slashes to prevent query mismatches
    let clean_path = path.trim_start_matches('/').to_string();

    debug!("Dynamic routing intercept received path: '/{}'", clean_path);

    if let Some(cached_html) = state.cache.get(&clean_path).await {
        debug!("Cache Hit! Serving '/{}' directly from RAM.", clean_path);
        return Ok(Html(cached_html));
    }

    let page = get_page_by_slug(&state.db, &clean_path)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Oops! The page '/{}' could not be found.", clean_path)))?;

    if page.status != "published" {
        return Err(AppError::NotFound(format!("The page '/{}' is currently unavailable.", clean_path)));
    }

    let rendered_html = render_page(&state, &page).await?;

    state.cache.insert(clean_path.clone(), rendered_html.clone()).await;
    debug!("Successfully saved compiled path '/{}' to in-memory cache.", clean_path);

    Ok(Html(rendered_html))
}

/// Handler for general sub-paths matching "/*path"
pub async fn handle_public_request(
    State(state): State<AppState>,
    Path(path): Path<String>,
) -> Result<Html<String>, AppError> {
    handle_public_request_with_path(state, path).await
}

/// Handler specifically for the root homepage "/" path
pub async fn handle_homepage_request(
    State(state): State<AppState>,
) -> Result<Html<String>, AppError> {
    handle_public_request_with_path(state, "index".to_string()).await
}