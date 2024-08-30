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

        info!("Received Context Action request: {},\n
            Context runner type: {}", context.container_image.unwrap(), runner_type);

        // Lock the Agent Pool (to ensure thread-safe access). This is a tokio Mutex, not a standard one.
        let pool = self.agent_pool.lock().await;

        // Same for the Action Queue
        let mut queue = self.action_queue.lock().await;

        // Create a new Action and add it to the Queue
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

        // Loop over the Action Queue and choose an Agent
        while let Some(action) = queue.pop() {
            info!("Scheduled Actio: {:?}", action);

            // Get the Agent with the lowest score from the Agent Pool
            let agent = match pool.peek() {
                Some(agent) => agent,
                None => {
                    warn!("No agents available to execute Action");
                    continue;  // Continue until an Agent is available.
                    // TODO: Tell Controller to implement a timeout mechanism for each Action. (As in Gitlab CI, etc.)
                    // OR: return an error. This avoids an infinite, 5s wait loop.
                    // return Err(tonic::Status::unavailable("No agents available"));
                }
            };
            // TODO: insert more precise Agent selection logic.
            // Else, an Agent can be overloaded with all the actions from a single batch.
            
            // Send the Action to the Agent using agent_client.rs
            if let Err(e) = agent_client::execution_action(action, agent.get_ip_address()).await {
                warn!("Failed to execute Action: {}", e);
                return Err(tonic::Status::internal("Failed to execute Action"));
            }

            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            // This is a (temporary? If there is nothing better) solution to avoid flooding an Agent with all the Actions from a batch.
            // It allows the Agent to recalibrate its score after each Action.
            // And, most necessarily, if there are no Agents available, it will not run into an fast-paced infinite loop.
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
