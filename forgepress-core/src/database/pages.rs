// /forgepress-core/src/database/pages.rs
use sqlx::AnyPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::types::Json;
use crate::error::AppError;

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Page {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub status: String,
    pub author_id: Option<Uuid>,
    /// sqlx::types::Json wraps parsing rules to support Postgres JSONB & SQLite Text dynamic mapping.
    pub content: Json<Value>,
    pub meta: Json<Value>,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub async fn create_page(
    pool: &AnyPool,
    title: &str,
    slug: &str,
    author_id: Option<Uuid>,
) -> Result<Page, AppError> {
    let id = Uuid::new_v4();
    let now = Utc::now();
    let default_content = Json(serde_json::json!([]));
    let default_meta = Json(serde_json::json!({}));

    sqlx::query(
        "INSERT INTO pages (id, title, slug, status, author_id, content, meta, created_at, updated_at) \
         VALUES ($1, $2, $3, 'draft', $4, $5, $6, $7, $8)"
    )
    .bind(id)
    .bind(title)
    .bind(slug)
    .bind(author_id)
    .bind(&default_content)
    .bind(&default_meta)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(Page {
        id,
        title: title.to_string(),
        slug: slug.to_string(),
        status: "draft".to_string(),
        author_id,
        content: default_content,
        meta: default_meta,
        published_at: None,
        created_at: now,
        updated_at: now,
    })
}

pub async fn get_page_by_slug(pool: &AnyPool, slug: &str) -> Result<Option<Page>, AppError> {
    let page = sqlx::query_as::<_, Page>("SELECT * FROM pages WHERE slug = $1")
        .bind(slug)
        .fetch_optional(pool)
        .await?;
    Ok(page)
}

pub async fn update_page(
    pool: &AnyPool,
    id: Uuid,
    title: &str,
    slug: &str,
    status: &str,
    content: Value,
    meta: Value,
) -> Result<(), AppError> {
    let now = Utc::now();
    let published_at = if status == "published" { Some(now) } else { None };

    sqlx::query(
        "UPDATE pages SET title = $1, slug = $2, status = $3, content = $4, meta = $5, \
         published_at = COALESCE($6, published_at), updated_at = $7 WHERE id = $8"
    )
    .bind(title)
    .bind(slug)
    .bind(status)
    .bind(Json(content))
    .bind(Json(meta))
    .bind(published_at)
    .bind(now)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_page(pool: &AnyPool, id: Uuid) -> Result<(), AppError> {
    sqlx::query("DELETE FROM pages WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}