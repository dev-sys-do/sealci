use tonic::{transport::Server, Request, Response, Status};
use release_agent_grpc::release_agent_server::{
    ReleaseAgentServer,
    ReleaseAgent,
};


pub mod release_agent_grpc {
    tonic::include_proto!("releaseagent");
}
#[derive(Debug, Default)]
pub struct ReleaseAgentService {}

#[tonic::async_trait]
impl ReleaseAgent for ReleaseAgentService {
    async fn create_release(
        &self,
        request: Request<release_agent_grpc::CreateReleaseRequest>,
    ) -> Result<Response<release_agent_grpc::CreateReleaseResponse>, Status> {
        println!("Got a request: {:?}", request);

        let response = release_agent_grpc::CreateReleaseResponse {
            status: release_agent_grpc::CreateReleaseStatus::Success.into(),
            release_id: "1234".to_string(),
        };

        Ok(Response::new(response))
    }

    async fn roll_pgp_keys(
        &self,
        request: Request<release_agent_grpc::RollPgpKeysRequest>,
    ) -> Result<Response<release_agent_grpc::PublicKey>, Status> {
        println!("Got a request: {:?}", request);

        let response = release_agent_grpc::PublicKey {
            key_id: "1234".to_string(),
            key_data: "1234".to_string(),
        };

        Ok(Response::new(response))
    }

    async fn get_public_key(
        &self,
        request: Request<release_agent_grpc::Empty>,
    ) -> Result<Response<release_agent_grpc::PublicKey>, Status> {
        println!("Got a request: {:?}", request);

        let response = release_agent_grpc::PublicKey {
            key_id: "1234".to_string(),
            key_data: "1234".to_string(),
        };

        Ok(Response::new(response))
    }
}
