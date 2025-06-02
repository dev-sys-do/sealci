mod minio;
use tonic::async_trait;

use std::fs::File;

use crate::core::ReleaseAgentError;

#[async_trait]
pub trait BucketClient: Clone + Send + Sync {
    // release is a string of the form "v1.2.3" that is the release name
    // the file is a tar.gz file containing the source code as a gzipped tarball and a .sig file
    // that is a signature of the tarball
    async fn put_release(&self, release: &str, file: File) -> Result<(), ReleaseAgentError>;
    // public_key is a string containing the public key that is used to verify the signature of the
    // release
    async fn put_public_key(&self, public_key: &str) -> Result<(), ReleaseAgentError>;
}
