use scheduler::proto::agent as agent;
use agent::agent_client::AgentClient;
use agent::agent_server::AgentServer;
use agent::Health;

use scheduler::proto::controller as controller;
use controller::controller_server::ControllerServer;

use scheduler::interfaces::server as server;
use server::agent_interface::AgentService;
use server::controller_interface::ControllerService;

use tonic::transport::Server;
use tonic::Request;
use std::error::Error;
use tokio::time::Duration;

#[tokio::test]
async fn test_register_agent() -> Result<(), Box<dyn Error>> {
    tokio::spawn(async {
        let addr = "[::1]:50051".parse().unwrap();
        let agent = AgentService::new();
        let controller = ControllerService::default();
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
    let request = Request::new(req);

    let response = client.register_agent(request).await?;

    assert_eq!(response.get_ref().id, 1);

    Ok(())
}
