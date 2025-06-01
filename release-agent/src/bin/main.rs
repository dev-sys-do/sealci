use clap::Parser;
use sealci_release_agent::{
    app::AppConfig,
    core::{self, ReleaseAgentError},
    grpc::{ReleaseAgentService, release_agent_grpc::release_agent_server::ReleaseAgent},
};

#[derive(Debug, Parser)]
#[clap(name = "release-agent", version)]
struct Config {
    #[clap(short,long, default_value_t = ("[::1]:50052".to_string()))]
    pub grpc: String,
}

#[tokio::main]
async fn main() -> Result<(), ReleaseAgentError> {
    let config = Config::parse();
    tracing_subscriber::fmt().init();

    let release_agent_grpc = sealci_release_agent::grpc::ReleaseAgentService::default();
    // add signer
    // add bucket
    // add git
    // add compress

    let core = sealci_release_agent::core::ReleaseAgent::new(
        sealci_release_agent::sign::ReleaseSigner::default(),
        sealci_release_agent::bucket::BucketClient::default(),
        sealci_release_agent::git::GitClient::default(),
        sealci_release_agent::compress::CompressClient::default(),
    );
    let app = sealci_release_agent::app::App::<ReleaseAgentService<core::ReleaseAgent<>>>::new(
        AppConfig { grpc: config.grpc },
        release_agent_grpc,
    );

    app.run().await?;

    Ok(())
}
