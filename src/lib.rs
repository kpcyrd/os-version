use anyhow::Result;

mod linux;
pub use crate::linux::Linux;
mod osx;
pub use crate::osx::OSX;
mod windows;
pub use crate::windows::Windows;
mod openbsd;
pub use crate::openbsd::OpenBSD;

#[cfg(target_os="windows")]
mod winapi;

pub fn detect() -> Result<OsVersion> {
    if cfg!(target_os = "linux") {
        Ok(OsVersion::Linux(Linux::detect()?))
    } else if cfg!(target_os = "macos") {
        Ok(OsVersion::OSX(OSX::detect()?))
    } else if cfg!(target_os = "windows") {
        Ok(OsVersion::Windows(Windows::detect()?))
    } else if cfg!(target_os = "openbsd") {
        Ok(OsVersion::OpenBSD(OpenBSD::detect()?))
    } else {
        Ok(OsVersion::Unknown)
    }
}

#[derive(Debug)]
pub enum OsVersion {
    Linux(Linux),
    OSX(OSX),
    Windows(Windows),
    OpenBSD(OpenBSD),
    Unknown,
}

impl ToString for OsVersion {
    fn to_string(&self) -> String {
        match self {
            OsVersion::Linux(v) => v.to_string(),
            OsVersion::OSX(v) => v.to_string(),
            OsVersion::Windows(v) => v.to_string(),
            OsVersion::OpenBSD(v) => v.to_string(),
            OsVersion::Unknown => String::from("unknown"),
        }
    }
}
