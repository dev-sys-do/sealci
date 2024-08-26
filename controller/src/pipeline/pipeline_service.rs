use std::sync::{Arc, Mutex};

use tokio::task;
use tonic::{transport::Channel, Request};

use crate::{
    grpc_scheduler::{self, controller_client::ControllerClient, ExecutionContext, RunnerType},
    parser::pipe_parser::{Action, ManifestParser, ParsingError, Pipeline},
    scheduler::SchedulerService,
};

pub struct PipelineService {
    client: Arc<SchedulerService>,
    parser: Arc<dyn ManifestParser>,
}

pub enum PipelineServiceError {
    ParsingError(ParsingError),
    SchedulerError,
}

impl PipelineService {
    pub fn new(client: Arc<SchedulerService>, parser: Arc<dyn ManifestParser>) -> Self {
        Self { client, parser }
    }

    pub fn create_pipeline(&self, manifest: String) -> Result<Pipeline, ParsingError> {
        self.parser.parse(manifest)
    }

    pub async fn send_actions(
        &self,
        client: Arc<SchedulerService>,
        pipeline: Pipeline,
    ) -> Result<(), PipelineServiceError> {
        for action in pipeline.actions {
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
            client.send_action(action);
        });
        Ok(())
    }
}
