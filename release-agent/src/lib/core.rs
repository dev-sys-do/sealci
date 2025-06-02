use crate::{bucket::BucketClient, compress::CompressClient, git::GitClient, sign::ReleaseSigner};
use tonic::async_trait;

#[async_trait]
pub trait ReleaseAgentCore: Clone + Send + Sync {
    async fn create_release(&self, release_id: &str) -> Result<String, ReleaseAgentError>;
}

pub struct ReleaseAgent<S: ReleaseSigner, B: BucketClient, G: GitClient, C: CompressClient> {
    pub signer: S,
    pub bucket: B,
    pub git_client: G,
    pub compress_client: C,
}

impl<S: ReleaseSigner, B: BucketClient, G: GitClient, C: CompressClient> ReleaseAgent<S, B, G, C> {
    pub fn new(signer: S, bucket: B, git_client: G, compress_client: C) -> Self {
        Self {
            signer,
            bucket,
            git_client,
            compress_client,
        }
    }

    pub async fn create_release(&self, release_id: &str) -> Result<String, ReleaseAgentError> {
        let codebase = self
            .git_client
            .download_release(release_id)
            .await
            .inspect_err(|e| {
                tracing::error!("Failed to download release: {}", e);
            })?;
        let compressed_codebase =
            self.compress_client
                .compress(codebase)
                .await
                .inspect_err(|e| {
                    tracing::error!("Failed to compress codebase: {}", e);
                })?;
        let signed_codebase = self
            .signer
            .sign_release(compressed_codebase)
            .inspect_err(|e| {
                tracing::error!("Failed to sign release: {}", e);
            })?;
        self.bucket
            .put_release(release_id, signed_codebase)
            .await
            .inspect_err(|e| {
                tracing::error!("Failed to upload release to bucket: {}", e);
            })?;

        Ok(release_id.to_string())
    }
}

#[derive(Debug)]
pub enum ReleaseAgentError {
    BucketNotAvailable,
    GitRepositoryNotFound,
    CompressionError,
    SigningError,
    ConfigError,
    TransportError(tonic::transport::Error), // add more errors here
}

impl std::fmt::Display for ReleaseAgentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BucketNotAvailable => write!(f, "Bucket not available"),
            Self::GitRepositoryNotFound => write!(f, "Git repository not found"),
            Self::CompressionError => write!(f, "Compression error"),
            Self::SigningError => write!(f, "Signing error"),
            ReleaseAgentError::ConfigError => write!(f, "Configuration error on startup"),
            ReleaseAgentError::TransportError(error) => write!(f, "Transport error: {}", error),
        }
    }
}
