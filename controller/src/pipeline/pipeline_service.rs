use sqlx::PgPool;
use std::sync::Arc;

use tokio::task;
use tracing::{error, info};

use crate::action::action_repository::Action;
use crate::action::action_service::{self, ActionDTO, ActionService};
use crate::grpc_scheduler::ActionStatus;
use crate::pipeline::pipeline_repository::PipelineRepository;
use crate::{
    parser::pipe_parser::{ManifestParser, ManifestPipeline, ParsingError},
    scheduler::SchedulerService,
};

use super::pipeline_repository::PipelineDTO;
use super::Pipeline;

pub struct PipelineService {
    client: Arc<SchedulerService>,
    parser: Arc<dyn ManifestParser>,
    repository: Arc<PipelineRepository>,
    action_service: Arc<ActionService>,
}

#[derive(Debug)]
pub enum PipelineServiceError {
    ParsingError(ParsingError),
    SchedulerError,
    StoringLogError,
}

impl PipelineService {
    pub fn new(
        client: Arc<SchedulerService>,
        parser: Arc<dyn ManifestParser>,
        pool: Arc<PgPool>,
        action_service: Arc<ActionService>,
    ) -> Self {
        let repository = Arc::new(PipelineRepository::new(pool.clone()));
        Self {
            client,
            parser,
            repository,
            action_service,
        }
    }

    pub async fn find_all(&self) -> Vec<PipelineDTO> {
        match self.repository.find_all().await {
            Ok(pipelines) => pipelines,
            Err(e) => {
                info!("Error while fetching pipelines: {:?}", e);
                vec![]
            }
        }
    }

    pub async fn create_pipeline(
        &self,
        repository_url: &String,
    ) -> Result<PipelineDTO, Box<dyn std::error::Error>> {
        info!("Creating pipeline for repository: {}", repository_url);
        let pipeline = self.repository.create(repository_url).await;
        match pipeline {
            Ok(pipeline) => {
                info!("Created pipeline: {:?}", pipeline);
                Ok(pipeline)
            }
            Err(e) => {
                info!("Error while creating pipeline: {:?}", e);
                Err(Box::new(e))
            }
        }
    }

    pub async fn create_pipeline_with_actions(
        &self,
        manifest: ManifestPipeline,
        repo_url: String,
    ) -> Result<Pipeline, Box<dyn std::error::Error>> {
        let pipeline = self.create_pipeline(&repo_url).await?;
        let mut actions = Vec::new();
        for action in manifest.actions {
            info!("Creating action: {:?}", action);
            let action = self
                .action_service
                .create(
                    &ActionDTO {
                        name: action.name,
                        pipeline_id: pipeline.id,
                        container_uri: action.configuration_version,
                        status: ActionStatus::Pending.as_str_name().to_string(),
                        r#type: action.configuration_type,
                        id: None,
                    },
                    action.commands,
                )
                .await?;
            actions.push(action);
        }

        Ok(Pipeline {
            id: pipeline.id,
            repository_url: repo_url,
            actions,
        })
    }

    pub fn try_parse_pipeline(&self, manifest: String) -> Result<ManifestPipeline, ParsingError> {
        self.parser.parse(manifest)
    }

    pub async fn send_action(
        &self,
        action: Arc<Action>,
        repo_url: String,
    ) -> Result<(), PipelineServiceError> {
        let client = Arc::clone(&self.client);
        task::spawn(async move {
            match client.send_action(action, repo_url).await {
                Ok(_) => info!("Action sent successfully"),
                Err(err) => error!("Error sending action: {:?}", err), //needs to store the error in database
            }
        });
        Ok(())
    }
}
