use tonic::transport::Server;
mod proto;
use proto::agent_server::AgentServer;
mod interfaces;
use interfaces::agent_interface::AgentService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let addr = "[::1]:50051".parse()?;

	let agent = AgentService::default();

	let service = tonic_reflection::server::Builder::configure()
		.register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
		.build()?;

	Server::builder()
		.add_service(service)
		.add_service(AgentServer::new(agent))
		.serve(addr)
		.await?;

	Ok(())
}
