// this is a mock server to avoid getting errors

use scheduler::{
    controller_server::{Controller, ControllerServer},
    ActionRequest, ActionResponse,
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

pub mod scheduler {
    tonic::include_proto!("scheduler");
}

#[derive(Debug)]
pub struct MockSchedulerService {}

#[tonic::async_trait]
impl Controller for MockSchedulerService {
    type ScheduleActionStream = ReceiverStream<Result<ActionResponse, Status>>;

    async fn schedule_action(
        &self,
        request: Request<ActionRequest>,
    ) -> Result<Response<Self::ScheduleActionStream>, Status> {
        let (tx, rx) = mpsc::channel(1);

        tx.send(Ok(ActionResponse {
            action_id: "yes".to_string(),
            log: "INFO: scheduled".to_string(),
            result: Some(scheduler::ActionResult {
                completion: 1,
                exit_code: Some(1),
            }),
        }))
        .await
        .expect("should be sent");

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    let scheduler_service = MockSchedulerService {};

    Server::builder()
        .add_service(ControllerServer::new(scheduler_service))
        .serve(addr)
        .await?;
    Ok(())
}
