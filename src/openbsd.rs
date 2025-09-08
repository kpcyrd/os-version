use anyhow::Result;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct OpenBSD {
    pub version: String,
}

impl OpenBSD {
    #[cfg(target_os = "openbsd")]
    pub fn detect() -> Result<OpenBSD> {
        let uname = uname::Info::new()?;
        Ok(OpenBSD {
            version: uname.release,
        })
    }

    #[cfg(not(target_os = "openbsd"))]
    pub fn detect() -> Result<OpenBSD> {
        unreachable!()
    }
}

impl fmt::Display for OpenBSD {
    fn fmt(&self, w: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(w, "openbsd {}", self.version)
    }
}
