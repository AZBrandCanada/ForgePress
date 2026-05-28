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

// 1. Implement Display for type conversions
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

// 2. Implement standard Error trait
impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(ref err) => {
                error!("Database Exception: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "A database processing error occurred.".to_string())
            }
            AppError::Template(ref err) => {
                error!("Template Exception: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "A rendering template error occurred.".to_string())
            }
            AppError::Auth(msg) => {
                error!("Auth Exception: {}", msg);
                (StatusCode::UNAUTHORIZED, msg)
            }
            AppError::Io(ref err) => {
                error!("I/O Exception: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "A file system operations error occurred.".to_string())
            }
            AppError::NotFound(msg) => {
                (StatusCode::NOT_FOUND, msg)
            }
            AppError::Plugin(msg) => {
                error!("Plugin Sandbox Exception: {}", msg);
                (StatusCode::BAD_GATEWAY, "An error occurred inside an active plugin extension.".to_string())
            }
            AppError::Internal(msg) => {
                error!("Internal Server Exception: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "An internal server error occurred.".to_string())
            }
        };

        let body = Json(json!({
            "status": "error",
            "message": error_message
        }));

        (status, body).into_response()
    }
}