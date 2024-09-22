use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pipeline {
    pub id: i64,
    pub name: String,
    pub repository_url: String,
}

impl Pipeline {
    pub fn new(id: i64, repository_url: String, name: String) -> Self {
        Self {
            id,
            repository_url,
            name,
        }
    }

    pub fn repository_url(&self) -> &String {
        &self.repository_url
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ManifestPipeline {
    pub name: String,
    pub actions: ActionsMap,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionsMap {
    #[serde(flatten)]
    pub actions: std::collections::HashMap<String, ActionManifest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActionManifest {
    pub configuration: Configuration,
    pub commands: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration {
    pub container: String,
}

#[derive(Debug, Error)]
pub enum CreatePipelineError {
    #[error("Error while creating pipeline: {0}")]
    Error(String),
}

#[derive(Debug, Error)]
pub enum PipelineError {
    #[error("Error while creating pipeline: {0}")]
    CreateError(String),
    #[error("Pipeline not found")]
    NotFound,

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error), // Erreurs liées à la base de données

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}
