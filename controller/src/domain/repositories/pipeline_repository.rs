use crate::domain::entities::pipeline::{Pipeline, PipelineError};
use async_trait::async_trait;

#[async_trait]
pub trait PipelineRepository: Send + Sync {
    async fn create(&self, repository_url: String, name: String)
        -> Result<Pipeline, PipelineError>;
    async fn find_all(&self) -> Result<Vec<Pipeline>, PipelineError>;
    async fn find_by_id(&self, pipeline_id: i64) -> Result<Pipeline, PipelineError>;
}
