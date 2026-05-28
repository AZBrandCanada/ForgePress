// /forgepress-core/src/media/optimizer.rs
use std::path::{Path, PathBuf};
use image::{ImageReader, ImageFormat};
use tracing::{info, error};
use crate::error::AppError;

/// Generates optimized, high-compression WebP variants of an image.
/// This includes a 150x150 square thumbnail and a maximum 1200px wide responsive variant.
pub fn generate_optimized_variants(original_path: &Path) -> Result<(PathBuf, PathBuf), AppError> {
    info!("Starting image optimization pipeline for: {:?}", original_path);

    // 1. Read and decode the original image (Using modern image v0.25 syntax)
    let img = ImageReader::open(original_path)
        .map_err(|e| AppError::Io(e))?
        .decode()
        .map_err(|e| AppError::Internal(format!("Failed to decode image data: {}", e)))?;

    // Prepare filename stems
    let file_stem = original_path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| AppError::Internal("Invalid file name structure.".to_string()))?;
        
    let parent_dir = original_path
        .parent()
        .ok_or_else(|| AppError::Internal("Could not identify parent directory.".to_string()))?;

    // 2. Generate the 150x150 Square Thumbnail
    let thumbnail_path = parent_dir.join(format!("{}-thumbnail.webp", file_stem));
    let thumbnail_img = img.thumbnail_exact(150, 150);
    
    thumbnail_img
        .save_with_format(&thumbnail_path, ImageFormat::WebP)
        .map_err(|e| AppError::Internal(format!("Failed to compile WebP thumbnail: {}", e)))?;

    // 3. Generate the 1200px Max-Width WebP Responsive variant
    let large_path = parent_dir.join(format!("{}-large.webp", file_stem));
    
    // Scale image down if it exceeds the 1200px limit, maintaining original aspect ratios
    let large_img = if img.width() > 1200 {
        img.resize(1200, 1200, image::imageops::FilterType::Lanczos3)
    } else {
        img.clone()
    };

    large_img
        .save_with_format(&large_path, ImageFormat::WebP)
        .map_err(|e| AppError::Internal(format!("Failed to compile large WebP variant: {}", e)))?;

    info!("Successfully optimized: {:?} and {:?}", thumbnail_path, large_path);

    Ok((thumbnail_path, large_path))
}

/// Wrapper designed to execute the optimizer pipeline within asynchronous tokio::spawn thread scopes.
pub async fn spawn_optimization_task(original_path_str: String) {
    let path = PathBuf::from(original_path_str);
    
    // Move block operation to blocking thread pool since raw image decoding is highly CPU intensive
    let task_run = tokio::task::spawn_blocking(move || {
        generate_optimized_variants(&path)
    })
    .await;

    match task_run {
        Ok(Ok((thumb, large))) => {
            info!("Background optimization task finished. Created variants: {:?} | {:?}", thumb, large);
        }
        Ok(Err(err)) => {
            error!("Background optimization failed: {:?}", err);
        }
        Err(join_err) => {
            error!("Background thread join failed: {:?}", join_err);
        }
    }
}