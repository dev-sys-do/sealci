use scheduler::proto::agent_client::AgentClient;
use scheduler::proto::agent_server::AgentServer;
use scheduler::proto::controller_server::ControllerServer;
use scheduler::interfaces::agent_interface::AgentService;
use scheduler::interfaces::controller_interface::ControllerService;
use tonic::transport::Server;
use tonic::transport::Channel;
use tonic::Request;
use tokio_stream::iter;
use std::error::Error;
use tokio::time::Duration;

#[tokio::test]
async fn test_report_health_status() -> Result<(), Box<dyn Error>> {
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

    let channel = Channel::from_static("http://[::1]:50051").connect().await?;
    let mut client = AgentClient::new(channel);

    let health_status1 = scheduler::proto::HealthStatus {
        agent_id: 1,
        health: Some(scheduler::proto::Health { cpu_usage: 80, memory_usage: 512 }),
    };

    let health_status2 = scheduler::proto::HealthStatus {
        agent_id: 2,
        health: Some(scheduler::proto::Health { cpu_usage: 60, memory_usage: 1024 }),
    };

    let health_status3 = scheduler::proto::HealthStatus {
        agent_id: 3,
        health: None,
    };

    let health_status_stream = iter(vec![health_status1, health_status2, health_status3]);

    let response = client.report_health_status(Request::new(health_status_stream)).await?;

    // Modify the assertion based on the correct field available in your response
    assert_eq!(response.get_ref(), &scheduler::proto::Empty {});

    Ok(())
}
