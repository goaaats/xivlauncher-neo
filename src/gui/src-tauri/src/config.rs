use lazy_static::lazy_static;
use libxl::config::{AccountEntry, AddonEntry, LauncherConfig, LauncherSettings, UidCacheEntry};
use libxl::config_old::OldLauncherConfig;
use log::{debug, error};
use std::sync::{RwLock, RwLockWriteGuard};
use libxl::error::XlError;

lazy_static! {
  static ref CONFIG: RwLock<LauncherConfig> = RwLock::new(load_config());
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

/// Get the launcher settings
#[tauri::command]
pub fn get_settings() -> Result<LauncherSettings, XlError> {
  let config = CONFIG.read()
    .map_err(|e| XlError::new(format!("Config rwlock panic: {:?}", e)))?;

  debug!("Getting settings");
  Ok(config.settings.clone())
}

/// Get the addon list
#[tauri::command]
pub fn get_addons() -> Result<Vec<AddonEntry>, XlError> {
  let config = CONFIG.read()
    .map_err(|e| XlError::new(format!("Config rwlock panic: {:#?}", e)))?;

  debug!("Getting addons");
  Ok(config.addons.clone())
}

/// Get the account list
#[tauri::command]
pub fn get_accounts() -> Result<Vec<AccountEntry>, XlError> {
  let config = CONFIG.read()
    .map_err(|e| XlError::new(format!("Config rwlock panic: {:#?}", e)))?;

  debug!("Getting accounts");
  Ok(config.accounts.clone())
}

/// Get the UID cache
#[tauri::command]
pub fn get_uid_cache() -> Result<Vec<UidCacheEntry>, XlError> {
  let config = CONFIG.read()
    .map_err(|e| XlError::new(format!("Config rwlock panic: {:#?}", e)))?;

  debug!("Getting the UID cache");
  Ok(config.uid_cache.clone())
}

/// Update the launcher settings with new values
/// # Arguments
/// * `settings` - Launcher settings
#[tauri::command]
pub fn update_settings(settings: LauncherSettings) -> Result<(), XlError> {
  let mut config = CONFIG.write()
    .map_err(|e| XlError::new(format!("Config rwlock panic: {:#?}", e)))?;

  debug!("Updating settings");
  config.settings = settings;
  save_config(config)
}

/// Update the addon list with new values
/// # Arguments
/// * `addons` - Addons list
#[tauri::command]
pub fn update_addons(addons: Vec<AddonEntry>) -> Result<(), XlError> {
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
pub fn update_accounts(accounts: Vec<AccountEntry>) -> Result<(), XlError> {
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
pub fn update_uid_cache(uid_cache: Vec<UidCacheEntry>) -> Result<(), XlError> {
  let mut config = CONFIG
    .write()
    .map_err(|e| XlError::new(format!("Config rwlock panic: {:#?}", e)))?;

  debug!("Updating UID cache");
  config.uid_cache = uid_cache;
  save_config(config)
}

fn save_config(config: RwLockWriteGuard<LauncherConfig>) -> Result<(), XlError> {
  match config.save() {
    Ok(()) => Ok(()),
    Err(e) => {
      let msg = format!("Could not save config: {:#?}", e);
      error!("{}", msg);
      Err(XlError::new(msg))
    }
  }
}