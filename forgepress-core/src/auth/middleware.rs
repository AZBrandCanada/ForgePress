// /forgepress-core/src/auth/middleware.rs
use axum::{
    body::Body,
    extract::State,
    http::Request,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app_state::AppState;
use crate::auth::roles::Role;
use crate::error::AppError;

/// Standard JWT payload containing claims.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject (User ID)
    pub sub: Uuid,
    /// Expiration timestamp
    pub exp: usize,
    /// User role
    pub role: String,
}

/// Asynchronous Axum middleware to enforce authentication.
/// Inspects the standard Authorization header, verifies the signature, and validates expiration.
pub async fn require_auth(
    State(state): State<AppState>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    // 1. Extract the Authorization header
    let auth_header = request
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::Auth("Authorization header is missing or malformed.".to_string()))?;

    // 2. Extract bearer token
    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Auth("Authorization method must be 'Bearer'.".to_string()));
    }
    let token = &auth_header[7..];

    // 3. Decode and validate the token signature using the global secret key
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(state.config.secret_key.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Auth("Invalid or expired session token.".to_string()))?;

    // 4. Inject validated claims into the request context extensions
    request.extensions_mut().insert(token_data.claims);

    // Proceed to next handler/middleware in the stack
    Ok(next.run(request).await)
}

/// Helper extension extractor to enforce role-specific permissions at the route-level.
pub fn require_role_permission(claims: &Claims, required_permission: super::roles::Permission) -> Result<(), AppError> {
    use std::str::FromStr;
    
    let user_role = Role::from_str(&claims.role)
        .map_err(|_| AppError::Auth("Invalid user role assigned to session.".to_string()))?;

    if !user_role.has_permission(required_permission) {
        return Err(AppError::Auth("Forbidden: You do not possess the required permissions to access this endpoint.".to_string()));
    }
    
    Ok(())
}