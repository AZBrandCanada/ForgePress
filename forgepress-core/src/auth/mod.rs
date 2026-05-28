// /forgepress-core/src/auth/mod.rs
pub mod roles;
pub mod passwords;
pub mod middleware;

// Re-exports for cleaner imports across the application
pub use middleware::{require_auth, require_role_permission, Claims};
pub use passwords::{hash_password, verify_password};
pub use roles::{Permission, Role};