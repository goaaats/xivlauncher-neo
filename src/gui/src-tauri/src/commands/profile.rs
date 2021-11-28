use crate::lib::error::XlResult;

#[tauri::command]
pub fn get_character_profile_picture_url(_name: &str, _server: &str) -> XlResult<String> {
  // TODO
  return Ok(String::from(""));
}

#[tauri::command]
pub fn create_account_shortcut(_path: &str) -> XlResult<()> {
  // TODO
  Ok(())
}
