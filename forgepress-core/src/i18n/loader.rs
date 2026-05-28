// /forgepress-core/src/i18n/loader.rs
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn};

use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};
use crate::error::AppError;

/// Custom Axum extractor that automatically parses and resolves the visitor's preferred locale.
pub struct Locale(pub String);

#[axum::async_trait]
impl<S> FromRequestParts<S> for Locale
where
    S: Send + Sync,
{
    type Rejection = AppError;

    // Fixed: Changed parts parameter to a mutable reference (&mut Parts) to match Axum v0.7
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 1. Inspect URL Query String for manually defined overrides (e.g., ?lang=es)
        let query_lang = parts.uri.query()
            .and_then(|q| q.split('&').find(|param| param.starts_with("lang=")))
            .map(|param| param[5..].to_string());

        if let Some(lang) = query_lang {
            return Ok(Locale(lang));
        }

        // 2. Parse HTTP Accept-Language Header (e.g., "en-US,en;q=0.9,es;q=0.8")
        let accept_lang = parts.headers
            .get(axum::http::header::ACCEPT_LANGUAGE)
            .and_then(|h| h.to_str().ok())
            .unwrap_or("en"); // Fallback to standard English default if missing

        let best_match = parse_accept_language(accept_lang);
        Ok(Locale(best_match))
    }
}

/// Helper function to parse Accept-Language header, resolving to the primary locale (e.g., "en")
fn parse_accept_language(header: &str) -> String {
    header
        .split(',')
        .filter_map(|segment| {
            let mut parts = segment.split(';');
            let locale = parts.next()?.trim();
            // Split regional codes (e.g., "en-US" becomes "en")
            let primary_lang = locale.split('-').next()?.to_string();
            Some(primary_lang)
        })
        .next()
        .unwrap_or_else(|| "en".to_string())
}

/// Manages multi-language dictionaries in memory.
#[derive(Clone)]
pub struct I18nLoader {
    /// Maps Locale (e.g., "es") -> Dictionary Keys (e.g., "login_failed" -> "Credenciales inválidas")
    translations: Arc<RwLock<HashMap<String, HashMap<String, String>>>>,
    default_locale: String,
}

impl I18nLoader {
    pub fn new(default_locale: &str) -> Self {
        Self {
            translations: Arc::new(RwLock::new(HashMap::new())),
            default_locale: default_locale.to_string(),
        }
    }

    /// Recursively loads JSON files from content/languages/ directly into memory.
    pub async fn discover_and_load(&self, languages_dir: &str) -> Result<(), AppError> {
        let dir_path = Path::new(languages_dir);
        if !dir_path.exists() {
            warn!("Languages directory '{}' does not exist. Creating path...", languages_dir);
            tokio::fs::create_dir_all(dir_path).await?;
            // Generate a default English dictionary
            let default_en = dir_path.join("en.json");
            tokio::fs::write(&default_en, r#"{"login_failed": "Invalid credentials.", "not_found": "Page not found."}"#).await?;
        }

        let mut dir_entries = tokio::fs::read_dir(dir_path).await?;
        let mut map = self.translations.write().await;

        while let Some(entry) = dir_entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("json") {
                if let Some(locale) = path.file_stem().and_then(|stem| stem.to_str()) {
                    let content = tokio::fs::read_to_string(&path).await?;
                    let dictionary: HashMap<String, String> = serde_json::from_str(&content)
                        .map_err(|e| AppError::Internal(format!("Failed to parse translation file at {:?}: {}", path, e)))?;
                    
                    map.insert(locale.to_string(), dictionary);
                    info!("Successfully loaded dictionary locale: '{}'", locale);
                }
            }
        }

        Ok(())
    }

    /// High-performance translation lookup.
    /// Safely falls back to the default system locale if the key is missing from the requested language.
    pub async fn translate(&self, locale: &str, key: &str) -> String {
        let map = self.translations.read().await;

        // Try 1: Find translation inside requested language (e.g., "es")
        if let Some(dict) = map.get(locale) {
            if let Some(val) = dict.get(key) {
                return val.clone();
            }
        }

        // Try 2: Fallback to the default system language dictionary (e.g., "en")
        if let Some(dict) = map.get(&self.default_locale) {
            if let Some(val) = dict.get(key) {
                return val.clone();
            }
        }

        // Try 3: Return raw key if translations are completely missing
        key.to_string()
    }
}