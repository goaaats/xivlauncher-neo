use std::ops::BitOr;
use windows::Win32::Foundation::{CloseHandle, BOOL, HANDLE, LUID};
use windows::Win32::Security::Authorization::{
    BuildExplicitAccessWithNameW, GetSecurityInfo, SetEntriesInAclW, SetSecurityInfo,
    EXPLICIT_ACCESS_W,
};
use windows::Win32::Security::{
    AdjustTokenPrivileges, InitializeSecurityDescriptor, LookupPrivilegeValueW, PrivilegeCheck,
    SetSecurityDescriptorDacl, ACL, LUID_AND_ATTRIBUTES, PRIVILEGE_SET, SECURITY_ATTRIBUTES,
    SECURITY_DESCRIPTOR, TOKEN_PRIVILEGES,
};
use windows::Win32::System::Threading::{
    CreateProcessW, GetCurrentProcess, OpenProcessToken, ResumeThread, WaitForInputIdle,
    PROCESS_INFORMATION, STARTUPINFOW,
};

const STANDARD_RIGHTS_ALL: u32 = 0x001F0000;
const SPECIFIC_RIGHTS_ALL: u32 = 0x0000FFFF;
const PROCESS_VM_WRITE: u32 = 0x0020;
const GRANT_ACCESS: u32 = 1;
const SECURITY_DESCRIPTOR_REVISION: u32 = 1;
const CREATE_SUSPENDED: u32 = 0x00000004;
const TOKEN_QUERY: u32 = 0x0008;
const TOKEN_ADJUST_PRIVILEGES: u32 = 0x0020;
const PRIVILEGE_SET_ALL_NECESSARY: u32 = 1;
const SE_PRIVILEGE_ENABLED: u32 = 0x00000002;
const SE_PRIVILEGE_REMOVED: u32 = 0x00000004;

#[cfg(windows)]
pub(crate) fn launch_with_fix(
    working_directory: &std::path::Path,
    executable: &std::path::Path,
    arguments: String,
    environmental_variables: String,
) -> Result<crate::win::Process, Box<dyn std::error::Error>> {
    // TODO ffi wrappers for all the unsafe and ffi windows stuff -> winsafe package?
    let mut username = username()?;
    let explicit_access = {
        let mut t = EXPLICIT_ACCESS_W::default();
        unsafe {
            BuildExplicitAccessWithNameW(
                &mut t,
                windows::Win32::Foundation::PWSTR(username.as_mut_ptr()),
                STANDARD_RIGHTS_ALL | SPECIFIC_RIGHTS_ALL & !PROCESS_VM_WRITE,
                windows::Win32::Security::Authorization::GRANT_ACCESS,
                windows::Win32::Security::NO_INHERITANCE,
            );
        };
        t
    };
    tracing::debug!("{:?}", explicit_access);
    let acl = {
        let mut t = std::ptr::null_mut();
        //tracing::debug!("Before {:?}", t);
        if unsafe { SetEntriesInAclW(1, &explicit_access, std::ptr::null(), &mut t) } != 0 {
            return Err(windows::runtime::Error::from_win32().into());
        }
        unsafe { tracing::debug!("PAST ACL: {:?}", *t) };
        t
    };
    tracing::debug!("Past Explicit Access ACL");
    let mut security_descriptor = {
        let mut t = SECURITY_DESCRIPTOR::default();
        unsafe { InitializeSecurityDescriptor(&mut t, SECURITY_DESCRIPTOR_REVISION) }.ok()?;
        t
    };
    tracing::debug!("Set Security Descriptor");
    unsafe { SetSecurityDescriptorDacl(&mut security_descriptor, true, acl, false) }.ok()?;
    // TODO(Chiv) Environment args string

    tracing::debug!("Past Security Descriptor");
    let process_attributes = SECURITY_ATTRIBUTES {
        nLength: std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32,
        lpSecurityDescriptor: (&mut security_descriptor as *mut SECURITY_DESCRIPTOR).cast(),
        bInheritHandle: windows::Win32::Foundation::BOOL::default(),
    };
    let startup_info = STARTUPINFOW {
        cb: std::mem::size_of::<STARTUPINFOW>() as u32,
        ..STARTUPINFOW::default()
    };

    // TODO (Chiv) Error, or duce canocolize or stuff :/. Strings and paths are hard
    let cmd_line = format!("{} {}", executable.as_os_str().to_str().unwrap(), arguments);
    // TODO (Chiv) Wrapper struct for process_information with all the needed methods like Process in C#.
    let process_information = {
        let mut t = PROCESS_INFORMATION::default();
        unsafe {
            CreateProcessW(
                windows::Win32::Foundation::PWSTR(std::ptr::null_mut()),
                cmd_line,
                &process_attributes,
                std::ptr::null(),
                false,
                windows::Win32::System::Threading::CREATE_SUSPENDED,
                std::ptr::null(),
                working_directory.as_os_str(),
                &startup_info,
                &mut t,
            )
        }
        .ok()?;
        t
    };

    disable_se_debug(process_information.hProcess)?;
    // TODO (Chiv) Error handling
    unsafe { ResumeThread(process_information.hThread) };

    // TODO (Chiv) Error handling
    unsafe {
        let result = WaitForInputIdle(process_information.hProcess, u32::MAX);
        /* C# source
          case 0:
           return true;
         case 258:
           return false;
        */
        tracing::debug!("WaitForInputIdle Result {:?}", result);
    };

    let acl = {
        let mut t = std::ptr::null_mut();
        unsafe {
            if GetSecurityInfo(
                GetCurrentProcess(),
                windows::Win32::Security::Authorization::SE_KERNEL_OBJECT,
                windows::Win32::Security::DACL_SECURITY_INFORMATION.0,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut t,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            ) != 0
            {
                return Err(windows::runtime::Error::from_win32().into());
            }
        }
        t
    };
    tracing::debug!("Past Second Get ACL");
    unsafe {
        if SetSecurityInfo(
            process_information.hProcess,
            windows::Win32::Security::Authorization::SE_KERNEL_OBJECT,
            windows::Win32::Security::DACL_SECURITY_INFORMATION
                .bitor(windows::Win32::Security::UNPROTECTED_DACL_SECURITY_INFORMATION)
                .0,
            windows::Win32::Foundation::PSID::default(),
            windows::Win32::Foundation::PSID::default(),
            acl,
            std::ptr::null_mut(),
        ) != 0
        {
            return Err(windows::runtime::Error::from_win32().into());
        }
    };
    tracing::debug!("Past setting second ACL");
    unsafe { CloseHandle(process_information.hThread) }.ok()?;
    tracing::debug!("Past closing thread handle");
    Ok(process_information.into())
}

