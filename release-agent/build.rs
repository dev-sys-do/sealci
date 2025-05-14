fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../api/proto/release-agent/controller.proto")?;
    Ok(())
}
