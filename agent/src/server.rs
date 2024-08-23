use crate::action::launch_action;
use crate::proto::{
    action_service_server::ActionService, ActionRequest, ActionResponseStream, ActionResult,
};
use futures_util::Stream;
use futures_util::StreamExt;
use log::info;
use std::pin::Pin;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tonic::{async_trait, Request, Response, Status};

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
        let (tx, rx) = mpsc::unbounded_channel();
        let mut request_body = request.into_inner();
        let context = match request_body.context {
            Some(context) => context,
            None => return Err(Status::invalid_argument("Context is missing")),
        };
        let container_image = match context.container_image {
            Some(container_image) => container_image,
            None => return Err(Status::invalid_argument("Container image is missing")),
        };

        info!(
            "Executing action {:?} in image {}",
            request_body.action_id, container_image
        );

        match launch_action(container_image, &mut request_body.commands).await {
            Ok(mut output) => {
                let tx_clone = tx.clone();
                tokio::spawn(async move {
                    while let Some(log) = output.next().await {
                        match log {
                            Ok(log_output) => {
                                let _ = tx_clone.send(Ok(ActionResponseStream {
                                    log: log_output.to_string(),
                                    action_id: request_body.action_id.to_string(),
                                    result: Some(ActionResult {
                                        completion: 0,
                                        exit_code: Some(0),
                                    }),
                                }));
                            }
                            Err(e) => {
                                let _ = tx_clone
                                    .send(Err(Status::cancelled(format!("Error happened {}", e))));
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
