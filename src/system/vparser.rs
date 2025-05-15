use super::version::Version;
use std::io;
use std::process::Command;

pub struct VParser;

impl VParser {
    pub fn parse_linux_version() -> io::Result<Version> {
        let output = Command::new("uname").arg("-r").output()?;

        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("uname command failed with status {}", output.status),
            ));
        }

        let version_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        parse_version(&version_str).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }

    pub fn parse_nftables_version() -> io::Result<Version> {
        let output = Command::new("nft").arg("--version").output()?;

        if !output.status.success() {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("nft command failed with status {}", output.status),
            ));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(pos) = stdout.find('v') {
            let after_v = &stdout[pos + 1..];
            let version_str = after_v
                .split(|c: char| c.is_whitespace() || c == '(')
                .next()
                .unwrap_or("");

            parse_version(version_str).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Failed to find version string",
            ))
        }
    }
}

fn parse_version(s: &str) -> Result<Version, String> {
    let version_part = s.split('-').next().unwrap_or("");
    let parts: Vec<&str> = version_part.split('.').collect();

    if parts.len() != 3 {
        return Err(format!(
            "Invalid version format: '{}', expected 'major.minor.patch'",
            s
        ));
    }

    let major = parts[0]
        .parse::<u32>()
        .map_err(|_| format!("Invalid major version in '{}'", s))?;
    let minor = parts[1]
        .parse::<u32>()
        .map_err(|_| format!("Invalid minor version in '{}'", s))?;
    let patch = parts[2]
        .parse::<u32>()
        .map_err(|_| format!("Invalid patch version in '{}'", s))?;

    Ok(Version::new(major, minor, patch))
}
