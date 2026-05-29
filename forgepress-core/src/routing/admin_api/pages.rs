// /forgepress-core/src/routing/admin_api/pages.rs
use axum::{
    extract::{Path, State},
    Extension, Json,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::app_state::AppState;
use crate::auth::{require_role_permission, Claims, Permission};
use crate::database::pages as db_pages;
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

// NEW: Handles GET /api/admin/pages requests, returning metadata list
pub async fn list_pages(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Value>, AppError> {
    // Ensure read permissions
    require_role_permission(&claims, Permission::ReadPosts)?;

    let pages_list = db_pages::get_all_pages(&state.db).await?;
    
    // Parse dynamic JSONB structures to deliver standard JSON objects to the client
    let mut sanitized_list = Vec::with_capacity(pages_list.len());
    for page in pages_list {
        let content_value: Value = serde_json::from_str(&page.content).unwrap_or_else(|_| json!([]));
        let meta_value: Value = serde_json::from_str(&page.meta).unwrap_or_else(|_| json!({}));
        let author_id_opt = if page.author_id.is_empty() { None } else { Some(page.author_id) };
        let published_at_opt = if page.published_at.is_empty() { None } else { Some(page.published_at) };

        sanitized_list.push(json!({
            "id": page.id,
            "title": page.title,
            "slug": page.slug,
            "status": page.status,
            "author_id": author_id_opt,
            "content": content_value,
            "meta": meta_value,
            "published_at": published_at_opt,
            "created_at": page.created_at,
            "updated_at": page.updated_at
        }));
    }

    Ok(Json(json!({
        "status": "success",
        "data": sanitized_list
    })))
}

pub async fn create_page(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreatePageRequest>,
) -> Result<Json<Value>, AppError> {
    require_role_permission(&claims, Permission::CreatePosts)?;

    if let Some(_) = db_pages::get_page_by_slug(&state.db, &payload.slug).await? {
        return Err(AppError::Internal(format!(
            "A page with the slug '{}' already exists.",
            payload.slug
        )));
    }

    let page = db_pages::create_page(&state.db, &payload.title, &payload.slug, Some(claims.sub.to_string())).await?;

    let content_value: Value = serde_json::from_str(&page.content).unwrap_or_else(|_| json!([]));
    let meta_value: Value = serde_json::from_str(&page.meta).unwrap_or_else(|_| json!({}));

    Ok(Json(json!({
        "status": "success",
        "data": {
            "id": page.id,
            "title": page.title,
            "slug": page.slug,
            "status": page.status,
            "author_id": if page.author_id.is_empty() { None } else { Some(&page.author_id) },
            "content": content_value,
            "meta": meta_value,
            "published_at": if page.published_at.is_empty() { None } else { Some(&page.published_at) },
            "created_at": page.created_at,
            "updated_at": page.updated_at
        }
    })))
}

pub async fn get_page(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Value>, AppError> {
    let page = db_pages::get_page_by_slug(&state.db, &slug)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Page '{}' not found.", slug)))?;

    let content_value: Value = serde_json::from_str(&page.content).unwrap_or_else(|_| json!([]));
    let meta_value: Value = serde_json::from_str(&page.meta).unwrap_or_else(|_| json!({}));

    Ok(Json(json!({ 
        "status": "success", 
        "data": {
            "id": page.id,
            "title": page.title,
            "slug": page.slug,
            "status": page.status,
            "author_id": if page.author_id.is_empty() { None } else { Some(&page.author_id) },
            "content": content_value,
            "meta": meta_value,
            "published_at": if page.published_at.is_empty() { None } else { Some(&page.published_at) },
            "created_at": page.created_at,
            "updated_at": page.updated_at
        }
    })))
}

pub async fn save_page(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(payload): Json<UpdatePageRequest>,
) -> Result<Json<Value>, AppError> {
    let required_perm = if payload.status == "published" {
        Permission::PublishPosts
    } else {
        Permission::CreatePosts
    };
    require_role_permission(&claims, required_perm)?;

    db_pages::update_page(
        &state.db,
        &id,
        &payload.title,
        &payload.slug,
        &payload.status,
        payload.content,
        payload.meta,
    )
    .await?;

    state.cache.invalidate(&payload.slug).await;

    Ok(Json(json!({
        "status": "success",
        "message": "Page updated successfully and caches invalidated."
    })))
}

pub async fn remove_page(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<Value>, AppError> {
    require_role_permission(&claims, Permission::PublishPosts)?;

    db_pages::delete_page(&state.db, &id).await?;

    Ok(Json(json!({
        "status": "success",
        "message": "Page deleted successfully."
    })))
}