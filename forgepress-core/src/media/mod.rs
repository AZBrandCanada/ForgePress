// /forgepress-core/src/media/mod.rs
pub mod upload;
pub mod optimizer;

// Re-exports
pub use optimizer::spawn_optimization_task;
pub use upload::{save_original_upload, validate_image_header};