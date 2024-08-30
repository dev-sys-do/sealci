use crate::proto::actions as proto;
use proto::action_service_client::ActionServiceClient as ActionClient;

use crate::logic::controller_logic::Action;

    use tonic::transport::Channel;
use tonic::Request;
use std::error::Error;
use log::{info, error};

pub(crate) async fn execution_action(action: Action) -> Result<(), Box<dyn Error>> {
    let channel = Channel::from_static("http://[::1]:50051").connect().await?;
    let mut client = ActionClient::new(channel);

    let request = Request::new(proto::ActionRequest {
        action_id: action.get_action_id(),
        context: Some(proto::ExecutionContext {
            r#type: action.get_runner_type(),
            container_image: Some(String::from(action.get_container_image())),
        }),
        commands: action.get_commands().iter().map(|s| s.to_string()).collect(),
    });
    
    /*
    let request = Request::new(proto::ActionRequest {
        action_id: 69420,
        context: Some(proto::ExecutionContext {
            r#type: proto::RunnerType::Docker.into(),
            container_image: Some(String::from("test_image")),
        }),
        commands: vec![String::from("echo 'salut les agents\n\n\n\n          O\\ w /O\n\n'"), String::from("shutdown now")],
    });
    */

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
