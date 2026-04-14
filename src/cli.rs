use clap::Parser;

#[derive(Parser)]
#[command(name = "crabstat", about = "Blazing fast system info")]
pub struct Cli {
    /// Show only shell info
    #[arg(long)]
    pub shell: bool,

    /// Show only network info
    #[arg(long)]
    pub net: bool,

    /// Show only OS info
    #[arg(long)]
    pub os: bool,
}

impl Cli {
    pub fn show_all(&self) -> bool {
        !self.shell && !self.net && !self.os
    }
}
