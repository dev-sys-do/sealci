use compactor::{Compactor};

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
async fn main() -> Result<(), SealedError> {
    tracing_subscriber::fmt().init();
    let config = Config::parse();
    let compactor_config = compactor::cli::Cli {
        image: "milou666/release-agent:latest".to_string(),
        tap_interface_name: "tap0".to_string(),
        interactive: false,
        env: vec![
            "DEBUG=true".to_string(), 
            "LOG_LEVEL=info".to_string(), 
            "SECRET_KEY=/tmp/keys/".to_string(),
            format!("PASSPHRASE={}", config.passphrase),
            "GIT_PATH=/tmp/git/".to_string(),
            format!("BUCKET_ADDR={}", config.bucket_addr),
            format!("BUCKET_ACCESS_KEY={}", config.bucket_access_key),
            format!("BUCKET_SECRET_KEY={}", config.bucket_secret_key),
            format!("BUCKET_NAME={}", config.bucket_name),
        ],
        num_vcpus: 4,
        mem_size_mb: 2048,
        transfer_files: vec![],
    };
    let mut compactor = Compactor::new(compactor_config).await.map_err(|e| {
        SealedError::ImageNotFound
    })?;

    Ok(())
}

pub enum SealedError {
    ImageNotFound,
    VMCouldntStart,
}
