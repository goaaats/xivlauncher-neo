use anyhow::{Context, Error};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use windows::core::GUID;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::Globalization::lstrlenW;
use windows::Win32::UI::Shell::SHGetKnownFolderPath;
use windows::Win32::UI::Shell::{FOLDERID_CommonStartMenu, FOLDERID_RoamingAppData};

/// Gets the path to application directory
/// This should be AppData\Roaming\XIVLauncher
#[cfg(target_os = "windows")]
pub fn get_data_dir_path() -> Result<PathBuf, Error> {
  get_roaming_appdata_path()
    .map(|path| path.join("XIVLauncher"))
}

/// Gets the path to application directory
/// This should be <wine_prefix>/???/XIVLauncher
#[cfg(target_os = "linux")]
pub fn get_data_dir_path() -> Result<PathBuf, Error> {
  Err(Error::msg("OS not supported"))
  // .map(|path| path.join("XIVLauncher"))
}

/// Gets the path to application directory
/// This should be <wine_prefix>/???/XIVLauncher
#[cfg(target_os = "macos")]
pub fn get_data_dir_path() -> Result<PathBuf, Error> {
  Err(Error::msg("OS not supported"))
  // .map(|path| path.join("XIVLauncher"))
}

/// Gets the path to the Dalamud plugin folder (installedPlugins)
pub fn get_dalamud_plugin_path() -> Result<PathBuf, Error> {
  get_data_dir_path().map(|path| path.join("installedPlugins"))
}

/// Gets the path to the Dalamud configuration file (dalamudConfig.json)
pub fn get_dalamud_config_path() -> Result<PathBuf, Error> {
  get_data_dir_path().map(|path| path.join("dalamudConfig.json"))
}

/// Gets the path to the launcher configuration file (launcherConfigV4.json)
pub fn get_launcher_config_path() -> Result<PathBuf, Error> {
  get_data_dir_path().map(|path| path.join("launcherConfigV4.json"))
}

/// Gets the path to the launcher configuration file (launcherConfigV3.json)
pub fn get_launcher_old_config_path() -> Result<PathBuf, Error> {
  get_data_dir_path().map(|path| path.join("launcherConfigV3.json"))
}

/// Gets the path to the launcher configuration file (accountsList.json)
pub fn get_launcher_old_accounts_path() -> Result<PathBuf, Error> {
  get_data_dir_path().map(|path| path.join("accountsList.json"))
}

/// Gets the path to the launcher configuration file (uidCache.json)
pub fn get_launcher_old_uid_cache_path() -> Result<PathBuf, Error> {
  get_data_dir_path().map(|path| path.join("uidCache.json"))
}

/// Gets the path to the special dir FOLDERID_CommonStartMenu
/// Typically, this is %ALLUSERSPROFILE%\Microsoft\Windows\Start Menu
#[cfg(target_os = "windows")]
pub fn get_common_start_menu_path() -> Result<PathBuf, Error> {
  get_folderid_path(&FOLDERID_CommonStartMenu, 0)
}

/// Gets the path to the special dir FOLDERID_RoamingAppData
/// Typically, this is %APPDATA% (%USERPROFILE%\AppData\Roaming)
#[cfg(target_os = "windows")]
pub fn get_roaming_appdata_path() -> Result<PathBuf, Error> {
  get_folderid_path(&FOLDERID_RoamingAppData, 0)
}

/// Gets the path to a FOLDERID_<GUID> directory
/// # Arguments:
/// * `guid` - FOLDERID GUID
/// * `flags` - GUID specific flags
#[cfg(target_os = "windows")]
pub fn get_folderid_path(guid: &GUID, flags: u32) -> Result<PathBuf, Error> {
  let wide_path = unsafe { SHGetKnownFolderPath(guid, flags, HANDLE::default()) };
  let wide_path = wide_path.with_context(|| "SHGetKnownFolderPath failed")?;

  let len = unsafe { lstrlenW(wide_path) } as usize;
  let path_str = OsString::from_wide(unsafe { std::slice::from_raw_parts(wide_path.0, len) });
  let path = PathBuf::from(path_str);

  Ok(path)
}
