fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .compile(
            // &["../api"]
        )
        .expect("Failed to compile protobuf files");
    Ok(())
}
