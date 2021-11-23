use core::fmt;
use lazy_static::lazy_static;
use linked_hash_map::LinkedHashMap;
use regex::Regex;
use std::collections::HashMap;
use strum_macros::EnumString;

use crate::game::constants;
use crate::game::language::GameLanguage;
use crate::game::platform::Platform;
use crate::game::request;

use super::region::AccountRegion;

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

#[derive(Debug)]
pub struct OauthLogin {
  pub session_id: String,
  pub can_play: bool,
  pub terms_accepted: bool,
  pub entitled_expansion: i16,
  pub region: AccountRegion,
}

impl OauthLogin {
  /// Login to Square Enix oauth with the supplied credentials
  pub async fn login(
    username: &str,
    password: &str,
    otp: &str,
    platform: Platform,
    steam_service: bool,
    region: AccountRegion,
  ) -> Result<Self, LoginError> {
    let stored = stored(steam_service, region).await?;

    let mut form = LinkedHashMap::new();
    form.insert("_STORED_", stored);
    form.insert("sqexid", username.to_string());
    form.insert("password", password.to_string());
    form.insert("otppw", otp.to_string());

    let req = request::launcher_post(constants::OAUTH_SEND_URL)
    .header("Cache-Control", "no-cache")
    .header(
      "Accept",
      "image/gif, image/jpeg, image/pjpeg, application/x-ms-application, application/xaml+xml, application/x-ms-xbap, */*",
    )
    .header("Accept-Encoding", "gzip, deflate")
    .header("Accept-Language", "en-US,en;q=0.9")
    .header("Referer", oauth_referer(GameLanguage::English, region, steam_service))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .header("Connection", "Keep-Alive")
    .header("Cookie", "_rsid=\"\"")
    .form(&form);

    let resp = req.send().await.map_err(LoginError::Reqwest)?;
    let text = resp.text().await.map_err(LoginError::Reqwest)?;

    lazy_static! {
      static ref RE_LOGINRES: Regex = Regex::new(r#"window\.external\.user\("login=auth,ok,(.*)"\);"#).unwrap();
      static ref RE_LOGINERR: Regex = Regex::new(r#"window\.external\.user\("login=auth,ng,err,(.*)"\);"#).unwrap();
    }

    if let Some(login_captures) = RE_LOGINRES.captures_iter(&text).next() {
      let val = login_captures[1].to_string();
      let parts: Vec<&str> = val.split(',').collect();

      return Ok(Self {
        session_id: parts[1].to_string(),
        can_play: parts[9] != "0",
        terms_accepted: parts[3] != "0",
        entitled_expansion: parts[13].parse().unwrap(),
        region: AccountRegion::from(parts[5].parse::<u8>().unwrap()),
    });
    }

    if let Some(error_captures) = RE_LOGINERR.captures_iter(&text).next() {
      return Err(LoginError::Account(error_captures[1].to_string()));
    }

    Err(LoginError::NoResultMatch)
  }
}

fn oauth_referer(language: GameLanguage, region: AccountRegion, steam_service: bool) -> String {
  format!(
    "https://ffxiv-login.square-enix.com/oauth/ffxivarr/login/top?lng={}&rgn={}&isft=0&cssmode=1&isnew=1&issteam={}",
    language.langcode_short(),
    u8::from(region),
    if steam_service { "1" } else { "0" }
  )
}

async fn stored(steam_service: bool, region: AccountRegion) -> Result<String, LoginError> {
  let url = constants::oauth_top_url(region, false, steam_service);

  let resp = request::launcher_get(url)
    .header("Referer", request::launcher_referer(GameLanguage::English))
    .header(
      "Accept",
      "image/gif, image/jpeg, image/pjpeg, application/x-ms-application, application/xaml+xml, application/x-ms-xbap, */*",
    )
    .header("Accept-Encoding", "gzip, deflate")
    .header("Accept-Language", "en-US")
    .header("Connection", "Keep-Alive")
    .header("Cookie", "_rsid=\"\"")
    .send()
    .await
    .map_err(LoginError::Reqwest)?;

  let text = resp.text().await.map_err(LoginError::Reqwest)?;

  lazy_static! {
    static ref RE_STORED: Regex = Regex::new(r#"(?m)<\s*input .* name="_STORED_" value="(.*)">"#).unwrap();
  }

  if let Some(stored_captures) = RE_STORED.captures_iter(&text).next() {
    return Ok(stored_captures[1].to_string());
  }

  Err(LoginError::NoStored)
}
