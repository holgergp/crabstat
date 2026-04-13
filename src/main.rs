struct SystemInfo {
    shell: ShellInfo,
    current_dir: String,
    current_ip: String,
    os: OsInfo,
    hostname: String,
    username: String,
}
struct ShellInfo {
    name: String,
    version: String,
}

struct OsInfo {
    os_name: String,
    arch: String,
    kernel_version: String,
    os_version: String,
}

fn main() {
    let info = get_system_info();
    println!("Shell: {}", info.shell.name);
    println!("Shell Version: {}", info.shell.version);
    println!("Current Dir: {}", info.current_dir);
    println!("IP: {}", info.current_ip);
    println!("OS Name: {}", info.os.os_name);
    println!("Architecture: {}", info.os.arch);
    println!("Kernel Version: {}", info.os.kernel_version);
    println!("OS Version: {}", info.os.os_version);
    println!("Hostname: {}", info.hostname);
    println!("User: {}", info.username);
}

fn get_system_info() -> SystemInfo {
    let shell = get_shell_info();
    let current_dir = get_current_dir();
    let current_ip = get_ip_address();
    let current_ip_parsed = match current_ip {
        Ok(ip) => ip,
        Err(_) => "No IP detected".to_string(),
    };
    let os_info = get_os_info();
    let hostname: String = get_hostname();
    let username: String = get_username();
    SystemInfo {
        shell,
        current_dir,
        current_ip: current_ip_parsed,
        os: os_info,
        hostname,
        username,
    }
}

fn get_shell_info() -> ShellInfo {
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

fn get_os_info() -> OsInfo {
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

fn get_hostname() -> String {
    let hostname = std::process::Command::new("hostname").output();
    match hostname {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
        Err(e) => e.to_string(),
    }
}

fn get_username() -> String {
    std::env::var("USER").unwrap_or_else(|_| "unknown".to_string())
}
