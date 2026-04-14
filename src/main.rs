use crate::cli::Cli;
use clap::Parser;

mod cli;
mod display;
mod network;
mod os;
mod shell;
mod system;
mod types;
mod user;

fn main() {
    let cli = Cli::parse();
    let info = system::get_system_info();
    display::print_system_info(&info, &cli);
}
