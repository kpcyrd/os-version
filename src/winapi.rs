// borrowed from https://github.com/DarkEld3r/os_info/blob/master/src/windows/winapi.rs

use anyhow::{Result, bail};
use std::mem;
use winapi::{
    shared::{
        minwindef::{DWORD, FARPROC},
        ntdef::{LPCSTR, NTSTATUS},
        ntstatus::STATUS_SUCCESS,
    },
    um::{
        libloaderapi::{GetModuleHandleA, GetProcAddress},
        sysinfoapi::{GetSystemInfo, SYSTEM_INFO},
        winnt::{PROCESSOR_ARCHITECTURE_AMD64, VER_NT_WORKSTATION, VER_SUITE_WH_SERVER},
        winuser::{GetSystemMetrics, SM_SERVERR2},
    },
};

#[cfg(target_arch = "x86")]
type OSVERSIONINFOEX = winapi::um::winnt::OSVERSIONINFOEXA;

#[cfg(not(target_arch = "x86"))]
type OSVERSIONINFOEX = winapi::um::winnt::OSVERSIONINFOEXW;

// Calls the Win32 API function RtlGetVersion to get the OS version information:
// https://msdn.microsoft.com/en-us/library/mt723418(v=vs.85).aspx
pub fn version_info() -> Result<OSVERSIONINFOEX> {
    let rtl_get_version = get_proc_address(b"ntdll\0", b"RtlGetVersion\0")?;

    type RtlGetVersion = unsafe extern "system" fn(&mut OSVERSIONINFOEX) -> NTSTATUS;
    let rtl_get_version: RtlGetVersion = unsafe { mem::transmute(rtl_get_version) };

    let mut info: OSVERSIONINFOEX = unsafe { mem::zeroed() };
    info.dwOSVersionInfoSize = mem::size_of::<OSVERSIONINFOEX>() as DWORD;

    if unsafe { rtl_get_version(&mut info) } == STATUS_SUCCESS {
        Ok(info)
    } else {
        bail!("Failed to get version")
    }
}

fn get_proc_address(module: &[u8], proc: &[u8]) -> Result<FARPROC> {
    assert!(
        *module.last().expect("Empty module name") == 0,
        "Module name should be zero-terminated"
    );
    assert!(
        *proc.last().expect("Empty procedure name") == 0,
        "Procedure name should be zero-terminated"
    );

    let handle = unsafe { GetModuleHandleA(module.as_ptr() as LPCSTR) };
    if handle.is_null() {
        bail!("GetModuleHandleA({}) failed", String::from_utf8_lossy(module));
    }

    unsafe { Ok(GetProcAddress(handle, proc.as_ptr() as LPCSTR)) }
}

// Examines data in the OSVERSIONINFOEX structure to determine the Windows edition:
// https://msdn.microsoft.com/en-us/library/windows/desktop/ms724833(v=vs.85).aspx
pub fn edition(version_info: &OSVERSIONINFOEX) -> String {
    match (
        version_info.dwMajorVersion,
        version_info.dwMinorVersion,
        version_info.wProductType,
    ) {
        // Windows 10.
        (10, 0, VER_NT_WORKSTATION) => "10".to_string(),
        (10, 0, _) => "server 2016".to_string(),
        // Windows Vista, 7, 8 and 8.1.
        (6, 3, VER_NT_WORKSTATION) => "8.1".to_string(),
        (6, 3, _) => "server 2012 r2".to_string(),
        (6, 2, VER_NT_WORKSTATION) => "8".to_string(),
        (6, 2, _) => "server 2012".to_string(),
        (6, 1, VER_NT_WORKSTATION) => "7".to_string(),
        (6, 1, _) => "server 2008 r2".to_string(),
        (6, 0, VER_NT_WORKSTATION) => "vista".to_string(),
        (6, 0, _) => "server 2008".to_string(),
        // Windows 2000, Home Server, 2003 Server, 2003 R2 Server, XP and XP Professional x64.
        (5, 1, _) => "xp".to_string(),
        (5, 0, _) => "2000".to_string(),
        (5, 2, _) if unsafe { GetSystemMetrics(SM_SERVERR2) } == 0 => {
            let mut info: SYSTEM_INFO = unsafe { mem::zeroed() };
            unsafe { GetSystemInfo(&mut info) };

            if Into::<DWORD>::into(version_info.wSuiteMask) & VER_SUITE_WH_SERVER
                == VER_SUITE_WH_SERVER
            {
                "home server".to_string()
            } else if version_info.wProductType == VER_NT_WORKSTATION
                && unsafe { info.u.s().wProcessorArchitecture } == PROCESSOR_ARCHITECTURE_AMD64
            {
                "xp professional x64 edition".to_string()
            } else {
                "server 2003".to_string()
            }
        },
        (major, minor, _) => format!("{}.{}", major, minor),
    }
}
