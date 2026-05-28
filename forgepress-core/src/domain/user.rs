// /forgepress-core/src/domain/user.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::auth::Role;

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
    pub fn is_valid_username(username: &str) -> bool {
        let len = username.len();
        if !(3..=30).contains(&len) {
            return false;
        }
        username.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    }

    pub fn is_valid_email(email: &str) -> bool {
        email.contains('@') && email.split('@').count() == 2
    }
}

impl From<crate::database::User> for PublicUser {
    fn from(db_user: crate::database::User) -> Self {
        use std::str::FromStr;
        let role = Role::from_str(&db_user.role).unwrap_or(Role::Subscriber);

        // Parse UTC strings back into type-safe Chrono DateTime
        let created_at = DateTime::parse_from_rfc3339(&db_user.created_at)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        let updated_at = DateTime::parse_from_rfc3339(&db_user.updated_at)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        Self {
            // Parse SQLite/Postgres TEXT into Uuid
            id: Uuid::from_str(&db_user.id).unwrap_or_default(),
            username: db_user.username,
            email: db_user.email,
            role,
            created_at,
            updated_at,
        }
    }
}