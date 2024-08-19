use std::{borrow::Borrow, error::Error};

use proto::RegisterAgentResponse;

mod proto {
    tonic::include_proto!("registration");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://[::1]:5001";
    let mut client: proto::agent_client::AgentClient<tonic::transport::Channel>;
    let mut tries = 0;
    while tries != 5 {  // tries < 5 ???
        println!("Try number: {}", tries);
        client = match proto::agent_client::AgentClient::connect(url).await {
            Ok(mut cli) => {
                println!("Connection succeeded");
                let req = proto::Health {
                    cpu_usage: 1,
                    memory_usage: 1,
                };
                let request = tonic::Request::new(req);
                cli.register_agent(request).await.unwrap();
                cli
            }
            Err(err) => {
                if tries == 4 {
                    println!("Connection failed: {:?}", err);
                    return Ok(());
                } else {
                    println!("Connection failed: {:?}", err);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

                    tries += 1;
                    continue;
                }
            }
        };
        return Ok(());

    }

    // stop the script if the connection is not established

    Ok(())
}
