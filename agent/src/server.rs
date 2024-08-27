use crate::action::launch_action;
use crate::proto::{action_service_server::ActionService, ActionRequest, ActionResponseStream};
use futures_util::Stream;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::{self};
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
        let (log_input, log_ouput) =
            mpsc::unbounded_channel::<Result<ActionResponseStream, Status>>();
        let mut request_body = request.into_inner();
        let context = match request_body.context {
            Some(context) => context,
            None => return Err(Status::invalid_argument("Context is missing")),
        };
        let container_image = match context.container_image {
            Some(container_image) => container_image,
            None => return Err(Status::invalid_argument("Container image is missing")),
        };
        let log_input = Arc::new(Mutex::new(log_input));
        let action_id = Arc::new(Mutex::new(request_body.action_id));
        tokio::spawn(async move {
            launch_action(
                container_image,
                &mut request_body.commands,
                log_input.clone(),
                action_id.clone(),
            )
            .await
            .map_err(|e| Status::aborted(format!("Launching error {}", e)));
        });

        let stream = UnboundedReceiverStream::new(log_ouput);
        Ok(Response::new(
            Box::pin(stream) as Self::ExecutionActionStream
        ))
    }
}
