use anyhow::Result;

#[derive(Debug, PartialEq)]
pub struct Android {
}

impl Android {
    #[cfg(target_os="android")]
    pub fn detect() -> Result<Android> {
        Ok(Android {
        })
    }

    #[cfg(not(target_os="android"))]
    pub fn detect() -> Result<Android> {
        unreachable!()
    }
}

impl ToString for Android {
    fn to_string(&self) -> String {
        "android".to_string()
    }
}
