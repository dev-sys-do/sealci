use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let _ = tonic_build::compile_protos("../api/proto/scheduler/agent.proto");
    let _ = tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("scheduler_descriptor.bin"))
        .compile(&["../../api/proto/scheduler/agent.proto"], &["proto"]);
    Ok(())
}