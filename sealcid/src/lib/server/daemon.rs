use std::{net::SocketAddr, sync::Arc};

use agent::{app::App as AgentApp, config::Config as AgentConfig};
use controller::{application::App as ControllerApp, config::Config as ControllerConfig};
use monitor::{app::App as MonitorApp, config::Config as MonitorConfig};
use sealci_scheduler::{app::App as SchedulerApp, config::Config as SchedulerConfig};
use sealcid_traits::App;
use tokio::sync::RwLock;
use tonic::transport::Server;

use crate::{
    common::{error::Error, proto::daemon_server::DaemonServer},
    server::{config::GlobalConfig, service::SealedService},
};

pub struct Daemon {
    pub global_config: Arc<RwLock<GlobalConfig>>,
    pub agent: SealedService<AgentApp, AgentConfig>,
    pub controller: SealedService<ControllerApp, ControllerConfig>,
    pub monitor: SealedService<MonitorApp, MonitorConfig>,
    pub scheduler: SealedService<SchedulerApp, SchedulerConfig>,
}

impl Daemon {
    pub async fn new(global_config: GlobalConfig) -> Result<Self, Error> {
        let scheduler = SchedulerApp::configure(global_config.clone().into())
            .await
            .map_err(Error::ConfigureSchedulerError)?;
        println!("dasfasdf");
        let agent = AgentApp::configure(global_config.clone().into())
            .await
            .map_err(Error::ConfigureAgentError)?;
        println!("dasfasdf");

        let controller = ControllerApp::configure(global_config.clone().into())
            .await
            .map_err(Error::ConfigureControllerError)?;
        let monitor = MonitorApp::configure(global_config.clone().into())
            .await
            .map_err(Error::ConfigureMonitorError)?;

        Ok(Self {
            global_config: Arc::new(RwLock::new(global_config.clone())),
            agent: SealedService::new(agent, global_config.clone()),
            controller: SealedService::new(controller, global_config.clone()),
            monitor: SealedService::new(monitor, global_config.clone()),
            scheduler: SealedService::new(scheduler, global_config.clone()),
        })
    }
    pub async fn start(self, port: u32) -> Result<(), String> {
        let addr: SocketAddr = format!("0.0.0.0:{}", port)
            .parse()
            .map_err(|e| format!("Failed to parse socket address: {}", e))?;

        let daemon_server = DaemonServer::new(self);
        Server::builder()
            .add_service(daemon_server)
            .serve(addr)
            .await
            .map_err(|e| format!("gRPC server error: {}", e))?;

        Ok(())
    }
}
