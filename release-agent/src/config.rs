use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "sealer-agent", version)]
pub struct App {
    #[clap(env, long, default = "[::0]:50051")]
    pub grpc: String,

    #[clap(env, long, description = "GPG private key value")]
    pub gpg: String,
}
