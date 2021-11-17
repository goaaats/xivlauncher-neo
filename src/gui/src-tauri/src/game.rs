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