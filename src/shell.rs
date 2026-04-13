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
    shell_path.rsplit('/').next().unwrap_or("none").to_string()
}

fn get_shell_version(shell_path: &str) -> String {
    let shell_version = std::process::Command::new(shell_path)
        .arg("--version")
        .output();

    match shell_version {
        Ok(output) => String::from_utf8_lossy(&output.stdout)
            .lines()
            .next()
            .unwrap_or("unknown")
            .trim()
            .to_string(),
        Err(e) => e.to_string(),
    }
}