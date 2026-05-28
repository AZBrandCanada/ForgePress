// /forgepress-core/src/routing/admin_api/media.rs (Updated)
use axum::{
    extract::{Multipart, State},
    Extension, Json,
};
use serde_json::{json, Value};
use tracing::{info, error};

use crate::app_state::AppState;
use crate::auth::{require_role_permission, Claims, Permission};
use crate::error::AppError;
use crate::media::{save_original_upload, spawn_optimization_task}; // Updated imports

pub async fn upload(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    mut multipart: Multipart,
) -> Result<Json<Value>, AppError> {
    // 1. Validate write permission
    require_role_permission(&claims, Permission::CreatePosts)?;

    // 2. Parse Multipart fields
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::Internal(format!("Multipart upload parsing failed: {}", e)))?
    {
        let name = field.name().unwrap_or_default().to_string();
        let file_name = field.file_name().unwrap_or_default().to_string();

        if name == "file" && !file_name.is_empty() {
            let data = field.bytes().await.map_err(|e| AppError::Io(e.into()))?;

            if data.len() > state.config.max_upload_size {
                return Err(AppError::Internal("File payload size exceeds configured maximum limits.".to_string()));
            }

            // 3. Save original file via secure signature validation
            let absolute_path = save_original_upload(&state.config.upload_dir, &file_name, &data).await?;
            let relative_url = absolute_path
                .to_string_lossy()
                .replace("\\", "/") // Standardize paths for Web URLs
                .replace("./", "/"); // Keep path relative to web root

            // Generate paths for anticipated WebP responsive image variants
            let relative_url_str = relative_url.clone();
            let file_stem_path = std::path::Path::new(&relative_url_str);
            let parent_dir = file_stem_path.parent().unwrap_or(std::path::Path::new("")).to_string_lossy();
            let file_stem = file_stem_path.file_stem().unwrap_or_default().to_string_lossy();

            let thumbnail_url = format!("{}/{}-thumbnail.webp", parent_dir, file_stem);
            let large_url = format!("{}/{}-large.webp", parent_dir, file_stem);

            // 4. Spawn a truly non-blocking background task to process responsive WebP scaling
            let original_path_str = absolute_path.to_string_lossy().to_string();
            tokio::spawn(async move {
                spawn_optimization_task(original_path_str).await;
            });

            // Return immediate, responsive URLs instantly to the client
            return Ok(Json(json!({
                "status": "success",
                "data": {
                    "original": relative_url,
                    "thumbnail": thumbnail_url,
                    "large": large_url
                }
            })));
        }
    }

    Err(AppError::Internal("No valid file field found in the upload request.".to_string()))
}