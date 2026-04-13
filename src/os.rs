pub struct OsInfo {
    pub os_name: String,
    pub arch: String,
    pub kernel_version: String,
    pub os_version: String,
}

pub fn get_os_info() -> OsInfo {
    let os_name = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let kernel_version = std::process::Command::new("uname").arg("-r").output();
    let kernel_version_parsed = match kernel_version {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
        Err(e) => e.to_string(),
    };

    let os_version = get_os_version();

    OsInfo {
        os_name: os_name.to_string(),
        arch: arch.to_string(),
        kernel_version: kernel_version_parsed,
        os_version,
    }
}

#[cfg(target_os = "macos")]
fn get_os_version() -> String {
    let os_version = std::process::Command::new("sw_vers")
        .arg("-productVersion")
        .output();
    match os_version {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
        Err(e) => e.to_string(),
    }
}

#[cfg(target_os = "linux")]
fn get_os_version() -> String {
    parse_release_document_to_pretty_name()
}

#[cfg(target_os = "linux")]
fn parse_release_document_to_pretty_name() -> String {
    let content = std::fs::read_to_string("/etc/os-release");
    match content {
        Ok(text) => text
            .lines()
            .find(|line| line.starts_with("PRETTY_NAME="))
            .map(|line| line.strip_prefix("PRETTY_NAME=").unwrap_or(line))
            .map(|val| val.trim_matches('"'))
            .unwrap_or("unknown")
            .trim()
            .to_string(),
        Err(e) => e.to_string(),
    }
}