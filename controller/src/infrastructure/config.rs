use clap::Parser;

#[derive(Parser, Default, Clone, Debug)]
pub struct Env {
  #[clap(env)]
  pub database_url: String,

  #[clap(env)]
  pub http: String,

  #[clap(env)]
  pub grpc: String,
}