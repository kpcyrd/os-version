use anyhow::Result;

#[derive(Debug, PartialEq)]
pub struct Linux {
    pub distro: String,
    pub version: Option<String>,
    pub version_name: Option<String>,
}

impl Linux {
    #[cfg(target_os="linux")]
    pub fn detect() -> Result<Linux> {
        let file = std::fs::read_to_string("/etc/os-release")?;
        parse(&file)
    }

    #[cfg(not(target_os="linux"))]
    pub fn detect() -> Result<Linux> {
        unreachable!()
    }
}

impl ToString for Linux {
    fn to_string(&self) -> String {
        if let Some(version) = &self.version {
            format!("{} {}", self.distro, version)
        } else {
            self.distro.to_string()
        }
    }
}

#[cfg(target_os="linux")]
fn parse(file: &str) -> Result<Linux> {
    use anyhow::Error;

    let mut distro = None;
    let mut version = None;
    let mut version_name = None;

    for line in file.lines() {
        if let Some(remaining) = line.strip_prefix("ID=") {
            distro = Some(parse_value(remaining)?);
        } else if let Some(remaining) = line.strip_prefix("VERSION_") {
            if let Some(remaining) = remaining.strip_prefix("ID=") {
                version = Some(parse_value(remaining)?);
            } else if let Some(remaining) = remaining.strip_prefix("CODENAME=") {
                version_name = Some(parse_value(remaining)?);
            }
        }
    }

    let distro = distro
        .ok_or_else(|| Error::msg("Mandatory ID= field is missing"))?;

    Ok(Linux {
        distro,
        version,
        version_name,
    })
}

#[cfg(target_os="linux")]
fn parse_value(mut value: &str) -> Result<String> {
    if value.starts_with('"') && value.ends_with('"') && value.len() >= 2 {
        value = &value[1..value.len()-1];
    }

    Ok(value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(target_os="linux")]
    fn detect_debian() {
        let os_release = parse(r#"
NAME="Debian GNU/Linux"
VERSION_ID="10"
VERSION="10 (buster)"
VERSION_CODENAME=buster
ID=debian
HOME_URL="https://www.debian.org/"
SUPPORT_URL="https://www.debian.org/support"
BUG_REPORT_URL="https://bugs.debian.org/"
"#).unwrap();
        assert_eq!(Linux {
            distro: "debian".to_string(),
            version: Some("10".to_string()),
            version_name: Some("buster".to_string()),
        }, os_release);
    }

    #[test]
    #[cfg(target_os="linux")]
    fn detect_archlinux() {
        let os_release = parse(r#"
NAME="Arch Linux"
PRETTY_NAME="Arch Linux"
ID=arch
BUILD_ID=rolling
ANSI_COLOR="0;36"
HOME_URL="https://www.archlinux.org/"
DOCUMENTATION_URL="https://wiki.archlinux.org/"
SUPPORT_URL="https://bbs.archlinux.org/"
BUG_REPORT_URL="https://bugs.archlinux.org/"
LOGO=archlinux
"#).unwrap();
        assert_eq!(Linux {
            distro: "arch".to_string(),
            version: None,
            version_name: None,
        }, os_release);
    }

    #[test]
    #[cfg(target_os="linux")]
    fn detect_alpine() {
        let os_release = parse(r#"
NAME="Alpine Linux"
ID=alpine
VERSION_ID=3.11.5
PRETTY_NAME="Alpine Linux v3.11"
HOME_URL="https://alpinelinux.org/"
BUG_REPORT_URL="https://bugs.alpinelinux.org/"
"#).unwrap();
        assert_eq!(Linux {
            distro: "alpine".to_string(),
            version: Some("3.11.5".to_string()),
            version_name: None,
        }, os_release);
    }

    #[test]
    #[cfg(target_os="linux")]
    fn detect_ubuntu() {
        let os_release = parse(r#"
NAME="Ubuntu"
VERSION="18.04.4 LTS (Bionic Beaver)"
ID=ubuntu
ID_LIKE=debian
PRETTY_NAME="Ubuntu 18.04.4 LTS"
VERSION_ID="18.04"
HOME_URL="https://www.ubuntu.com/"
SUPPORT_URL="https://help.ubuntu.com/"
BUG_REPORT_URL="https://bugs.launchpad.net/ubuntu/"
PRIVACY_POLICY_URL="https://www.ubuntu.com/legal/terms-and-policies/privacy-policy"
VERSION_CODENAME=bionic
UBUNTU_CODENAME=bionic
"#).unwrap();
        assert_eq!(Linux {
            distro: "ubuntu".to_string(),
            version: Some("18.04".to_string()),
            version_name: Some("bionic".to_string()),
        }, os_release);
    }

    #[test]
    #[cfg(target_os="linux")]
    fn detect_centos() {
        let os_release = parse(r#"
NAME="CentOS Linux"
VERSION="8 (Core)"
ID="centos"
ID_LIKE="rhel fedora"
VERSION_ID="8"
PLATFORM_ID="platform:el8"
PRETTY_NAME="CentOS Linux 8 (Core)"
ANSI_COLOR="0;31"
CPE_NAME="cpe:/o:centos:centos:8"
HOME_URL="https://www.centos.org/"
BUG_REPORT_URL="https://bugs.centos.org/"

CENTOS_MANTISBT_PROJECT="CentOS-8"
CENTOS_MANTISBT_PROJECT_VERSION="8"
REDHAT_SUPPORT_PRODUCT="centos"
REDHAT_SUPPORT_PRODUCT_VERSION="8"

"#).unwrap();
        assert_eq!(Linux {
            distro: "centos".to_string(),
            version: Some("8".to_string()),
            version_name: None,
        }, os_release);
    }
}
