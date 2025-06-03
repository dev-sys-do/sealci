use std::path::PathBuf;
use tracing::info;
use tracing::error;

use tonic::async_trait;

use crate::core::ReleaseAgentError;

#[async_trait]
pub trait GitClient: Clone + Send + Sync {
    // revision is a string of the form "v1.2.3" that is the release name, it returns the path to
    // the folder that contains the codebase
    async fn download_release(&self, repository_url: String, revision: String) -> Result<PathBuf, ReleaseAgentError>;
}

#[derive(Debug, Clone)]
pub struct Git2Client {
    pub path: String
}

impl Git2Client {
    pub fn new(path: String) -> Self {
        Self { path }
    }
}

#[async_trait]
impl GitClient for Git2Client {
    async fn download_release(&self, repository_url: String, revision: String) -> Result<PathBuf, ReleaseAgentError> {
        let repository_name = repository_url.split("/").last().unwrap();
        let folder_name = format!("{}/{}-{}", self.path, repository_name, revision);
        std::fs::create_dir_all(&folder_name).map_err(|e| {
            error!("Error creating folder: {}", e);
            ReleaseAgentError::GitRepositoryNotFound
        })?;
        info!("Cloning repository '{repository_url}' to '{folder_name}'.");
        let path = PathBuf::from(folder_name.as_str());
        let mut builder = git2::build::RepoBuilder::new();
        builder.branch(revision.as_str());
        builder.clone(repository_url.as_str(), path.as_path()).map_err(|e| {
            error!("Error cloning repository: {}", e);
            ReleaseAgentError::GitRepositoryNotFound
        })?;

        Ok(path)
    }
}
