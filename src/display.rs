use crate::cli::Cli;
use crate::system::SystemInfo;
use crate::types::InfoValue;
use colored::Colorize;

pub fn print_system_info(info: &SystemInfo, cli: &Cli) {
    println!("╔══════════════════╗");
    println!("║  🦀 crabstat     ║");
    println!("╚══════════════════╝");
    if cli.show_all() || cli.shell {
        print_row("Shell", &info.shell.to_string());
    }
    if cli.show_all() {
        print_row("Current Dir", &info.current_dir);
    }
    if cli.show_all() || cli.net {
        print_info_row("IP", &info.network.ip);
    }
    if cli.show_all() || cli.os {
        print_row("OS Name", &info.os.os_name);
    }
    if cli.show_all() || cli.os {
        print_row("Architecture", &info.os.arch);
    }
    if cli.show_all() || cli.os {
        print_row("Kernel Version", &info.os.kernel_version);
    }
    if cli.show_all() || cli.os {
        print_row("OS Version", &info.os.os_version);
    }
    if cli.show_all() || cli.net {
        print_row("Hostname", &info.network.hostname);
    }
    if cli.show_all() {
        print_row("User", &info.username);
    }
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
