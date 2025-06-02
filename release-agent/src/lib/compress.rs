use std::{fs::File, path::PathBuf};
use tonic::async_trait;

use crate::core::ReleaseAgentError;

#[async_trait]
pub trait CompressClient: Clone + Send + Sync {
    // path is a string containing the path to the folder that contains the codebase to be compressed
    // the File object returned contains the compressed codebase
    async fn compress(&self, path: PathBuf) -> Result<File, ReleaseAgentError>;
}
