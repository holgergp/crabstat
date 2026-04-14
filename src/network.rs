use crate::types::InfoValue;

pub struct NetworkInfo {
    pub ip: InfoValue,
    pub hostname: String,
}

fn get_ip_address() -> Result<String, std::io::Error> {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("8.8.8.8:80")?;
    let addr = socket.local_addr()?;
    Ok(addr.ip().to_string())
}

fn get_hostname() -> String {
    let hostname = std::process::Command::new("hostname").output();
    match hostname {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
        Err(e) => e.to_string(),
    }
}

pub fn get_network_info() -> NetworkInfo {
    let hostname = get_hostname();
    NetworkInfo {
        ip: InfoValue::from_result(get_ip_address(), "No IP detected"),
        hostname,
    }
}
