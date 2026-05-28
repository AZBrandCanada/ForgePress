// /forgepress-core/src/database/pages.rs
use sqlx::AnyPool;
use chrono::Utc;
use serde_json::Value;
use crate::error::AppError;

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Page {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub status: String,
    pub author_id: String,     // Changed from Option<String> to String
    pub content: String,
    pub meta: String,
    pub published_at: String,  // Changed from Option<String> to String
    pub created_at: String,
    pub updated_at: String,
}

pub async fn create_page(
    pool: &AnyPool,
    title: &str,
    slug: &str,
    author_id: Option<String>,
) -> Result<Page, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();
    
    let default_content = "[]".to_string();
    let default_meta = "{}".to_string();

    sqlx::query(
        "INSERT INTO pages (id, title, slug, status, author_id, content, meta, created_at, updated_at) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
    )
    .bind(&id)
    .bind(title)
    .bind(slug)
    .bind("draft") 
    .bind(&author_id)
    .bind(&default_content)
    .bind(&default_meta)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    Ok(Page {
        id,
        title: title.to_string(),
        slug: slug.to_string(),
        status: "draft".to_string(),
        author_id: author_id.unwrap_or_default(), // Convert Option to String
        content: default_content,
        meta: default_meta,
        published_at: "".to_string(), // Empty string fallback
        created_at: now.clone(),
        updated_at: now,
    })
}

pub async fn get_page_by_slug(pool: &AnyPool, slug: &str) -> Result<Option<Page>, AppError> {
    // Fixed: Wrapped nullable fields in COALESCE to force the Any driver to parse them as String
    let page = sqlx::query_as::<_, Page>(
        "SELECT id, title, slug, status, COALESCE(author_id, '') AS author_id, content, meta, \
         COALESCE(published_at, '') AS published_at, created_at, updated_at \
         FROM pages WHERE slug = $1"
    )
    .bind(slug)
    .fetch_optional(pool)
    .await?;
    Ok(page)
}

pub async fn update_page(
    pool: &AnyPool,
    id: &str,
    title: &str,
    slug: &str,
    status: &str,
    content: Value,
    meta: Value,
) -> Result<(), AppError> {
    let now = Utc::now().to_rfc3339();
    let published_at = if status == "published" { Some(now.clone()) } else { None };

    let content_str = serde_json::to_string(&content)
        .map_err(|e| AppError::Internal(format!("Failed to serialize page content: {}", e)))?;
    let meta_str = serde_json::to_string(&meta)
        .map_err(|e| AppError::Internal(format!("Failed to serialize page metadata: {}", e)))?;

    sqlx::query(
        "UPDATE pages SET title = $1, slug = $2, status = $3, content = $4, meta = $5, \
         published_at = COALESCE($6, published_at), updated_at = $7 WHERE id = $8"
    )
    .bind(title)
    .bind(slug)
    .bind(status)
    .bind(&content_str)
    .bind(&meta_str)
    .bind(published_at)
    .bind(&now)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_page(pool: &AnyPool, id: &str) -> Result<(), AppError> {
    sqlx::query("DELETE FROM pages WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}