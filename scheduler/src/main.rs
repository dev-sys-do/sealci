use env_logger;
use log::info;
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

	let agent = AgentService::new();
	let controller = ControllerService::new();

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
