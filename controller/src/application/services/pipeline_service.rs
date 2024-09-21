use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    application::ports::{action_service::ActionService, pipeline_service::PipelineService},
    domain::{
        entities::{
            action::{ActionStatus, ActionType},
            pipeline::{ManifestPipeline, Pipeline, PipelineError},
        },
        repositories::pipeline_repository::PipelineRepository,
    },
};

pub struct PipelineServiceImpl {
    repository: Arc<Box<dyn PipelineRepository + Send + Sync>>,
    action_service: Arc<Box<dyn ActionService + Send + Sync>>,
}

impl PipelineServiceImpl {
    pub fn new(
        repository: Arc<Box<dyn PipelineRepository + Send + Sync>>,
        action_service: Arc<Box<dyn ActionService + Send + Sync>>,
    ) -> Self {
        Self {
            repository,
            action_service,
        }
    }
}

#[async_trait]
impl PipelineService for PipelineServiceImpl {
    async fn find_all(&self) -> Vec<Pipeline> {
        self.repository.find_all().await.unwrap_or_else(|_| vec![])
    }

    async fn create_pipeline(
        &self,
        repository_url: String,
        name: String,
    ) -> Result<Pipeline, PipelineError> {
        self.repository.create(repository_url, name).await
    }

    async fn find_by_id(&self, pipeline_id: i64) -> Result<Pipeline, PipelineError> {
        self.repository.find_by_id(pipeline_id).await
    }

    async fn create_manifest_pipeline(
        &self,
        manifest: ManifestPipeline,
        repository_url: String,
    ) -> Result<Pipeline, PipelineError> {
        let pipeline = self.create_pipeline(repository_url, manifest.name).await?;

        for (action_name, action_data) in manifest.actions.actions.iter() {
            let _action = self
                .action_service
                .create(
                    pipeline.id,
                    action_name.to_owned(),
                    action_data.configuration.container.clone(),
                    ActionType::Container,
                    ActionStatus::Pending.to_string(),
                    Some(action_data.commands.clone()),
                )
                .await;
        }

        Ok(pipeline)
    }
}
