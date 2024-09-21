use std::sync::Arc;

use tokio::sync::Mutex;

use tonic::{transport::Channel, Request};
use tracing::{error, info};

use crate::{
    action::{action_repository::Action, action_service::ActionService},
    grpc_scheduler::{
        self, controller_client::ControllerClient, ActionStatus, ExecutionContext, RunnerType,
    },
    logs::log_repository::LogRepository,
    pipeline::pipeline_service::PipelineServiceError,
};

pub struct SchedulerService {
    client: Arc<Mutex<ControllerClient<Channel>>>,
    log_repository: Arc<LogRepository>,
    action_service: Arc<ActionService>,
}

impl SchedulerService {
    pub fn new(
        client: Arc<Mutex<ControllerClient<Channel>>>,
        log_repository: Arc<LogRepository>,
        action_service: Arc<ActionService>,
    ) -> Self {
        Self {
            client,
            log_repository,
            action_service,
        }
    }

    pub async fn send_action(
        &self,
        action: Arc<Action>,
        repo_url: String,
    ) -> Result<(), PipelineServiceError> {
        let id: Result<u32, _> = action.id.try_into();
        let action_request = grpc_scheduler::ActionRequest {
            context: Some(ExecutionContext {
                r#type: RunnerType::Docker.into(), //for now we only support container actions
                container_image: Some(action.container_uri.clone()),
            }),
            action_id: id.map_err(|e| {
                error!("Error while converting action id: {:?}", e);
                PipelineServiceError::SchedulerError
            })?,
            commands: action.commands.clone(),
            repo_url: repo_url.clone(),
        };

        let request = Request::new(action_request);
        let mut client = self.client.lock().await;

        let mut stream = client
            .schedule_action(request)
            .await
            .map_err(|_err| {
                error!("Error while sending action to scheduler : {:?}", _err);
                PipelineServiceError::SchedulerError
            })?
            .into_inner();

        while let Some(response) = stream.message().await.map_err(|_err| {
            error!("Error while receiving message from scheduler : {:?}", _err);
            PipelineServiceError::SchedulerError
        })? {
            info!("[SCHEDULER] RESPONSE={:?}", response);
            self.log_repository
                .create(i64::from(response.action_id), &response.log)
                .await
                .map_err(|e| {
                    error!("Error while storing log: {:?}", e);
                    PipelineServiceError::StoringLogError
                })?;

            let status = ActionStatus::as_str_name(&response.result.unwrap().completion()); //TODO: for now we are going to unwrap all cast probable errors. Though we should handle them properly by sending a Error message through gRPC to the Scheduler

            info!("[SCHEDULER] STATUS={:?}", status);
            self.action_service
                .update_status(
                    i64::from(response.action_id),
                    &ActionStatus::from_str_name(status).unwrap(), //We can unwrap here because we are sure that the status is a valid one
                )
                .await
                .map_err(|e| {
                    error!("Error while updating action status: {:?}", e);
                    PipelineServiceError::SchedulerError
                })?; //same here we should be sending an error status to the scheduler

            info!("[SCHEDULER] RESPONSE={:?}", response);
        }

        Ok(())
    }
}
