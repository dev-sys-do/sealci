use scheduler::proto::agent_client::AgentClient;
use scheduler::proto::agent_server::AgentServer;
use scheduler::proto::controller_server::ControllerServer;
use scheduler::interfaces::agent_interface::AgentService;
use scheduler::interfaces::controller_interface::ControllerService;
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

    let req = scheduler::proto::Health { cpu_avail: 123, memory_avail: 321 };
    let request = Request::new(req);

    let response = client.register_agent(request).await?;

    // Modify the assertion based on the correct field available in your response
    assert_eq!(response.get_ref().id, 1);

    Ok(())
}
