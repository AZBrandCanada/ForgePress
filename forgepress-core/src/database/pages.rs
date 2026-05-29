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
    pub author_id: String,
    pub content: String,
    pub meta: String,
    pub published_at: String,
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
         VALUES (CAST($1 AS uuid), $2, $3, $4, CAST($5 AS uuid), CAST($6 AS jsonb), CAST($7 AS jsonb), CAST($8 AS timestamptz), CAST($9 AS timestamptz))"
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
        author_id: author_id.unwrap_or_default(),
        content: default_content,
        meta: default_meta,
        published_at: "".to_string(),
        created_at: now.clone(),
        updated_at: now,
    })
}

pub async fn get_page_by_slug(pool: &AnyPool, slug: &str) -> Result<Option<Page>, AppError> {
    let page = sqlx::query_as::<_, Page>(
        "SELECT CAST(id AS VARCHAR) AS id, title, slug, status, \
         COALESCE(CAST(author_id AS VARCHAR), '') AS author_id, \
         CAST(content AS VARCHAR) AS content, \
         CAST(meta AS VARCHAR) AS meta, \
         COALESCE(CAST(published_at AS VARCHAR), '') AS published_at, \
         CAST(created_at AS VARCHAR) AS created_at, \
         CAST(updated_at AS VARCHAR) AS updated_at \
         FROM pages WHERE slug = $1"
    )
    .bind(slug)
    .fetch_optional(pool)
    .await?;
    Ok(page)
}

pub async fn get_all_pages(pool: &AnyPool) -> Result<Vec<Page>, AppError> {
    let pages = sqlx::query_as::<_, Page>(
        "SELECT CAST(id AS VARCHAR) AS id, title, slug, status, \
         COALESCE(CAST(author_id AS VARCHAR), '') AS author_id, \
         CAST(content AS VARCHAR) AS content, \
         CAST(meta AS VARCHAR) AS meta, \
         COALESCE(CAST(published_at AS VARCHAR), '') AS published_at, \
         CAST(created_at AS VARCHAR) AS created_at, \
         CAST(updated_at AS VARCHAR) AS updated_at \
         FROM pages ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;
    
    Ok(pages)
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
        "UPDATE pages SET title = $1, slug = $2, status = $3, \
         content = CAST($4 AS jsonb), meta = CAST($5 AS jsonb), \
         published_at = COALESCE(CAST($6 AS timestamptz), published_at), \
         updated_at = CAST($7 AS timestamptz) WHERE id = CAST($8 AS uuid)"
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
    sqlx::query("DELETE FROM pages WHERE id = CAST($1 AS uuid)")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}