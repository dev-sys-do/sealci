fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../api/proto/scheduler/agent.proto")?;
    tonic_build::compile_protos("../api/proto/agent/actions.proto")?;

    Ok(())
}
