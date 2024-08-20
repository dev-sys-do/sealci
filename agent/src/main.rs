use registering_service::register_agent;
use tonic::transport::Server;
use std::error::Error;
mod registering_service;
use lazy_static::lazy_static;
use std::sync::Mutex;

mod proto {
    tonic::include_proto!("scheduler");
}

lazy_static! {
    static ref AGENT_ID: Mutex<i32> = Mutex::new(-1);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    // "http://[::1]:5001"
    let scheduler_url = &args[1];

    match register_agent(&scheduler_url).await {
        Ok(_) => println!("Connection succeeded"),
        Err(err) => {
            println!("Connection failed: {:?}", err);
            return Ok(());
        }
    };
    println!("Agent id: {}", AGENT_ID.lock().unwrap());
    println!("Starting server...");
    Ok(())
}
