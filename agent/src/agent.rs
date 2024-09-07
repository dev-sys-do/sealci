use std::{error::Error, thread};

use log::info;
use tonic::transport::Server;

use crate::{
    health_service::report_health, proto::action_service_server::ActionServiceServer,
    registering_service::register_agent, server::ActionsLauncher,
};

pub async fn start_agent(
    scheduler_host: &String,
    agent_host: &String,
    agent_port: u32,
) -> Result<(), Box<dyn Error>> {
    let (mut client, id) = match register_agent(scheduler_host, agent_host, agent_port).await {
        Ok(res) => {
            println!("Connection succeeded");
            res
        }
        Err(err) => {
            println!("Connection failed: {:?}", err);
            return Err(err);
        }
    };
    tokio::spawn(async move {
        let _ = report_health(&mut client, id).await;
    });

    let addr = format!("127.0.0.1:{}", agent_port).parse()?;

    info!("Agent id: {}", id);
    info!("Starting server on {}", addr);

    let service = ActionServiceServer::new(ActionsLauncher::default());
    let server_thread = thread::spawn(move || async move {
        Server::builder().add_service(service).serve(addr).await
        // .serve_with_shutdown(addr, terminate_server_receiver.map(|_| ()))
       let t =  thread::current();
       t.
    });
    info!("Server started");
    Ok(())
}
