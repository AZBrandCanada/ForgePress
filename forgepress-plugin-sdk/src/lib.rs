// /forgepress-plugin-sdk/src/lib.rs

// 1. Auto-generate the low-level Wasm Component guest bindings
wit_bindgen::generate!({
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

// Re-export low-level bindings if needed
use exports::forgepress::plugin::render_filter::BlockData;

/// An idiomatic, developer-friendly representation of a page block.
/// It wraps the raw JSON string variables into safe, easy-to-use serde_json values.
#[derive(Debug, Clone)]
pub struct RichBlock {
    pub block_type: String,
    pub settings: serde_json::Value,
    pub blocks: serde_json::Value,
}

/// The core trait that third-party developers must implement to write a ForgePress Wasm filter.
pub trait ForgePressFilter {
    fn filter_blocks(&self, blocks: Vec<RichBlock>) -> Vec<RichBlock>;
}

/// Low-level helper to convert WIT bindings to rich SDK structures.
fn to_rich_blocks(raw_blocks: Vec<BlockData>) -> Vec<RichBlock> {
    raw_blocks
        .into_iter()
        .map(|raw| {
            let settings = serde_json::from_str(&raw.settings_json)
                .unwrap_or(serde_json::Value::Null);
            let blocks = serde_json::from_str(&raw.blocks_json)
                .unwrap_or(serde_json::Value::Null);

            RichBlock {
                block_type: raw.block_type,
                settings,
                blocks,
            }
        })
        .collect()
}

/// Low-level helper to convert rich structures back into raw WIT bindings.
fn to_raw_blocks(rich_blocks: Vec<RichBlock>) -> Vec<BlockData> {
    rich_blocks
        .into_iter()
        .map(|rich| {
            let settings_json = serde_json::to_string(&rich.settings).unwrap_or_else(|_| "{}".to_string());
            let blocks_json = serde_json::to_string(&rich.blocks).unwrap_or_else(|_| "[]".to_string());

            BlockData {
                block_type: rich.block_type,
                settings_json,
                blocks_json,
            }
        })
        .collect()
}

/// Central macros to register and export your custom plugin to the WebAssembly runtime.
/// 
/// # Example
/// ```rust
/// struct MyPlugin;
/// impl ForgePressFilter for MyPlugin {
///     fn filter_blocks(&self, mut blocks: Vec<RichBlock>) -> Vec<RichBlock> {
///         // Custom logic here...
///         blocks
///     }
/// }
/// forgepress_export!(MyPlugin);
/// ```
#[macro_export]
macro_rules! forgepress_export {
    ($plugin_struct:ty) => {
        struct Component;

        impl $crate::exports::forgepress::plugin::render_filter::Guest for Component {
            fn filter_blocks(
                raw_blocks: Vec<$crate::exports::forgepress::plugin::render_filter::BlockData>,
            ) -> Vec<$crate::exports::forgepress::plugin::render_filter::BlockData> {
                use $crate::ForgePressFilter;
                
                // 1. Convert WIT structs to developer-friendly RichBlocks
                let rich_blocks = $crate::to_rich_blocks(raw_blocks);
                
                // 2. Instantiate and execute the user's plugin logic
                let plugin = <$plugin_struct>::default();
                let filtered_rich_blocks = plugin.filter_blocks(rich_blocks);
                
                // 3. Convert back to raw WIT structs for host delivery
                $crate::to_raw_blocks(filtered_rich_blocks)
            }
        }

        // Export generated bindings
        $crate::export!(Component);
    };
}

// Re-export the structural translator utilities required by the macro
#[doc(hidden)]
pub use to_raw_blocks;
#[doc(hidden)]
pub use to_rich_blocks;