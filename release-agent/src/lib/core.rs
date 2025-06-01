use crate::{sign::ReleaseSigner, bucket::BucketClient, compress::CompressClient, git::GitClient};
pub trait ReleaseAgentCore : Clone + Send + Sync {
    async fn create_release(&self, release_id: &str) -> Result<String, Box<dyn std::error::Error>>;
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

    pub async fn create_release(&self, release_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        let codebase = self.git_client.download_release(release_id).await?;
        let compressed_codebase = self.compress_client.compress(codebase).await?;
        let signed_codebase = self.signer.sign_release(compressed_codebase)?;
        self.bucket.put_release(release_id, signed_codebase).await?;

        Ok(release_id.to_string())
    }
}
