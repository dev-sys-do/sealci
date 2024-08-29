use std::sync::{Arc, Mutex};

use env_logger;
use log::info;
use logic::agent_logic::AgentPool;
use logic::controller_logic::ActionsQueue;
use tonic::transport::Server;

mod proto;
use proto::agent::agent_server::AgentServer;
use proto::controller::controller_server::ControllerServer;

mod interfaces;
use interfaces::server::agent_interface::AgentService;
use interfaces::server::controller_interface::ControllerService;

mod logic;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::init();

	let addr = "[::1]:50051".parse()?;

	// Initializes the Agent Pool and Action queue. They are lost when the Scheduler dies.
	let agent_pool = Arc::new(Mutex::new(AgentPool::new()));
	let action_queue = Arc::new(Mutex::new(ActionsQueue::new()));

	// Pass the shared Agent Pool to Agent and Controller services.
	let agent = AgentService::new(agent_pool.clone());
	let controller = ControllerService::new(action_queue.clone(), agent_pool.clone());

	let service = tonic_reflection::server::Builder::configure()
		.register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
		.build()?;

  info!("Starting gRPC server at {}", addr);
	Server::builder()
		.add_service(service)
		.add_service(AgentServer::new(agent))
		.add_service(ControllerServer::new(controller))
		.serve(addr)
		.await?;

	Ok(())
}
