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
use std::fs;

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

// Handles GET /api/admin/pages requests, returning metadata list
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

/// Atomically de-registers any existing homepage and sets the selected page as the new homepage.
pub async fn set_homepage(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<Value>, AppError> {
    require_role_permission(&claims, Permission::PublishPosts)?;

    // 1. Begin database transaction
    let mut tx = state.db.begin().await.map_err(|e| AppError::Database(e))?;

    // 2. Locate any existing page matching the "index" slug
    let existing_homepage: Option<(String, String)> = sqlx::query_as(
        "SELECT CAST(id AS VARCHAR) as id, title FROM pages WHERE slug = 'index'"
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(|e| AppError::Database(e))?;

    if let Some((old_id, _old_title)) = existing_homepage {
        // Safe-swap: Assign a temporary archived slug and revert its status back to draft
        let temp_slug = format!("archived-home-{}", &uuid::Uuid::new_v4().to_string()[..8]);
        let now = chrono::Utc::now().to_rfc3339();

        sqlx::query(
            "UPDATE pages SET slug = $1, status = 'draft', updated_at = CAST($2 AS timestamptz) WHERE id = CAST($3 AS uuid)"
        )
        .bind(&temp_slug)
        .bind(&now)
        .bind(&old_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| AppError::Database(e))?;
    }

    // 3. Mark the target page as the active homepage ("index") and publish it
    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        "UPDATE pages SET slug = 'index', status = 'published', updated_at = CAST($1 AS timestamptz) WHERE id = CAST($2 AS uuid)"
    )
    .bind(&now)
    .bind(&id)
    .execute(&mut *tx)
    .await
    .map_err(|e| AppError::Database(e))?;

    tx.commit().await.map_err(|e| AppError::Database(e))?;

    // 4. Invalidate the public-facing homepage caches
    state.cache.invalidate("index").await;

    Ok(Json(json!({
        "status": "success",
        "message": "Page successfully configured as the front homepage."
    })))
}

/// Recursively scans and parses the block JSON schemas inside the active theme.
pub async fn get_block_registry() -> Result<Value, AppError> {
    let content_dir = std::env::var("CONTENT_DIR").unwrap_or_else(|_| "./content".to_string());
    
    // Explicit std::path::Path usage resolves the Axum type namespace collision
    let blocks_dir = std::path::Path::new(&content_dir)
        .join("themes")
        .join("default")
        .join("templates")
        .join("blocks");

    let mut registry = Vec::new();

    if blocks_dir.exists() && blocks_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(blocks_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                // Dynamically find and read all companion .json files
                if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        // Safe type parsing directly on the deserialization call
                        if let Ok(schema) = serde_json::from_str::<Value>(&content) {
                            registry.push(schema);
                        }
                    }
                }
            }
        }
    }

    Ok(serde_json::Value::Array(registry))
}

/// Axum controller to deliver block schemas to the Svelte page builder.
pub async fn list_blocks(
    State(_state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Value>, AppError> {
    crate::auth::require_role_permission(&claims, crate::auth::Permission::ReadPosts)?;
    let registry = get_block_registry().await?;
    Ok(Json(json!({
        "status": "success",
        "data": registry
    })))
}