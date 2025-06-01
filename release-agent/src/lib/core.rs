use crate::{sign::ReleaseSigner, bucket::BucketClient, compress::CompressClient, git::GitClient};
use tracing::error;
use tonic::async_trait;

#[async_trait]
pub trait ReleaseAgentCore : Clone + Send + Sync {
    async fn create_release(&self, release_id: &str) -> Result<String, ReleaseAgentError>;
}

pub struct ReleaseAgent<S: ReleaseSigner, B: BucketClient, G: GitClient , C: CompressClient> {
    pub signer: S,
    pub bucket: B,
    pub git_client: G,
    pub compress_client: C,
}

impl<S: ReleaseSigner, B: BucketClient, G: GitClient , C: CompressClient> ReleaseAgent<S, B, G, C> {
    pub fn new(signer: S, bucket: B, git_client: G, compress_client: C) -> Self {
        Self { signer, bucket, git_client , compress_client }
    }

    pub async fn create_release(&self, release_id: &str) -> Result<String, ReleaseAgentError> {
        let codebase = self.git_client.download_release(release_id).await.map_err(|e| {
            error!("Error downloading release: {}", e);
            ReleaseAgentError::GitRepositoryNotFound
        })?;
        let compressed_codebase = self.compress_client.compress(codebase).await.map_err(|e| {
            error!("Error compressing release: {}", e);
            ReleaseAgentError::CompressionError
        })?;
        let signed_codebase = self.signer.sign_release(compressed_codebase).map_err(|e| {
            error!("Error signing release: {}", e);
            ReleaseAgentError::SigningError
        })?;
        self.bucket.put_release(release_id, signed_codebase).await.map_err(|e| {
            error!("Error uploading release: {}", e);
            ReleaseAgentError::BucketNotAvailable
        })?;

        Ok(release_id.to_string())
    }
}

pub enum ReleaseAgentError {
    BucketNotAvailable,
    GitRepositoryNotFound,
    CompressionError,
    SigningError,
    // add more errors here
}

impl std::fmt::Display for ReleaseAgentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BucketNotAvailable => write!(f, "Bucket not available"),
            Self::GitRepositoryNotFound => write!(f, "Git repository not found"),
            Self::CompressionError => write!(f, "Compression error"),
            Self::SigningError => write!(f, "Signing error"),
        }
    }
}
