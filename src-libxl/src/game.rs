pub mod oauth;
pub mod status;

mod constants;
mod request;

pub enum ClientLanguage {
    Japanese,
    English,
    German,
    French,
}

impl ClientLanguage {
    fn get_langcode(&self) -> &str {
        match self {
            ClientLanguage::Japanese => "ja",
            ClientLanguage::English => "en-gb", // TODO: Handle NA
            ClientLanguage::German => "de",
            ClientLanguage::French => "fr",
        }
    }

    fn get_langcode_underscore(&self) -> &str {
        match self {
            ClientLanguage::Japanese => "ja",
            ClientLanguage::English => "en_gb", // TODO: Handle NA
            ClientLanguage::German => "de",
            ClientLanguage::French => "fr",
        }
    }
}
