use tonic::transport::Server;
use crate::scheduler_server::controller_interface::ControllerService;

use scheduler::controller_server::ControllerServer;
pub mod scheduler {
    tonic::include_proto!("scheduler");
}

pub async fn launch() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse().unwrap();
    let controller_interface = ControllerService::get_server();

    println!("Scheduler Server listening on {}", addr);

    //TODO add the agent interface
    Server::builder()
        .add_service(controller_interface)
        .serve(addr)
        .await?;

    Ok(())
}
