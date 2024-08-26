use std::sync::{Arc, Mutex};

use tokio::task;

use crate::{
    parser::pipe_parser::{Action, ManifestParser, ParsingError, Pipeline},
    scheduler::SchedulerService,
};

pub struct PipelineService {
    client: Arc<SchedulerService>,
    parser: Arc<dyn ManifestParser>,
}

#[derive(Debug)]
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

    pub async fn send_actions(&self, pipeline: Pipeline) -> Result<(), PipelineServiceError> {
        let client = Arc::clone(&self.client);
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
            client.send_action(action).await.unwrap();
        });
        Ok(())
    }
}
