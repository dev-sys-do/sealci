pub mod health;

use clap::Parser;
use futures::StreamExt;

use health::health_client::HealthClient;
use health::{MetricRequest, MetricReply};
use tonic::client;

#[derive(Debug, Parser)]
struct Options {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    Get(GetOptions),
    Watch(GetOptions),
}

#[derive(Debug, Parser)]
struct GetOptions {
    #[clap(short, long)]
    sku: String,
}

async fn get(opts: GetOptions) -> Result<(), Box<dyn std::error::Error>> {

  let mut client = HealthClient::connect("http://127.0.0.1:9001").await?;

    let request = tonic::Request::new(MetricRequest {
        name: opts.sku,
    });

    let response = client.get(request).await?.into_inner();
    println!("RESPONSE={:?}", response);

    Ok(())
}

async fn watch(opts: GetOptions) -> Result<(), Box<dyn std::error::Error>> {
  let mut client = HealthClient::connect("http://127.0.0.1:9001").await?;

  let mut stream = client
    .watch(MetricRequest {
      name: opts.sku.clone(),
    })
    .await?
    .into_inner();

  println!("Watching for changes");

  while let Some(metric) = stream.next().await {
    println!("UPDATE={:?}", metric?);
  }
  println!("stream closed");

  Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Options::parse();

    use Command::*;
    match opts.command {
        Get(opts) => get(opts).await?,
        Watch(opts) => watch(opts).await?,
    };

    Ok(())
}