#[cfg(windows)]
fn disable_se_debug(process: HANDLE) -> Result<(), Box<dyn std::error::Error>> {
    let token = {
        let mut t = windows::Win32::Foundation::HANDLE::default();
        unsafe {
            OpenProcessToken(
                process,
                windows::Win32::Security::TOKEN_QUERY
                    .bitor(windows::Win32::Security::TOKEN_ADJUST_PRIVILEGES),
                &mut t,
            )
        }
        .ok()?;
        t
    };
    let luid_debug_privilege = {
        let mut t = LUID::default();
        unsafe {
            LookupPrivilegeValueW(
                windows::Win32::Foundation::PWSTR(std::ptr::null_mut()),
                "SeDebugPrivilege",
                &mut t,
            )
        }
        .ok()?;
        t
    };
    let privilege_enabled = {
        let mut required_privileges = PRIVILEGE_SET {
            PrivilegeCount: 1,
            Control: PRIVILEGE_SET_ALL_NECESSARY,
            Privilege: [LUID_AND_ATTRIBUTES {
                Luid: luid_debug_privilege,
                Attributes: windows::Win32::Security::SE_PRIVILEGE_ENABLED,
            }],
        };
        let mut result = 0;
        unsafe { PrivilegeCheck(token, &mut required_privileges, &mut result) }.ok()?;
        result == 1 // result is BOOL; if TRUE => 1
    };
    // SeDebugPrivilege is enabled; try disabling
    if privilege_enabled {
        let mut token_privileges = TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            Privileges: [LUID_AND_ATTRIBUTES {
                Luid: luid_debug_privilege,
                Attributes: windows::Win32::Security::SE_PRIVILEGE_REMOVED,
            }],
        };
        unsafe {
            AdjustTokenPrivileges(
                token,
                false,
                &mut token_privileges,
                0,
                std::ptr::null_mut(),
                &mut 0,
            )
        }
        .ok()?;
    }
    unsafe { CloseHandle(token) }.ok()?;
    Ok(())
}

#[cfg(test)]
#[test]
fn bitwise_permissions_same_as_c_sharp() {
    assert_eq!(
        0x1F_FF_DF, //PInvoke.STANDARD_RIGHTS_ALL | PInvoke.SPECIFIC_RIGHTS_ALL & ~PInvoke.PROCESS_VM_WRITE
        STANDARD_RIGHTS_ALL | SPECIFIC_RIGHTS_ALL & !PROCESS_VM_WRITE
    );
}

#[cfg(windows)]
fn username() -> Result<Vec<u16>, Box<dyn std::error::Error>> {
    // UNLEN + 1
    // https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getusernamea#parameters
    let mut buffer = [0u16; 257];
    let mut capacity = 257;
    unsafe {
        windows::Win32::System::WindowsProgramming::GetUserNameW(
            windows::Win32::Foundation::PWSTR(buffer.as_mut_ptr()),
            &mut capacity,
        )
    }
    .ok()?;
    tracing::debug!("Username Capacity returned: {:?}", capacity);
    tracing::debug!(
        "Username returned: {:?}",
        String::from_utf16(&buffer[0..capacity as usize])
    );
    // TODO (Chiv) This works because ? does converting magic. Feels not correct, might be a non-issue
    //  as soon as actual error types and not dyn Error are defined.
    Ok(Vec::from(&buffer[0..capacity as usize]))
}
