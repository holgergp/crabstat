mod display;
mod network;
mod os;
mod shell;
mod system;
mod types;
mod user;

fn main() {
    let info = system::get_system_info();
    display::print_system_info(&info);
}
