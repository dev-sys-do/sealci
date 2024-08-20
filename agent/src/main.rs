use lazy_static::lazy_static;
use registering_service::register_agent;
use server::ActionsLauncher;
use std::error::Error;
use std::sync::Mutex;
use tonic::transport::Server;
use crate::actions::action_service_server::ActionServiceServer;
pub mod actions;
mod action;
mod container;
mod registering_service;
mod server;
mod proto {
    tonic::include_proto!("scheduler");
    tonic::include_proto!("actions");
}

lazy_static! {
    static ref AGENT_ID: Mutex<u32> = Mutex::new(0);
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
    
    report_health(&mut client, id).await?;

    println!("Agent id: {}", id);
    println!("Starting server...");
    let addr = "127.0.0.1:9001".parse()?;
    let actions = ActionsLauncher::default();

    Server::builder()
        .add_service(ActionServiceServer::new(actions))
        .serve(addr)
        .await?;
    println!("Server started on {}", addr);

    Ok(())
}
