fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../api/proto/agent/health.proto")?;
    tonic_build::compile_protos("../api/proto/scheduler/agent.proto")?;

    Ok(())
}
