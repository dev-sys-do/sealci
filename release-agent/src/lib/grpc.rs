use std::sync::Arc;

use tonic::{Request, Response, Status};
use release_agent_grpc::release_agent_server::{
    ReleaseAgent,
};
use crate::core::ReleaseAgentCore;



pub mod release_agent_grpc {
    tonic::include_proto!("releaseagent");
}
#[derive(Debug, Default, Clone)]
pub struct ReleaseAgentService<C: ReleaseAgentCore> {
    core: Arc<C>
}

#[tonic::async_trait]
impl<C: ReleaseAgentCore + 'static> ReleaseAgent for ReleaseAgentService<C> {
    async fn create_release(
        &self,
        request: Request<release_agent_grpc::CreateReleaseRequest>,
    ) -> Result<Response<release_agent_grpc::CreateReleaseResponse>, Status> {
        let service = Arc::clone(&self.core);
        let result = std::thread::spawn(move || service.create_release(&request.into_inner().release_id)).join();

        Ok(Response::new(release_agent_grpc::CreateReleaseResponse {
            status: release_agent_grpc::CreateReleaseStatus::Success.into(),
            release_id: "1234".to_string(), // TODO: return the release id
        }))
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
