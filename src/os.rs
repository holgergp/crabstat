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
        Ok(text) => parse_release_document_contents(&text),
        Err(e) => e.to_string(),
    }
}

#[allow(dead_code)]
fn parse_release_document_contents(text: &str) -> String {
    text.lines()
        .find(|line| line.starts_with("PRETTY_NAME="))
        .map(|line| line.strip_prefix("PRETTY_NAME=").unwrap_or(line))
        .map(|val| val.trim_matches('"'))
        .unwrap_or("unknown")
        .trim()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_release_document_contents() {
        const OS_RELEASE: &str = "\
NAME=\"Fedora Linux\"\n\
VERSION=\"41 (Workstation Edition)\"\n\
ID=fedora\n\
VERSION_ID=41\n\
VERSION_CODENAME=\"\"\n\
PLATFORM_ID=\"platform:f41\"\n\
PRETTY_NAME=\"Fedora Linux 41 (Workstation Edition)\"\n\
ANSI_COLOR=\"0;38;2;60;110;180\"\n\
LOGO=fedora-logo-icon\n\
CPE_NAME=\"cpe:/o:fedoraproject:fedora:41\"\n\
DEFAULT_HOSTNAME=\"fedora\"\n\
HOME_URL=\"https://fedoraproject.org/\"\n\
DOCUMENTATION_URL=\"https://docs.fedoraproject.org/en-US/fedora/f41/system-administrators-guide/\"\n\
SUPPORT_URL=\"https://ask.fedoraproject.org/\"\n\
BUG_REPORT_URL=\"https://bugzilla.redhat.com/\"\n\
REDHAT_BUGZILLA_PRODUCT=\"Fedora\"\n\
REDHAT_BUGZILLA_PRODUCT_VERSION=41\n\
REDHAT_SUPPORT_TICKET_URL=\"https://access.redhat.com/support/cases/#/case/new\"\n\
SUPPORT_END=\"2025-05-13\"\n\
VARIANT=\"Workstation Edition\"\n\
VARIANT_ID=workstation";
        assert_eq!(
            "Fedora Linux 41 (Workstation Edition)",
            parse_release_document_contents(OS_RELEASE)
        )
    }

    #[test]
    fn test_parse_release_document_missing_prettyname() {
        const OS_RELEASE: &str = "\
NAME=\"Fedora Linux\"\n\
ANSI_COLOR=\"0;38;2;60;110;180\"\n\
LOGO=fedora-logo-icon\n\
VARIANT_ID=workstation";
        assert_eq!("unknown", parse_release_document_contents(OS_RELEASE))
    }

    #[test]
    fn test_parse_release_document_empty_file() {
        const OS_RELEASE: &str = "";
        assert_eq!("unknown", parse_release_document_contents(OS_RELEASE))
    }

    #[test]
    fn test_parse_release_document_additional_quote() {
        const OS_RELEASE: &str = "\
NAME=\"Fedora Linux\"\n\
ANSI_COLOR=\"0;38;2;60;110;180\"\n\
PRETTY_NAME=\"\"Fedora Linux 41 (Workstation Edition)\"\"\n\
LOGO=fedora-logo-icon\n\
VARIANT_ID=workstation";
        assert_eq!(
            "Fedora Linux 41 (Workstation Edition)",
            parse_release_document_contents(OS_RELEASE)
        )
    }

    #[test]
    fn test_parse_release_document_no_quote() {
        const OS_RELEASE: &str = "\
NAME=\"Fedora Linux\"\n\
ANSI_COLOR=\"0;38;2;60;110;180\"\n\
PRETTY_NAME=Fedora Linux 41 (Workstation Edition)\n\
LOGO=fedora-logo-icon\n\
VARIANT_ID=workstation";
        assert_eq!(
            "Fedora Linux 41 (Workstation Edition)",
            parse_release_document_contents(OS_RELEASE)
        )
    }
}
