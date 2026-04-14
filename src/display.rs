use crate::system::SystemInfo;
use crate::types::InfoValue;
use colored::Colorize;

pub fn print_system_info(info: &SystemInfo) {
    println!("╔══════════════════╗");
    println!("║  🦀 crabstat     ║");
    println!("╚══════════════════╝");
    print_row("Shell", &info.shell.to_string());
    print_row("Current Dir", &info.current_dir);
    print_info_row("IP", &info.network.ip);
    print_row("OS Name", &info.os.os_name);
    print_row("Architecture", &info.os.arch);
    print_row("Kernel Version", &info.os.kernel_version);
    print_row("OS Version", &info.os.os_version);
    print_row("Hostname", &info.network.hostname);
    print_row("User", &info.username);
}
fn print_info_row(label: &str, info: &InfoValue) {
    match info {
        InfoValue::Available(v) => print_row(label, v),
        InfoValue::Unavailable(reason) => print_row_error(label, reason),
    }
}
fn print_row(label: &str, value: &str) {
    println!("{:<15}: {}", label.blue().bold(), value);
}

fn print_row_error(label: &str, value: &str) {
    println!("{:<15}: {}", label.red().bold(), value);
}
