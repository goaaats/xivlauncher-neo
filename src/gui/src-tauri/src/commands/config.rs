use crate::lib::config::{DalamudConfig, LauncherConfig};
use crate::lib::config_old::OldLauncherConfig;
use crate::lib::error::{XlError, XlResult};
use log::debug;

/// Get the launcher configuration
#[tauri::command]
pub fn get_config() -> XlResult<LauncherConfig> {
  debug!("Loading configuration");

  let err = match LauncherConfig::load() {
    Ok(config) => {
      debug!("Loaded config");
      return Ok(config);
    }
    Err(e) => e,
  };

  debug!("Could not load config, loading previous version: {:#?}", err);
  let old = match OldLauncherConfig::load() {
    Err(e) => {
      debug!("Could not load old config, using default: {:#?}", e);
      return Ok(LauncherConfig::default());
    }
    Ok(old) => old,
  };

  debug!("Loaded old config, attempting upgrade");
  match old.upgrade() {
    Ok(config) => {
      debug!("Config upgrade successful");
      Ok(config)
    }
    Err(e) => {
      debug!("Upgrade failed, using default: {:#?}", e);
      Ok(LauncherConfig::default())
    }
  }
}

/// Save the launcher configuration
/// # Arguments
/// * `config` - Launcher config
#[tauri::command]
pub fn save_config(config: LauncherConfig) -> XlResult<()> {
  debug!("Saving configuration");
  config.save()
    .map_err(|e| XlError::from(e))
}

/// Get if Dalamud testing is enabled
#[tauri::command]
pub fn is_dalamud_testing() -> XlResult<bool> {
  DalamudConfig::load()
    .and_then(|config| Ok(config.testing_enabled))
    .map_err(|e| XlError::from(e))
}
