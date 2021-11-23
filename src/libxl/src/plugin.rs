use std::fs;
use std::fs::read_dir;
use anyhow::{Context, Error, Result};
use serde::{Deserialize, Serialize};
use crate::util::path::get_config_data_path;

/// Get a list of plugins currently installed
pub fn get_plugins() -> Result<Vec<PluginEntry>> {
  let path = get_config_data_path()?;

  let mut vec: Vec<PluginEntry> = Vec::new();

  let path = path.join("installedPlugins");
  let path = path.as_path();

  if !path.exists() {
    return Ok(vec);
  }

  let results = read_dir(path)
    .with_context(|| format!("Could not read plugin dir: {:?}", path))?;

  for result in results {
    let result = result
      .with_context(|| format!("Could not read plugins dir result: {:?}", path))?;

    let plugin_path = result.path();
    let plugin_path = plugin_path.as_path();
    if !plugin_path.is_dir() {
      continue;
    }

    let plugin_name = plugin_path.file_name()
      .with_context(|| format!("Could not get path name: {:?}", plugin_path))?;

    let plugin_name = plugin_name.to_os_string().into_string()
      .map_err(|_| Error::msg(format!("Could not stringify path name: {:?}", plugin_name)))?;

    let plugin_results = read_dir(plugin_path)
      .with_context(|| format!("Could not read plugin dir: {:?}", plugin_path))?;

    for plugin_result in plugin_results {
      let plugin_result = plugin_result
        .with_context(|| format!("Could not read plugin dir result: {:?}", plugin_path))?;

      let version_path = plugin_result.path();
      let version_path = version_path.as_path();
      if !version_path.is_dir() {
        continue;
      }

      let plugin_version = version_path.file_name()
        .with_context(|| format!("Could not get path name: {:?}", version_path))?;

      let plugin_version = plugin_version.to_os_string().into_string()
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
        manifest: manifest_json,
      });
    }
  }

  Ok(vec)
}

/// Update a plugin manifest in the installedPlugins directory
/// We don't use ser/de here, so as to not need to map out the Dalamud plugin manifest
/// Let the frontend update the "disabled" mapping as an anonymous map
/// # Arguments
/// * `entry` - Plugin entry
pub fn update_plugin_manifest(entry: PluginEntry) -> Result<()> {
  let path = get_config_data_path()?;
  let path = path
    .join("installedPlugins")
    .join(entry.name.clone())
    .join(entry.version)
    .join(format!("{}.json", entry.name));

  let path = path.as_path();
  fs::write(path, entry.manifest)
    .with_context(|| format!("Could not update plugin manifest: {:?}", path))?;

  Ok(())
}

/// Delete the given plugin from the installedPlugins directory
/// # Arguments
/// * `entry` - Plugin entry
pub fn remove_plugin(entry: PluginEntry) -> Result<()> {
  let path = get_config_data_path()?;
  let path = path
    .join("installedPlugins")
    .join(entry.name)
    .join(entry.version);

  let path = path.as_path();
  fs::remove_dir_all(path)
    .with_context(|| format!("Could not remove plugin: {:?}", path))?;

  Ok(())
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PluginEntry {
  pub name: String,
  pub version: String,
  pub manifest: String,
}