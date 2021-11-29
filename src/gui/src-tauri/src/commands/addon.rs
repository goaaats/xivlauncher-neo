use anyhow::Context;
use crate::lib::error::XlResult;

/// Get the path to the Advanced Combat Tracker, if it exists.
#[tauri::command]
pub fn find_advanced_combat_tracker() -> XlResult<String> {
  let path = crate::lib::addon::find_advanced_combat_tracker()
    .with_context(|| "Error resolving ACT path")?;
  Ok(path)
}
