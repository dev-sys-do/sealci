use clap::Parser;

#[derive(Parser, Debug, Clone)]
pub struct Env {
    #[clap(env)]
    pub database_url: String
}