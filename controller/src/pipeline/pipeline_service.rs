use std::sync::Arc;
use sqlx::PgPool;

use tokio::task;
use tracing::info;

use crate::{
    parser::pipe_parser::{Action, ManifestParser, ParsingError, Pipeline},
    scheduler::SchedulerService,
};
use crate::pipeline::pipeline_repository::PipelineRepository;

pub struct PipelineService {
    client: Arc<SchedulerService>,
    parser: Arc<dyn ManifestParser>,
    repository: Arc<PipelineRepository>
}

#[derive(Debug)]
pub enum PipelineServiceError {
    ParsingError(ParsingError),
    SchedulerError,
}

impl PipelineService {
    pub fn new(client: Arc<SchedulerService>, parser: Arc<dyn ManifestParser>, pool: Arc<PgPool>) -> Self {
        let repository = Arc::new(PipelineRepository::new(pool.clone()));
        Self { client, parser, repository }
    }

    pub fn try_parse_pipeline(&self, manifest: String) -> Result<Pipeline, ParsingError> {
        self.parser.parse(manifest)
    }

    pub async fn send_actions(&self, pipeline: Pipeline) -> Result<(), PipelineServiceError> {
        let client = Arc::clone(&self.client);
        for action in pipeline.actions {
            info!("Sending action: {:?}", action);
            self.send_action(client.clone(), Arc::new(action)).await?;
        }
        Ok(())
    }

    pub async fn send_action(
        &self,
        client: Arc<SchedulerService>,
        action: Arc<Action>,
    ) -> Result<(), PipelineServiceError> {
        task::spawn(async move {
            client.send_action(action).await.unwrap();
        });
        Ok(())
    }
}
