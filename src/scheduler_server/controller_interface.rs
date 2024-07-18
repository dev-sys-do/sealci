use std::pin::Pin;
use futures::channel::mpsc::Receiver;
use futures::SinkExt;
use tonic::{Request, Response, Status};
use tonic::codegen::tokio_stream::Stream;
use tonic::transport::Server;

pub mod scheduler {
    tonic::include_proto!("scheduler");
}

use scheduler::{
    controller_server::{Controller, ControllerServer},
    ActionResponse,
    ActionRequest,
    ActionResult,
};

#[derive(Debug)]
pub struct ControllerService {}

impl ControllerService {
    pub fn get_server() -> ControllerServer<ControllerService> {
       ControllerServer::new(Self {})
    }
}

#[tonic::async_trait]
impl Controller for ControllerService {
    type ScheduleActionStream = Pin<Box<Receiver<Result<ActionResponse, Status>>>>;
    async fn schedule_action(
        &self,
        request: Request<ActionRequest>,
    ) -> Result<Response<Self::ScheduleActionStream>, Status> {
        //TODO implement schedule logic
        println!("Got a request: {:?}", request);
        let response = ActionResponse {
            action_id: "Hello".to_string(),
            log: "World".to_string(),
            result: Some(ActionResult {
                completion: 0,
                exit_code: Some(0),
            }),
        };
        let (mut tx, rx) = futures::channel::mpsc::channel(4);
        let _ = tx.send(Ok(response)).await;
        Ok(Response::new(Box::pin(rx)))
    }
}



