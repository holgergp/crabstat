use crate::network;
use crate::os;
use crate::shell;
use crate::user;

pub struct SystemInfo {
    pub shell: shell::ShellInfo,
    pub current_dir: String,
    pub os: os::OsInfo,
    pub username: String,
    pub network: network::NetworkInfo,
}

pub fn get_system_info() -> SystemInfo {
    let shell = shell::get_shell_info();
    let current_dir = user::get_current_dir();
    let network_info = network::get_network_info();
    let os_info = os::get_os_info();
    let username: String = user::get_username();
    SystemInfo {
        shell,
        current_dir,
        network: network_info,
        os: os_info,
        username,
    }
}
