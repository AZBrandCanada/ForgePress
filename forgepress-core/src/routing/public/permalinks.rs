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

/// Global catch-all handler for public web traffic.
pub async fn handle_public_request(
    State(state): State<AppState>,
    Path(mut path): Path<String>,
) -> Result<Html<String>, AppError> {
    // Normalize root-level routing index calls
    if path.is_empty() || path == "/" {
        path = "index".to_string();
    }

    debug!("Dynamic routing intercept received path: '/{}'", path);

    // 1. Search the high-performance in-memory Moka cache (Nanosecond check)
    if let Some(cached_html) = state.cache.get(&path).await {
        debug!("Cache Hit! Serving '/{}' directly from RAM.", path);
        return Ok(Html(cached_html));
    }

    // 2. Cache Miss: Fetch page details from Database
    let page = get_page_by_slug(&state.db, &path)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Oops! The page '/{}' could not be found.", path)))?;

    // 3. Verify Publication Status
    if page.status != "published" {
        return Err(AppError::NotFound(format!("The page '/{}' is currently unavailable.", path)));
    }

    // 4. Compile the templates dynamically via MiniJinja
    let rendered_html = render_page(&state.template_env, &page)?;

    // 5. Save the compiled payload to the in-memory cache for subsequent calls
    state.cache.insert(path.clone(), rendered_html.clone()).await;
    debug!("Successfully saved compiled path '/{}' to in-memory cache.", path);

    Ok(Html(rendered_html))
}