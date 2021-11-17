use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum LauncherLanguage {
  Japanese,
  English,
  German,
  French,
  Italian,
  Spanish,
  Portuguese,
  Korean,
  Norwegian,
  Russian,
  SimplifiedChinese,
}

impl LauncherLanguage {
  pub fn langcode(&self) -> &str {
    match self {
      LauncherLanguage::Japanese => "ja",
      LauncherLanguage::English => "en",
      LauncherLanguage::German => "de",
      LauncherLanguage::French => "fr",
      LauncherLanguage::Italian => "it",
      LauncherLanguage::Spanish => "es",
      LauncherLanguage::Portuguese => "pt",
      LauncherLanguage::Korean => "ko",
      LauncherLanguage::Norwegian => "no",
      LauncherLanguage::Russian => "ru",
      LauncherLanguage::SimplifiedChinese => "zh",
    }
  }

  pub fn from_langcode(langcode: &str) -> LauncherLanguage {
    match langcode {
      "ja" => LauncherLanguage::Japanese,
      "en" => LauncherLanguage::English,
      "de" => LauncherLanguage::German,
      "fr" => LauncherLanguage::French,
      "it" => LauncherLanguage::Italian,
      "es" => LauncherLanguage::Spanish,
      "pt" => LauncherLanguage::Portuguese,
      "ko" => LauncherLanguage::Korean,
      "no" => LauncherLanguage::Norwegian,
      "ru" => LauncherLanguage::Russian,
      "zh" => LauncherLanguage::SimplifiedChinese,
      &_ => LauncherLanguage::English,
    }
  }
}

impl Display for LauncherLanguage {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.langcode())
  }
}
