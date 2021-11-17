/// Get the system locale
pub fn get_system_locale() -> String {
  sys_locale::get_locale()
    .unwrap_or_else(|| String::from("en-us"))
    .to_lowercase()
}

/// Determine if the system locale is within North America
pub fn is_north_america() -> bool {
  let locale = get_system_locale();
  locale.ends_with("-us") || locale.ends_with("-ca") || locale.ends_with("-mx")
}
