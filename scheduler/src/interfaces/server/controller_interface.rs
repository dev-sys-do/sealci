use crate::proto::controller as proto;
use proto::controller_server::Controller;
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc;
use log::{info, warn};

#[derive(Default)]
pub struct ControllerService {}

type ScheduleActionStream = ReceiverStream<Result<proto::ActionResponse, tonic::Status>>;

#[tonic::async_trait]
impl Controller for ControllerService {
    type ScheduleActionStream = ScheduleActionStream;

    async fn schedule_action(
        &self,
        request: tonic::Request<proto::ActionRequest>,
    ) -> Result<tonic::Response<Self::ScheduleActionStream>, tonic::Status> {
        let action_request = request.into_inner();

        if let Some(ref context) = action_request.context {
            if let Some(container_image) = &context.container_image {
                info!("Received ActionRequest: {:?}", action_request);
                info!("stuff: {} and {} - {} and {:?}", action_request.action_id, context.r#type, container_image, action_request.commands);  // "type" field is despetialised
            } else {
                warn!("container_image optional field not given in {:?}", context);
            }
        } else {
            warn!("Missing context field for action {:?}!!", action_request);
        }

        // Create mock data for the response stream
        let mock_action_response = proto::ActionResponse {
            action_id: 69420,
            log: String::from("Mock log message"),
            result: Some(proto::ActionResult {
                completion: proto::ActionStatus::Completed.into(),
                exit_code: Some(0),
            }),
        };

        // Use an mpsc channel to create the response stream
        let (tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            tx.send(Ok(mock_action_response)).await.unwrap();
        });

        let response_stream = ReceiverStream::new(rx);
        
        Ok(tonic::Response::new(response_stream))
    }
}
