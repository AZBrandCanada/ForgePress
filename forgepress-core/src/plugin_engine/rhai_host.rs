// /forgepress-core/src/plugin_engine/rhai_host.rs
use rhai::{Engine, Scope};
use serde_json::Value;
use std::path::Path;
use tracing::{debug, warn};
use crate::error::AppError;

/// Safely executes a registered Rhai script filter function.
pub fn execute_rhai_filter(
    script_path: &Path,
    function_name: &str,
    data: Value,
) -> Result<Value, AppError> {
    debug!("Executing Rhai filter '{}' from {:?}", function_name, script_path);

    // 1. Initialize a secure, restricted Rhai engine
    let mut engine = Engine::new();

    // Prevent scripts from infinitely loops or exhausting server threads
    engine.set_max_operations(100_000); 

    // Register safe standard utilities inside the script scope
    engine.register_fn("log_info", |msg: &str| {
        debug!("Plugin Log: {}", msg);
    });
    engine.register_fn("log_warn", |msg: &str| {
        warn!("Plugin Warning: {}", msg);
    });

    // 2. Compile the script from disk
    let ast = engine
        .compile_file(script_path.to_path_buf())
        .map_err(|e| AppError::Plugin(format!("Failed to compile Rhai script: {}", e)))?;

    // 3. Setup scope and invoke the function passing standard serde JSON payloads
    let mut scope = Scope::new();
    let result_value: Value = engine
        .call_fn(&mut scope, &ast, function_name, (data,))
        .map_err(|e| AppError::Plugin(format!("Rhai execution runtime error: {}", e)))?;

    Ok(result_value)
}