// /forgepress-core/src/database/users.rs
use sqlx::AnyPool;
use chrono::Utc;
use crate::error::AppError;

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: String,
    pub updated_at: String,
}

// /forgepress-core/src/database/users.rs

pub async fn create_user(
    pool: &AnyPool,
    username: &str,
    email: &str,
    password_hash: &str,
    role: &str,
) -> Result<User, AppError> {
    let id = uuid::Uuid::new_v4().to_string();
    let now = Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO users (id, username, email, password_hash, role, created_at, updated_at) \
         VALUES (CAST($1 AS uuid), $2, $3, $4, $5, CAST($6 AS timestamptz), CAST($7 AS timestamptz))"
    )
    .bind(&id)
    .bind(username)
    .bind(email)
    .bind(password_hash)
    .bind(role)
    .bind(&now)
    .bind(&now)
    .execute(pool)
    .await?;

    Ok(User {
        id,
        username: username.to_string(),
        email: email.to_string(),
        password_hash: password_hash.to_string(),
        role: role.to_string(),
        created_at: now.clone(),
        updated_at: now,
    })
}
pub async fn get_user_by_id(pool: &AnyPool, id: &str) -> Result<Option<User>, AppError> {
    let user = sqlx::query_as::<_, User>(
        "SELECT CAST(id AS VARCHAR) AS id, username, email, password_hash, role, \
         CAST(created_at AS VARCHAR) AS created_at, CAST(updated_at AS VARCHAR) AS updated_at \
         FROM users WHERE id = CAST($1 AS uuid)"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    Ok(user)
}

pub async fn get_user_by_username(pool: &AnyPool, username: &str) -> Result<Option<User>, AppError> {
    let user = sqlx::query_as::<_, User>(
        "SELECT CAST(id AS VARCHAR) AS id, username, email, password_hash, role, \
         CAST(created_at AS VARCHAR) AS created_at, CAST(updated_at AS VARCHAR) AS updated_at \
         FROM users WHERE username = $1"
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;
    Ok(user)
}