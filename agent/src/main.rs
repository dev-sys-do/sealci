use crate::registering_service::register_agent;
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

    let (mut client, id) = match register_agent(scheduler_url).await {
        Ok(res) => {
            println!("Connection succeeded");
            res
        }
        Err(err) => {
            println!("Connection failed: {:?}", err);
            return Err(err);
        }
    };

    println!("Agent id: {}", id);
    println!("Starting server...");

    Ok(())
}
