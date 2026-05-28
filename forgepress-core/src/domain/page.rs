// /forgepress-core/src/domain/page.rs
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Represents a single visual node or layout block in a page builder hierarchy.
/// Supports infinite nested child nodes (e.g., sections > columns > rows > widgets).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Matches the visual block type identifier (e.g., "hero_section", "button", "rich_text")
    #[serde(rename = "type")]
    pub block_type: String,
    
    /// Block-level presentation settings (e.g., padding, alignment, background_color)
    pub settings: Option<Value>,
    
    /// Content variables specific to this component (e.g., header text, image URLs)
    pub data: Option<Value>,
    
    /// Nested child layout blocks
    pub blocks: Option<Vec<Block>>,
}

impl Block {
    /// Performs a depth-first traversal to clean up or sanitize HTML block contents.
    pub fn sanitize_html_blocks(&mut self) {
        if let Some(ref mut data) = self.data {
            if let Some(text_val) = data.get_mut("text") {
                if let Some(text_str) = text_val.as_str() {
                    // Optional: Strip out malicious <script> injections here...
                    let sanitized = text_str.replace("<script>", "&lt;script&gt;");
                    *text_val = Value::String(sanitized);
                }
            }
        }
        
        // Recursively clean up nested children
        if let Some(ref mut children) = self.blocks {
            for child in children {
                child.sanitize_html_blocks();
            }
        }
    }
}

/// Helper functions to enforce safe permalink routing.
pub struct PageValidator;

impl PageValidator {
    /// Ensures slugs only contain lowercase letters, numbers, hyphens, and single slashes.
    pub fn is_valid_slug(slug: &str) -> bool {
        if slug.is_empty() || slug.starts_with('/') || slug.ends_with('/') {
            return false;
        }
        
        let mut prev_char = ' ';
        for c in slug.chars() {
            if !c.is_lowercase() && !c.is_numeric() && c != '-' && c != '/' {
                return false;
            }
            // Disallow repeating hyphens or slashes (e.g., "about--us" or "blog//post")
            if (c == '-' && prev_char == '-') || (c == '/' && prev_char == '/') {
                return false;
            }
            prev_char = c;
        }
        
        true
    }
}