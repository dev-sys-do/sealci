use dumplet::{DumpletError, export_docker_image};

async fn run() -> Result<(), DumpletError> {
    export_docker_image("ubuntu:latest", "/tmp/ubuntu_fs.tar").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{}", e);
    }
}
