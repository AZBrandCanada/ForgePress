use axum::{extract::State, Json};
use serde_json::{json, Value};

use crate::app_state::AppState;
use crate::i18n::Locale; // Imports our extractor
use crate::error::AppError;

pub async fn secure_admin_checkpoint(
    State(state): State<AppState>,
    locale: Locale, // <-- Automatically parsed and injected here!
) -> Result<Json<Value>, AppError> {
    
    // Perform simulated check, then resolve dynamic string keys
    let error_message = state.i18n.translate(&locale.0, "login_failed").await;

    Err(AppError::Auth(error_message))
}