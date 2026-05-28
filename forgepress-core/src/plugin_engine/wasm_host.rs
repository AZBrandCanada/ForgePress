// /forgepress-core/src/plugin_engine/wasm_host.rs
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};
use crate::error::AppError;

// Auto-generates type-safe Rust structures and traits dynamically
// from an inline WebAssembly Interface Type (WIT) contract definition.
wasmtime::component::bindgen!({
    inline: r#"
        package forgepress:plugin;

        interface render-filter {
            record block-data {
                block-type: string,
                settings-json: string,
                blocks-json: string,
            }

            filter-blocks: func(blocks: list<block-data>) -> list<block-data>;
        }

        world plugin-world {
            export render-filter;
        }
    "#,
    world: "plugin-world"
});

// Re-export generated types so they are accessible to core controllers
pub use forgepress::plugin::render_filter::BlockData;

/// Safely instantiates and executes a WebAssembly filter within a sandboxed runtime.
pub async fn execute_wasm_filter(
    wasm_bytes: &[u8],
    blocks: Vec<BlockData>,
) -> Result<Vec<BlockData>, AppError> {
    // 1. Configure the Wasmtime engine with the Component Model enabled
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true); // Supports non-blocking async execution

    let engine = Engine::new(&config)
        .map_err(|e| AppError::Plugin(format!("Failed to initialize Wasmtime engine: {}", e)))?;

    // 2. Compile the binary bytes into a WebAssembly Component
    let component = Component::new(&engine, wasm_bytes)
        .map_err(|e| AppError::Plugin(format!("Wasm Component Compilation Failed: {}", e)))?;

    let linker = Linker::new(&engine);
    let mut store = Store::new(&engine, ());

    // 3. Instantiate the plugin component inside the store
    let (plugin, _) = PluginWorld::instantiate_async(&mut store, &component, &linker)
        .await
        .map_err(|e| AppError::Plugin(format!("Wasm instantiation failed: {}", e)))?;

    // 4. Access the exported render-filter interface and execute
    let interface = plugin.forgepress_plugin_render_filter();
    let filtered_blocks = interface
        .call_filter_blocks(&mut store, &blocks)
        .await
        .map_err(|e| AppError::Plugin(format!("Wasm runtime execution crashed: {}", e)))?;

    Ok(filtered_blocks)
}