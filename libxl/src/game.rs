use serde::Serialize;
use std::fmt::{Display, Formatter};

pub mod oauth;
pub mod status;

mod constants;
mod request;

#[derive(Copy, Clone, Serialize)]
pub enum ClientLanguage {
    Japanese,
    English,
    German,
    French,
}

impl ClientLanguage {
    fn langcode(&self) -> &str {
        match self {
            ClientLanguage::Japanese => "ja",
            ClientLanguage::English => "en-gb", // TODO: Handle NA
            ClientLanguage::German => "de",
            ClientLanguage::French => "fr",
        }
    }

    fn langcode_underscore(&self) -> &str {
        match self {
            ClientLanguage::Japanese => "ja",
            ClientLanguage::English => "en_gb", // TODO: Handle NA
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