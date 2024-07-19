fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .compile(
            &["../api/proto/scheduler/controller.proto"],
            &["../api/proto/scheduler"],
        )
        .expect("Building scheduler protobuf failed");
    Ok(())
}
