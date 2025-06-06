use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    /// Port to listen on
    #[clap(short, long)]
    pub port: u32,
}
