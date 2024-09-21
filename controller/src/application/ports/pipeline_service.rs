use crate::domain::entities::pipeline::{ManifestPipeline, Pipeline, PipelineError};
use async_trait::async_trait;

#[async_trait]
pub trait PipelineService: Send + Sync {
    async fn find_all(&self) -> Vec<Pipeline>;
    async fn find_by_id(&self, pipeline_id: i64) -> Result<Pipeline, PipelineError>;
    async fn create_pipeline(
        &self,
        repository_url: String,
        name: String,
    ) -> Result<Pipeline, PipelineError>;
    async fn create_manifest_pipeline(
        &self,
        manifest: ManifestPipeline,
        repository_url: String,
    ) -> Result<Pipeline, PipelineError>;
}
