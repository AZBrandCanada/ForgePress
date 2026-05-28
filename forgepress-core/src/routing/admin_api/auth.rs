// /forgepress-core/src/routing/admin_api/auth.rs
use axum::{extract::State, Json};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::str::FromStr;

use crate::app_state::AppState;
use crate::auth::{verify_password, Claims};
use crate::database::users::get_user_by_username;
use crate::domain::user::PublicUser;
use crate::error::AppError;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: PublicUser,
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let user = get_user_by_username(&state.db, &payload.username)
        .await?
        .ok_or_else(|| AppError::Auth("Invalid username or password.".to_string()))?;

    let is_valid = verify_password(&payload.password, &user.password_hash)?;
    if !is_valid {
        return Err(AppError::Auth("Invalid username or password.".to_string()));
    }

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .ok_or_else(|| AppError::Internal("Failed to calculate token expiration.".to_string()))?
        .timestamp() as usize;

    // Parse the database string key safely into Uuid
    let user_id = uuid::Uuid::from_str(&user.id)
        .map_err(|e| AppError::Internal(format!("Invalid User ID syntax in database: {}", e)))?;

    let claims = Claims {
        sub: user_id,
        exp: expiration,
        role: user.role.clone(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.config.secret_key.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("JWT Generation Error: {}", e)))?;

    let public_user = PublicUser::from(user);

    Ok(Json(LoginResponse {
        token,
        user: public_user,
    }))
}