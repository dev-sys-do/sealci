use crate::proto as proto;
use proto::agent_server::Agent;
use tokio_stream::StreamExt;
use log::{info, error};
use crate::logic::agent_logic::{compute_score, AgentPool};
use crate::logic::agent_logic::Agent as PoolAgent;
use std::sync::{Arc, Mutex};

pub struct AgentService {
	agent_pool: Arc<Mutex<AgentPool>>,  // Use Arc and Mutex are used for shared access across async tasks, and thread safety
}

impl AgentService {
	pub fn new() -> Self {
		Self {
			agent_pool: Arc::new(Mutex::new(AgentPool::new())), // Initialize the AgentPool
		}
	}
}

#[tonic::async_trait]
impl Agent for AgentService {
	async fn register_agent(
		&self,
		request: tonic::Request<proto::Health>,
	) -> Result<tonic::Response<proto::RegisterAgentResponse>, tonic::Status> {
		let input = request.get_ref();

		info!("Received request from Agent: {:?}", input);
		info!("\n  - Agent CPU usage: {}\n  - Agent memory usage: {}", input.cpu_usage, input.memory_usage);

		// Lock the agent pool (to ensure thread-safe access) and handle potential mutex poisoning
		let mut pool = match self.agent_pool.lock() {
			Ok(pool) => pool,
			Err(poisoned) => {
					error!("Agent pool lock poisoned, recovering...");
					poisoned.into_inner()
			}
		};

		let id = pool.generate_unique_id();
		let score = compute_score(input.cpu_usage, input.memory_usage);

		// Create a new Agent and add it to the Pool (it gets sorted)
		let new_agent = PoolAgent {
			id,
			score,
		};

		pool.push(new_agent);

		// Respond with the newly created Agent's ID.
		let response = proto::RegisterAgentResponse {
			id: id,
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
							info!("Received health status from Agent {}: CPU: {}, Memory: {}",
									status.agent_id,
									health.cpu_usage,
									health.memory_usage
							);

							// Lock the agent pool (to ensure thread-safe access) and handle potential mutex poisoning
							let mut pool = match self.agent_pool.lock() {
								Ok(pool) => pool,
								Err(poisoned) => {
										error!("Agent pool lock poisoned, recovering...");
										poisoned.into_inner()
								}
							};

							// Find the Agent in the Pool
							if let Some(agent) = pool.find_agent(status.agent_id) {
								// Compute the Agent's new score.
								let updated_score = compute_score(health.cpu_usage, health.memory_usage);

								// Check if the Agent's position in the Pool is now out of order
								if let is_out_of_order = pool.check_agent_neighbors(agent.id) {
									if is_out_of_order {
										// Resort the Pool if the Agent is out of order
										pool.sort();
									}
								}
							} else {
									error!("Agent ID {} not found in the Pool", status.agent_id);
									// Should probably return an error to the gRPC client stream here. Cut the stream? How to handle on Agent side?
							}
						} else {
							error!("Health field is missing for Agent {}", status.agent_id);
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
