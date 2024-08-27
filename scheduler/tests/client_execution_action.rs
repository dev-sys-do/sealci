use scheduler::proto::{actions_client::ActionsClient, ActionRequest, ExecutionContext, RunnerType};
use tonic::transport::Channel;
use tonic::Request;
use std::error::Error;

#[tokio::test]
async fn test_execution_action() -> Result<(), Box<dyn Error>> {
    let channel = Channel::from_static("http://[::1]:50051").connect().await?;
    let mut client = ActionsClient::new(channel);

    let request = Request::new(ActionRequest {
        action_id: String::from("azuyr-91881FZ68fz681ZF-f"),
        context: Some(ExecutionContext {
            r#type: RunnerType::Docker.into(),
            container_image: Some(String::from("test_image")),
        }),
        commands: vec![String::from("echo 'salut les agents\n\n\n\n          O\\ w /O\n\n'"), String::from("shutdown now")],
    });

    let mut response_stream = client.execution_action(request).await?.into_inner();

    while let Some(response) = response_stream.message().await? {
        // Modify the assertion based on the correct field available in your response
        assert_eq!(response.action_id, "mock_action_id");
    }

    Ok(())
}
