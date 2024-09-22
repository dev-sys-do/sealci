use sqlx::PgPool;
use std::sync::Arc;

use tokio::task;
use tracing::{error, info};

use crate::action::action_repository::Action;
use crate::action::action_service::{ActionDTO, ActionService};
use crate::grpc_scheduler::ActionStatus;
use crate::logs::log_repository::LogRepository;
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
    logs_repository: Arc<LogRepository>,
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
        let logs_repository = Arc::new(LogRepository::new(pool.clone()));
        Self {
            client,
            parser,
            repository,
            logs_repository,
            action_service,
        }
    }

    pub async fn find_all(&self, verbose: bool) -> Vec<Pipeline> {
        match self.repository.find_all().await {
            Ok(mut pipelines) => {
                if verbose {
                    for pipeline in &mut pipelines {
                        if let Err(e) = self.add_verbose_details(pipeline).await {
                            error!("Error while fetching verbose details for pipeline id {}: {:?}", pipeline.id, e);
                        } else {
                            info!("Verbose details added for pipeline id: {}", pipeline.id);
                        }
                    }
                }
                pipelines
            }
            Err(e) => {
                info!("Error while fetching pipelines: {:?}", e);
                vec![]
            }
        }
    }

    pub async fn find(&self, id: i64, verbose: bool) -> Option<Pipeline> {
        match self.repository.find_by_id(id).await {
            Ok(mut pipeline) => {
                if verbose {
                    if let Err(e) = self.add_verbose_details(&mut pipeline).await {
                        error!("Error while fetching verbose details for pipeline: {:?}", e);
                    } else {
                        info!("Verbose details added for pipeline id: {}", id);
                    }
                }
                Some(pipeline)
            }
            Err(e) => {
                println!("{:}", e);
                error!("Error while fetching pipeline: {:?}", e);
                None
            }
        }
    }

    async fn add_verbose_details(&self, pipeline: &mut Pipeline) -> Result<(), String> {
        for action in &mut pipeline.actions {
            info!("Fetching verbose details for action: {:?}", action);
    
            match self.logs_repository.find_by_action_id(action.id).await {
                Ok(logs) => {
                    action.logs = Some(logs.into_iter().map(|log| log.message).collect());
                }
                Err(e) => {
                    return Err(format!("Error fetching logs for action {}: {}", action.name, e));
                }
            }
        }
        Ok(())
    }

    pub async fn create_pipeline(
        &self,
        repository_url: &String,
        name: &String,
    ) -> Result<PipelineDTO, Box<dyn std::error::Error>> {
        info!("Creating pipeline for repository: {}", repository_url);
        let pipeline = self.repository.create(repository_url, name).await;
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
        let pipeline = self.create_pipeline(&repo_url, &manifest.name).await?;
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
                .await
                .map_err(|e| Box::new(e))?;
            actions.push(action);
        }

        Ok(Pipeline::new(
            pipeline.id,
            pipeline.repository_url,
            pipeline.name,
            actions,
        ))
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
