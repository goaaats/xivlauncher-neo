use libxl::error::XlResult;

/*

use libxl::error::XlError;
use libxl::game::language::ClientLanguage;
use libxl::game::launcher::headline::Headline;
use libxl::xl_error;
use log::error;

#[tauri::command]
pub async fn get_headline(langcode: &str) -> Result<Headline, XlError> {
  let language = ClientLanguage::from_langcode(langcode);
  let result = Headline::get(language).await;
  return match result {
    Ok(headline) => Ok(headline),
    Err(error) => Err(xl_error!(&format!("Error fetching headlines: {:?}", error))),
  };
}
*/


#[tauri::command]
pub async fn start_backup_tool() -> XlResult<()> {
  // TODO Process.Start(Path.Combine(ViewModel.GamePath, "boot", "ffxivconfig.exe"));
  Ok(())
}

#[tauri::command]
pub async fn start_original_launcher(_use_steam: bool) -> XlResult<()> {
  // TODO Process.Start(Path.Combine(gamePath.FullName, "boot", "ffxivboot.exe"), isSteam ? "-issteam" : string.Empty);
  Ok(())
}
