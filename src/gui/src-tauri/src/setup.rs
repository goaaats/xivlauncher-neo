use log::{debug, error};
use libxl::error::{XlError, XlResult};

/// Get the current system locale
#[tauri::command]
pub fn get_system_locale() -> XlResult<String> {
  match libxl::locale::get_system_locale() {
    Ok(locale) => {
      debug!("Detected system locale as {:?}", locale);
      Ok(locale)
    }
    Err(e) => {
      let msg = format!("Error resolving system locale: {:#?}", e);
      error!("{}", msg);
      Err(XlError::new(msg))
    },
  }
}

/// Get the path to the Advanced Combat Tracker, if it exists.
#[tauri::command]
pub fn find_advanced_combat_tracker() -> XlResult<String> {
  match libxl::addon::act::find_advanced_combat_tracker() {
    Ok(path) => {
      debug!("Detected ACT path {:?}", path);
      Ok(path)
    }
    Err(e) => {
      let msg = format!("Error resolving ACT path: {:#?}", e);
      error!("{}", msg);
      Err(XlError::new(msg))
    }
  }
}
