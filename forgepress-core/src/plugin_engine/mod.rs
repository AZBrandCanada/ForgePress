// /forgepress-core/src/plugin_engine/mod.rs
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, warn, error};
use serde::{Deserialize, Serialize};

pub mod rhai_host;
pub mod wasm_host;

#[derive(Debug, Deserialize, Clone)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub author: String,
    pub entrypoint: String,
}

#[derive(Clone)]
pub struct PluginManager {
    rhai_plugins: Arc<RwLock<HashMap<String, PathBuf>>>,
    wasm_plugins: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            rhai_plugins: Arc::new(RwLock::new(HashMap::new())),
            wasm_plugins: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn discover_and_load(&self, base_dir: &str) -> Result<(), crate::error::AppError> {
        let plugins_path = Path::new(base_dir);
        if !plugins_path.exists() {
            warn!("Plugins directory '{}' does not exist. Skipping plugin discovery.", base_dir);
            // Fixed: Wrapped the return value in a standard Result tuple
            return Ok(());
        }

        let mut dir_entries = tokio::fs::read_dir(plugins_path).await?;
        while let Some(entry) = dir_entries.next_entry().await? {
            let path = entry.path();
            if path.is_dir() {
                let manifest_path = path.join("plugin.toml");
                if manifest_path.exists() {
                    if let Err(e) = self.load_single_plugin(&path, &manifest_path).await {
                        error!("Failed to load plugin at {:?}: {}", path, e);
                    }
                }
            }
        }

        Ok(())
    }

    async fn load_single_plugin(&self, plugin_dir: &Path, manifest_path: &Path) -> Result<(), crate::error::AppError> {
        let manifest_content = tokio::fs::read_to_string(manifest_path).await?;
        let manifest: PluginManifest = toml::from_str(&manifest_content)
            .map_err(|e| crate::error::AppError::Plugin(format!("Failed to parse plugin.toml: {}", e)))?;

        let entrypoint_path = plugin_dir.join(&manifest.entrypoint);
        if !entrypoint_path.exists() {
            return Err(crate::error::AppError::Plugin(format!(
                "Configured entrypoint '{:?}' not found for plugin '{}'",
                entrypoint_path, manifest.name
            )));
        }

        if manifest.entrypoint.ends_with(".rhai") {
            let mut rhai_map = self.rhai_plugins.write().await;
            rhai_map.insert(manifest.name.clone(), entrypoint_path);
            info!("Successfully registered Rhai script plugin: {} v{}", manifest.name, manifest.version);
        } else if manifest.entrypoint.ends_with(".wasm") {
            let wasm_bytes = tokio::fs::read(&entrypoint_path).await?;
            let mut wasm_map = self.wasm_plugins.write().await;
            wasm_map.insert(manifest.name.clone(), wasm_bytes);
            info!("Successfully pre-loaded WebAssembly binary plugin: {} v{}", manifest.name, manifest.version);
        } else {
            return Err(crate::error::AppError::Plugin(format!(
                "Unsupported plugin entrypoint format: {}",
                manifest.entrypoint
            )));
        }

        Ok(())
    }

    pub async fn run_rhai_filters(&self, hook_name: &str, mut data: serde_json::Value) -> serde_json::Value {
        let rhai_map = self.rhai_plugins.read().await;
        for (plugin_name, script_path) in rhai_map.iter() {
            match rhai_host::execute_rhai_filter(script_path, hook_name, data.clone()) {
                Ok(altered_data) => {
                    data = altered_data;
                }
                Err(err) => {
                    error!("Rhai Filter '{}' failed inside plugin '{}': {:?}", hook_name, plugin_name, err);
                }
            }
        }
        data
    }
}