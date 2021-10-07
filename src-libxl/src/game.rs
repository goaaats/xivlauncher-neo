pub mod oauth;
pub mod status;

mod constants;
mod request;

#[derive(Copy, Clone)]
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