mod network;
mod os;
mod shell;
mod system;
mod user;

fn main() {
    let info = system::get_system_info();
    println!("Shell: {}", info.shell.name);
    println!("Shell Version: {}", info.shell.version);
    println!("Current Dir: {}", info.current_dir);
    println!("IP: {}", info.network.ip);
    println!("OS Name: {}", info.os.os_name);
    println!("Architecture: {}", info.os.arch);
    println!("Kernel Version: {}", info.os.kernel_version);
    println!("OS Version: {}", info.os.os_version);
    println!("Hostname: {}", info.network.hostname);
    println!("User: {}", info.username);
}
