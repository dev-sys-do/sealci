use std::error::Error;
use tonic::async_trait;
use std::fs::File;

#[async_trait]
pub trait CompressClient: Clone + Send + Sync {
    // path is a string containing the path to the folder that contains the codebase to be compressed
    // the File object returned contains the compressed codebase
    async fn compress(&self, path: String) -> Result<File, Box<dyn Error>>;
}
