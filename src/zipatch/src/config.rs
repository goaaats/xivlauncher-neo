pub enum PlatformId {
    Win32 = 0,
    PS3 = 1,
    PS4 = 2,
    Unk = 3,
}

impl From<u8> for PlatformId {
    fn from(val: u8) -> Self {
        match val {
            0 => PlatformId::Win32,
            1 => PlatformId::PS3,
            2 => PlatformId::PS4,
            3 => PlatformId::Unk,
            _ => panic!("Unimplemented platform")
        }
    }
}

impl From<PlatformId> for u8 {
    fn from(val: PlatformId) -> Self {
        val as u8
    }
}