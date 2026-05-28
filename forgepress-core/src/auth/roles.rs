// /forgepress-core/src/auth/roles.rs
use serde::{Deserialize, Serialize};
use std::fmt;

/// Bitwise representations of capabilities in the CMS.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Permission {
    ReadPosts = 1 << 0,
    CreatePosts = 1 << 1,
    PublishPosts = 1 << 2,
    ManagePlugins = 1 << 3,
    ManageThemes = 1 << 4,
    ManageUsers = 1 << 5,
}

/// Dynamic user roles matching the traditional CMS lifecycle.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Role {
    Administrator,
    Editor,
    Contributor,
    Subscriber,
}

impl Role {
    /// Evaluates if a given role possesses a specific capability.
    pub fn has_permission(&self, permission: Permission) -> bool {
        let mask = match self {
            Role::Administrator => u32::MAX, // Admins possess all privileges
            Role::Editor => {
                Permission::ReadPosts as u32
                    | Permission::CreatePosts as u32
                    | Permission::PublishPosts as u32
            }
            Role::Contributor => {
                Permission::ReadPosts as u32 | Permission::CreatePosts as u32
            }
            Role::Subscriber => Permission::ReadPosts as u32,
        };

        (mask & (permission as u32)) != 0
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let role_str = match self {
            Role::Administrator => "Administrator",
            Role::Editor => "Editor",
            Role::Contributor => "Contributor",
            Role::Subscriber => "Subscriber",
        };
        write!(f, "{}", role_str)
    }
}

impl std::str::FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Administrator" => Ok(Role::Administrator),
            "Editor" => Ok(Role::Editor),
            "Contributor" => Ok(Role::Contributor),
            "Subscriber" => Ok(Role::Subscriber),
            _ => Err(format!("Unknown user role: {}", s)),
        }
    }
}