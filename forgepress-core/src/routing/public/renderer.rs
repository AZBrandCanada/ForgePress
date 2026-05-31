use std::future::Future;
use std::pin::Pin;
use minijinja::context;
use serde_json::Value;
use tracing::{warn, error, debug};

use crate::app_state::AppState;
use crate::database::pages::Page;
use crate::domain::page::Block;
use crate::error::AppError;

/// Compiles page blocks recursively. (Untouched)
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

/// Post-processing helper that optimizes, minifies, and packages the rendered page.
fn optimize_and_minify_html(raw_html: &str) -> String {
    // 1. Universal Image loading optimization:
    // Automatically inject native lazy loading and asynchronous image decoding.
    let optimized_html = raw_html
        .replace("<img ", "<img loading=\"lazy\" decoding=\"async\" ");

    // 2. Minifier: Collapse layout whitespace into a single-line flow, 
    // while ensuring structural comments remain on their own lines.
    let mut minified = String::new();
    let mut in_preserved_block = false;

    for line in optimized_html.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Prevent squishing of preformatted code or text blocks where whitespace matters
        if trimmed.contains("<pre") || trimmed.contains("<code") || trimmed.contains("<script") {
            in_preserved_block = true;
        }
        if trimmed.contains("</pre>") || trimmed.contains("</code>") || trimmed.contains("</script>") {
            in_preserved_block = false;
        }

        if in_preserved_block {
            minified.push_str(line);
            minified.push('\n');
        } else if trimmed.starts_with("<!--") && trimmed.ends_with("-->") {
            // Isolate structural template comments on their own line
            if !minified.is_empty() && !minified.ends_with('\n') {
                if minified.ends_with(' ') {
                    minified.pop(); // Cleanly remove the trailing space from the previous element
                }
                minified.push('\n');
            }
            minified.push_str(trimmed);
            minified.push('\n');
        } else {
            // Collapse standard tags and elements into a spaced layout flow
            minified.push_str(trimmed);
            minified.push(' ');
        }
    }

    // 3. Inject only your clean production header signature
    let header_signature = format!(
        "<!--\n  ⚡ Rendered by ForgePress Engine\n  ⚡ Native Asset Optimization & Minification Applied\n-->\n"
    );

    format!("{}{}", header_signature, minified.trim())
}
/// Renders the page using templates and returns the optimized, production-ready HTML markup.
pub async fn render_page(state: &AppState, page: &Page) -> Result<String, AppError> {
    debug!("Starting page rendering pipeline for page: '{}'", page.title);

    let blocks: Vec<Block> = serde_json::from_str(&page.content)
        .map_err(|e| AppError::Internal(format!("Failed to deserialize page content layout: {}", e)))?;

    let body_content = compile_blocks(state, &blocks).await?;

    let single_template = state.template_env.get_template("single.html")
        .map_err(|e| {
            error!("Active theme is missing 'single.html' master template.");
            AppError::Template(e)
        })?;

    let meta_value: Value = serde_json::from_str(&page.meta).unwrap_or_else(|_| serde_json::json!({}));

    let full_html = single_template.render(context! {
        title => page.title,
        slug => page.slug,
        meta => meta_value,
        body => body_content,
        published_at => if page.published_at.is_empty() { "".to_string() } else { page.published_at.clone() }
    })
    .map_err(|e| AppError::Template(e))?;

    // Optimize the HTML before returning it (compacting layout, lazy-loading media, adding meta header)
    let optimized_output = optimize_and_minify_html(&full_html);

    Ok(optimized_output)
}