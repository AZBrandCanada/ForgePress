// /forgepress-core/src/template_engine/mod.rs (Simplified)
use minijinja::Environment;
use crate::error::AppError;

pub mod filters;

/// Instantiates and configures a clean MiniJinja Environment with custom template filters.
pub fn create_environment() -> Result<Environment<'static>, AppError> {
    let mut env = Environment::new();
    
    // Set standard relative file loader
    env.set_loader(minijinja::path_loader("content/themes/default/templates"));
    
    // Register custom template filters
    env.add_filter("format_date", filters::format_date);
    env.add_filter("pretty_json", filters::pretty_json);

    Ok(env)
}