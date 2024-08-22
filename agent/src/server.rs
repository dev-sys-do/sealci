use crate::action::launch_action;
use crate::proto::{
    action_service_server::ActionService, ActionRequest, ActionResponseStream, ActionResult,
};
use futures_util::Stream;
use futures_util::StreamExt;
use serde::Deserialize;
use serde_json::Error;
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::metadata::MetadataMap; // Import the StreamExt trait
use tonic::{async_trait, Request, Response, Status};

#[derive(Deserialize)]
struct Context {
    #[serde(rename = "type")]
    context_type: String,
    container_image: String,
}

#[derive(Deserialize)]
struct ActionReq {
    action_id: String,
    context: Context,
    commands: Vec<String>,
}

#[derive(Default)]
pub struct ActionsLauncher {}

#[async_trait]
impl ActionService for ActionsLauncher {
    type ExecutionActionStream =
        Pin<Box<dyn Stream<Item = Result<ActionResponseStream, Status>> + Send>>;

    async fn execution_action(
        &self,
        request: Request<ActionRequest>,
    ) -> Result<Response<Self::ExecutionActionStream>, Status> {
        let mut commands = vec!["echo test2", "echo test1"];
        let (tx, rx) = mpsc::unbounded_channel();

        let image_name = get_container_image(request.metadata());
        let name = match image_name {
            Ok(name) => name,
            Err(_) => return Err(Status::invalid_argument("image_name is missing")),
        };

        match launch_action(name, &mut commands).await {
            Ok(mut output) => {
                let tx_clone = tx.clone();
                tokio::spawn(async move {
                    while let Some(log) = output.next().await {
                        match log {
                            Ok(log_output) => {
                                let _ = tx_clone.send(Ok(ActionResponseStream {
                                    log: log_output.to_string(),
                                    action_id: "1".to_string(),
                                    result: Some(ActionResult {
                                        completion: 0,
                                        exit_code: Some(0),
                                    }),
                                }));
                            }
                            Err(e) => {
                                let _ = tx_clone
                                    .send(Err(Status::internal(format!("error happened {}", e))));
                            }
                        }
                    }
                });
            }
            Err(_) => return Err(Status::internal("Error happened when launching action")),
        };

        let stream = UnboundedReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(stream) as Self::ExecutionActionStream
        ))
    }
}

fn get_container_image(metadata: &MetadataMap) -> Result<String, Error> {
    // Extract the JSON string from the metadata
    let json_str = metadata
        .get("request")
        .and_then(|value| value.to_str().ok());
    let val = match json_str {
        Some(value) => value,
        _ => "test",
    };
    println!("{}", val);
    // Parse the JSON string into a serde_json::Value
    let request: ActionReq = serde_json::from_str(val)?;

    // Extract the container_image
    Ok(request.context.container_image)
}
