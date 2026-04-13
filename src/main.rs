fn main() {
    let (shell_name, shell_version) = get_shell_info();
    println!("Shell: {}", shell_name);
    println!("Shell Version: {}", shell_version)
}

fn get_shell_info() -> (String, String) {
    let shell_path = std::env::var("SHELL").unwrap_or_else(|_| "unknown".to_string());
    let shell_name = get_shell_name(&shell_path);

    let shell_version = get_shell_version(&shell_path);

    (shell_name.to_string(), shell_version)
}

fn get_shell_name(shell_path: &String) -> String {
    shell_path.rsplit('/').next().unwrap_or("none").to_string()
}

fn get_shell_version(shell_path: &String) -> String {
    let shell_version = std::process::Command::new(&shell_path)
        .arg("--version")
        .output();

    match shell_version {
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(e) => e.to_string(),
    }
}
