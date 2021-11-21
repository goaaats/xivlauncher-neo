#[cfg(windows)]
impl<'a> crate::arguments::Builder<'a> {
    pub(crate) fn derive_key() -> u32 {
        unsafe {
            use windows::Win32::System::SystemInformation::GetTickCount;
            GetTickCount() & 0xFFFF_0000
        }
    }
}