fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .compile(
            &["../api/proto/controller/scheduler.proto"],
            &["../api/proto/controller"],
        )
        .expect("Building scheduler protobuf failed");
    Ok(())
}
