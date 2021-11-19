use crate::util::path::get_common_start_menu_path;
use anyhow::{Context, Error, Result};

/// Get the path to the Advanced Combat Tracker, if it exists.
pub fn find_advanced_combat_tracker() -> Result<String> {
  // https://doc.rust-lang.org/std/env/consts/constant.OS.html
  match std::env::consts::OS {
    "windows" => find_advanced_combat_tracker_windows(),
    _ => Err(Error::msg("OS not supported")),
  }
}

#[cfg(target_os = "windows")]
fn find_advanced_combat_tracker_windows() -> Result<String> {
  let path = get_common_start_menu_path()?;

  let shortcut_dir_path = path.join("Programs").join("Advanced Combat Tracker");
  let shortcut_dir_path = shortcut_dir_path.as_path();

  let shortcut_path = shortcut_dir_path.join("ACT - Advanced Combat Tracker.lnk");
  let shortcut_path = shortcut_path.as_path();
  if !shortcut_path.exists() {
    return Err(Error::msg(format!(
      "ACT shortcut file does not exist at {:?}",
      shortcut_path
    )));
  }

  let lnk = parselnk::Lnk::try_from(shortcut_path)
    .with_context(|| format!("Could not parse ACT shortcut file: {:?}", shortcut_path))?;

  let rel_path = lnk
    .relative_path()
    .with_context(|| format!("Could not read path from ACT shortcut: {:?}", shortcut_path))?;

  // Save the CWD
  let cwd_path = std::env::current_dir().with_context(|| "Current working directory was not valid")?;
  let cwd_path = cwd_path.as_path();

  // Change directory to canonicalize the shortcut
  std::env::set_current_dir(shortcut_dir_path)
    .with_context(|| format!("Could not chdir for canonicalize: {:?}", shortcut_dir_path))?;

  let rel_path = rel_path.as_path();
  let abs_path = dunce::canonicalize(rel_path)
    .with_context(|| format!("Could not canonicalize ACT shortcut path: {:?}", rel_path))?;

  let abs_path = abs_path.as_path();
  if !abs_path.exists() {
    return Err(Error::msg(format!("ACT shortcut path did not exist: {:?}", abs_path)));
  }

  // Revert to the previous cwd
  std::env::set_current_dir(cwd_path)
    .with_context(|| format!("Could not revert chdir for canonicalize: {:?}", cwd_path))?;

  let result_str = abs_path.to_str().with_context(|| format!("Could not stringify ACT shortcut path: {:?}", abs_path))?;

  let result_string = result_str.to_string();
  return Ok(result_string);
}
