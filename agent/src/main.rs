use bollard::Docker;
use lazy_static::lazy_static;
use log::info;
use registering_service::register_agent;
use server::ActionsLauncher;
use std::error::Error;
use std::sync::Mutex;
use tonic::transport::Server;
mod action;
mod container;
mod health_service;
mod registering_service;
pub mod server;
use crate::health_service::report_health;
use crate::proto::action_service_server::ActionServiceServer;
mod proto {
    tonic::include_proto!("scheduler");
    tonic::include_proto!("actions");
}

lazy_static! {
    static ref AGENT_ID: Mutex<u32> = Mutex::new(0);
    pub static ref dockerLocal: Docker = Docker::connect_with_socket_defaults().unwrap(); //TODO: manage this error
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let args: Vec<String> = std::env::args().collect();
    // "http://[::1]:5001"
    let scheduler_url = &args[1];

    let (mut client, id) = match register_agent(scheduler_url).await {
        Ok(res) => {
            info!("Connection succeeded");
            res
        }
        Err(err) => {
            error!("Connection failed: {:?}", err);
            return Err(err);
        }
    };
    tokio::spawn(async move {
        loop {
            let _ = report_health(&mut client, id).await;
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }
    });

    info!("Agent id: {}", id);
    info!("Starting server...");
    let addr = "127.0.0.1:9001".parse()?;
    info!("Starting server on {}", addr);

    let actions = ActionsLauncher::default();
    let server = ActionServiceServer::new(actions);
    Server::builder().add_service(server).serve(addr).await?;

    Ok(())
}
