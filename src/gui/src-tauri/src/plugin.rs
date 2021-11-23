use libxl::plugin::PluginEntry;
use libxl::error::{XlError, XlResult};

/// Get a list of plugins currently installed
#[tauri::command]
pub fn get_plugins() -> XlResult<Vec<PluginEntry>> {
  libxl::plugin::get_plugins()
    .map_err(|e| XlError::new(format!("Could not read plugins: {:#?}", e)))
}

/// Disable the given plugin in the installedPlugins directory
/// # Arguments
/// * `entry` - Plugin entry
#[tauri::command]
pub fn update_plugin(entry: PluginEntry) -> XlResult<()> {
  libxl::plugin::update_plugin_manifest(entry)
    .map_err(|e| XlError::new(format!("Could not update plugin manifest: {:#?}", e)))
}

/// Delete the given plugin from the installedPlugins directory
/// # Arguments
/// * `entry` - Plugin entry
#[tauri::command]
pub fn remove_plugin(entry: PluginEntry) -> XlResult<()> {
  libxl::plugin::remove_plugin(entry)
    .map_err(|e| XlError::new(format!("Could not remove plugin: {:#?}", e)))
}
