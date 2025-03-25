
use clap::{self, arg, command, Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct VmmConfig {
    #[arg(long, default_value = "512", help = "Memory size in megabytes")]
    pub mem_size_mb: u32,

    #[arg(long, help = "Path to the kernel image")]
    pub kernel_path: String,
}


impl VmmConfig {
    pub fn new(mem_size_mb: u32, kernel_path: String) -> Self {
        Self {
            mem_size_mb,
            kernel_path,
        }
    }
}
