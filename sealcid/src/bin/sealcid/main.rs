use clap::Parser;
use sealcid::{
    common::error::Error,
    server::{cli::Cli, config::GlobalConfig, daemon::Daemon},
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let sealcid_config = Cli::parse();
    let global_config = GlobalConfig::default();
    let deamon = Daemon::new(global_config).await?;
    deamon
        .start(sealcid_config.port)
        .await
        .map_err(Error::StartGrpcError)?;
    Ok(())
}
