use bollard::Docker;
use clap::Parser;
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
    pub static ref dockerLocal: Docker = Docker::connect_with_socket_defaults().unwrap();
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The host URL of the scheduler
    #[clap(long, default_value = "http://[::1]:50051")]
    shost: String,

    /// The host URL you want the scheduler to contact the agent on
    #[clap(long, default_value = "http://[::1]")]
    ahost: String,

    /// The port of the agent to listen on
    #[clap(long, default_value = "9001")]
    port: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let args: Args = Args::parse();
    dockerLocal.ping().await?;
    info!("Connecting to scheduler at {}", args.shost);

    let (mut client, id) = match register_agent(&args.shost, &args.ahost, args.port).await {
        Ok(res) => {
            info!("Connection succeeded");
            res
        }
        Err(err) => {
            info!("Connection failed: {:?}", err);
            return Err(err);
        }
    };
    tokio::spawn(async move {
        let _ = report_health(&mut client, id).await;
    });

    let addr = format!("127.0.0.1:{}", args.port).parse()?;

    info!("Agent id: {}", id);
    info!("Starting server on {}", addr);

    let actions = ActionsLauncher::default();
    let server = ActionServiceServer::new(actions);
    Server::builder().add_service(server).serve(addr).await?;

    Ok(())
}
