use proto::{
    agent_server::{Agent, AgentServer},
    RegisterAgentResponse,
};
use tonic::{transport::Server, Response};
mod proto {
    tonic::include_proto!("scheduler");
}


#[derive(Debug, Default)]
struct RegistrationService {}

#[tonic::async_trait]
impl Agent for RegistrationService {
    async fn register_agent(
        &self,
        _request: tonic::Request<proto::Health>,
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
