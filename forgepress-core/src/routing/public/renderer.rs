// /forgepress-core/src/routing/public/renderer.rs
use minijinja::{context, Environment};
use serde_json::Value;
use tracing::{warn, error};

use crate::database::pages::Page;
use crate::domain::page::Block;
use crate::error::AppError;

/// Compiles a vector of layout blocks recursively into an HTML string.
pub fn compile_blocks(env: &Environment, blocks: &[Block]) -> Result<String, AppError> {
    let mut compiled_html = String::new();

    for mut block in blocks.iter().cloned() {
        // Run input sanitization to filter malicious inline script injections
        block.sanitize_html_blocks();

        // 1. If this block has nested child blocks, compile them recursively first
        let nested_html = if let Some(ref children) = block.blocks {
            compile_blocks(env, children)?
        } else {
            String::new()
        };

        // 2. Locate the MiniJinja template corresponding to this block type (e.g., blocks/hero_section.html)
        let template_name = format!("blocks/{}.html", block.block_type);
        
        let block_html = match env.get_template(&template_name) {
            Ok(template) => {
                // Pass the block's settings, data, and recursively compiled nested children into the context
                template.render(context! {
                    settings => block.settings.unwrap_or(Value::Null),
                    data => block.data.unwrap_or(Value::Null),
                    nested_content => nested_html
                })
                .map_err(|e| AppError::Template(e))?
            }
            Err(err) => {
                // EDGE CASE FALLBACK: If the theme is missing a template for a custom plugin block,
                // log a warning and render a safe fallback container instead of returning a 500 Server Error.
                warn!("Missing block template '{}'. Error: {:?}", template_name, err);
                format!(
                    "<div class=\"forge-block-fallback\" data-block-type=\"{}\">{}</div>",
                    block.block_type, nested_html
                )
            }
        };

        compiled_html.push_str(&block_html);
    }

    Ok(compiled_html)
}

/// Entry point to render a full page using the master layout files.
pub fn render_page(env: &Environment, page: &Page) -> Result<String, AppError> {
    // 1. Deserialize the dynamic JSONB page structure into type-safe Block structs
    let blocks: Vec<Block> = serde_json::from_value(page.content.0.clone())
        .map_err(|e| AppError::Internal(format!("Failed to deserialize page content layout: {}", e)))?;

    // 2. Compile the inner block layouts recursively
    let body_content = compile_blocks(env, &blocks)?;

    // 3. Render the main page shell (single.html), injecting metadata and compiled body
    let single_template = env.get_template("single.html")
        .map_err(|e| {
            error!("Active theme is missing 'single.html' master template.");
            AppError::Template(e)
        })?;

    let full_html = single_template.render(context! {
        title => page.title,
        slug => page.slug,
        meta => page.meta.0,
        body => body_content
    })
    .map_err(|e| AppError::Template(e))?;

    Ok(full_html)
}