use clap::{Parser };

#[derive(Debug, Parser)]
#[clap(name = "release-agent", version)]
pub struct AppConfig{
    #[clap(short,long, default_value_t = ("[::1]:50051".to_string()))]
    pub grpc: String
}
