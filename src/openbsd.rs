use anyhow::Result;

#[derive(Debug)]
pub struct OpenBSD {
    pub version: String,
}

impl OpenBSD {
    #[cfg(target_os="openbsd")]
    pub fn detect() -> Result<OpenBSD> {
        let uname = uname::Info::new()?;
        Ok(OpenBSD {
            version: uname.release,
        })
    }

    #[cfg(not(target_os="openbsd"))]
    pub fn detect() -> Result<OpenBSD> {
        unreachable!()
    }
}

impl ToString for OpenBSD {
    fn to_string(&self) -> String {
        format!("openbsd {}", self.version)
    }
}
