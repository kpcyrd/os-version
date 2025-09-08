use crate::{Android, Linux, MacOS, OpenBSD, OsVersion, Windows};
use anyhow::Result;

pub async fn detect() -> Result<OsVersion> {
    if cfg!(target_os = "linux") {
        Ok(OsVersion::Linux(Linux::detect_async().await?))
    } else if cfg!(target_os = "macos") {
        Ok(OsVersion::MacOS(MacOS::detect_async().await?))
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
