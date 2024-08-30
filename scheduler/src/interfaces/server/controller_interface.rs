use crate::interfaces::client::agent_client;

use crate::logic::agent_logic::AgentPool;
use crate::logic::controller_logic::{Action, ActionsQueue};

//use crate::proto::controller as proto;
use crate::proto::scheduler as proto;
use proto::controller_server::Controller;

use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc;
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn};

pub struct ControllerService {
    action_queue: Arc<Mutex<ActionsQueue>>,
    agent_pool: Arc<Mutex<AgentPool>>,
}

impl ControllerService {
    pub fn new(action_queue: Arc<Mutex<ActionsQueue>>, agent_pool: Arc<Mutex<AgentPool>>) -> Self {
        Self {
            action_queue,
            agent_pool,
        }
    }
}

type ScheduleActionStream = ReceiverStream<Result<proto::ActionResponse, tonic::Status>>;

#[tonic::async_trait]
impl Controller for ControllerService {
    type ScheduleActionStream = ScheduleActionStream;

    async fn schedule_action(
        &self,
        request: tonic::Request<proto::ActionRequest>,
    ) -> Result<tonic::Response<Self::ScheduleActionStream>, tonic::Status> {
        let action_request = request.into_inner();

        let context = match action_request.context.clone() {
            Some(context) => context,
            None => {
                warn!("Context field is missing for ActionRequest {}", action_request.action_id);
                return Err(tonic::Status::invalid_argument("Context field is missing"));
            }
        };

        let runner_type = match context.r#type.into() {
            Some(runner_type) => runner_type,
            None => {
                warn!("Invalid RunnerType in ExecutionContext for ActionRequest {}", action_request.action_id);
                return Err(tonic::Status::invalid_argument("Invalid RunnerType"));
            }
        };
        
        let container_image = match context.container_image.clone() {
            Some(container_image) => container_image,
            None => {
                warn!("ContainerImage field is missing for ActionRequest {}", action_request.action_id);
                return Err(tonic::Status::invalid_argument("ContainerImage field is missing"));
            }
        };

        info!("Received Context action request: {},\n
            Context runner type: {}", context.container_image.unwrap(), runner_type);

        // Lock the Agent Pool (to ensure thread-safe access). This is a tokio Mutex, not a standard one.
        let mut pool = self.agent_pool.lock().await;

        // Same for the Action Queue
        let mut queue = self.action_queue.lock().await;

        // Create a new Action and add it to the queue (it gets sorted)
        let new_action = Action::new(
            action_request.action_id,
            proto::ExecutionContext {
                container_image: Some(container_image),
                r#type: runner_type,
            },
            action_request.commands,
        );

        // Add the Action to the Action Queue
        queue.push(new_action);

        // Loop over the action queue
        /*while let Some(action) = queue.pop() {
            // Send the action to the Agent using agent_client.rs
            agent_client::execution_action().await;
        }*/

        // Loop over the action queue
        while let Some(action) = queue.pop() {
            // Send the action to the Agent using agent_client.rs
            if let Err(e) = agent_client::execution_action().await {
                warn!("Failed to execute action: {}", e);
                // You can choose to handle the error or propagate it up
                return Err(tonic::Status::internal("Failed to execute action"));
            }
        }

        // Create mock data for the response stream. This is the Log transfer.
        let mock_action_response = proto::ActionResponse {
            action_id: 69420,
            log: String::from("Mock log message"),
            result: Some(proto::ActionResult {
                completion: proto::ActionStatus::Completed.into(),
                exit_code: Some(0),
            }),
        };

        // Use an mpsc channel to create the response stream
        let (tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            tx.send(Ok(mock_action_response)).await.unwrap();
        });

        let response_stream = ReceiverStream::new(rx);
        
        Ok(tonic::Response::new(response_stream))
    }
}
