// /forgepress-core/src/error.rs
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;
use tracing::error;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    Template(minijinja::Error),
    Auth(String),
    Io(std::io::Error),
    NotFound(String),
    Plugin(String),
    Internal(String),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err)
    }
}

impl From<minijinja::Error> for AppError {
    fn from(err: minijinja::Error) -> Self {
        AppError::Template(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Database(e) => write!(f, "Database error: {}", e),
            AppError::Template(e) => write!(f, "Template error: {}", e),
            AppError::Auth(s) => write!(f, "Auth error: {}", s),
            AppError::Io(e) => write!(f, "I/O error: {}", e),
            AppError::NotFound(s) => write!(f, "Not found: {}", s),
            AppError::Plugin(s) => write!(f, "Plugin error: {}", s),
            AppError::Internal(s) => write!(f, "Internal error: {}", s),
        }
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Template(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Auth(_) => StatusCode::UNAUTHORIZED,
            AppError::Io(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Plugin(_) => StatusCode::BAD_GATEWAY,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        // 1. Log the full detailed systems error inside the server console/terminal
        error!("AppError Exception: {:?}", self);

        // 2. Determine error message to return to client
        // If we are in local development (debug), return the raw, detailed system error!
        // If in production (release), return a sanitized string to protect server internals.
        let error_message = if cfg!(debug_assertions) {
            match self {
                AppError::Database(err) => format!("Database Exception: {}", err),
                AppError::Template(err) => format!("Template Exception: {}", err),
                AppError::Auth(msg) => msg,
                AppError::Io(err) => format!("I/O Exception: {}", err),
                AppError::NotFound(msg) => msg,
                AppError::Plugin(msg) => format!("Plugin Exception: {}", msg),
                AppError::Internal(msg) => format!("Internal Exception: {}", msg),
            }
        } else {
            match self {
                AppError::Database(_) => "A database processing error occurred.".to_string(),
                AppError::Template(_) => "A rendering template error occurred.".to_string(),
                AppError::Auth(msg) => msg,
                AppError::Io(_) => "A file system operations error occurred.".to_string(),
                AppError::NotFound(msg) => msg,
                AppError::Plugin(_) => "An error occurred inside an active plugin extension.".to_string(),
                AppError::Internal(_) => "An internal server error occurred.".to_string(),
            }
        };

        let body = Json(json!({
            "status": "error",
            "message": error_message
        }));

        (status, body).into_response()
    }
}