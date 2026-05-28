// /forgepress-core/src/domain/taxonomy.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxonomyInfo {
    pub name: String,
    pub slug: String,
    pub taxonomy_type: String,
}

impl TaxonomyInfo {
    /// Utility function to sanitize and normalize text inputs into clean taxonomy slugs.
    /// Converts "My Dynamic Category!" into "my-dynamic-category".
    pub fn normalize_slug(input: &str) -> String {
        let mut slug = String::with_capacity(input.len());
        let mut prev_was_hyphen = false;

        for c in input.chars() {
            if c.is_alphanumeric() {
                slug.push(c.to_ascii_lowercase());
                prev_was_hyphen = false;
            } else if !prev_was_hyphen {
                slug.push('-');
                prev_was_hyphen = true;
            }
        }

        // Clean up trailing or leading hyphens
        let trimmed = slug.trim_matches('-');
        trimmed.to_string()
    }
}