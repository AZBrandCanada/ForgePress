// /forgepress-core/src/plugin_engine/wasm_host.rs
use wasmtime::component::{Component, Linker};
use wasmtime::{Config, Engine, Store};
use crate::error::AppError;

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

// Fixed: Resolved generated module namespace paths locally
pub use self::exports::forgepress::plugin::render_filter::BlockData;

pub async fn execute_wasm_filter(
    wasm_bytes: &[u8],
    blocks: Vec<BlockData>,
) -> Result<Vec<BlockData>, AppError> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.async_support(true); 

    let engine = Engine::new(&config)
        .map_err(|e| AppError::Plugin(format!("Failed to initialize Wasmtime engine: {}", e)))?;

    let component = Component::new(&engine, wasm_bytes)
        .map_err(|e| AppError::Plugin(format!("Wasm Component Compilation Failed: {}", e)))?;

    let linker = Linker::new(&engine);
    let mut store = Store::new(&engine, ());

    let (plugin, _) = PluginWorld::instantiate(&mut store, &component, &linker)
        .map_err(|e| AppError::Plugin(format!("Wasm instantiation failed: {}", e)))?;

    let interface = plugin.forgepress_plugin_render_filter();
    
    let filtered_blocks = interface
        .call_filter_blocks(&mut store, &blocks)
        .map_err(|e| AppError::Plugin(format!("Wasm runtime execution crashed: {}", e)))?;

    Ok(filtered_blocks)
}