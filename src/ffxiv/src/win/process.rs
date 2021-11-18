use std::pin::Pin;
use windows::Win32::UI::WindowsAndMessaging::{
    EnumThreadWindows, EnumWindows, GetWindow, GetWindowThreadProcessId, IsWindowVisible,
};

// TODO (Chiv) Drop?
// TODO (Chiv) Addons? Wrap addons? Only thread FFXIV process special?
#[derive(Debug, Copy, Clone)]
pub struct Process {
    process_handle: windows::Win32::Foundation::HANDLE,
    process_id: u32,
    thread_id: u32,
}

impl Process {
    pub fn id(&self) -> u32 {
        self.process_id
    }

    pub(crate) fn has_exited(&self) -> bool {
        //TODO
        false
    }

    pub(crate) fn main_window(
        &self,
    ) -> Result<windows::Win32::Foundation::HWND, Box<dyn std::error::Error>> {
        // TODO(Chiv) C# Process does a few more security checks.
        //  Might need to find a sophisticated package or roll one ourselves maybe
        // TODO (Chiv) Should that be Pin<T>? Should that be better with just a struct?
        tracing::debug!("Enum windows");
        let mut finder = Box::new(WindowFinder {
            process_id: self.process_id,
            best_handle: Default::default(),
        });
        finder.find_main_window()
    }
}

impl From<windows::Win32::System::Threading::PROCESS_INFORMATION> for Process {
    fn from(t: windows::Win32::System::Threading::PROCESS_INFORMATION) -> Self {
        Self {
            process_handle: t.hProcess,
            process_id: t.dwProcessId,
            thread_id: t.dwThreadId,
        }
    }
}

struct WindowFinder {
    process_id: u32,
    best_handle: windows::Win32::Foundation::HWND,
}

impl WindowFinder {
    fn find_main_window(
        mut self: Box<Self>,
    ) -> Result<windows::Win32::Foundation::HWND, Box<dyn std::error::Error>> {
        let ptr = Box::into_raw(self);
        unsafe {
            EnumWindows(
                Some(Self::enum_callback),
                windows::Win32::Foundation::LPARAM(ptr as _),
            )
        }; //TODO (Chiv) TO we care about errors here?

        let this = unsafe { Box::from_raw(ptr) };
        tracing::debug!("Found best: {:?}", &this.best_handle);
        Ok(this.best_handle)
    }

    unsafe fn is_main_window(hwnd: windows::Win32::Foundation::HWND) -> bool {
        !(GetWindow(hwnd, windows::Win32::UI::WindowsAndMessaging::GW_OWNER).0 != 0)
            && IsWindowVisible(hwnd).as_bool()
    }

    unsafe extern "system" fn enum_callback(
        hwnd: windows::Win32::Foundation::HWND,
        this: windows::Win32::Foundation::LPARAM,
    ) -> windows::Win32::Foundation::BOOL {
        let mut process_id = 0;
        tracing::debug!("In enum call back with: {:?}", hwnd);
        GetWindowThreadProcessId(hwnd, &mut process_id);
        if process_id != (*(this.0 as *mut Self)).process_id || !Self::is_main_window(hwnd) {
            true.into()
        } else {
            (*(this.0 as *mut Self)).best_handle = hwnd;
            tracing::debug!("In enum call back; BEST HANDLE: {:?}", hwnd);
            false.into()
        }
    }
}
