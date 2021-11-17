use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum ClientLanguage {
  Japanese,
  EnglishGB,
  EnglishUS,
  German,
  French,
}

impl ClientLanguage {
  pub fn langcode(&self) -> &str {
    match self {
      ClientLanguage::Japanese => "ja",
      ClientLanguage::EnglishGB => "en-gb",
      ClientLanguage::EnglishUS => "en-us",
      ClientLanguage::German => "de",
      ClientLanguage::French => "fr",
    }
  }

  pub fn langcode_short(&self) -> &str {
    match self {
      ClientLanguage::Japanese => "ja",
      ClientLanguage::EnglishGB => "en",
      ClientLanguage::EnglishUS => "en",
      ClientLanguage::German => "de",
      ClientLanguage::French => "fr",
    }
  }

  pub fn langcode_underscore(&self) -> &str {
    match self {
      ClientLanguage::Japanese => "ja",
      ClientLanguage::EnglishGB => "en_gb",
      ClientLanguage::EnglishUS => "en_us",
      ClientLanguage::German => "de",
      ClientLanguage::French => "fr",
    }
  }

  pub fn from_langcode(langcode: &str) -> ClientLanguage {
    match langcode {
      "ja" => ClientLanguage::Japanese,
      "en" => ClientLanguage::EnglishGB,
      "en-gb" => ClientLanguage::EnglishGB,
      "en-us" => ClientLanguage::EnglishUS,
      "de" => ClientLanguage::German,
      "fr" => ClientLanguage::French,
      &_ => ClientLanguage::EnglishUS,
    }
  }
}

impl Display for ClientLanguage {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.langcode())
  }
}
