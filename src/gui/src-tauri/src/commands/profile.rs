use crate::lib::config::AccountEntry;
use crate::lib::error::{XlError, XlResult};
use anyhow::{Context, Error};
use libxl::either;
use std::io::Cursor;

/// Get the Lodestone profile picture for the given character
/// # Arguments
/// * `_name` - Character name
/// * `_server` - Character server
#[tauri::command]
pub fn get_character_profile_picture_url(_name: String, _server: String) -> XlResult<String> {
  // TODO
  return Ok(String::from(""));
}

/// Create a launcher shortcut for the given account profile
/// # Arguments
/// * `entry` - Account entry to generate for
#[tauri::command]
#[cfg(target_os = "windows")]
pub async fn create_account_shortcut(entry: AccountEntry) -> XlResult<()> {
  // Setup paths
  let desktop_path = crate::lib::path::get_desktop_path()?;

  let shortcut_name = format!(
    "XIVLauncher - {}{}",
    entry.username,
    either!(entry.use_steam => " (Steam)"; "")
  );
  let shortcut_path = desktop_path.join(format!("{}.lnk", shortcut_name));
  let shortcut_path = shortcut_path.as_path();

  let app_path = std::env::current_exe().with_context(|| "Could not determine the app path")?;
  let app_path = app_path.as_path();

  let app_path =
    dunce::canonicalize(app_path).with_context(|| format!("Could not canonicalize app path: {:?}", app_path))?;
  let app_path = app_path.as_path();

  let image_folder_path = crate::lib::path::get_profile_icon_path()?;
  let image_folder_path = image_folder_path.as_path();
  if !image_folder_path.exists() {
    std::fs::create_dir(image_folder_path)
      .with_context(|| format!("Could not create image folder: {:?}", image_folder_path))?;
  }

  let image_path = image_folder_path.join(format!("{}.ico", entry.username));
  let image_path = image_path.as_path();

  // Fetch the image
  let resp = reqwest::get(entry.thumbnail_url.as_str())
    .await
    .with_context(|| format!("Could not get thumbnail: {:?}", entry.thumbnail_url))?;

  if !resp.status().is_success() {
    return Err(XlError::from(Error::msg(format!(
      "Could not get thumbnail: {:?}",
      entry.thumbnail_url
    ))));
  }

  let bytes = resp
    .bytes()
    .await
    .with_context(|| format!("Could not read thumbnail bytes: {:?}", entry.thumbnail_url))?;

  // Convert to ICO and save
  let reader = image::io::Reader::new(Cursor::new(bytes))
    .with_guessed_format()
    .with_context(|| format!("Image format guess failed: {:?}", entry.thumbnail_url))?;

  let image = reader
    .decode()
    .with_context(|| format!("Image decoding failed: {:?}", entry.thumbnail_url))?;

  image
    .save_with_format(image_path, image::ImageFormat::Ico)
    .with_context(|| format!("Could not save profile image: {:?}", image_path))?;

  // Create the LNK
  let lnk_icon_path = image_path
    .to_str()
    .with_context(|| format!("Could not stringify image path: {:?}", image_path))?
    .to_string();

  let mut lnk =
    mslnk::ShellLink::new(app_path).with_context(|| format!("Could not create account shortcut: {:?}", app_path))?;

  lnk.set_arguments(Some(format!("--account={}", entry.username.clone())));
  lnk.set_icon_location(Some(lnk_icon_path));
  lnk
    .create_lnk(shortcut_path)
    .with_context(|| format!("Could not save account shortcut: {:?}", shortcut_path))?;

  Ok(())
}

#[tauri::command]
#[cfg(target_os = "linux")]
pub fn create_account_shortcut(entry: AccountEntry) -> XlResult<()> {
  Err(Error::msg("OS not supported"))
}

#[tauri::command]
#[cfg(target_os = "macos")]
pub fn create_account_shortcut(entry: AccountEntry) -> XlResult<()> {
  Err(Error::msg("OS not supported"))
}
