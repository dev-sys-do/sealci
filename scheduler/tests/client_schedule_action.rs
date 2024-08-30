//use scheduler::proto::agent as agent;
use scheduler::proto::scheduler as agent;
use agent::agent_server::AgentServer;

//use scheduler::proto::controller as controller;
use scheduler::proto::scheduler as controller;
use controller::controller_server::ControllerServer;
use controller::{controller_client::ControllerClient, ActionRequest, ExecutionContext, RunnerType};

use scheduler::interfaces::server as server;
use server::agent_interface::AgentService;
use server::controller_interface::ControllerService;

use scheduler::logic as logic;
use logic::agent_logic::AgentPool;
use logic::controller_logic::ActionsQueue;

use tonic::transport::Server;
use tonic::transport::Channel;
use tonic::Request;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::Duration;

#[tokio::test]
async fn test_schedule_action() -> Result<(), Box<dyn Error>> {
    tokio::spawn(async {
        let addr = "[::1]:50051".parse().unwrap();
        let agent_pool = Arc::new(Mutex::new(AgentPool::new()));
        let action_queue = Arc::new(Mutex::new(ActionsQueue::new()));
        let agent = AgentService::new(agent_pool.clone());
        let controller = ControllerService::new(action_queue.clone(), agent_pool.clone());
        let service = tonic_reflection::server::Builder::configure()
            .register_encoded_file_descriptor_set(scheduler::proto::FILE_DESCRIPTOR_SET)
            .build()
            .unwrap();

        Server::builder()
            .add_service(service)
            .add_service(AgentServer::new(agent))
            .add_service(ControllerServer::new(controller))
            .serve(addr)
            .await
            .unwrap();
    });

    tokio::time::sleep(Duration::from_secs(1)).await;

    let channel = Channel::from_static("http://[::1]:50051").connect().await?;
    let mut client = ControllerClient::new(channel);

    let request = Request::new(ActionRequest {
        action_id: 69420,
        context: Some(ExecutionContext {
            r#type: RunnerType::Docker.into(),
            container_image: Some("test_image".to_string()),
        }),
        commands: vec![String::from("echo 'Salut les zagennntss!!!'"), String::from("shutdown now")],
    });

    let mut response_stream = client.schedule_action(request).await?.into_inner();

    while let Some(response) = response_stream.message().await? {
        assert_eq!(response.action_id, 69420);
    }

    Ok(())
}
