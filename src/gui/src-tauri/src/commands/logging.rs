use log::{debug, info, warn, error};

/// Log a debug message
/// # Arguments
/// * `msg` - Message to log
#[tauri::command]
pub fn debug(msg: String) {
  debug!("{}", msg)
}

/// Log an info message
/// # Arguments
/// * `msg` - Message to log
#[tauri::command]
pub fn info(msg: String) {
  info!("{}", msg)
}

/// Log a warning message
/// # Arguments
/// * `msg` - Message to log
#[tauri::command]
pub fn warn(msg: String) {
  warn!("{}", msg)
}

/// Log an error message
/// # Arguments
/// * `msg` - Message to log
#[tauri::command]
pub fn error(msg: String) {
  error!("{}", msg)
}
