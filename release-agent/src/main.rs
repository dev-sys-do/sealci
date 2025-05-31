mod grpc;
mod config;
mod app;

use clap::Parser;
use grpc::ReleaseAgentService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::AppConfig::parse();
    tracing_subscriber::fmt().init();
    let release_agent = ReleaseAgentService::default();
    let app = app::App::<ReleaseAgentService>::new(config, release_agent);
    app.run().await?;
    Ok(())
}
