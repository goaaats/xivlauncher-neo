use crate::game::language::ClientLanguage;
use crate::language::LauncherLanguage;
use crate::util::path::get_launcher_config_path;
use anyhow::{Context, Error, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct LauncherConfig {
  pub settings: LauncherSettings,
  pub addons: Vec<AddonEntry>,
  pub accounts: Vec<AccountEntry>,
  pub uid_cache: Vec<UidCacheEntry>,
}

impl Display for LauncherConfig {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let repr = serde_json::to_string(self).expect("Panic");
    f.write_str(repr.as_str())
  }
}

impl LauncherConfig {
  pub fn exists() -> Result<bool> {
    let path = get_launcher_config_path()?;
    let result = path.exists();
    Ok(result)
  }

  pub fn load() -> Result<LauncherConfig> {
    let path = get_launcher_config_path()?;
    let path = path.as_path();
    if !path.exists() {
      return Err(Error::msg(format!("Config not found at {:?}", path)));
    }

    let content = fs::read_to_string(path).with_context(|| format!("Could not read {:?}", path))?;

    let config: LauncherConfig =
      serde_json::from_str(content.as_str()).with_context(|| format!("Could not deserialize {:?}", path))?;

    Ok(config)
  }

  pub fn save(&self) -> Result<()> {
    let path = get_launcher_config_path()?;
    let path = path.as_path();

    if !path.exists() {
      let path_dir = path
        .parent()
        .with_context(|| format!("Could not get parent directory of {:?}", path))?;

      fs::create_dir_all(path_dir).with_context(|| format!("Failed creating directory tree of {:?}", path))?;
    }

    let content = serde_json::to_string_pretty(self).with_context(|| "Failed to serialize configuration")?;

    fs::write(path, content).with_context(|| format!("Failed to write configuration to {:?}", path))?;

    Ok(())
  }

  pub fn default() -> LauncherConfig {
    LauncherConfig {
      settings: LauncherSettings::default(),
      addons: vec![],
      accounts: vec![],
      uid_cache: vec![],
    }
  }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LauncherSettings {
  pub game_path: String,
  pub use_dx11: bool,
  pub use_autologin: bool,
  pub enable_uid_cache: bool,
  pub extra_game_args: String,
  pub enable_dalamud: bool,
  pub enable_otp_server: bool,
  pub enable_steam_integration: bool,
  pub client_language: ClientLanguage,
  pub launcher_language: LauncherLanguage,
  pub current_account_id: String,
  pub encrypt_args: bool,
  pub patch_path: String,
  pub ask_before_patching: bool,
  pub download_speed_limit_bytes: u64,
  pub dalamud_injection_delay_ms: u64,
  pub keep_patches: bool,
  pub opt_out_mb_collection: bool,
  pub has_admin_complaints: bool,
  pub last_version: String,
  pub has_shown_auto_launch_warning: bool,
}

impl LauncherSettings {
  pub fn default() -> LauncherSettings {
    LauncherSettings {
      game_path: "".to_string(),
      use_dx11: true,
      use_autologin: false,
      enable_uid_cache: false,
      extra_game_args: "".to_string(),
      enable_dalamud: false,
      enable_otp_server: false,
      enable_steam_integration: false,
      client_language: ClientLanguage::English,
      launcher_language: LauncherLanguage::English,
      current_account_id: "".to_string(),
      encrypt_args: true,
      patch_path: "".to_string(),
      ask_before_patching: true,
      download_speed_limit_bytes: 0,
      dalamud_injection_delay_ms: 0,
      keep_patches: false,
      opt_out_mb_collection: false,
      has_admin_complaints: false,
      last_version: "".to_string(),
      has_shown_auto_launch_warning: false,
    }
  }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AddonEntry {
  pub is_enabled: bool,
  pub path: String,
  pub command_line: String,
  pub run_as_admin: bool,
  pub run_on_close: bool,
  pub kill_after_close: bool,
}

impl AddonEntry {
  pub fn default() -> AddonEntry {
    AddonEntry {
      is_enabled: true,
      path: "".to_string(),
      command_line: "".to_string(),
      run_as_admin: false,
      run_on_close: false,
      kill_after_close: false,
    }
  }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AccountEntry {
  pub character_name: String,
  pub character_world: String,
  pub thumbnail_url: String,
  pub username: String,
  pub save_password: bool,
  pub use_steam: bool,
  pub use_otp: bool,
}

impl AccountEntry {
  pub fn default() -> AccountEntry {
    AccountEntry {
      character_name: "".to_string(),
      character_world: "".to_string(),
      thumbnail_url: "".to_string(),
      username: "".to_string(),
      save_password: false,
      use_steam: false,
      use_otp: false,
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UidCacheEntry {
  pub username: String,
  pub unique_id: String,
  pub region: u32,
  pub expansion_level: u32,
  pub creation_date: DateTime<Utc>,
}

impl UidCacheEntry {
  pub fn default() -> UidCacheEntry {
    UidCacheEntry {
      username: "".to_string(),
      unique_id: "".to_string(),
      region: 0,
      expansion_level: 0,
      creation_date: Utc::now(),
    }
  }
}

#[derive(Deserialize, Debug)]
pub struct DalamudConfig {
  #[serde(rename = "DoDalamudTest")]
  pub dalamud_test: bool,
}
