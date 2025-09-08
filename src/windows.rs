use anyhow::Result;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Windows {
    pub version: String,
}

impl Windows {
    #[cfg(target_os = "windows")]
    pub fn detect() -> Result<Windows> {
        use crate::winapi;

        let version = winapi::version_info()?;
        let version = winapi::edition(&version);

        Ok(Windows { version })
    }

    #[cfg(not(target_os = "windows"))]
    pub fn detect() -> Result<Windows> {
        unreachable!()
    }
}

impl fmt::Display for Windows {
    fn fmt(&self, w: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(w, "windows {}", self.version)
    }
}
