use crate::interfaces::client::agent_client;

use crate::logic::action_queue_logic::Action;
use crate::logic::agent_pool_logic::AgentPool;

use crate::proto::scheduler::ActionStatus;
//use crate::proto::controller as proto
use crate::proto::scheduler as proto;
use proto::controller_server::Controller;

use log::{info, warn};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio_stream::wrappers::UnboundedReceiverStream;

pub struct ControllerService {
    agent_pool: Arc<Mutex<AgentPool>>,
}

impl ControllerService {
    pub fn new(agent_pool: Arc<Mutex<AgentPool>>) -> Self {
        Self { agent_pool }
    }
}

type ScheduleActionStream = UnboundedReceiverStream<Result<proto::ActionResponse, tonic::Status>>;

#[tonic::async_trait]
impl Controller for ControllerService {
    type ScheduleActionStream = ScheduleActionStream;

    async fn schedule_action(
        &self,
        request: tonic::Request<proto::ActionRequest>,
    ) -> Result<tonic::Response<Self::ScheduleActionStream>, tonic::Status> {
        let action_request = request.into_inner();

        // Validate ActionRequest fields
        let (runner_type, container_image) = self.validate_action_request(&action_request)?;

        info!(
            "Received Action request: {}, Runner type: {}",
            container_image.clone().unwrap_or_default(),
            runner_type.as_str_name()
        );

        // Lock the agent pool a moment to check for available agents
        let pool = self.agent_pool.lock().await;
        let agent = match pool.peek() {
            Some(agent) => agent,
            None => {
                warn!("No Agents available to execute Action");
                // Send back an error response now, and close the stream.
                let (tx, rx) = mpsc::unbounded_channel();
                let error_response = proto::ActionResponse {
                    action_id: action_request.action_id,
                    log: "No agents available".to_string(),
                    result: Some(proto::ActionResult {
                        completion: proto::ActionStatus::Error.into(),
                        exit_code: None,
                    }),
                };
                tx.send(Ok(error_response)).unwrap_or_default(); // Send Ok or Err back? need to say schedule_action errored!!
                return Ok(tonic::Response::new(UnboundedReceiverStream::new(rx)));
            }
        };

        let agent_ip = agent.get_ip_address().to_string();

        // Create the action object
        let action = Action::new(
            action_request.action_id,
            proto::ExecutionContext {
                container_image,
                r#type: runner_type.into(),
            },
            action_request.commands,
            action_request.repo_url,
        );

        // Use an unbounded channel to create the response stream
        let (tx, rx) = mpsc::unbounded_channel();
        // The transmitter is passed into the spawned task to send the response back to the client.

        // Spawn an async task to handle action execution
        tokio::spawn(async move {
            // Send the action to the agent and forward the response/transfer the logs
            // The tokio::spawn function is used to create a new asynchronous task. To call execution_action without blocking the main schedule_action procedure.
            // execution_action returns a Stream, which is validated, error-handled, and passed to schedule action's response stream. This is the log transfer operation.
            match agent_client::execution_action(action, agent_ip).await {
                // The response stream from the Agent is received and processed here directly; in a spawned task. This is simply because it is much easier than handling multiple streams by ID.
                // Each received message is forwarded back to the controller.
                Ok(mut response_stream) => {
                    while let Some(response) = response_stream.message().await.unwrap_or(None) {
                        // Use match to handle the presence or absence of a result in the response
                        match response.result {
                            Some(result) => {
                                println!("Received a response with a result {:?}", result);
                                let completion = match result.exit_code {
                                    Some(exit_code) => {
                                        if exit_code == 0 {
                                            3
                                        } else {
                                            4
                                        }
                                    }
                                    None => result.completion,
                                };
                                let action_response = proto::ActionResponse {
                                    action_id: response.action_id,
                                    log: response.log,
                                    result: Some(proto::ActionResult {
                                        completion: completion.into(),
                                        exit_code: result.exit_code,
                                    }),
                                };

                                if tx.send(Ok(action_response)).is_err() {
                                    warn!("Failed to send action response");
                                    break;
                                }
                            }
                            None => {
                                warn!("Received a response with no result");
                            }
                        }
                    }
                }
                Err(e) => {
                    warn!("Failed to execute Action: {}", e);
                    let _ = tx.send(Err(tonic::Status::internal("Failed to execute Action")));
                }
            }
        });

        // Now outside the spawned task, the response stream is created and the receiver side of the channel is returned to the client/calling service.

        let response_stream = UnboundedReceiverStream::new(rx);
        Ok(tonic::Response::new(response_stream))
    }
}

impl ControllerService {
    fn validate_action_request(
        &self,
        action_request: &proto::ActionRequest,
    ) -> Result<(proto::RunnerType, Option<String>), tonic::Status> {
        let context = action_request
            .context
            .clone()
            .ok_or_else(|| tonic::Status::invalid_argument("Context field is missing"))?;

        // Convert `context.r#type` (which is an `i32`) to a `RunnerType`
        let runner_type = proto::RunnerType::from_i32(context.r#type)
            .ok_or_else(|| tonic::Status::invalid_argument("Invalid RunnerType"))?;

        let container_image = context
            .container_image
            .clone()
            .ok_or_else(|| tonic::Status::invalid_argument("ContainerImage field is missing"))?;

        Ok((runner_type, Some(container_image)))
    }
}
