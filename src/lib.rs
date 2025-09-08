use anyhow::Result;
use std::fmt;

mod linux;
pub use crate::linux::Linux;
mod macos;
pub use crate::macos::MacOS;
mod windows;
pub use crate::windows::Windows;
mod android;
pub use crate::android::Android;
mod openbsd;
pub use crate::openbsd::OpenBSD;

#[cfg(target_os = "windows")]
mod winapi;

#[cfg(feature = "tokio")]
pub mod tokio;

pub fn detect() -> Result<OsVersion> {
    if cfg!(target_os = "linux") {
        Ok(OsVersion::Linux(Linux::detect()?))
    } else if cfg!(target_os = "macos") {
        Ok(OsVersion::MacOS(MacOS::detect()?))
    } else if cfg!(target_os = "windows") {
        Ok(OsVersion::Windows(Windows::detect()?))
    } else if cfg!(target_os = "android") {
        Ok(OsVersion::Android(Android::detect()?))
    } else if cfg!(target_os = "openbsd") {
        Ok(OsVersion::OpenBSD(OpenBSD::detect()?))
    } else {
        Ok(OsVersion::Unknown)
    }
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum OsVersion {
    Linux(Linux),
    MacOS(MacOS),
    Windows(Windows),
    Android(Android),
    OpenBSD(OpenBSD),
    Unknown,
}

impl fmt::Display for OsVersion {
    fn fmt(&self, w: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OsVersion::Linux(v) => v.fmt(w),
            OsVersion::MacOS(v) => v.fmt(w),
            OsVersion::Windows(v) => v.fmt(w),
            OsVersion::Android(v) => v.fmt(w),
            OsVersion::OpenBSD(v) => v.fmt(w),
            OsVersion::Unknown => write!(w, "unknown"),
        }
    }
}
