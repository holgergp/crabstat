use crate::network;
use crate::os;
use crate::shell;
use crate::user;
use std::thread;

pub struct SystemInfo {
    pub shell: shell::ShellInfo,
    pub current_dir: String,
    pub os: os::OsInfo,
    pub username: String,
    pub network: network::NetworkInfo,
}

pub fn get_system_info() -> SystemInfo {
    let shell_handle = thread::spawn(shell::get_shell_info);
    let os_handle = thread::spawn(os::get_os_info);
    let network_handle = thread::spawn(network::get_network_info);
    let current_dir = user::get_current_dir();
    let username: String = user::get_username();
    let shell = shell_handle.join().unwrap();
    let os = os_handle.join().unwrap();
    let network = network_handle.join().unwrap();
    SystemInfo {
        shell,
        current_dir,
        network,
        os,
        username,
    }
}
