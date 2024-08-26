use std::sync::{Arc, Mutex};

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

    pub async fn send_action(&self, action: Arc<Action>) -> Result<(), PipelineServiceError> {
        let action_request = grpc_scheduler::ActionRequest {
            context: Some(ExecutionContext {
                r#type: 1,
                container_image: Some("node:latest".to_string()),
            }),
            action_id: action.name.clone(),
            commands: action.commands.clone(),
        };

        let request = Request::new(action_request);
        let mut stream = self
            .client
            .lock()
            .unwrap()
            .schedule_action(request)
            .await
            .map_err(|err| PipelineServiceError::SchedulerError)?
            .into_inner();

        while let Some(response) = stream
            .message()
            .await
            .map_err(|err| PipelineServiceError::SchedulerError)?
        {
            println!("RESPONSE={:?}", response);
        }

        Ok(())
    }
}
