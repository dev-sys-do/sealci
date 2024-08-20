use proto::scheduler_client::SchedulerClient;
use std::error::Error;

pub mod proto {
	tonic::include_proto!("agent");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let url = "http://[::1]:50051";
	let mut client = SchedulerClient::connect(url).await?;

	let req = proto::SchedulerRequest { a: 4, b: 5 };
	let request = tonic::Request::new(req);

	let response = client.add(request).await?;

	println!("Response: {:?}", response.get_ref().result);

	Ok(())
}
