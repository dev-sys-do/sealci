mod grpc;
mod config;

use grpc::{ReleaseAgentService, release_agent_grpc::release_agent_server::ReleaseAgentServer};
use tonic::transport::Server;
use clap::Parser;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::App::parse();
    let release_agent_service = ReleaseAgentService::default();

    let addr = config.grpc.parse()?;

    tracing_subscriber::fmt().init();

    info!("Starting grpc server at {}", addr);
    Server::builder()
        .add_service(ReleaseAgentServer::new(release_agent_service))
        .serve(addr)
        .await?;


    Ok(())
}
