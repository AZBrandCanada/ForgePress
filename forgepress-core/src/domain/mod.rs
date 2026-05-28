// /forgepress-core/src/domain/mod.rs
pub mod user;
pub mod page;
pub mod taxonomy;

// Re-exports
pub use page::{Block, PageValidator};
pub use taxonomy::TaxonomyInfo;
pub use user::PublicUser;