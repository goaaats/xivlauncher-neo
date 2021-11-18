use crate::game::language::ClientLanguage;
use crate::language::LauncherLanguage;
use crate::util::path::{get_config_data_path, get_launcher_old_accounts_path, get_launcher_old_config_path, get_launcher_old_uid_cache_path};
use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::fs;
use crate::config::{AccountEntry, AddonEntry, LauncherConfigV4, LauncherSettings};

#[derive(Serialize, Deserialize, Debug)]
pub struct OldLauncherConfig {
  pub settings: OldLauncherSettings,
  pub addons: Vec<OldAddonEntry>,
  pub accounts: Vec<OldAccountEntry>,
}

impl Display for OldLauncherConfig {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let repr = serde_json::to_string(self).expect("Panic");
    f.write_str(repr.as_str())
  }
}

impl OldLauncherConfig {
  pub fn load() -> Result<OldLauncherConfig, Error> {
    // Settings
    let path = get_launcher_old_config_path()?;
    let content = fs::read_to_string(path)?;
    let settings: OldLauncherSettings = serde_json::from_str(content.as_str()).expect("Panic (Settings)");

    // Addons
    let addon_json = settings.addons_json.clone().unwrap_or_else(|| String::from("[]"));
    let addons = serde_json::from_str(addon_json.as_str()).expect("Panic (Addons)");

    // Accounts
    let path =get_launcher_old_accounts_path()?;
    let content = fs::read_to_string(path)?;
    let accounts: Vec<OldAccountEntry> = serde_json::from_str(content.as_str()).expect("Panic (Accounts)");

    let config = OldLauncherConfig { settings, addons, accounts };
    return Ok(config);
  }

