use crate::proto as proto;
use proto::agent_server::Agent;
use tokio_stream::StreamExt;
use log::{info, error};

#[derive(Debug, Default)]
pub struct AgentService {}

#[tonic::async_trait]
impl Agent for AgentService {
	async fn register_agent(
		&self,
		request: tonic::Request<proto::Health>,
	) -> Result<tonic::Response<proto::RegisterAgentResponse>, tonic::Status> {
		let input = request.get_ref();

		info!("Received request from agent: {:?}", input);
		info!("\n  - Agent CPU usage: {}\n  - Agent memory usage: {}", input.cpu_usage, input.memory_usage);

		let response = proto::RegisterAgentResponse {
			id: String::from("your-id-0193748304AZORIHAER1203R238"),  /* TODO: Function to generate unique id (check agent pool) (use SHA1?) */
		};

		Ok(tonic::Response::new(response))
	}

	async fn report_health_status(
		&self,
		request: tonic::Request<tonic::Streaming<proto::HealthStatus>>,
	) -> Result<tonic::Response<proto::Empty>, tonic::Status> {
		let mut stream = request.into_inner();

		while let Some(health_status) = stream.next().await {
			match health_status {
					Ok(status) => {
						// the fields must be unwrapped because they are Option<T> (they can be None) ; we use Some(T) to retrieve the wrapped value.
						if let Some(health) = status.health {
							info!("Received health status from agent {}: CPU: {}, Memory: {}",
									status.agent_id,
									health.cpu_usage,
									health.memory_usage
							);
							/* TODO: handle health status (update Agent pool) */
						} else {
							error!("Health field is missing for agent {}", status.agent_id);
						}
					} Err(e) => {
							error!("Error receiving health status: {:?}", e);
							return Err(tonic::Status::internal("Error receiving health status"));
					}
			}
		}

		let response = proto::Empty {};
		Ok(tonic::Response::new(response))
	}
}
