use crate::lib::error::{XlError, XlResult};
use log::{debug, error};

/// Get the current system locale
#[tauri::command]
pub fn get_system_locale() -> XlResult<String> {
  match sys_locale::get_locale() {
    Some(locale) => {
      let locale = locale.to_lowercase();
      debug!("Detected system locale as {:?}", locale);
      Ok(locale)
    }
    None => {
      let msg = format!("Error resolving system locale");
      error!("{}", msg);
      Err(XlError::new(msg))
    }
  }
}
