use std::sync::Arc;
use tokio::sync::Mutex;

use env_logger;
use log::info;
use logic::agent_pool_logic::AgentPool;
// use logic::action_queue_logic::ActionsQueue;
use tonic::transport::Server;

mod proto;
//use proto::agent::agent_server::AgentServer;
//use proto::controller::controller_server::ControllerServer;
use proto::scheduler::agent_server::AgentServer;
use proto::scheduler::controller_server::ControllerServer;

mod interfaces;
use interfaces::server::agent_interface::AgentService;
use interfaces::server::controller_interface::ControllerService;

mod logic;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	env_logger::init();

	let addr = "[::0]:50051".parse()?;

	// Initializes the Agent Pool and Action queue. They are lost when the Scheduler dies.
	let agent_pool = Arc::new(Mutex::new(AgentPool::new()));
	//let action_queue = Arc::new(Mutex::new(ActionsQueue::new()));

	// Pass the shared Agent Pool to Agent and Controller services.
	let agent = AgentService::new(agent_pool.clone());
	let controller = ControllerService::new(agent_pool.clone());

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
