#[cfg(windows)]
impl<'a> crate::arguments::Builder<'a> {
    pub(crate) unsafe fn derive_key() -> u32 {
        use windows::Win32::System::SystemInformation::GetTickCount;
        let tick_count = GetTickCount() & 0xFFFF_FFFF;
    
        tick_count & 0xFFFF_0000
    }
}