use log::debug;

/// Get the current system locale
#[tauri::command]
pub fn get_system_locale() -> String {
  let locale = libxl::locale::get_system_locale();
  debug!("Detected system locale as {:?}", locale);
  locale
}

/// Get the path to the Advanced Combat Tracker, if it exists.
#[tauri::command]
pub fn find_advanced_combat_tracker() -> String {
  match libxl::addon::act::find_advanced_combat_tracker() {
    Ok(path) => {
      debug!("Detected ACT path {:?}", path);
      path
    }
    Err(e) => {
      debug!("Could not find ACT: {:?}", e);
      String::from("")
    }
  }
}
