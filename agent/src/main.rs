use proto::{
    agent_server::{Agent, AgentServer},
    RegisterAgentResponse,
};
use tonic::{transport::Server, Response};
mod proto {
    tonic::include_proto!("registration");
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("registration_descriptor");
}

#[derive(Debug, Default)]
struct RegistrationService {}

#[tonic::async_trait]
impl Agent for RegistrationService {
    async fn register_agent(
        &self,
        request: tonic::Request<proto::Health>,
    ) -> Result<tonic::Response<proto::RegisterAgentResponse>, tonic::Status> {
        Ok(Response::new(RegisterAgentResponse {
            id: "1".to_string(),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:5001".parse()?;

    let reg = RegistrationService::default();
    Server::builder()
        .add_service(AgentServer::new(reg)) // Corrected line
        .serve(addr)
        .await?;

    Ok(())
}
