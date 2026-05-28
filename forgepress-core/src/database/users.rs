// /forgepress-core/src/database/users.rs
use sqlx::{AnyPool, Row};
use uuid::Uuid;
use chrono::Utc;
use crate::error::AppError;

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

pub async fn create_user(
    pool: &AnyPool,
    username: &str,
    email: &str,
    password_hash: &str,
    role: &str,
) -> Result<User, AppError> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query(
        "INSERT INTO users (id, username, email, password_hash, role, created_at, updated_at) \
         VALUES ($1, $2, $3, $4, $5, $6, $7)"
    )
    .bind(id)
    .bind(username)
    .bind(email)
    .bind(password_hash)
    .bind(role)
    .bind(now)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(User {
        id,
        username: username.to_string(),
        email: email.to_string(),
        password_hash: password_hash.to_string(),
        role: role.to_string(),
        created_at: now,
        updated_at: now,
    })
}

pub async fn get_user_by_id(pool: &AnyPool, id: Uuid) -> Result<Option<User>, AppError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(user)
}

pub async fn get_user_by_username(pool: &AnyPool, username: &str) -> Result<Option<User>, AppError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await?;
    Ok(user)
}