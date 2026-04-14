use std::{io::Error, process::Output};

pub struct ShellInfo {
    pub name: String,
    pub version: String,
}

pub fn get_shell_info() -> ShellInfo {
    let shell_path = std::env::var("SHELL").unwrap_or_else(|_| "unknown".to_string());
    let shell_name = get_shell_name(&shell_path);

    let shell_version = get_shell_version(&shell_path);

    ShellInfo {
        name: shell_name,
        version: shell_version,
    }
}

fn get_shell_name(shell_path: &str) -> String {
    shell_path
        .rsplit('/')
        .find(|s| !s.is_empty())
        .unwrap_or("")
        .to_string()
}

fn get_shell_version(shell_path: &str) -> String {
    let shell_version = std::process::Command::new(shell_path)
        .arg("--version")
        .output();
    parse_shell_version(shell_version)
}

fn parse_shell_version(shell_version_string: Result<Output, Error>) -> String {
    match shell_version_string {
        Ok(output) => parse_shell_version_output(&String::from_utf8_lossy(&output.stdout)),
        Err(e) => e.to_string(),
    }
}

fn parse_shell_version_output(raw: &str) -> String {
    let line = raw.lines().next().unwrap_or("").trim();
    if line.is_empty() {
        "unknown".to_string()
    } else {
        line.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_shell_name_happy_path() {
        assert_eq!("fish", get_shell_name("/opt/homebrew/bin/fish"))
    }

    #[test]
    fn test_get_shell_name_trailing_slash() {
        assert_eq!("fish", get_shell_name("/opt/homebrew/bin/fish/"))
    }

    #[test]
    fn test_get_shell_name_no_slash() {
        assert_eq!("fish", get_shell_name("fish"))
    }

    #[test]
    fn test_get_shell_name_empty_string() {
        assert_eq!("", get_shell_name(""))
    }

    #[test]
    fn test_parse_shell_version_output_on_fish() {
        let shell_version_string = "fish, version 4.6.0";
        assert_eq!(
            shell_version_string,
            parse_shell_version_output(shell_version_string)
        )
    }

    #[test]
    fn test_parse_shell_version_output_on_bash() {
        let shell_version_string = "GNU bash, version 3.2.57(1)-release (arm64-apple-darwin25)\nCopyright (C) 2007 Free Software Foundation, Inc.";
        assert_eq!(
            "GNU bash, version 3.2.57(1)-release (arm64-apple-darwin25)",
            parse_shell_version_output(shell_version_string)
        )
    }

    #[test]
    fn test_parse_shell_version_output_empty_string() {
        let shell_version_string = "";
        assert_eq!("unknown", parse_shell_version_output(shell_version_string))
    }

    #[test]
    fn test_parse_shell_version_output_whitespace_string() {
        let shell_version_string = " ";
        assert_eq!("unknown", parse_shell_version_output(shell_version_string))
    }
}
