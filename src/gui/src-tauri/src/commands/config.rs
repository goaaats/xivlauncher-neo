use crate::lib::config::{AccountEntry, AddonEntry, LauncherConfig, LauncherSettings, UidCacheEntry};
use crate::lib::config_old::OldLauncherConfig;
use crate::lib::error::{XlError, XlResult};
use lazy_static::lazy_static;
use log::{debug, error};
use std::sync::{RwLock, RwLockWriteGuard};

lazy_static! {
  static ref CONFIG: RwLock<LauncherConfig> = RwLock::new(load_config());
}

/// Get the launcher settings
#[tauri::command]
pub fn get_settings() -> XlResult<LauncherSettings> {
  let config = CONFIG
    .read()
    .map_err(|e| XlError::new(format!("Config rwlock panic: {:?}", e)))?;

  debug!("Getting settings");
  Ok(config.settings.clone())
}

/// Get the addon list
#[tauri::command]
pub fn get_addons() -> XlResult<Vec<AddonEntry>> {
  let config = CONFIG
    .read()
    .map_err(|e| XlError::new(format!("Config rwlock panic: {:#?}", e)))?;

  debug!("Getting addons");
  Ok(config.addons.clone())
}

/// Get the account list
#[tauri::command]
pub fn get_accounts() -> XlResult<Vec<AccountEntry>> {
  let config = CONFIG
    .read()
    .map_err(|e| XlError::new(format!("Config rwlock panic: {:#?}", e)))?;

  debug!("Getting accounts");
  Ok(config.accounts.clone())
}

/// Get the UID cache
#[tauri::command]
pub fn get_uid_cache() -> XlResult<Vec<UidCacheEntry>> {
  let config = CONFIG
    .read()
    .map_err(|e| XlError::new(format!("Config rwlock panic: {:#?}", e)))?;

  debug!("Getting UID cache");
  Ok(config.uid_cache.clone())
}

/// Update the launcher settings with new values
/// # Arguments
/// * `settings` - Launcher settings
#[tauri::command]
pub fn update_settings(settings: LauncherSettings) -> XlResult<()> {
  let mut config = CONFIG
    .write()
    .map_err(|e| XlError::new(format!("Config rwlock panic: {:#?}", e)))?;

  debug!("Updating settings");
  config.settings = settings;
  save_config(config)
}

/// Update the addon list with new values
/// # Arguments
/// * `addons` - Addons list
#[tauri::command]
pub fn update_addons(addons: Vec<AddonEntry>) -> XlResult<()> {
  let mut config = CONFIG
    .write()
    .map_err(|e| XlError::new(format!("Config rwlock panic: {:#?}", e)))?;

  debug!("Updating addons");
  config.addons = addons;
  save_config(config)
}

/// Update the account list with new values
/// # Arguments
/// * `accounts` - Accounts list
#[tauri::command]
pub fn update_accounts(accounts: Vec<AccountEntry>) -> XlResult<()> {
  let mut config = CONFIG
    .write()
    .map_err(|e| XlError::new(format!("Config rwlock panic: {:#?}", e)))?;

  debug!("Updating accounts");
  config.accounts = accounts;
  save_config(config)
}

/// Update the UID cache list with new values
/// # Arguments
/// * `uid_cache` - UID cache list
#[tauri::command]
pub fn update_uid_cache(uid_cache: Vec<UidCacheEntry>) -> XlResult<()> {
  let mut config = CONFIG
    .write()
    .map_err(|e| XlError::new(format!("Config rwlock panic: {:#?}", e)))?;

  debug!("Updating UID cache");
  config.uid_cache = uid_cache;
  save_config(config)
}

fn load_config() -> LauncherConfig {
  match LauncherConfig::load() {
    Ok(config) => {
      debug!("Loaded config");
      return config;
    }
    Err(e) => {
      debug!("Could not load config: {:#?}", e);
    }
  }

  debug!("Attempting to upgrade old config");
  let old = match OldLauncherConfig::load() {
    Err(e) => {
      debug!("Could not load old config: {:#?}", e);
      debug!("Using default config");
      return LauncherConfig::default();
    }
    Ok(old) => {
      debug!("Loaded old config");
      old
    }
  };

  debug!("Attempting upgrade");
  match old.upgrade() {
    Ok(config) => {
      debug!("Config upgrade successful");
      config
    }
    Err(e) => {
      debug!("Upgrade failed: {:#?}", e);
      debug!("Using default config");
      LauncherConfig::default()
    }
  }
}

fn save_config(config: RwLockWriteGuard<LauncherConfig>) -> XlResult<()> {
  match config.save() {
    Ok(()) => Ok(()),
    Err(e) => {
      let msg = format!("Could not save config: {:#?}", e);
      error!("{}", msg);
      Err(XlError::new(msg))
    }
  }
}
