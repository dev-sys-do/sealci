use crate::proto as proto;
use proto::agent_server::Agent;
use tokio_stream::StreamExt;
use log::{info, error};
use crate::logic::agent_logic::{compute_score, AgentPool};
use crate::logic::agent_logic::Agent as PoolAgent;
use std::sync::{Arc, Mutex};

pub struct AgentService {
    agent_pool: Arc<Mutex<AgentPool>>,  // Use Arc and Mutex for shared access across async tasks, and thread safety
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
        let new_agent = PoolAgent::new(id, score);

        // Response is the newly created Agent's ID.
        let response = proto::RegisterAgentResponse { id: new_agent.get_id() };

        pool.push(new_agent);

        Ok(tonic::Response::new(response))
    }

    async fn report_health_status(
        &self,
        request: tonic::Request<tonic::Streaming<proto::HealthStatus>>,
    ) -> Result<tonic::Response<proto::Empty>, tonic::Status> {
        let mut stream = request.into_inner();

        while let Some(health_status) = stream.next().await {
            let status = match health_status {
                Ok(status) => status,
                Err(e) => {
                    error!("Error receiving health status: {:?}", e);
                    return Err(tonic::Status::internal("Error receiving health status"));
                }
            };

            let health = match status.health {
                Some(health) => health,
                None => {
                    error!("Health field is missing for Agent {}", status.agent_id);
                    continue;  // Skip to the next health status if health data is missing
                }
            };

            info!("Received health status from agent {}: CPU: {}, Memory: {}",
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
            let agent = match pool.find_agent_mut(status.agent_id) {
                Some(agent) => agent,
                None => {
                    error!("Agent ID {} not found in the Pool", status.agent_id);
                    continue;  // Skip to the next health status if the agent is not found
                }
            }; 

            // Compute the Agent's new score and set it.
            let updated_score = compute_score(health.cpu_usage, health.memory_usage);
            agent.set_score(updated_score);

            // Check if the Agent's position in the Pool is now out of order
            let is_out_of_order = pool.check_agent_neighbors(status.agent_id);
            if is_out_of_order {
                pool.sort();  // Resort the Pool if the Agent is out of order
            }
        }

        Ok(tonic::Response::new(proto::Empty {}))
    }
}
