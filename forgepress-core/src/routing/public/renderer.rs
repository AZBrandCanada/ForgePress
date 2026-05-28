// /forgepress-core/src/routing/public/renderer.rs
use std::future::Future;
use std::pin::Pin;
use minijinja::{context, Environment};
use serde_json::Value;
use tracing::{warn, error, debug};

use crate::app_state::AppState;
use crate::database::pages::Page;
use crate::domain::page::Block;
use crate::error::AppError;

pub fn compile_blocks<'a>(
    state: &'a AppState,
    blocks: &'a [Block],
) -> Pin<Box<dyn Future<Output = Result<String, AppError>> + Send + 'a>> {
    Box::pin(async move {
        let mut compiled_html = String::new();

        for mut block in blocks.iter().cloned() {
            block.sanitize_html_blocks();

            let nested_html = if let Some(ref children) = block.blocks {
                compile_blocks(state, children).await?
            } else {
                String::new()
            };

            let mut settings = block.settings.unwrap_or(Value::Null);
            let mut data = block.data.unwrap_or(Value::Null);

            settings = state.plugins.run_rhai_filters(&format!("filter_{}_settings", block.block_type), settings).await;
            data = state.plugins.run_rhai_filters(&format!("filter_{}_data", block.block_type), data).await;

            let template_name = format!("blocks/{}.html", block.block_type);
            
            let block_html = match state.template_env.get_template(&template_name) {
                Ok(template) => {
                    template.render(context! {
                        settings => settings,
                        data => data,
                        nested_content => nested_html
                    })
                    .map_err(|e| AppError::Template(e))?
                }
                Err(err) => {
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
    })
}

pub async fn render_page(state: &AppState, page: &Page) -> Result<String, AppError> {
    debug!("Starting page rendering pipeline for page: '{}'", page.title);

    // 1. Deserialize the plain database String directly into type-safe Block structs
    let blocks: Vec<Block> = serde_json::from_str(&page.content)
        .map_err(|e| AppError::Internal(format!("Failed to deserialize page content layout: {}", e)))?;

    // 2. Compile the inner block layouts recursively
    let body_content = compile_blocks(state, &blocks).await?;

    let single_template = state.template_env.get_template("single.html")
        .map_err(|e| {
            error!("Active theme is missing 'single.html' master template.");
            AppError::Template(e)
        })?;

    // Deserialize metadata schema
    let meta_value: Value = serde_json::from_str(&page.meta).unwrap_or_else(|_| serde_json::json!({}));

    let full_html = single_template.render(context! {
        title => page.title,
        slug => page.slug,
        meta => meta_value,
        body => body_content,
        // Fixed: Cloned standard String directly, completely bypassing to_rfc3339() call
        published_at => page.published_at.clone().unwrap_or_default()
    })
    .map_err(|e| AppError::Template(e))?;

    Ok(full_html)
}