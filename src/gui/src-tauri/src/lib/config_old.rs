use crate::lib::config::{AccountEntry, AddonEntry, LauncherConfig};
use crate::lib::language::LauncherLanguage;
use crate::lib::path::{get_launcher_old_accounts_path, get_launcher_old_config_path, get_launcher_old_uid_cache_path};
use anyhow::{Context, Error, Result};
use libxl::game::language::GameLanguage;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::PathBuf;

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
    let path = path.as_path();
    if !path.exists() {
      return Err(Error::msg(format!("File not found: {:?}", path)));
    }

    let content = fs::read_to_string(path).with_context(|| format!("Could not read {:?}", path))?;

    let settings: OldLauncherSettings =
      serde_json::from_str(content.as_str()).with_context(|| format!("Could not deserialize {:?}", path))?;

    // Addons
    let addon_json = settings.addons_json.clone().unwrap_or_else(|| String::from("[]"));
    let addons = serde_json::from_str(addon_json.as_str())
      .with_context(|| format!("Could not deserialize addons in {:?}", path))?;

    // Accounts
    let path = get_launcher_old_accounts_path()?;
    let path = path.as_path();
    let content = fs::read_to_string(path).with_context(|| format!("Could not read {:?}", path))?;

    let accounts: Vec<OldAccountEntry> =
      serde_json::from_str(content.as_str()).with_context(|| format!("Could not deserialize {:?}", path))?;

    let config = OldLauncherConfig {
      settings,
      addons,
      accounts,
    };
    Ok(config)
  }

  pub fn upgrade(&self) -> Result<LauncherConfig, Error> {
    // Settings
    let mut config = LauncherConfig::default();
    config.game_path = conv_str(self.settings.game_path.clone(), config.game_path);
    config.use_dx11 = conv_bool(self.settings.use_dx11.clone(), config.use_dx11);
    config.use_autologin = conv_bool(self.settings.use_autologin.clone(), config.use_autologin);
    config.enable_uid_cache = conv_bool(self.settings.enable_uid_cache.clone(), config.enable_uid_cache);
    config.extra_game_args = conv_str(self.settings.extra_game_args.clone(), config.extra_game_args);
    config.enable_dalamud = conv_bool(self.settings.enable_dalamud.clone(), config.enable_dalamud);
    config.enable_otp_server = conv_bool(self.settings.enable_otp_server.clone(), config.enable_otp_server);
    config.enable_steam_integration = conv_bool(self.settings.enable_steam_integration.clone(), config.enable_steam_integration);
    config.game_language = self.settings.client_language.unwrap_or(GameLanguage::English);
    config.launcher_language = self.settings.launcher_language.unwrap_or(LauncherLanguage::English);
    config.current_account_id = conv_str(self.settings.current_account_id.clone(), config.current_account_id);
    config.encrypt_args = conv_bool(self.settings.encrypt_args.clone(), config.encrypt_args);
    config.patch_path = conv_str(self.settings.patch_path.clone(), config.patch_path);
    config.ask_before_patching = conv_bool(self.settings.ask_before_patching.clone(), config.ask_before_patching);
    config.download_speed_limit_bytes = conv_t(self.settings.download_speed_limit_bytes.clone(), config.download_speed_limit_bytes);
    config.dalamud_injection_delay_ms = conv_t(self.settings.dalamud_injection_delay_ms.clone(), config.dalamud_injection_delay_ms);
    config.keep_patches = conv_bool(self.settings.keep_patches.clone(), config.keep_patches);
    config.opt_out_mb_collection = conv_bool(self.settings.opt_out_mb_collection.clone(), config.opt_out_mb_collection);
    config.has_admin_complaints = conv_bool(self.settings.has_admin_complaints.clone(), config.has_admin_complaints);
    config.last_version = conv_str(self.settings.last_version.clone(), config.last_version);
    config.has_shown_auto_launch_warning = conv_bool(self.settings.has_shown_auto_launch_warning.clone(), config.has_shown_auto_launch_warning);

    // Addons
    for addon in self.addons.iter() {
      config.addons.push(AddonEntry {
        is_enabled: addon.is_enabled,
        path: addon.details.path.clone(),
        command_line: addon.details.command_line.clone(),
        run_as_admin: addon.details.run_as_admin,
        run_on_close: addon.details.run_on_close,
        kill_after_close: addon.details.kill_after_close,
      });
    }

    // Accounts
    for account in self.accounts.iter() {
      config.accounts.push(AccountEntry {
        character_name: account.character_name.clone().unwrap_or_default(),
        character_world: account.character_world.clone().unwrap_or_default(),
        thumbnail_url: account.thumbnail_url.clone().unwrap_or_default(),
        username: account.username.clone(),
        save_password: account.save_password,
        use_steam: account.use_steam,
        use_otp: account.use_otp,
      });
    }

    // UID Cache
    // Invalidate the cache

    config.save()?;

    let path = get_launcher_old_config_path()?;
    if path.exists() {
      Self::backup_file(path)?
    }

    let path = get_launcher_old_accounts_path()?;
    if path.exists() {
      Self::backup_file(path)?
    }

    let path = get_launcher_old_uid_cache_path()?;
    if path.exists() {
      Self::backup_file(path)?
    }

    Ok(config)
  }

  #[allow(unused_variables)]
  fn backup_file(path: PathBuf) -> Result<()> {
    let path = path.as_path();

    let filename = path
      .file_name()
      .with_context(|| format!("Failed to get filename of {:?}", path))?
      .to_os_string()
      .into_string()
      .map_err(|_| Error::msg(format!("Failed string conversion of {:?}", path)))?;

    let filename = filename + ".backup";
    let new_path = path.to_path_buf();
    let new_path = new_path.with_file_name(filename);

    // TODO: Re-enable file backup on release
    // fs::rename(path, new_path)
    //   .with_context(|| format!("Failed backup renaming of {:?}", path))?;

    Ok(())
  }
}

#[derive(Serialize, Deserialize, Debug, Default)]
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
  pub client_language: Option<GameLanguage>,
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
  pub character_name: Option<String>,
  #[serde(rename = "ChosenCharacterWorld")]
  pub character_world: Option<String>,
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
  str.unwrap_or(default)
}

fn conv_bool(str: Option<String>, default: bool) -> bool {
  str
    .unwrap_or_else(|| default.to_string())
    .to_lowercase()
    .parse()
    .unwrap_or(default)
}

fn conv_t<T>(str: Option<String>, default: T) -> T
  where
    T: std::str::FromStr + std::string::ToString,
{
  str.unwrap_or_else(|| default.to_string()).parse().unwrap_or(default)
}
