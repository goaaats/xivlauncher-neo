use std::fmt::{Display, Formatter};
use serde::Serialize;

pub mod oauth;
pub mod status;
pub mod platform;
pub mod version;
pub mod repository;
pub mod launcher;

mod constants;
mod request;

#[derive(Copy, Clone, Serialize)]
pub enum ClientLanguage {
    Japanese,
    EnglishGB,
    EnglishUS,
    German,
    French,
}

impl ClientLanguage {
    fn langcode(&self) -> &str {
        match self {
            ClientLanguage::Japanese => "ja",
            ClientLanguage::EnglishGB => "en-gb",
            ClientLanguage::EnglishUS => "en-us",
            ClientLanguage::German => "de",
            ClientLanguage::French => "fr",
        }
    }

    fn langcode_short(&self) -> &str {
        match self {
            ClientLanguage::Japanese => "ja",
            ClientLanguage::EnglishGB => "en",
            ClientLanguage::EnglishUS => "en",
            ClientLanguage::German => "de",
            ClientLanguage::French => "fr",
        }
    }

    fn langcode_underscore(&self) -> &str {
        match self {
            ClientLanguage::Japanese => "ja",
            ClientLanguage::EnglishGB => "en_gb",
            ClientLanguage::EnglishUS => "en_us",
            ClientLanguage::German => "de",
            ClientLanguage::French => "fr",
        }
    }
}

impl Display for ClientLanguage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.langcode())
    }
}