use registering_service::register_agent;
use std::error::Error;
mod registering_service;

mod proto {
    tonic::include_proto!("scheduler");
    tonic::include_proto!("actions");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    // "http://[::1]:5001"
    let scheduler_url = &args[1];

    let agent_id = match register_agent(&scheduler_url).await {
        Ok(agent_id) => {
            println!("Connection succeeded");
            agent_id
        }
        Err(err) => {
            println!("Connection failed: {:?}", err);
            return Ok(());
        }
    };

    println!("Agent id: {}", agent_id);
    println!("Starting server...");
    Ok(())
}
