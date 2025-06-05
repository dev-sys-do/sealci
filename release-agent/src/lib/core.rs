use crate::{bucket::BucketClient, compress::CompressClient, git::GitClient, sign::ReleaseSigner};
use tracing::info;
use tonic::async_trait;

#[async_trait]
pub trait ReleaseAgentCore<S: ReleaseSigner, B: BucketClient, G: GitClient, C: CompressClient>: Clone + Send + Sync {
    async fn create_release(&self, revision: &str, repository_url: &str) -> Result<Release, ReleaseAgentError>;
    async fn get_root_public_key(&self) -> Result<PublicKey, ReleaseAgentError>;
}

#[derive(Debug, Clone)]
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

}

#[derive(Debug)]
pub struct Release {
    pub revision: String,
    pub public_key: PublicKey,
}

#[derive(Debug, Clone)]
pub struct PublicKey {
    pub key_data: String,
    pub fingerprint: String,
}

#[async_trait]
impl<S: ReleaseSigner, B: BucketClient, G: GitClient, C: CompressClient> ReleaseAgentCore<S,B,G,C> for ReleaseAgent<S, B, G, C> {
        async fn create_release(&self, revision: &str, repository_url: &str) -> Result<Release, ReleaseAgentError> {
        //get last two parts separated by '/'
        let repo_owner = repository_url.split('/').nth_back(1).unwrap();
        let repo_name = repository_url.split('/').nth_back(0).unwrap();

        info!("Creating release for repository '{repo_name}' owned by '{repo_owner}'.");
        let codebase = self
            .git_client
            .download_release(repository_url.to_string(), revision.to_string())
            .await
            .inspect_err(|e| {
                tracing::error!("Failed to download release: {}", e);
            })?;
        let (_, compressed_path) =
            self.compress_client
                .compress(codebase.clone())
                .await
                .inspect_err(|e| {
                    tracing::error!("Failed to compress codebase: {}", e);
                })?;
        let (public_key, signed_codebase) = self .signer
            .sign_release(compressed_path.clone())
            .inspect_err(|e| {
                tracing::error!("Failed to sign release: {}", e);
            })?;

        let release =format!("{repo_owner}/{repo_name}/{revision}");
        self.bucket
            .put_release(release.clone(), compressed_path, signed_codebase)
            .await
            .inspect_err(|e| {
                tracing::error!("Failed to upload release to bucket: {}", e);
            })?;

        Ok(Release {
            revision: release,
            public_key,
        })
    }

    async fn get_root_public_key(&self) -> Result<PublicKey, ReleaseAgentError> {
        self.signer.get_public_key()
    }
}


#[derive(Debug)]
pub enum ReleaseAgentError {
    BucketNotAvailable,
    InvalidBucketEndpoint,
    GitRepositoryNotFound,
    GitRepositoryCheckoutFailed,
    CompressionError,
    SigningError,
    ConfigError,
    KeyLoadingError,
    KeyNotFoundError,
    KeyDecryptionError,
    GitTagNotFound,
    KeyGenerationError,
    TransportError(tonic::transport::Error), // add more errors here
}

impl std::fmt::Display for ReleaseAgentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidBucketEndpoint => write!(f, "Invalid bucket endpoint"),
            Self::BucketNotAvailable => write!(f, "Bucket not available"),
            Self::KeyLoadingError => write!(f, "Error loading key"),
            Self::KeyNotFoundError => write!(f, "Key not found"),
            Self::KeyDecryptionError => write!(f, "Error decrypting key"),
            Self::GitRepositoryCheckoutFailed => write!(f, "Git repository checkout failed"),
            Self::GitTagNotFound => write!(f, "Git tag not found"),
            Self::GitRepositoryNotFound => write!(f, "Git repository not found"),
            Self::CompressionError => write!(f, "Compression error"),
            Self::SigningError => write!(f, "Signing error"),
            Self::ConfigError => write!(f, "Configuration error on startup"),
            Self::KeyGenerationError => write!(f, "Error generating key"),
            Self::TransportError(error) => write!(f, "Transport error: {}", error),
        }
    }
}
