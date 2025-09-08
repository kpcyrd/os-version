use anyhow::Result;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Android {}

impl Android {
    #[cfg(target_os = "android")]
    pub fn detect() -> Result<Android> {
        Ok(Android {})
    }

    #[cfg(not(target_os = "android"))]
    pub fn detect() -> Result<Android> {
        unreachable!()
    }
}

impl fmt::Display for Android {
    fn fmt(&self, w: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(w, "android")
    }
}
