use crate::game::oauth::AccountRegion;
use crate::util;

pub fn frontier_gate_status_url() -> String {
    
    format!(
        "https://frontier.ffxiv.com/worldStatus/gate_status.json?_={}",
        util::now_millis()
    )
}

pub fn frontier_login_status_url() -> String {
    format!(
        "https://frontier.ffxiv.com/worldStatus/login_status.json?_={}",
        util::now_millis()
    )
}

pub const PATCH_CLIENT_USER_AGENT: &str = "FFXIV PATCH CLIENT";

/// fmt 1: language code
/// fmt 2: unix time
pub const FRONTIER_NOTICE_ULR: &str = "https://frontier.ffxiv.com/v2/notice/{}/message.json?_={}";

pub const PATCH_BOOTVER_URL: &str =
    "http://patch-bootver.ffxiv.com/http/win32/ffxivneo_release_boot/{}/?time={}";
pub const PATCH_GAMEVER_URL: &str =
    "https://patch-gamever.ffxiv.com/http/win32/ffxivneo_release_game/{}/{}";

pub const OAUTH_TOP_URL: &str = "https://ffxiv-login.square-enix.com/oauth/ffxivarr/login/top?lng=en&rgn={}&isft=0&cssmode=1&isnew=1&issteam={}";

pub fn oauth_top_url(region: &AccountRegion, free_trial: bool, steam_service: bool) -> String {
    format!("https://ffxiv-login.square-enix.com/oauth/ffxivarr/login/top?lng=en&rgn={}&isft={}&cssmode=1&isnew=1&issteam={}", *region as u8, free_trial as u8, steam_service as u8)
}

pub const OAUTH_SEND_URL: &str =
    "https://ffxiv-login.square-enix.com/oauth/ffxivarr/login/login.send";

pub const STEAM_APP_ID: i32 = 39210;

pub const PATCH_GAMEVER_HASHES: &[&str; 6] = &[
    "ffxivboot.exe",
    "ffxivboot64.exe",
    "ffxivlauncher.exe",
    "ffxivlauncher64.exe",
    "ffxivupdater.exe",
    "ffxivupdater64.exe",
];