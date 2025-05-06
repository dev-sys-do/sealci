use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum, Default)]
pub enum AppEnv {
    #[default]
    Development,
    Production,
}

#[derive(Debug, Clone, Default, Parser)]
pub struct Env {
    #[clap(env, long)]
    pub http: String,

    #[clap(env, long)]
    pub database_url: String,

    #[clap(env, long)]
    pub grpc: String,

    #[clap(env, default_value = "development", value_enum)]
    pub env: AppEnv,
}