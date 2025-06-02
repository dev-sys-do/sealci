use std::path::PathBuf;

use tonic::async_trait;

use crate::core::ReleaseAgentError;

#[async_trait]
pub trait GitClient: Clone + Send + Sync {
    // revision is a string of the form "v1.2.3" that is the release name, it returns the path to
    // the folder that contains the codebase
    async fn download_release(&self, revision: &str) -> Result<PathBuf, ReleaseAgentError>;
}
