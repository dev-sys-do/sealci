use crate::proto::actions as proto;
use proto::action_service_client::ActionServiceClient as ActionClient;

use crate::logic::action_queue_logic::Action;

use tonic::transport::Channel;
use tonic::Request;
use std::error::Error;
use log::{info, error};

pub(crate) async fn execution_action(action: Action, agent_address: String) -> Result<tonic::Streaming<proto::ActionResponseStream>, Box<dyn Error + Send + Sync>> {
    // Handle case where hostname is empty
    if agent_address == "unknown:unknown" {
        error!("Hostname is empty. Cannot resolve IP address.");
        return Err(Box::from("Hostname is empty. Cannot resolve IP address."));
    }

    let channel = Channel::builder(agent_address.parse()?).connect().await?;
    let mut client = ActionClient::new(channel);

    let request = Request::new(proto::ActionRequest {
        action_id: action.get_action_id(),
        context: Some(proto::ExecutionContext {
            r#type: action.get_runner_type(),
            container_image: Some(String::from(action.get_container_image())),
        }),
        commands: action.get_commands().iter().map(|comm: &String| String::from(comm)).collect(),
        repo_url: action.get_repo_url().clone(),
    });

    /*while let Some(response) = response_stream.message().await? {
        info!("\nresponse action ID: {}\n log: {}", response.action_id, response.log);
        if let Some(result) = &response.result {
            info!("\n result:{}\nwith code: {}", result.completion, result.exit_code.unwrap());
        } else {
            error!("No result in response");
        }
    }*/

    // The response stream is returned to the caller function for further processing.
    let response_stream = client.execution_action(request).await?.into_inner();
    Ok(response_stream)
}
