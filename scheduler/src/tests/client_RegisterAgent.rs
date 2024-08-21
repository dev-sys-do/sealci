use proto::agent_client::AgentClient;
use std::error::Error;

mod proto {
	tonic::include_proto!("scheduler");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let url = "http://[::1]:50051";
	let mut client = AgentClient::connect(url).await?;

	let req = proto::Health { cpu_usage: 123, memory_usage: 321 };
	let request = tonic::Request::new(req);

	let response = client.register_agent(request).await?;

	println!("Response: {:?}", response.get_ref());

	Ok(())
}
