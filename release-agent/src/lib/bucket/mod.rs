use tonic::async_trait;
pub mod minio;

use std::{fs::File, path::PathBuf};

use crate::core::ReleaseAgentError;

#[async_trait]
pub trait BucketClient: Clone + Send + Sync {
    // release is a string of the form "v1.2.3" that is the release name
    // the file is a tar.gz file containing the source code as a gzipped tarball and a .sig file
    // that is a signature of the tarball
    async fn put_release(&self, release: String, release_path: PathBuf, sig_path: PathBuf) -> Result<(), ReleaseAgentError>;
}
