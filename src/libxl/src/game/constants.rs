use crate::util;

use super::region::AccountRegion;

pub fn frontier_gate_status_url() -> String {
  format!(
    "https://frontier.ffxiv.com/worldStatus/gate_status.json?_={}",
    util::time::utc_now_millis()
  )
}

pub fn frontier_login_status_url() -> String {
  format!(
    "https://frontier.ffxiv.com/worldStatus/login_status.json?_={}",
    util::time::utc_now_millis()
  )
}

pub fn patch_bootver_url(version: &str) -> String {
  format!(
    "http://patch-bootver.ffxiv.com/http/win32/ffxivneo_release_boot/{}/?time={}",
    version,
    util::time::utc_now_launcher_formatted_long()
  )
}

pub fn patch_gamever_url(version: &str, sid: &str) -> String {
  format!(
    "https://patch-gamever.ffxiv.com/http/win32/ffxivneo_release_game/{}/{}",
    version,
    sid
  )
}

/// fmt 1: language code
/// fmt 2: unix time
pub const FRONTIER_NOTICE_ULR: &str = "https://frontier.ffxiv.com/v2/notice/{}/message.json?_={}";

pub fn oauth_top_url(region: AccountRegion, free_trial: bool, steam_service: bool) -> String {
  format!(
    "https://ffxiv-login.square-enix.com/oauth/ffxivarr/login/top?lng=en&rgn={}&isft={}&cssmode=1&isnew=1&issteam={}",
    region as u8, free_trial as u8, steam_service as u8
  )
}

pub const OAUTH_SEND_URL: &str = "https://ffxiv-login.square-enix.com/oauth/ffxivarr/login/login.send";

pub const STEAM_APP_ID: i32 = 39210;

pub const DEFAULT_VERSION: &str = "2012.01.01.0000.0000";

pub const PATCH_GAMEVER_HASHES: &[&str; 6] = &[
  "ffxivboot.exe",
  "ffxivboot64.exe",
  "ffxivlauncher.exe",
  "ffxivlauncher64.exe",
  "ffxivupdater.exe",
  "ffxivupdater64.exe",
];
