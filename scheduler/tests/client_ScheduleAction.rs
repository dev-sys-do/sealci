use proto::controller_client::ControllerClient;
use tonic::transport::Channel;
use tonic::Request;
use crate::proto::{ActionRequest, ExecutionContext, RunnerType};


mod proto {
	tonic::include_proto!("scheduler");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client connected to the gRPC server
    let channel = Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;

    let mut client = ControllerClient::new(channel);

    // Create a test ActionRequest
    let request = Request::new(ActionRequest {
        action_id: "test_action_id".to_string(),
        context: Some(ExecutionContext {
            r#type: RunnerType::Docker.into(),
            container_image: Some("test_image".to_string()),
        }),
        commands: vec!["echo".to_string(), "Hello, World!".to_string()],
    });

    // Call the schedule_action method and receive the stream
    let mut response_stream = client.schedule_action(request).await?.into_inner();

    // Collect and print the responses from the stream
    while let Some(response) = response_stream.message().await? {
        println!("Received ActionResponse: {:?}", response);
    }

    Ok(())
}
