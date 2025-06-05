use clap::Parser;
use std::{path::PathBuf, sync::Arc};
use sealci_release_agent::{
    app::AppConfig, bucket::minio::MinioClient, compress::Flate2Client, core::{ReleaseAgentError}, git::Git2Client, sign::SequoiaPGPManager
};

#[derive(Debug, Parser)]
#[clap(name = "release-agent", version)]
struct Config {
    #[clap(long, default_value_t = ("[::1]:50052".to_string()))]
    pub grpc: String,

    #[clap(short, long)]
    pub passphrase: String,

    #[clap(short, long)]
    pub secret_key: String,

    #[clap(long, default_value_t = ("/tmp".to_string()))]
    pub git_path: String,

    #[clap(long, default_value_t = ("http://127.0.0.1:9000".to_string()))]
    pub bucket_addr: String,

    #[clap(long)]
    pub bucket_access_key: String,

    #[clap(long)]
    pub bucket_secret_key: String,

    #[clap(long)]
    pub bucket_name: String,

}

#[tokio::main]
async fn main() -> Result<(), ReleaseAgentError> {
    let config = Config::parse();
    tracing_subscriber::fmt().init();

    let cert_path = PathBuf::from(config.secret_key);

    // add signer
    let signer = SequoiaPGPManager::new(cert_path, config.passphrase)?;
    // add bucket
    let bucket_client = MinioClient::new(config.bucket_addr, config.bucket_access_key, config.bucket_secret_key, config.bucket_name).await?;
    // add git
    let git_client = Git2Client::new(config.git_path);
    // add compress
    let compress_client = Flate2Client::new();

    let core = sealci_release_agent::core::ReleaseAgent::new(
        signer.clone(),
        bucket_client.clone(),
        git_client.clone(),
        compress_client.clone(),
    );
    let release_agent_grpc = sealci_release_agent::grpc::ReleaseAgentService::new(Arc::new(core), signer, bucket_client, git_client, compress_client);
    let app =
        sealci_release_agent::app::App::new(
            AppConfig { grpc: config.grpc },
            release_agent_grpc,
        );

    app.run().await?;

    Ok(())
}
