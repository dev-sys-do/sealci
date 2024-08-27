use scheduler::proto::{controller_client::ControllerClient, ActionRequest, ExecutionContext, RunnerType};
use scheduler::proto::agent_server::AgentServer;
use scheduler::proto::controller_server::ControllerServer;
use scheduler::interfaces::agent_interface::AgentService;
use scheduler::interfaces::controller_interface::ControllerService;
use tonic::transport::Server;
use tonic::transport::Channel;
use tonic::Request;
use std::error::Error;
use tokio::time::Duration;

#[tokio::test]
async fn test_execution_action() -> Result<(), Box<dyn Error>> {
    let channel = Channel::from_static("http://[::1]:50051").connect().await?;
    let mut client = ControllerClient::new(channel);

    let request = Request::new(ActionRequest {
        action_id: "test_action_id".to_string(),
        context: Some(ExecutionContext {
            r#type: RunnerType::Docker.into(),
            container_image: Some("test_image".to_string()),
        }),
        commands: vec!["echo".to_string(), "Hello, World!".to_string()],
    });

    let mut response_stream = client.execution_action(request).await?.into_inner();

    while let Some(response) = response_stream.message().await? {
        // Modify the assertion based on the correct field available in your response
        assert_eq!(response.action_id, "mock_action_id");
    }

    Ok(())
}
