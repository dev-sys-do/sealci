use std::sync::Arc;

use crate::{
    bucket::BucketClient, compress::CompressClient, core::ReleaseAgentCore, git::GitClient,
    sign::ReleaseSigner,
};
use release_agent_grpc::release_agent_server::ReleaseAgent;
use tonic::{Request, Response, Status};

pub mod release_agent_grpc {
    tonic::include_proto!("releaseagent");
}
#[derive(Debug, Default, Clone)]
pub struct ReleaseAgentService<R, S, B, G, C>
where
    S: ReleaseSigner,
    B: BucketClient,
    G: GitClient,
    C: CompressClient,
    R: ReleaseAgentCore<S, B, G, C>,
{
    core: Arc<R>,
    _signer: S,
    _bucket: B,
    _git_client: G,
    _compress_client: C,
}

#[tonic::async_trait]
impl<R, S, B, G, C> ReleaseAgent for ReleaseAgentService<R, S, B, G, C>
where
    S: ReleaseSigner + 'static,
    B: BucketClient + 'static,
    G: GitClient + 'static,
    C: CompressClient + 'static,
    R: ReleaseAgentCore<S, B, G, C> + 'static,
{
    async fn create_release(
        &self,
        request: Request<release_agent_grpc::CreateReleaseRequest>,
    ) -> Result<Response<release_agent_grpc::CreateReleaseResponse>, Status> {
        let request = request.into_inner().clone();
        let repository_url = request.clone().repo_url;
        let revision = request.revision;
        match self.core.create_release(&revision, &repository_url).await {
            Ok(release_id) => {
                let response = release_agent_grpc::CreateReleaseResponse {
                    release_id,
                    status: release_agent_grpc::CreateReleaseStatus::Success as i32,
                };
                Ok(Response::new(response))
            }
            Err(e) => {
                println!("Error creating release: {}", e);
                Err(Status::internal(e.to_string()))
            }
        }
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

impl<R, S, B, G, C> ReleaseAgentService<R, S, B, G, C>
where
    S: ReleaseSigner,
    B: BucketClient,
    G: GitClient,
    C: CompressClient,
    R: ReleaseAgentCore<S, B, G, C>,
{
    pub fn new(core: Arc<R>, signer: S, bucket: B, git_client: G, compress_client: C) -> Self {
        Self {
            core,            
            _signer: signer,
            _bucket: bucket,
            _git_client: git_client,
            _compress_client: compress_client,
        }
    }
}
