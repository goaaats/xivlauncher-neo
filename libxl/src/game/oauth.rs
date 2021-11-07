use core::fmt;
use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;
use reqwest::multipart;

use crate::game::{constants, request, platform::Platform, ClientLanguage};

#[derive(Debug)]
pub enum LoginError {
    NoStored,
    NoResultMatch,

    Account(String),
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
    platform: Platform,
    steam_service: bool,
    region: AccountRegion,
) -> Result<LoginResult, LoginError> {
    let stored = stored(steam_service, region).await?;

    let mut form = HashMap::new();
    form.insert("_STORED_", stored);
    form.insert("sqexid", username.to_string());
    form.insert("password", password.to_string());
    form.insert("otppw", otp.to_string());

    let resp = request::launcher_get(constants::OAUTH_SEND_URL)
        .header("Cache-Control", "no-cache")
        .header("Accept", "image/gif, image/jpeg, image/pjpeg, application/x-ms-application, application/xaml+xml, application/x-ms-xbap, */*")
        .header("Accept-Encoding", "gzip, deflate")
        .header("Accept-Language", "en-US,en;q=0.9")
        .header("Referer", request::launcher_referer(ClientLanguage::English))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&form)
        .send().await.map_err(LoginError::Reqwest)?;

    let text = resp.text().await.map_err(LoginError::Reqwest)?;

    lazy_static! {
        static ref RE_LOGINRES: Regex =
            Regex::new(r#"(?m)<\s*input .* name="_STORED_" value="(.*)">"#).unwrap();
    }
    
    if let Some(login_captures) = RE_LOGINRES.captures_iter(&text).next() {
        let params = login_captures[1]

        return Ok(login_captures[1].to_string());
    }
    
    Err(LoginError::NoResultMatch)
}

fn get_oauth_referer(language: ClientLanguage, region: AccountRegion, steam_service: bool) -> String {
    
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
