use crate::proto::actions as proto;
use proto::action_service_client::ActionServiceClient as ActionClient;
use tonic::transport::Channel;
use tonic::Request;
use std::error::Error;
use log::{info, error};

#[tokio::main]
async fn execution_action() -> Result<(), Box<dyn Error>> {
    let channel = Channel::from_static("http://[::1]:50051").connect().await?;
    let mut client = ActionClient::new(channel);

    let request = Request::new(proto::ActionRequest {
        action_id: 1,
        context: Some(proto::ExecutionContext {
            r#type: proto::RunnerType::Docker.into(),
            container_image: Some(String::from("test_image")),
        }),
        commands: vec![String::from("echo 'salut les agents\n\n\n\n          O\\ w /O\n\n'"), String::from("shutdown now")],
    });

    let mut response_stream = client.execution_action(request).await?.into_inner();

    while let Some(response) = response_stream.message().await? {
        info!("\nresponse action ID: {}\n log: {}", response.action_id, response.log);
        if let Some(result) = &response.result {
            info!("\n result:{}\nwith code: {}", result.completion, result.exit_code.unwrap());
        } else {
            error!("No result in response");
        }
    }

    Ok(())
}
