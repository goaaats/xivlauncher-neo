use anyhow::{Error, Result};

/// Get the system locale, lowercase
pub fn get_system_locale() -> Result<String> {
  match sys_locale::get_locale() {
    Some(loc) => Ok(loc.to_lowercase()),
    None => Err(Error::msg("Could not determine locale")),
  }
}

/// Determine if the system locale is within North America
/// Defaults to false if locale bugs out
pub fn is_north_america() -> bool {
  match get_system_locale() {
    Ok(loc) => loc.ends_with("-us") || loc.ends_with("-ca") || loc.ends_with("-mx"),
    Err(_) => false,
  }
}
