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
      Self::Japanese => "ja",
      Self::English => "en",
      Self::German => "de",
      Self::French => "fr",
      Self::Italian => "it",
      Self::Spanish => "es",
      Self::Portuguese => "pt",
      Self::Korean => "ko",
      Self::Norwegian => "no",
      Self::Russian => "ru",
      Self::SimplifiedChinese => "zh",
    }
  }

  pub fn from_langcode(langcode: &str) -> LauncherLanguage {
    match langcode {
      "ja" => Self::Japanese,
      "en" => Self::English,
      "de" => Self::German,
      "fr" => Self::French,
      "it" => Self::Italian,
      "es" => Self::Spanish,
      "pt" => Self::Portuguese,
      "ko" => Self::Korean,
      "no" => Self::Norwegian,
      "ru" => Self::Russian,
      "zh" => Self::SimplifiedChinese,
      &_ => Self::English,
    }
  }
}

impl Display for LauncherLanguage {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.langcode())
  }
}
