use clap::Parser;
use compactor::{Compactor, cli::Cli, error::Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Cli::parse();
    let mut compactor = Compactor::new(config).await?;
    compactor.run()?;

    Ok(())
}
