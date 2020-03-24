use anyhow::Result;

#[derive(Debug)]
pub struct OSX {
    pub version: String,
}

impl OSX {
    #[cfg(target_os="macos")]
    pub fn detect() -> Result<OSX> {
        let file = std::fs::read_to_string("/System/Library/CoreServices/SystemVersion.plist")?;
        parse(&file)
    }

    #[cfg(not(target_os="macos"))]
    pub fn detect() -> Result<OSX> {
        unreachable!()
    }
}

impl ToString for OSX {
    fn to_string(&self) -> String {
        format!("osx {}", self.version)
    }
}

#[cfg(target_os="macos")]
fn parse(file: &str) -> Result<OSX> {
    use anyhow::Error;

    let cur = std::io::Cursor::new(file.as_bytes());
    let v = plist::Value::from_reader(cur)?;

    let version = v.as_dictionary()
        .ok_or_else(|| Error::msg("SystemVersion.plist is not a dictionary"))?
        .get("ProductVersion")
        .ok_or_else(|| Error::msg("ProductVersion is missing"))?;

    let version = version.as_string()
        .ok_or_else(|| Error::msg("Version is not a string"))?
        .to_string();

    Ok(OSX {
        version,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(target_os="macos")]
    fn detect_osx() {
        let version = parse(r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>ProductBuildVersion</key>
    <string>17G11023</string>
    <key>ProductCopyright</key>
    <string>1983-2020 Apple Inc.</string>
    <key>ProductName</key>
    <string>Mac OS X</string>
    <key>ProductUserVisibleVersion</key>
    <string>10.13.6</string>
    <key>ProductVersion</key>
    <string>10.13.6</string>
</dict>
</plist>
"#);
        assert_eq!(OSX {
            version: "10.13.6".to_string(),
        }, version);
    }
}
