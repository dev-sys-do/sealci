use sqlx::PgPool;
use std::sync::Arc;

use tokio::task;
use tracing::{error, info};

use crate::pipeline::pipeline_repository::PipelineRepository;
use crate::{
    parser::pipe_parser::{Action, ManifestParser, ParsingError, PipelineYaml},
    scheduler::SchedulerService,
};

use super::pipeline_repository::Pipeline;

pub struct PipelineService {
    client: Arc<SchedulerService>,
    parser: Arc<dyn ManifestParser>,
    repository: Arc<PipelineRepository>,
}

#[derive(Debug)]
pub enum PipelineServiceError {
    ParsingError(ParsingError),
    SchedulerError,
}

impl PipelineService {
    pub fn new(
        client: Arc<SchedulerService>,
        parser: Arc<dyn ManifestParser>,
        pool: Arc<PgPool>,
    ) -> Self {
        let repository = Arc::new(PipelineRepository::new(pool.clone()));
        Self {
            client,
            parser,
            repository,
        }
    }

    pub async fn create_pipeline(
        &self,
        pipeline: &PipelineYaml,
    ) -> Result<Pipeline, Box<dyn std::error::Error>> {
        let pipeline = self.repository.create(&pipeline.name).await?;
        Ok(pipeline)
    }

    pub fn try_parse_pipeline(&self, manifest: String) -> Result<PipelineYaml, ParsingError> {
        self.parser.parse(manifest)
    }

    pub async fn send_actions(
        &self,
        _pipeline: Pipeline,
        _repo_url: String,
    ) -> Result<(), PipelineServiceError> {
        let _client = Arc::clone(&self.client);
        // for action in pipeline.actions {
        //     info!("Sending action: {:?}", action);
        //     self.send_action(client.clone(), Arc::new(action), repo_url.clone())
        //         .await?;
        // }
        Ok(())
    }

    pub async fn send_action(
        &self,
        client: Arc<SchedulerService>,
        action: Arc<Action>,
        repo_url: String,
    ) -> Result<(), PipelineServiceError> {
        task::spawn(async move {
            match client.send_action(action, repo_url).await {
                Ok(_) => info!("Action sent successfully"),
                Err(err) => error!("Error sending action: {:?}", err), //needs to store the error in database
            }
        });
        Ok(())
    }
}
