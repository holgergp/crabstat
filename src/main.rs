fn main() {
    let (shell_name, shell_version) = get_shell_info();
    println!("Shell: {}", shell_name);
    println!("Shell Version: {}", shell_version);

    let current_dir = get_current_dir();
    println!("Current Dir: {}", current_dir);

    let current_ip = get_ip_address();
    match current_ip {
        Ok(ip) => println!("IP: {}", ip),
        Err(_) => println!("No IP detected")
    }
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
        Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
        Err(e) => e.to_string(),
    }
}

fn get_current_dir() -> String {
    let current_dir = std::env::current_dir();
    match current_dir {
        Ok(path_buff) => path_buff.display().to_string(),
        Err(e) => e.to_string()
    }
}

fn get_ip_address() -> Result<String, std::io::Error> {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("8.8.8.8:80")?;
    let addr = socket.local_addr()?;
    Ok(addr.ip().to_string())
}