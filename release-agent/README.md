# Release Agent

The release agent is a gRPC server that provides a set of APIs for signing and publishing releases on SealCI.

## Running the Release Agent

To run the release agent, you need to have Rust and Cargo installed on your system.

```bash
cargo run --bin sealci-release-agent
```

## Using the Release Agent as a Library

To use the release agent as a library, you can add it as a dependency in your Rust project.

```toml
[dependencies]
sealci-release-agent = "0.1.0"
```

Then, you can create an instance of the release agent and use it in your code.

### Example : 

```rust
#[derive(Debug, Parser)]
#[clap(name = "release-agent", version)]
struct Config {
    #[clap(short,long, default_value_t = ("[::1]:50052".to_string()))]
    pub grpc: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::parse();
    tracing_subscriber::fmt().init();

    let release_agent_grpc = sealci_release_agent::grpc::ReleaseAgentService::default();
    let app = sealci_release_agent::app::App::<ReleaseAgentService>::new(
        AppConfig { grpc: config.grpc },
        release_agent_grpc,
    );

    app.run().await?;

    Ok(())
}
```


