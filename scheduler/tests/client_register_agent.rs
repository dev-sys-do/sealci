use scheduler::proto::agent as agent;
use agent::agent_client::AgentClient;
use agent::agent_server::AgentServer;
use agent::{Health, RegisterAgentRequest};

use scheduler::proto::controller as controller;
use controller::controller_server::ControllerServer;

use scheduler::interfaces::server as server;
use server::agent_interface::AgentService;
use server::controller_interface::ControllerService;

use scheduler::logic as logic;
use logic::agent_logic::AgentPool;
use logic::controller_logic::ActionsQueue;

use tonic::transport::Server;
use tonic::Request;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::Duration;

#[tokio::test]
async fn test_register_agent() -> Result<(), Box<dyn Error>> {
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

    let mut client = AgentClient::connect("http://[::1]:50051").await?;

    let req = Health { cpu_avail: 123, memory_avail: 321 };
    let request = Request::new(RegisterAgentRequest { health: Some(req), hostname: Some(agent::Hostname { host: "localhost".to_string(), port: 1234 }) });

    let response = client.register_agent(request).await?;

    assert_eq!(response.get_ref().id, 1);

    Ok(())
}
