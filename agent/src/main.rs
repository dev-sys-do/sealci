use agent::start_agent;
use bollard::Docker;
use clap::Parser;
use lazy_static::lazy_static;
use std::error::Error;
use std::sync::Mutex;
use tokio::sync::oneshot;
mod action;
mod agent;
mod container;
mod health_service;
mod registering_service;
pub mod server;
mod proto {
    tonic::include_proto!("scheduler");
    tonic::include_proto!("actions");
}

lazy_static! {
    static ref AGENT_ID: Mutex<u32> = Mutex::new(0);
    static ref test: Mutex<u32> = Mutex::new(1);
    pub static ref dockerLocal: Docker = Docker::connect_with_socket_defaults().unwrap(); //TODO: manage this error
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
    let (terminate_server_sender, terminate_server_receiver) = oneshot::channel::<()>();
    println!("Connecting to scheduler at {}", args.shost);
    *test.lock().unwrap() += 1;
    println!("Result: {}", *test.lock().unwrap());


    // let _ = start_agent(&args.shost, &args.ahost, args.port);
    Ok(())
}
