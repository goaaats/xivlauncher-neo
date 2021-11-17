pub enum Platform {
  Win32,
  Mac,
}

impl Platform {
  pub fn get_patch_useragent(&self) -> &'static str {
    match self {
      Platform::Win32 => "FFXIV PATCH CLIENT",
      Platform::Mac => "FFXIV PATCH CLIENT",
    }
  }
}
