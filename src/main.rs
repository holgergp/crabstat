fn main() {
    let (shell_name, shell_version) = get_shell_info();
    println!("Shell: {}", shell_name);
    println!("Shell Version: {}", shell_version);

    let current_dir = get_current_dir();
    println!("Current Dir: {}", current_dir);

    let current_ip = get_ip_address();
    match current_ip {
        Ok(ip) => println!("IP: {}", ip),
        Err(_) => println!("No IP detected"),
    }

    let (os_name, arch, kernel_version, os_version) = get_os_info();
    println!("OS Name: {}", os_name);
    println!("Architecture: {}", arch);
    println!("Kernel Version: {}", kernel_version);
    println!("OS Version: {}", os_version);
}

fn get_shell_info() -> (String, String) {
    let shell_path = std::env::var("SHELL").unwrap_or_else(|_| "unknown".to_string());
    let shell_name = get_shell_name(&shell_path);

    let shell_version = get_shell_version(&shell_path);

    (shell_name, shell_version)
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

fn get_current_dir() -> String {
    let current_dir = std::env::current_dir();
    match current_dir {
        Ok(path_buff) => path_buff.display().to_string(),
        Err(e) => e.to_string(),
    }
}

fn get_ip_address() -> Result<String, std::io::Error> {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("8.8.8.8:80")?;
    let addr = socket.local_addr()?;
    Ok(addr.ip().to_string())
}

fn get_os_info() -> (&'static str, &'static str, String, String) {
    let os_name = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let kernel_version = std::process::Command::new("uname").arg("-r").output();
    let kernel_version_parsed = match kernel_version {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
        Err(e) => e.to_string(),
    };

    let os_version = get_os_version();

    (os_name, arch, kernel_version_parsed, os_version)
}

#[cfg(target_os = "macos")]
fn get_os_version() -> String {
    let os_version = std::process::Command::new("sw_vers")
        .arg("-productVersion")
        .output();
    match os_version {
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(e) => e.to_string(),
    }
}

#[cfg(target_os = "linux")]
fn get_os_version() -> String {
    parse_release_document_to_pretty_name()
}

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
