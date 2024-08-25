use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _proto_file = "agent/health.proto";
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // tonic_build::compile_protos("../api/proto/agent/health.proto")?;
    // tonic_build::compile_protos("../api/proto/scheduler/agent.proto")?;

    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .file_descriptor_set_path(out_dir.join("health_descriptor.bin"))
        .out_dir("./src")
        .compile(
            &[
                "../api/proto/agent/health.proto",
                "../api/proto/scheduler/agent.proto",
            ],
            &["../api/proto/agent", "../api/proto/scheduler"],
        )?;

    Ok(())
}
