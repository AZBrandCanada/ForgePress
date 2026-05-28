// /forgepress-core/src/auth/passwords.rs
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use crate::error::AppError;

/// Hashes a raw, plain-text password using Argon2id.
/// Salt is generated using a cryptographically secure random number generator (OsRng).
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    
    let hashed = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(format!("Failed to compute password hash: {}", e)))?
        .to_string();
        
    Ok(hashed)
}

/// Verifies a plain-text password against a stored hash value.
pub fn verify_password(password: &str, stored_hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(stored_hash)
        .map_err(|e| AppError::Internal(format!("Invalid password hash syntax: {}", e)))?;
        
    let matches = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();
        
    Ok(matches)
}