  pub fn upgrade(&self) -> Result<LauncherConfigV4, Error> {
    // Settings
    let default_settings = LauncherSettings::default();
    let settings = LauncherSettings {
      game_path: conv_str(self.settings.game_path.clone(), default_settings.game_path),
      use_dx11: conv_bool(self.settings.use_dx11.clone(), default_settings.use_dx11),
      use_autologin: conv_bool(self.settings.use_autologin.clone(), default_settings.use_autologin),
      enable_uid_cache: conv_bool(self.settings.enable_uid_cache.clone(), default_settings.enable_uid_cache),
      extra_game_args: conv_str(self.settings.extra_game_args.clone(), default_settings.extra_game_args),
      enable_dalamud: conv_bool(self.settings.enable_dalamud.clone(), default_settings.enable_dalamud),
      enable_otp_server: conv_bool(self.settings.enable_otp_server.clone(), default_settings.enable_otp_server),
      enable_steam_integration: conv_bool(self.settings.enable_steam_integration.clone(), default_settings.enable_steam_integration),
      client_language: self.settings.client_language.clone().unwrap_or_else(|| ClientLanguage::English),
      launcher_language: self.settings.launcher_language.clone().unwrap_or_else(|| LauncherLanguage::English),
      current_account_id: conv_str(self.settings.current_account_id.clone(), default_settings.current_account_id),
      encrypt_args: conv_bool(self.settings.encrypt_args.clone(), default_settings.encrypt_args),
      patch_path: conv_str(self.settings.patch_path.clone(), default_settings.patch_path),
      ask_before_patching: conv_bool(self.settings.ask_before_patching.clone(), default_settings.ask_before_patching),
      download_speed_limit_bytes: conv_t(self.settings.download_speed_limit_bytes.clone(), default_settings.download_speed_limit_bytes),
      dalamud_injection_delay_ms: conv_t(self.settings.dalamud_injection_delay_ms.clone(), default_settings.dalamud_injection_delay_ms),
      keep_patches: conv_bool(self.settings.keep_patches.clone(), default_settings.keep_patches),
      opt_out_mb_collection: conv_bool(self.settings.opt_out_mb_collection.clone(), default_settings.opt_out_mb_collection),
      has_admin_complaints: conv_bool(self.settings.has_admin_complaints.clone(), default_settings.has_admin_complaints),
      last_version: conv_str(self.settings.last_version.clone(), default_settings.last_version),
      has_shown_auto_launch_warning: conv_bool(self.settings.has_shown_auto_launch_warning.clone(), default_settings.has_shown_auto_launch_warning),
    };

    let mut config = LauncherConfigV4 {
      settings,
      addons: vec![],
      accounts: vec![],
      uid_cache: vec![],
    };

    // Addons
    let addons_json = self.settings.addons_json.clone().unwrap_or_else(|| String::from("[]"));
    let addons: Vec<OldAddonEntry> = serde_json::from_str(addons_json.as_str())?;
    for addon in addons {
      config.addons.push(AddonEntry {
        is_enabled: addon.is_enabled.clone(),
        path: addon.details.path.clone(),
        command_line: addon.details.command_line.clone(),
        run_as_admin: addon.details.run_as_admin.clone(),
        run_on_close: addon.details.run_on_close.clone(),
        kill_after_close: addon.details.kill_after_close.clone(),
      });
    }

    // Accounts
    for account in self.accounts.iter() {
      config.accounts.push(AccountEntry {
        character_name: account.character_name.clone(),
        character_world: account.character_world.clone(),
        thumbnail_url: account.thumbnail_url.clone().unwrap_or_default(),
        username: account.username.clone(),
        save_password: account.save_password.clone(),
        use_steam: account.use_steam.clone(),
        use_otp: account.use_otp.clone(),
      });
    }

    // UID Cache
    // Invalidate the cache

    config.save()?;
    // fs::remove_file(get_launcher_old_config_path()?);
    // fs::remove_file(get_launcher_old_accounts_path()?);
    // fs::remove_file(get_launcher_old_uid_cache_path()?);
    Ok(config)
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OldLauncherSettings {
  #[serde(rename = "GamePath")]
  pub game_path: Option<String>,
  #[serde(rename = "IsDx11")]
  pub use_dx11: Option<String>,
  #[serde(rename = "AutologinEnabled")]
  pub use_autologin: Option<String>,
  #[serde(rename = "AddonList")]
  pub addons_json: Option<String>,
  #[serde(rename = "UniqueIdCacheEnabled")]
  pub enable_uid_cache: Option<String>,
  #[serde(rename = "AdditionalLaunchArgs")]
  pub extra_game_args: Option<String>,
  #[serde(rename = "InGameAddonEnabled")]
  pub enable_dalamud: Option<String>,
  #[serde(rename = "OtpServerEnabled")]
  pub enable_otp_server: Option<String>,
  #[serde(rename = "SteamIntegrationEnabled")]
  pub enable_steam_integration: Option<String>,
  #[serde(rename = "Language")]
  pub client_language: Option<ClientLanguage>,
  #[serde(rename = "LauncherLanguage")]
  pub launcher_language: Option<LauncherLanguage>,
  #[serde(rename = "CurrentAccountId")]
  pub current_account_id: Option<String>,
  #[serde(rename = "EncryptArguments")]
  pub encrypt_args: Option<String>,
  #[serde(rename = "PatchPath")]
  pub patch_path: Option<String>,
  #[serde(rename = "AskBeforePatchInstall")]
  pub ask_before_patching: Option<String>,
  #[serde(rename = "SpeedLimitBytes")]
  pub download_speed_limit_bytes: Option<String>,
  #[serde(rename = "DalamudInjectionDelayMs")]
  pub dalamud_injection_delay_ms: Option<String>,
  #[serde(rename = "KeepPatches")]
  pub keep_patches: Option<String>,
  #[serde(rename = "OptOutMbCollection")]
  pub opt_out_mb_collection: Option<String>,
  #[serde(rename = "HasComplainedAboutAdmin")]
  pub has_admin_complaints: Option<String>,
  #[serde(rename = "LastVersion")]
  pub last_version: Option<String>,
  #[serde(rename = "HasShownAutoLaunchDisclaimer")]
  pub has_shown_auto_launch_warning: Option<String>,
  #[serde(rename = "AcceptLanguage")]
  pub accept_language: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OldAccountEntry {
  #[serde(rename = "ChosenCharacterName")]
  pub character_name: String,
  #[serde(rename = "ChosenCharacterWorld")]
  pub character_world: String,
  #[serde(rename = "ThumbnailUrl")]
  pub thumbnail_url: Option<String>,
  #[serde(rename = "UserName")]
  pub username: String,
  #[serde(rename = "SavePassword")]
  pub save_password: bool,
  #[serde(rename = "UseSteamServiceAccount")]
  pub use_steam: bool,
  #[serde(rename = "UseOtp")]
  pub use_otp: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OldAddonEntry {
  #[serde(rename = "IsEnabled")]
  pub is_enabled: bool,
  #[serde(rename = "Addon")]
  pub details: OldAddonDetails,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OldAddonDetails {
  #[serde(rename = "Path")]
  pub path: String,
  #[serde(rename = "CommandLine")]
  pub command_line: String,
  #[serde(rename = "RunAsAdmin")]
  pub run_as_admin: bool,
  #[serde(rename = "RunOnClose")]
  pub run_on_close: bool,
  #[serde(rename = "KillAfterClose")]
  pub kill_after_close: bool,
  #[serde(rename = "Name")]
  pub name: String,
}

fn conv_str(str: Option<String>, default: String) -> String {
  str.unwrap_or_else(|| default)
}

fn conv_bool(str: Option<String>, default: bool) -> bool {
  str.unwrap_or_else(|| default.to_string()).to_lowercase()
    .parse().unwrap_or_else(|_| default)
}

fn conv_t<T>(str: Option<String>, default: T) -> T where T: std::str::FromStr + std::string::ToString {
  str.unwrap_or_else(|| default.to_string())
    .parse().unwrap_or_else(|_| default)
}
