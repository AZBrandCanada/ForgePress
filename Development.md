Using your new SDK, a third-party developer can build a compiled Wasm plugin with very minimal code. Here is a complete walkthrough of how they would do it:

#### Developer’s `/Cargo.toml`
```toml
[package]
name = "censor-plugin"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
# Import your newly built SDK directly!
forgepress-plugin-sdk = { path = "../forgepress-plugin-sdk" }
```

#### Developer’s `/src/lib.rs`
```rust
use forgepress_plugin_sdk::{forgepress_export, ForgePressFilter, RichBlock};

#[derive(Default)]
struct CensorPlugin;

impl ForgePressFilter for CensorPlugin {
    fn filter_blocks(&self, mut blocks: Vec<RichBlock>) -> Vec<RichBlock> {
        for block in &mut blocks {
            if block.block_type == "rich_text" {
                // If the block contains data text, replace targeted words
                if let Some(text) = block.blocks.get_mut("text") {
                    if let Some(text_str) = text.as_str() {
                        let sanitized = text_str.replace("banned_word", "*******");
                        *text = serde_json::Value::String(sanitized);
                    }
                }
            }
        }
        blocks
    }
}

// Register and export the plugin securely using the SDK's macro!
forgepress_export!(CensorPlugin);
```

To compile this, the developer simply runs `cargo build --target wasm32-wasip2 --release` and drops the resulting `.wasm` file into their `content/plugins/` directory. Your host server will load it, parse it via `wasmtime`, and execute it seamlessly.