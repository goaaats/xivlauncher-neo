use crate::game::oauth::AccountRegion;
use crate::util;

pub fn get_frontier_gate_status_url() -> String {
    format!("https://frontier.ffxiv.com/worldStatus/gate_status.json?_={}", util::get_utc_millis())
}

pub fn get_frontier_login_status_url() -> String {
    format!("https://frontier.ffxiv.com/worldStatus/login_status.json?_={}", util::get_utc_millis())
}

pub static PATCH_CLIENT_USER_AGENT: &str = "FFXIV PATCH CLIENT";

/// fmt 1: language code
/// fmt 2: unix time
pub static FRONTIER_NOTICE_ULR: &str = "https://frontier.ffxiv.com/v2/notice/{}/message.json?_={}";

pub static PATCH_BOOTVER_URL: &str = "http://patch-bootver.ffxiv.com/http/win32/ffxivneo_release_boot/{}/?time={}";
pub static PATCH_GAMEVER_URL: &str = "https://patch-gamever.ffxiv.com/http/win32/ffxivneo_release_game/{}/{}";

pub static OAUTH_TOP_URL: &str = "https://ffxiv-login.square-enix.com/oauth/ffxivarr/login/top?lng=en&rgn={}&isft=0&cssmode=1&isnew=1&issteam={}";

pub fn get_oauth_top_url(region: &AccountRegion, free_trial: bool, steam_service: bool) -> String {
    format!("https://ffxiv-login.square-enix.com/oauth/ffxivarr/login/top?lng=en&rgn={}&isft={}&cssmode=1&isnew=1&issteam={}", *region as u8, free_trial as u8, steam_service as u8).to_string()
}

pub static OAUTH_SEND_URL: &str = "https://ffxiv-login.square-enix.com/oauth/ffxivarr/login/login.send";

pub static STEAM_APP_ID: i32 = 39210;

pub static PATCH_GAMEVER_HASHES: [&'static str; 6] = [
    "ffxivboot.exe",
    "ffxivboot64.exe",
    "ffxivlauncher.exe",
    "ffxivlauncher64.exe",
    "ffxivupdater.exe",
    "ffxivupdater64.exe",
];
