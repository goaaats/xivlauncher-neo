use crate::either;
use crate::util::locale::is_north_america;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum GameLanguage {
  Japanese,
  English,
  German,
  French,
}

impl GameLanguage {
  pub fn langcode(&self) -> &str {
    match self {
      Self::Japanese => "ja",
      Self::English => either!(is_north_america() => "en-us"; "en-gb"),
      Self::German => "de",
      Self::French => "fr",
    }
  }

  pub fn langcode_short(&self) -> &str {
    match self {
      Self::Japanese => "ja",
      Self::English => "en",
      Self::German => "de",
      Self::French => "fr",
    }
  }

  pub fn langcode_underscore(&self) -> &str {
    match self {
      Self::Japanese => "ja",
      Self::English => either!(is_north_america() => "en_us"; "en_gb"),
      Self::German => "de",
      Self::French => "fr",
    }
  }

  pub fn from_langcode(langcode: &str) -> Self {
    let first = langcode.split('-').next().unwrap();
    match first {
      "ja" => Self::Japanese,
      "en" => Self::English,
      "de" => Self::German,
      "fr" => Self::French,
      &_ => Self::English,
    }
  }
}

impl Display for GameLanguage {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.langcode())
  }
}
