use crate::logic::agent_logic::Agent as PoolAgent;
use crate::logic::agent_logic::{compute_score, AgentPool};
use crate::proto::agent::{self as proto, Health};
use log::{error, info};
use proto::agent_server::Agent;
use std::sync::{Arc, Mutex};
use tokio_stream::StreamExt;
//std::sync::atomic::AtomicU32;

pub struct AgentService {
    agent_pool: Arc<Mutex<AgentPool>>, // The ArcMutex is on the agent_pool, for the highest level of granularity on concurrency control
}

impl AgentService {
    pub fn new() -> Self {
        Self {
            agent_pool: Arc::new(Mutex::new(AgentPool::new())), // Initialize the Agent Pool. It is lost when the Scheduler dies.
        }
    }
}

#[tonic::async_trait]
impl Agent for AgentService {
    async fn register_agent(
        &self,
        request: tonic::Request<proto::RegisterAgentRequest>,
    ) -> Result<tonic::Response<proto::RegisterAgentResponse>, tonic::Status> {
        let input = request.into_inner().health.unwrap();

        info!("Received request from Agent: {:?}", input);
        info!(
            "\n  - Agent CPU usage: {}\n  - Agent memory usage: {}",
            input.cpu_avail, input.memory_avail
        );

        let hostname = request.into_inner().hostname.unwrap();

        info!("Received request from Agent: {:?}", hostname);
        info!(
            "\n  - Agent host usage: {}\n  - Agent port usage: {}",
            hostname.host, hostname.port
        );

        let hostname = request.into_inner().hostname.unwrap();

        info!("Received request from Agent: {:?}", hostname);
        info!(
            "\n  - Agent host usage: {}\n  - Agent port usage: {}",
            hostname.host, hostname.port
        );

        // Lock the agent pool (to ensure thread-safe access) and panic if mutex is poisonned.
        let mut pool = self.agent_pool.lock().unwrap();

        let id = pool.generate_unique_id();
        let score = compute_score(input.cpu_avail, input.memory_avail);

        // Create a new Agent and add it to the Pool (it gets sorted)
        let new_agent = PoolAgent::new(id, score);

        // Response is the newly created Agent's ID.
        let response = proto::RegisterAgentResponse {
            id: new_agent.get_id(),
        };

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
                    continue; // Skip to the next health status if health data is missing
                }
            };

            info!(
                "Received health status from agent {}: CPU: {}, Memory: {}",
                status.agent_id, health.cpu_avail, health.memory_avail
            );

            // Lock the agent pool (to ensure thread-safe access) and panic if mutex is poisonned.
            let mut pool = self.agent_pool.lock().unwrap();

            // Find the Agent in the Pool
            let agent = match pool.find_agent_mut(status.agent_id) {
                Some(agent) => agent,
                None => {
                    error!("Agent ID {} not found in the Pool", status.agent_id);
                    continue; // Skip to the next health status if the agent is not found
                }
            };

            // Compute the Agent's new score and set it.
            let updated_score = compute_score(health.cpu_avail, health.memory_avail);
            agent.set_score(updated_score);

            // Check if the Agent's position in the Pool is now out of order
            let is_out_of_order = pool.check_agent_neighbors(status.agent_id);
            if is_out_of_order {
                pool.sort(); // Resort the Pool if the Agent is out of order
            }
        }

        Ok(tonic::Response::new(proto::Empty {}))
    }
}
