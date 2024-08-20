use proto::agent_server::{Agent, AgentServer};
use tonic::transport::Server;
use tokio_stream::StreamExt;

mod proto {
	tonic::include_proto!("scheduler");

	pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
		tonic::include_file_descriptor_set!("scheduler_descriptor");
}

#[derive(Debug, Default)]
struct AgentService {}

#[tonic::async_trait]
impl Agent for AgentService {
	async fn register_agent(
		&self,
		request: tonic::Request<proto::Health>,
	) -> Result<tonic::Response<proto::RegisterAgentResponse>, tonic::Status> {
		let input = request.get_ref();

		println!("Received request from agent: {:?}", input);
		println!("  - Agent CPU usage: {}\n  - Agent memory usage: {}", input.cpu_usage, input.memory_usage);

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
						// the fields blabla options
						if let Some(health) = status.health {
							println!("Received health status from agent {}: CPU: {}, Memory: {}",
									status.agent_id,
									health.cpu_usage,
									health.memory_usage
							);
							/* TODO: handle health status (update Agent pool) */
						} else {
							eprintln!("Health field is missing for agent {}", status.agent_id);
						}
					} Err(e) => {
							eprintln!("Error receiving health status: {:?}", e);
							return Err(tonic::Status::internal("Error receiving health status"));
					}
			}
		}

			let response = proto::Empty {};
			Ok(tonic::Response::new(response))
	}
}

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
