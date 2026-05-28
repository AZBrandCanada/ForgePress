// /forgepress-core/src/media/upload.rs
use std::path::{Path, PathBuf};
use chrono::Utc;
use tokio::fs::{create_dir_all, File};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use crate::error::AppError;

/// Binary signatures (magic bytes) of allowed web image formats.
const MAGIC_JPEG: &[u8] = &[0xFF, 0xD8, 0xFF];
const MAGIC_PNG: &[u8] = &[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A];
const MAGIC_WEBP: &[u8] = &[0x52, 0x49, 0x46, 0x46]; // "RIFF" header prefix

/// Validates that the file binary payload matches its asserted image format.
/// This prevents malicious execution exploits disguised behind a .jpg extension.
pub fn validate_image_header(data: &[u8]) -> Result<(), AppError> {
    if data.len() < 12 {
        return Err(AppError::Internal("File payload is too small to analyze.".to_string()));
    }

    if data.starts_with(MAGIC_JPEG) {
        return Ok(());
    }
    if data.starts_with(MAGIC_PNG) {
        return Ok(());
    }
    // WebP checks for RIFF at the start and "WEBP" at bytes 8-11
    if data.starts_with(MAGIC_WEBP) && &data[8..12] == b"WEBP" {
        return Ok(());
    }

    Err(AppError::Internal("Security Validation Failed: File signature does not match allowed image formats (JPEG/PNG/WebP).".to_string()))
}

/// Generates a standardized nested upload folder path: /uploads/YYYY/MM/
pub async fn prepare_upload_directory(base_upload_dir: &str) -> Result<PathBuf, AppError> {
    let now = Utc::now();
    let year = now.format("%Y").to_string();
    let month = now.format("%m").to_string();

    let target_path = Path::new(base_upload_dir)
        .join(year)
        .join(month);

    create_dir_all(&target_path).await?;
    Ok(target_path)
}

/// Saves the original uploaded file payload asynchronously.
pub async fn save_original_upload(
    base_upload_dir: &str,
    filename: &str,
    data: &[u8],
) -> Result<PathBuf, AppError> {
    // 1. Verify binary magic bytes
    validate_image_header(data)?;

    // 2. Prepare dynamic directory structures
    let target_dir = prepare_upload_directory(base_upload_dir).await?;

    // 3. Create a unique, sanitized filename
    let extension = Path::new(filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("jpg");

    let unique_filename = format!("{}.{}", Uuid::new_v4(), extension);
    let absolute_path = target_dir.join(&unique_filename);

    // 4. Save file to disk
    let mut file = File::create(&absolute_path).await?;
    file.write_all(data).await?;

    Ok(absolute_path)
}