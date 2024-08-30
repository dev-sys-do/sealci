use std::sync::Arc;

use tokio::sync::Mutex;

use tonic::{transport::Channel, Request};

use crate::{
    grpc_scheduler::{self, controller_client::ControllerClient, ExecutionContext},
    parser::pipe_parser::Action,
    pipeline::pipeline_service::PipelineServiceError,
};

pub struct SchedulerService {
    client: Arc<Mutex<ControllerClient<Channel>>>,
}

impl SchedulerService {
    pub fn new(client: Arc<Mutex<ControllerClient<Channel>>>) -> Self {
        Self { client }
    }

    pub async fn send_action(
        &self,
        action: Arc<Action>,
        repo_url: String,
    ) -> Result<(), PipelineServiceError> {
        let action_request = grpc_scheduler::ActionRequest {
            context: Some(ExecutionContext {
                r#type: 1,
                container_image: Some(action.configuration_version.clone()),
            }),
            action_id: action.name.clone(),
            commands: action.commands.clone(),
            repo_url: repo_url.clone(),
        };

        let request = Request::new(action_request);
        let mut client = self.client.lock().await;

        let mut stream = client
            .schedule_action(request)
            .await
            .map_err(|_err| PipelineServiceError::SchedulerError)?
            .into_inner();

        while let Some(response) = stream
            .message()
            .await
            .map_err(|_err| PipelineServiceError::SchedulerError)?
        {
            println!("RESPONSE={:?}", response);
        }

        Ok(())
    }
}
