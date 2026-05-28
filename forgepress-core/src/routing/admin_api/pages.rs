// /forgepress-core/src/routing/admin_api/pages.rs
use axum::{
    extract::{Path, State},
    Extension, Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;

use crate::app_state::AppState;
use crate::auth::{require_role_permission, Claims, Permission};
use crate::database::pages::{create_page, delete_page, get_page_by_slug, update_page};
use crate::error::AppError;

#[derive(Deserialize)]
pub struct CreatePageRequest {
    pub title: String,
    pub slug: String,
}

#[derive(Deserialize)]
pub struct UpdatePageRequest {
    pub title: String,
    pub slug: String,
    pub status: String,
    pub content: Value,
    pub meta: Value,
}

pub async fn create_page(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreatePageRequest>,
) -> Result<Json<Value>, AppError> {
    // Require standard write capabilities
    require_role_permission(&claims, Permission::CreatePosts)?;

    // Validate if the slug is already registered
    if let Some(_) = get_page_by_slug(&state.db, &payload.slug).await? {
        return Err(AppError::Internal(format!(
            "A page with the slug '{}' already exists.",
            payload.slug
        )));
    }

    let page = create_page(&state.db, &payload.title, &payload.slug, Some(claims.sub)).await?;

    Ok(Json(json!({
        "status": "success",
        "data": page
    })))
}

pub async fn get_page(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Value>, AppError> {
    let page = get_page_by_slug(&state.db, &slug)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Page '{}' not found.", slug)))?;

    Ok(Json(json!({ "status": "success", "data": page })))
}

pub async fn save_page(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePageRequest>,
) -> Result<Json<Value>, AppError> {
    // Require publish or update capabilities depending on target status
    let required_perm = if payload.status == "published" {
        Permission::PublishPosts
    } else {
        Permission::CreatePosts
    };
    require_role_permission(&claims, required_perm)?;

    update_page(
        &state.db,
        id,
        &payload.title,
        &payload.slug,
        &payload.status,
        payload.content,
        payload.meta,
    )
    .await?;

    // Invalidate local in-memory caches matching this updated route path
    state.cache.remove(&payload.slug).await;

    Ok(Json(json!({
        "status": "success",
        "message": "Page updated successfully and caches invalidated."
    })))
}

pub async fn remove_page(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, AppError> {
    require_role_permission(&claims, Permission::PublishPosts)?;

    delete_page(&state.db, id).await?;

    Ok(Json(json!({
        "status": "success",
        "message": "Page deleted successfully."
    })))
}