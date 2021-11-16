extern crate dunce;
extern crate log;
extern crate sys_locale;
extern crate windows;

use log::{debug, error, warn};
use std::convert::TryFrom;
use std::os::windows::ffi::OsStringExt;
use std::path::PathBuf;
use windows::Win32;

/// Get the current system locale
#[tauri::command]
pub fn get_system_locale() -> String {
  let locale = sys_locale::get_locale().unwrap_or_else(|| String::from("en-us")).to_lowercase();
  debug!("Detected system locale as {:?}", locale);
  return locale;
}

/// Get the path to the Advanced Combat Tracker, if it exists.
#[tauri::command]
pub fn find_advanced_combat_tracker() -> String {
  // https://doc.rust-lang.org/std/env/consts/constant.OS.html
  return match std::env::consts::OS {
    "windows" => find_advanced_combat_tracker_windows(),
    _ => empty_string(),
  };
}

#[cfg(target_os = "windows")]
fn find_advanced_combat_tracker_windows() -> String {
  let path = match get_folderid_path(&Win32::UI::Shell::FOLDERID_CommonStartMenu, 0) {
    Ok(path) => path,
    Err(e) => {
      error!("Could not get CommonStartMenu folder: {:?}", e);
      return empty_string();
    }
  };

  let shortcut_dir_pathbuf = path.join("Programs").join("Advanced Combat Tracker");
  let shortcut_dir_path = shortcut_dir_pathbuf.as_path();

  let shortcut_pathbuf = shortcut_dir_path.join("ACT - Advanced Combat Tracker.lnk");
  let shortcut_path = shortcut_pathbuf.as_path();

  if !shortcut_path.exists() {
    debug!("ACT shortcut file does not exist at {:?}", shortcut_path);
    return empty_string();
  }

  let lnk = match parselnk::Lnk::try_from(shortcut_path) {
    Ok(lnk) => lnk,
    Err(e) => {
      error!(
        "Could not parse ACT shortcut file: {:?}, {:?}",
        shortcut_path, e
      );
      return empty_string();
    }
  };

  /*
  // no method named `readable` found for struct `Lnk` in the current scope
  if !lnk.readable() {
      error!("Could not read ACT shortcut file at {:?}", path);
      return empty_str();
  }
  */

  let rel_pathbuf = match lnk.relative_path() {
    Some(path) => path,
    None => {
      error!("Could not read path from ACT shortcut: {:?}", shortcut_path);
      return empty_string();
    }
  };

  let cwd_pathbuf = match std::env::current_dir() {
    Ok(dir) => dir,
    Err(e) => {
      error!("Current working directory was not valid: {:?}", e);
      return empty_string();
    }
  };
  let cwd_path = cwd_pathbuf.as_path();

  // Change directory to canonicalize the shortcut
  match std::env::set_current_dir(shortcut_dir_path) {
    Err(e) => {
      error!(
        "Could not chdir for canonicalize: {:?}, {:?}",
        shortcut_dir_path, e
      );
      return empty_string();
    }
    _ => {}
  };

  let rel_path = rel_pathbuf.as_path();
  let abs_pathbuf = match dunce::canonicalize(rel_path) {
    Ok(path) => path,
    Err(e) => {
      error!(
        "Could not canonicalize ACT shortcut path: {:?}, {:?}",
        rel_path, e
      );
      return empty_string();
    }
  };

  // Revert to the previous cwd
  match std::env::set_current_dir(cwd_path) {
    Err(e) => {
      error!(
        "Could not revert chdir for canonicalize: {:?}, {:?}",
        cwd_path, e
      );
      return empty_string();
    }
    _ => {}
  }

  let abs_path = abs_pathbuf.as_path();
  if !abs_path.exists() {
    warn!("ACT shortcut path did not exist: {:?}", abs_path);
    return empty_string();
  }

  let result_str = match abs_path.to_str() {
    Some(path) => path,
    None => {
      error!("Could not stringify ACT shortcut path: {:?}", abs_path);
      return empty_string();
    }
  };

  let result_string = result_str.to_string();
  debug!("Found ACT installed at {:?}", result_string);
  return result_string;
}

#[cfg(target_os = "windows")]
fn get_folderid_path(
  guid: &windows::core::GUID,
  flags: u32,
) -> Result<PathBuf, windows::core::Error> {
  let result = unsafe {
    Win32::UI::Shell::SHGetKnownFolderPath(
      guid,
      flags,
      windows::Win32::Foundation::HANDLE::default(),
    )
  };

  let wide_path = match result {
    Ok(result) => result,
    Err(e) => return Err(e),
  };

  let len = unsafe { windows::Win32::Globalization::lstrlenW(wide_path) } as usize;

  let path_str =
    std::ffi::OsString::from_wide(unsafe { std::slice::from_raw_parts(wide_path.0, len) });

  return Ok(PathBuf::from(path_str));
}

fn empty_string() -> String {
  return String::from("");
}
