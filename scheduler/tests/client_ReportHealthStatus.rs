use proto::agent_client::AgentClient;
use tonic::transport::Channel;
use tonic::Request;

mod proto {
	tonic::include_proto!("scheduler");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  // Create a channel to connect to the gRPC server
  let channel = Channel::from_static("http://localhost:50051").connect().await?;

  // Create a gRPC client
  let mut client = AgentClient::new(channel);

  // Build one health status message
  let health_status1 = proto::HealthStatus {
    agent_id: String::from("pkn1308Dean"),  // You could also use "agent1".to_string(), but it uses the toString method that is implemented on lots of stuff. So String::from("agent1") is much clearer.
    health: Some(proto::Health {
      cpu_usage: 80,
      memory_usage: 512,
    }),
  };

  // Another.
  let health_status2 = proto::HealthStatus {
    agent_id: String::from("uoazbd91BE9ebde"),
    health: Some(proto::Health {
      cpu_usage: 60,
      memory_usage: 1024,
    }),
  };

  // Another one. Except this one doesn't send a health field! Oops.
  let health_status3 = proto::HealthStatus {
    agent_id: "pieneda2038B23".to_string(),
    health: None,
  };

  // Create a stream of health status messages
  let health_status_stream = tokio_stream::iter(vec![health_status1, health_status2, health_status3]);

  // Send the health status stream request (the 2 messages) to the server
  let response = client.report_health_status(Request::new(health_status_stream)).await?;
  // The server answers after we are done sending the stream

  println!("Response: {:?}", response.get_ref());  // remove .get_ref() to see all the fields of the response (date of reception, etc.)

  Ok(())
}

