use core::fmt;
use lazy_static::lazy_static;
use regex::Regex;

use crate::game::{constants, request, ClientLanguage};

#[derive(Debug)]
pub enum LoginError {
    NoStored,

    Reqwest(reqwest::Error),
}

impl fmt::Display for LoginError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

#[derive(Copy, Clone)]
pub enum AccountRegion {
    Japan = 1,
    NorthAmerica = 2,
    Europe = 3,
}

pub struct LoginResult {
    session_id: String,
    can_play: bool,
    terms_accepted: bool,
    entitled_expansion: i16,
    region: AccountRegion,
}

/// Login to Square Enix oauth with the supplied credentials
pub async fn login(
    username: &str,
    password: &str,
    otp: &str,
    steam_service: bool,
    region: AccountRegion,
) -> Result<LoginResult, LoginError> {
    let stored = stored(steam_service, region).await?;
    println!("{}", stored);

    Ok(LoginResult {
        session_id: "0".to_string(),
        can_play: true,
        terms_accepted: true,
        entitled_expansion: 4,
        region: AccountRegion::Japan,
    })
}

async fn stored(steam_service: bool, region: AccountRegion) -> Result<String, LoginError> {
    let url = constants::oauth_top_url(region, false, steam_service);
    println!("{}", url);

    let resp = request::launcher_get(url)
        .header("Referer", request::launcher_referer(ClientLanguage::English))
        .header("Accept", "image/gif, image/jpeg, image/pjpeg, application/x-ms-application, application/xaml+xml, application/x-ms-xbap, */*")
        .header("Accept-Encoding", "gzip, deflate")
        .header("Accept-Language", "en-US")
        .send().await.map_err(LoginError::Reqwest)?;

    let text = resp.text().await.map_err(LoginError::Reqwest)?;

    lazy_static! {
        static ref RE_STORED: Regex =
            Regex::new(r#"(?m)<\s*input .* name="_STORED_" value="(.*)">"#).unwrap();
    }

    if let Some(stored_captures) = RE_STORED.captures_iter(&text).next() {
        return Ok(stored_captures[1].to_string());
    }

    Err(LoginError::NoStored)
}
