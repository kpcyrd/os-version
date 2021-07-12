use anyhow::Result;

#[derive(Debug)]
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

impl ToString for Windows {
    fn to_string(&self) -> String {
        format!("windows {}", self.version)
    }
}
