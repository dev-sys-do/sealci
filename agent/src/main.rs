use registering_service::register_agent;
use std::error::Error;
mod registering_service;
mod proto {
    tonic::include_proto!("scheduler");
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
    println!("Starting server...");
    Ok(())
}
