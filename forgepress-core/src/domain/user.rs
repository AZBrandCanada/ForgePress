// /forgepress-core/src/domain/user.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::auth::Role;

/// A sanitized representation of a user suitable for public APIs.
/// Excludes sensitive authentication data such as password hashes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicUser {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl PublicUser {
    /// Validates username requirements (alphanumeric, 3-30 chars).
    pub fn is_valid_username(username: &str) -> bool {
        let len = username.len();
        if !(3..=30).contains(&len) {
            return false;
        }
        username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    }

    /// Performs basic validation of an email format.
    pub fn is_valid_email(email: &str) -> bool {
        // Minimal standard check for email formatting
        email.contains('@') && email.split('@').count() == 2
    }
}

// Convert from database row representation to sanitized domain representation
impl From<crate::database::User> for PublicUser {
    fn from(db_user: crate::database::User) -> Self {
        use std::str::FromStr;
        let role = Role::from_str(&db_user.role).unwrap_or(Role::Subscriber);

        Self {
            id: db_user.id,
            username: db_user.username,
            email: db_user.email,
            role,
            created_at: db_user.created_at,
            updated_at: db_user.updated_at,
        }
    }
}