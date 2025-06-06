use std::fmt::Display;
use clap::Parser;

#[derive(Clone, Parser)]
pub struct Config {
    /// The address to bind the gRPC server to
    #[clap(short, long, default_value = "0.0.0.0:50051",
            help = "The address to bind the gRPC server to")]
    pub addr: String,
}

impl Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Config(addr: {})", self.addr)
    }
}
