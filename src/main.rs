mod network;
mod os;
mod shell;
mod system;
mod user;
use colored::*;
fn main() {
    let info = system::get_system_info();
    print_row("Shell", &info.shell.name);
    print_row("Shell Version", &info.shell.version);
    print_row("Current Dir", &info.current_dir);
    print_row("IP", &info.network.ip);
    print_row("OS Name", &info.os.os_name);
    print_row("Architecture", &info.os.arch);
    print_row("Kernel Version", &info.os.kernel_version);
    print_row("OS Version", &info.os.os_version);
    print_row("Hostname", &info.network.hostname);
    print_row("User", &info.username);
}

fn print_row(label: &str, value: &str) {
    println!("{:<15}: {}", label.blue().bold(), value);
}
