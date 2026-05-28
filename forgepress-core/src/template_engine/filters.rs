// /forgepress-core/src/template_engine/filters.rs
use minijinja::Error;
use chrono::{DateTime, Utc};
use serde_json::Value;

/// MiniJinja Custom Filter: Formats raw UTC timestamps into clean, human-readable date strings.
pub fn format_date(value: String) -> Result<String, Error> {
    if value.is_empty() {
        return Ok("Draft".to_string());
    }

    match DateTime::parse_from_rfc3339(&value) {
        Ok(dt) => {
            let utc_dt: DateTime<Utc> = dt.with_timezone(&Utc);
            Ok(utc_dt.format("%B %d, %Y").to_string())
        }
        Err(_) => {
            // Fallback to standard string representation if parsing fails
            Ok(value)
        }
    }
}

/// MiniJinja Custom Filter: Formats a raw JSON object into a pretty-printed, indented string.
/// Extremely useful for development debugging in template files (e.g., `{{ block | pretty_json }}`).
pub fn pretty_json(value: Value) -> Result<String, Error> {
    serde_json::to_string_pretty(&value)
        .map_err(|e| Error::new(minijinja::ErrorKind::InvalidOperation, e.to_string()))
}