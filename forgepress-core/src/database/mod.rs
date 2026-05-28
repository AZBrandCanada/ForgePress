// /forgepress-core/src/database/mod.rs
pub mod connection;
pub mod pages;
pub mod users;
pub mod taxonomies;
pub mod options;

// Re-exports
pub use connection::{establish_pool, run_migrations};
pub use pages::{create_page, get_page_by_slug, update_page, delete_page, Page};
pub use users::{create_user, get_user_by_id, get_user_by_username, User};
pub use taxonomies::{create_taxonomy, link_page_to_taxonomy, get_taxonomies_for_page, Taxonomy};
pub use options::{get_option, set_option};