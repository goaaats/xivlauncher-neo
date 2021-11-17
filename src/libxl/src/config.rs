use crate::error::XlError;
use crate::game::language::ClientLanguage;
use crate::language::LauncherLanguage;
use crate::util::path::get_launcher_config_path;
use crate::xl_error;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::fs;
use std::io::{BufReader, BufWriter, Write};

#[derive(Serialize, Deserialize, Debug)]
pub struct LauncherConfig {
  #[serde(rename = "GamePath")]
  pub game_path: String,
  #[serde(rename = "IsDx11")]
  pub use_dx11: String,
  #[serde(rename = "AutologinEnabled")]
  pub use_autologin: String,
  #[serde(rename = "AddonList")]
  pub addons_json: String,
  #[serde(rename = "UniqueIdCacheEnabled")]
  pub enable_uid_cache: String,
  #[serde(rename = "AdditionalLaunchArgs")]
  pub extra_game_args: String,
  #[serde(rename = "InGameAddonEnabled")]
  pub enable_dalamud: String,
  #[serde(rename = "OtpServerEnabled")]
  pub enable_otp_server: String,
  #[serde(rename = "SteamIntegrationEnabled")]
  pub enable_steam: String,
  #[serde(rename = "Language")]
  pub client_language: ClientLanguage,
  #[serde(rename = "LauncherLanguage")]
  pub launcher_language: LauncherLanguage,
  #[serde(rename = "CurrentAccountId")]
  pub current_account_id: String,
  #[serde(rename = "EncryptArguments")]
  pub encrypt_args: String,
  #[serde(rename = "PatchPath")]
  pub patch_path: String,
  #[serde(rename = "AskBeforePatchInstall")]
  pub ask_before_patching: String,
  #[serde(rename = "SpeedLimitBytes")]
  pub download_speed_limit_bytes: String,
  #[serde(rename = "DalamudInjectionDelayMs")]
  pub dalamud_injection_delay_ms: String,
  #[serde(rename = "KeepPatches")]
  pub keep_patches: String,
  #[serde(rename = "OptOutMbCollection")]
  pub opt_out_mb_collection: String,
  #[serde(rename = "HasComplainedAboutAdmin")]
  pub has_admin_complaints: String,
  #[serde(rename = "LastVersion")]
  pub last_version: String,
  #[serde(rename = "HasShownAutoLaunchDisclaimer")]
  pub has_shown_auto_launch_warning: String,
  #[serde(rename = "AcceptLanguage")]
  pub accept_language: String,
}

impl Display for LauncherConfig {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let repr = serde_json::to_string(self).expect("Uh oh");
    f.write_str(repr.as_str())
  }
}

impl LauncherConfig {
  pub fn load() -> Result<LauncherConfig> {
    let pathbuf = get_launcher_config_path()?;
    let path = pathbuf.as_path();

    if !path.exists() {
      let path_dir = path.parent().ok_or(xl_error!("Launcher config file has no parent"))?;
      fs::create_dir_all(path_dir)?;

      let mut file = fs::File::create(path)?;
      file.write("{}".as_bytes())?;
      file.flush()?;
    }

    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let config: LauncherConfig = serde_json::from_reader(reader)?;

    return Ok(config);
  }

  pub fn save(&self) -> Result<()> {
    let pathbuf = get_launcher_config_path()?;
    let path = pathbuf.as_path();

    if !path.exists() {
      let path_dir = path.parent().ok_or(xl_error!("Launcher config file has no parent"))?;
      fs::create_dir_all(path_dir)?;
    }

    let file = fs::OpenOptions::new().write(true).open(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, self)?;

    return Ok(());
  }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct AddonEntry {
  #[serde(rename = "IsEnabled")]
  pub is_enabled: bool,
  #[serde(rename = "Addon")]
  pub addon_deprecated: String,
  #[serde(rename = "Path")]
  pub path: String,
  #[serde(rename = "CommandLine")]
  pub command_line: String,
  #[serde(rename = "RunAsAdmin")]
  pub run_as_admin: bool,
  #[serde(rename = "RunOnClose")]
  pub run_on_clone: bool,
  #[serde(rename = "KillAfterClose")]
  pub kill_after_close: bool,
  #[serde(rename = "Name")]
  pub name: String,
}

// No serialization allowed!
#[derive(Deserialize, Debug)]
pub struct DalamudConfig {
  #[serde(rename = "DoDalamudTest")]
  pub dalamud_test: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountListEntry {
  #[serde(rename = "ChosenCharacterName")]
  pub character_name: String,
  #[serde(rename = "ChosenCharacterWorld")]
  pub character_world: String,
  #[serde(rename = "ThumbnailUrl")]
  pub thumbnail_url: String,
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
pub struct UidCacheEntry {
  #[serde(rename = "UserName")]
  pub username: String,
  #[serde(rename = "UniqueId")]
  pub unique_id: String,
  #[serde(rename = "Region")]
  pub region: u32,
  #[serde(rename = "ExpansionLevel")]
  pub expansion_level: u32,
  #[serde(rename = "CreationDate")]
  pub creation_date: DateTime<Utc>,
}
