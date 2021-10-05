pub static FRONTIER_GATE_STATUS_URL: &str = "https://frontier.ffxiv.com/worldStatus/gate_status.json";
pub static FRONTIER_LOGIN_STATUS_URL: &str = "https://frontier.ffxiv.com/worldStatus/login_status.json";

pub static PATCH_BOOTVER_URL: &str = "http://patch-bootver.ffxiv.com/http/win32/ffxivneo_release_boot/{}/?time={}";
pub static PATCH_GAMEVER_URL: &str = "https://patch-gamever.ffxiv.com/http/win32/ffxivneo_release_game/{}/{}";

pub static OAUTH_TOP_URL: &str = "https://ffxiv-login.square-enix.com/oauth/ffxivarr/login/top?lng=en&rgn={}&isft=0&cssmode=1&isnew=1&issteam={}";
pub static OAUTH_SEND_URL: &str = "https://ffxiv-login.square-enix.com/oauth/ffxivarr/login/login.send";

pub static USER_AGENT_TEMPLATE: &str = "SQEXAuthor/2.0.0(Windows 6.2; ja-jp; {0})";

pub static STEAM_APP_ID: i32 = 39210;

pub static PATCH_GAMEVER_HASHES: [&'static str; 6] = [
    "ffxivboot.exe",
    "ffxivboot64.exe",
    "ffxivlauncher.exe",
    "ffxivlauncher64.exe",
    "ffxivupdater.exe",
    "ffxivupdater64.exe",
];
