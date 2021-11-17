use crate::game::client_language::ClientLanguage;
use crate::game::launcher_language::LauncherLanguage;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LauncherConfig {
  #[serde(rename = "GamePath")]
  pub game_path: String,
  #[serde(rename = "IsDx11")]
  pub use_dx11: bool,
  #[serde(rename = "AutologinEnabled")]
  pub use_autologin: bool,
  #[serde(rename = "AddonList")]
  pub addons: Vec<AddonEntry>,
  #[serde(rename = "UniqueIdCacheEnabled")]
  pub enable_uid_cache: bool,
  #[serde(rename = "AdditionalLaunchArgs")]
  pub extra_game_args: String,
  #[serde(rename = "InGameAddonEnabled")]
  pub enable_dalamud: bool,
  #[serde(rename = "OtpServerEnabled")]
  pub enable_otp_server: bool,
  #[serde(rename = "SteamIntegrationEnabled")]
  pub enable_steam: bool,
  #[serde(rename = "ClientLanguage")]
  pub client_language: ClientLanguage,
  #[serde(rename = "LauncherLanguage")]
  pub launcher_language: LauncherLanguage,
  #[serde(rename = "CurrentAccountId")]
  pub current_account_id: String,
  #[serde(rename = "EncryptArguments")]
  pub encrypt_args: bool,
  #[serde(rename = "PatchPath")]
  pub patch_path: String,
  #[serde(rename = "AskBeforePatchInstall")]
  pub ask_before_patching: bool,
  #[serde(rename = "SpeedLimitBytes")]
  pub download_speed_limit_bytes: u64,
  #[serde(rename = "DalamudInjectionDelayMs")]
  pub dalamud_injection_delay_ms: f64,
  #[serde(rename = "KeepPatches")]
  pub keep_patches: bool,
  #[serde(rename = "OptOutMbCollection")]
  pub opt_out_mb_collection: bool,
  #[serde(rename = "HasComplainedAboutAdmin")]
  pub has_admin_complaints: bool,
  #[serde(rename = "LastVersion")]
  pub last_version: String,
  #[serde(rename = "HasShownAutoLaunchDisclaimer")]
  pub has_shown_auto_launch_warning: bool,
  #[serde(rename = "AcceptLanguage")]
  pub accept_language: String,
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
