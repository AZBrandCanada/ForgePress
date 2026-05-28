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

pub async fn handle_public_request(
    State(state): State<AppState>,
    Path(mut path): Path<String>,
) -> Result<Html<String>, AppError> {
    if path.is_empty() || path == "/" {
        path = "index".to_string();
    }

    debug!("Dynamic routing intercept received path: '/{}'", path);

    if let Some(cached_html) = state.cache.get(&path).await {
        debug!("Cache Hit! Serving '/{}' directly from RAM.", path);
        return Ok(Html(cached_html));
    }

    let page = get_page_by_slug(&state.db, &path)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Oops! The page '/{}' could not be found.", path)))?;

    if page.status != "published" {
        return Err(AppError::NotFound(format!("The page '/{}' is currently unavailable.", path)));
    }

    // Fixed: Passed &state instead of &state.template_env, and appended .await prior to ?
    let rendered_html = render_page(&state, &page).await?;

    state.cache.insert(path.clone(), rendered_html.clone()).await;
    debug!("Successfully saved compiled path '/{}' to in-memory cache.", path);

    Ok(Html(rendered_html))
}