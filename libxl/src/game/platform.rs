pub enum Platform {
    Win32,
    Mac,
}

pub fn get_patch_useragent(platform: Platform) -> &'static str {
    match platform {
        Platform::Win32 => "FFXIV PATCH CLIENT",
        Platform::Mac => "FFXIV PATCH CLIENT",
    }
}