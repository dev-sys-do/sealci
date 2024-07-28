use std::sync::Arc;

use tonic::transport::Channel;

use crate::{
    parser::pipe_parser::{ManifestParser, ParsingError, Pipeline},
    scheduler::{self, controller_client::ControllerClient, ExecutionContext, RunnerType},
};

pub struct PipelineService {
    client: Arc<ControllerClient<Channel>>,
    parser: Arc<dyn ManifestParser>,
}

impl PipelineService {
    pub fn new(client: Arc<ControllerClient<Channel>>, parser: Arc<dyn ManifestParser>) -> Self {
        Self { client, parser }
    }

    pub fn create_pipeline(&self, manifest: String) -> Result<Pipeline, ParsingError> {
        self.parser.parse(manifest)
    }

    pub fn send_actions(&self, pipeline: Pipeline) {
        for action in pipeline.actions {
            let request = scheduler::ActionRequest {
                context: Some(ExecutionContext {
                    r#type: 1,
                    container_image: Some("node:latest".to_string()),
                }),
                action_id: action.name,
                commands: action.commands,
            };
        }
    }
}
