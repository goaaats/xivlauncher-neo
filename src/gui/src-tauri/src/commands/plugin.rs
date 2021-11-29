use crate::lib::error::XlResult;
use anyhow::{Context, Error};
use serde::{Deserialize, Serialize};
use std::fs;

/// Get a list of plugins currently installed
#[tauri::command]
pub fn get_plugins() -> XlResult<Vec<PluginEntry>> {
  let path = crate::lib::path::get_dalamud_plugin_path()?;
  let path = path.as_path();

  let mut vec: Vec<PluginEntry> = Vec::new();

  if !path.exists() {
    return Ok(vec);
  }

  let results = fs::read_dir(path).with_context(|| format!("Could not read plugin dir: {:?}", path))?;

  for result in results {
    let result = result.with_context(|| format!("Could not read plugins dir result: {:?}", path))?;

    let plugin_path = result.path();
    let plugin_path = plugin_path.as_path();
    if !plugin_path.is_dir() {
      continue;
    }

    let plugin_name = plugin_path
      .file_name()
      .with_context(|| format!("Could not get path name: {:?}", plugin_path))?;

    let plugin_name = plugin_name
      .to_os_string()
      .into_string()
      .map_err(|_| Error::msg(format!("Could not stringify path name: {:?}", plugin_name)))?;

    let plugin_results =
      fs::read_dir(plugin_path).with_context(|| format!("Could not read plugin dir: {:?}", plugin_path))?;

    for plugin_result in plugin_results {
      let plugin_result =
        plugin_result.with_context(|| format!("Could not read plugin dir result: {:?}", plugin_path))?;

      let version_path = plugin_result.path();
      let version_path = version_path.as_path();
      if !version_path.is_dir() {
        continue;
      }

      let plugin_version = version_path
        .file_name()
        .with_context(|| format!("Could not get path name: {:?}", version_path))?;

      let plugin_version = plugin_version
        .to_os_string()
        .into_string()
        .map_err(|_| Error::msg(format!("Could not stringify plugin version: {:?}", plugin_version)))?;

      let manifest_path = version_path.join(format!("{}.json", plugin_name));
      let manifest_path = manifest_path.as_path();
      if !manifest_path.exists() {
        continue;
      }

      let manifest_json = fs::read_to_string(manifest_path)
        .with_context(|| format!("Could not read plugin manifest: {:?}", manifest_path))?;

      vec.push(PluginEntry {
        name: plugin_name.clone(),
        version: plugin_version.clone(),
        manifest_json,
      });
    }
  }

  Ok(vec)
}

/// Disable the given plugin in the installedPlugins directory
/// # Arguments
/// * `entry` - Plugin entry
#[tauri::command]
pub fn update_plugin(entry: PluginEntry) -> XlResult<()> {
  let path = crate::lib::path::get_dalamud_plugin_path()?;
  let path = path
    .join(entry.name.clone())
    .join(entry.version)
    .join(format!("{}.json", entry.name));

  let path = path.as_path();
  fs::write(path, entry.manifest_json).with_context(|| format!("Could not update plugin manifest: {:?}", path))?;

  Ok(())
}

/// Delete the given plugin from the installedPlugins directory
/// # Arguments
/// * `entry` - Plugin entry
#[tauri::command]
pub fn remove_plugin(entry: PluginEntry) -> XlResult<()> {
  let path = crate::lib::path::get_dalamud_plugin_path()?;
  let path = path.join(entry.name).join(entry.version);

  let path = path.as_path();
  fs::remove_dir_all(path).with_context(|| format!("Could not remove plugin: {:?}", path))?;

  Ok(())
}

/// Open the plugin directory in the OS file explorer
#[tauri::command]
pub fn open_dalamud_plugin_dir() -> XlResult<()> {
  let path = crate::lib::path::get_dalamud_plugin_path()?;
  let path = path.as_path();
  let path_arg = path.as_os_str();

  open::that(path_arg).with_context(|| "Could not open file browser to Dalamud plugin dir")?;

  Ok(())
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PluginEntry {
  pub name: String,
  pub version: String,
  pub manifest_json: String,
}
